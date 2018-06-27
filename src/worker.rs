use std::{mem, collections::HashMap, hash::{BuildHasher, Hasher}, ops::DerefMut,
          sync::{Arc, Mutex}};
use rand::{FromEntropy, RngCore, prng::XorShiftRng};
use futures::{Future, Stream, sync::oneshot::{self, Sender}};

use insta::InstaFeeder;
use db::Mongodb;
use images::size::{MultipleOf, Size, SmallerThan};
use mosaic::{MosaicArt, MosaicArtGenerator};

pub struct WorkerManager {
    insta_feeder: Arc<InstaFeeder>,
    db: Mongodb,
    container: WorkerContainer,
}

impl WorkerManager {
    pub fn new(db: Mongodb) -> WorkerManager {
        let feeder = Arc::new(InstaFeeder::new(db.clone()));
        WorkerManager {
            insta_feeder: feeder,
            db: db,
            container: WorkerContainer::new(),
        }
    }

    pub fn start_worker<S, SS>(&mut self, generator: MosaicArtGenerator<S, SS>) -> WorkerId
    where
        S: Size + MultipleOf<SS>,
        SS: Size + SmallerThan<S>,
    {
        let worker = Worker::start(self.insta_feeder.clone(), self.db.clone(), generator);
        self.container.add(worker)
    }

    pub fn get_worker(&self, id: WorkerId) -> Option<&Worker> {
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

pub struct Worker {
    current_art: Arc<Mutex<Arc<MosaicArt>>>,
    shutdown_tx: Sender<()>,
}

impl Worker {
    fn start<S, SS>(
        insta_feeder: Arc<InstaFeeder>,
        db: Mongodb,
        mut generator: MosaicArtGenerator<S, SS>,
    ) -> Worker
    where
        S: Size + MultipleOf<SS>,
        SS: Size + SmallerThan<S>,
    {
        // Create some thread sahred items
        let art = Arc::new(Mutex::new(Arc::new(generator.current_art())));
        let art2 = art.clone();
        let (tx, rx) = oneshot::channel();

        ::std::thread::spawn(move || {
            let post_stream = insta_feeder.run(&generator.hashtags());
            let running = post_stream.for_each(move |post| {
                let new_art = generator.apply_post(post);
                *art2.lock().unwrap().deref_mut() = Arc::new(new_art);
                Ok(())
            });
            let shutdown = running.select2(rx).map_err(|_e| ()).map(|_| ());
            ::tokio::run(shutdown);
        });

        Worker {
            current_art: art,
            shutdown_tx: tx,
        }
    }

    pub fn get_art(&self) -> Arc<MosaicArt> {
        self.current_art.lock().unwrap().clone()
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

struct WorkerContainer {
    container: HashMap<u64, Worker, NothingU64HasherBuilder>,
    id_gen: IdGenerator,
}

impl WorkerContainer {
    fn new() -> WorkerContainer {
        WorkerContainer {
            container: HashMap::with_hasher(NothingU64HasherBuilder),
            id_gen: IdGenerator::new(),
        }
    }

    fn add(&mut self, worker: Worker) -> WorkerId {
        let id = self.id_gen.next_id();
        self.container.insert(id, worker);
        WorkerId(id)
    }

    fn get(&self, id: WorkerId) -> Option<&Worker> {
        self.container.get(&id.0)
    }

    fn take(&mut self, id: WorkerId) -> Option<Worker> {
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
