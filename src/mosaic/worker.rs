use std::{collections::HashSet, sync::{Arc, Mutex}, time::{Duration, Instant}};
use tokio::{executor::thread_pool::ThreadPool, timer::Interval};
use futures::{Future, IntoFuture, Stream, stream::iter_ok};

use mosaic::{GrayscalePositionFinder, InstaApi, InstaPartialPost, MosaicArt};
use images::{Image, ImageFetcher, size::{MultipleOf, Size, SmallerThan}};
use error::Error;

const REFRESH_INTERVAL: u64 = 3;

pub struct Worker {
    block_users: Arc<Mutex<HashSet<String>>>,
    insta_api: Arc<InstaApi>,
    image_fetcher: Arc<ImageFetcher>,
    thread_pool: ThreadPool,
}

impl Worker {
    pub fn new(insta_api_server_host: String) -> Worker {
        Worker {
            block_users: Arc::new(Mutex::new(HashSet::new())),
            insta_api: Arc::new(InstaApi::new(insta_api_server_host)),
            image_fetcher: Arc::new(ImageFetcher::new()),
            thread_pool: ThreadPool::new(),
        }
    }

    pub fn add_block_user(&self, user_name: String) {
        self.block_users.lock().unwrap().insert(user_name);
    }

    pub fn run<S, SS, I>(&self, hashtags: Vec<String>, origin_image: I) -> Arc<Mutex<MosaicArt<S>>>
    where
        S: Size + MultipleOf<SS>,
        SS: Size + SmallerThan<S>,
        I: Image<Size = S>,
    {
        let position_finder = GrayscalePositionFinder::new(origin_image);
        let mosaic_art = Arc::new(Mutex::new(MosaicArt::new(hashtags.clone())));
        let mosaic_art2 = mosaic_art.clone();
        let mosaic_art3 = mosaic_art.clone();
        let mut hashtags_cycle = HashtagCycle::new(hashtags);
        let insta_api = self.insta_api.clone();
        let insta_api2 = self.insta_api.clone();
        let block_users = self.block_users.clone();
        let image_fetcher = self.image_fetcher.clone();

        let f = Interval::new(Instant::now(), Duration::new(REFRESH_INTERVAL, 0))
            .map_err(|e| Error::from(e))
            .map(move |_| hashtags_cycle.next().unwrap())
            .and_then(move |hashtag| insta_api.get_posts_by_hashtag(hashtag.as_str()))
            .map(|posts| iter_ok::<_, Error>(posts))
            .flatten()
            .filter(move |p| !mosaic_art.lock().unwrap().has_post(&p.post_id))
            .and_then(move |p| insta_api2.get_post_by_id(&p.post_id))
            .filter(move |p| !block_users.lock().unwrap().contains(&p.user_name))
            .and_then(move |p| {
                image_fetcher
                    .fetch_image::<SS>(p.image_url.as_str())
                    .into_future()
                    .and_then(|img_fut| img_fut)
                    .map(move |img| (img, p))
            })
            .for_each(move |(piece_img, p)| {
                let pos = position_finder.find_position(&piece_img);
                mosaic_art2
                    .lock()
                    .unwrap()
                    .apply_new_image(piece_img, p, pos);
                Ok(())
            })
            .map_err(|e| println!("{:?}", e));

        self.thread_pool.spawn(f);

        mosaic_art3
    }
}

struct HashtagCycle {
    hashtags: Vec<String>,
    next_idx: usize,
}

impl HashtagCycle {
    pub fn new(hashtags: Vec<String>) -> HashtagCycle {
        HashtagCycle {
            hashtags: hashtags,
            next_idx: 0,
        }
    }
}

impl Iterator for HashtagCycle {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let hashtag = self.hashtags[self.next_idx].clone();
        self.next_idx += 1;
        if !(self.next_idx < self.hashtags.len()) {
            self.next_idx = 0;
        }
        Some(hashtag)
    }
}
