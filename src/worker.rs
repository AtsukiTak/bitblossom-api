use std::{mem, collections::HashMap, hash::{BuildHasher, Hasher}, ops::DerefMut,
          sync::{Arc, Mutex}};
use rand::{FromEntropy, RngCore, prng::XorShiftRng};
use futures::{Future, Stream, sync::{mpsc::{self, UnboundedSender}, oneshot::{self, Sender}}};

use insta::InstaFeeder;
use db::Mongodb;
use post::{BluummPost, GenericPost};
use images::size::{MultipleOf, Size, SmallerThan};
use mosaic::{MosaicArt, MosaicArtGenerator};
use error::Error;

pub struct WorkerManager<S, SS> {
    insta_feeder: Arc<InstaFeeder>,
    db: Mongodb,
    container: WorkerContainer<S, SS>,
}

impl<S, SS> WorkerManager<S, SS>
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    pub fn new(db: Mongodb) -> WorkerManager<S, SS> {
        let feeder = Arc::new(InstaFeeder::new(db.clone()));
        WorkerManager {
            insta_feeder: feeder,
            db: db,
            container: WorkerContainer::new(),
        }
    }

    pub fn start_worker(&mut self, generator: MosaicArtGenerator<S, SS>) -> WorkerId {
        let worker = Worker::start(self.insta_feeder.clone(), self.db.clone(), generator);
        self.container.add(worker)
    }

    pub fn get_worker(&self, id: WorkerId) -> Option<&Worker<S, SS>> {
        self.container.get(id)
    }

    pub fn get_worker_ids<'a>(&'a self) -> impl Iterator<Item = WorkerId> + 'a {
        self.container.ids()
    }

    pub fn stop_worker(&mut self, id: WorkerId) -> bool {
        if let Some(worker) = self.container.take(id) {
            worker.stop();
            true
        } else {
            false
        }
    }
}

const FILL_PROCESS_BOOST: usize = 4;

pub struct Worker<S, SS> {
    current_art: Arc<Mutex<Arc<MosaicArt<S, SS>>>>,
    bluumm_post_tx: UnboundedSender<BluummPost<SS>>,
    shutdown_tx: Sender<()>,
}

impl<S, SS> Worker<S, SS>
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    fn start(
        insta_feeder: Arc<InstaFeeder>,
        db: Mongodb,
        mut generator: MosaicArtGenerator<S, SS>,
    ) -> Worker<S, SS> {
        // Initialize
        info!("Initializing mosaic art...");
        let piece_n = ((S::WIDTH * S::HEIGHT) / (SS::WIDTH * SS::HEIGHT)) as i64;
        let mut init_insta_posts = db.find_insta_posts_by_hashtags(&generator.hashtags(), piece_n);
        let mut init_bluumm_posts =
            db.find_bluumm_posts_by_hashtags(&generator.hashtags(), piece_n);
        let init_insta_posts_iter = init_insta_posts
            .drain(..)
            .map(|p| GenericPost::InstaPost(p));
        let init_bluumm_posts_iter = init_bluumm_posts
            .drain(..)
            .map(|p| GenericPost::BluummPost(p));

        let init_posts = init_bluumm_posts_iter // BluummPost have priority over InstaPost
            .chain(init_insta_posts_iter)
            .take(piece_n as usize);
        for post in init_posts {
            let _applied = generator.apply_post(post);
        }
        info!("Initialized!!");

        // Create some thread sahred items
        let art = Arc::new(Mutex::new(Arc::new(generator.current_art())));
        let art2 = art.clone();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (bluumm_post_tx, bluumm_post_rx) = mpsc::unbounded();
        let hashtags = generator.hashtags();
        let generator = Arc::new(Mutex::new(generator));
        let generator2 = generator.clone();

        ::std::thread::spawn(move || {
            let post_stream = {
                let insta_post_stream = {
                    let init_insta_post_stream = insta_feeder
                        .get_bunch_of_posts(&hashtags)
                        .take_while(move |_| {
                            Ok::<_, Error>(!generator.lock().unwrap().has_enough_pieces())
                        });
                    let update_insta_post_stream = insta_feeder.get_update_posts(&hashtags);
                    init_insta_post_stream
                        .chain(update_insta_post_stream)
                        .map(|p| GenericPost::InstaPost(p))
                };
                let bluumm_post_stream = bluumm_post_rx
                    .map(|p| GenericPost::BluummPost(p))
                    .then(|res| Ok::<_, Error>(res.unwrap()));
                insta_post_stream.select(bluumm_post_stream)
            };

            let running = post_stream.for_each(move |post| {
                let mut generator = generator2.lock().unwrap();
                // Copy a new arrived post if art does not have enough pieces.
                let boost = generator.has_enough_pieces() as usize * FILL_PROCESS_BOOST;
                for _ in 0..boost {
                    let _art = generator.apply_post(post.clone());
                }

                // Always apply at least one time.
                let art = generator.apply_post(post);
                *art2.lock().unwrap().deref_mut() = Arc::new(art); // replace old art with new art
                Ok(())
            });
            let shutdown = running.select2(shutdown_rx).map_err(|_e| ()).map(|_| ());
            ::tokio::run(shutdown);
        });

        Worker {
            current_art: art,
            bluumm_post_tx: bluumm_post_tx,
            shutdown_tx: shutdown_tx,
        }
    }

    pub fn get_art(&self) -> Arc<MosaicArt<S, SS>> {
        self.current_art.lock().unwrap().clone()
    }

    pub fn add_bluumm_post(&self, post: BluummPost<SS>) {
        self.bluumm_post_tx.unbounded_send(post).unwrap();
    }

    fn stop(self) {
        let _ = self.shutdown_tx.send(());
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WorkerId(pub u64);

impl ::std::fmt::Display for WorkerId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

struct WorkerContainer<S, SS> {
    container: HashMap<u64, Worker<S, SS>, NothingU64HasherBuilder>,
    id_gen: IdGenerator,
}

impl<S, SS> WorkerContainer<S, SS> {
    fn new() -> WorkerContainer<S, SS> {
        WorkerContainer {
            container: HashMap::with_hasher(NothingU64HasherBuilder),
            id_gen: IdGenerator::new(),
        }
    }

    fn add(&mut self, worker: Worker<S, SS>) -> WorkerId {
        let id = self.id_gen.next_id();
        self.container.insert(id, worker);
        WorkerId(id)
    }

    fn get(&self, id: WorkerId) -> Option<&Worker<S, SS>> {
        self.container.get(&id.0)
    }

    fn take(&mut self, id: WorkerId) -> Option<Worker<S, SS>> {
        self.container.remove(&id.0)
    }

    fn ids<'a>(&'a self) -> impl Iterator<Item = WorkerId> + 'a {
        self.container.keys().map(|k| WorkerId(*k))
    }
}

struct IdGenerator {
    rng: XorShiftRng,
}

impl IdGenerator {
    fn new() -> IdGenerator {
        IdGenerator {
            rng: XorShiftRng::from_entropy(),
        }
    }

    fn next_id(&mut self) -> u64 {
        self.rng.next_u64()
    }
}

struct NothingU64HasherBuilder;

impl BuildHasher for NothingU64HasherBuilder {
    type Hasher = NothingU64Hasher;
    fn build_hasher(&self) -> Self::Hasher {
        NothingU64Hasher { n: 0 }
    }
}

struct NothingU64Hasher {
    n: u64,
}

impl Hasher for NothingU64Hasher {
    fn finish(&self) -> u64 {
        self.n
    }

    fn write(&mut self, bytes: &[u8]) {
        self.n = match bytes.len() {
            0 => 0,
            1 => bytes[0] as u64,
            2 => unsafe {
                let b = *(bytes as *const _ as *const [u8; 2]);
                mem::transmute::<[u8; 2], u16>(b) as u64
            },
            4 => unsafe {
                let b = *(bytes as *const _ as *const [u8; 4]);
                mem::transmute::<[u8; 4], u32>(b) as u64
            },
            8 => unsafe {
                let b = *(bytes as *const _ as *const [u8; 8]);
                mem::transmute::<[u8; 8], u64>(b) as u64
            },
            i => panic!("Unexpected hasher input : {}", i),
        };
    }

    fn write_u64(&mut self, i: u64) {
        self.n = i;
    }
}
