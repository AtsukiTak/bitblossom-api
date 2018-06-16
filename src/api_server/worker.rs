use std::{collections::HashSet, sync::{Arc, Mutex}, time::Duration};
use futures::{Future, Stream};

use mosaic::{GrayscalePositionFinder, SharedMosaicArt};
use insta::InstaFeeder;
use images::{Image, size::{MultipleOf, Size, SmallerThan}};

const REFRESH_INTERVAL: u64 = 3;

pub struct Worker {
    block_users: Arc<Mutex<HashSet<String>>>,
    insta_feeder: Arc<InstaFeeder>,
}

impl Worker {
    pub fn new(insta_api_server_host: String) -> Worker {
        Worker {
            block_users: Arc::new(Mutex::new(HashSet::new())),
            insta_feeder: Arc::new(InstaFeeder::new(insta_api_server_host)),
        }
    }

    pub fn add_block_user(&self, user_name: String) {
        self.block_users.lock().unwrap().insert(user_name);
    }

    pub fn run<S, SS, I>(&self, hashtags: Vec<String>, origin_image: I) -> SharedMosaicArt<S, SS>
    where
        S: Size + MultipleOf<SS>,
        SS: Size + SmallerThan<S>,
        I: Image<Size = S>,
    {
        let insta_feeder = self.insta_feeder.clone();
        let block_users = self.block_users.clone();
        let mosaic_art = SharedMosaicArt::new(hashtags.clone());
        let mosaic_art2 = mosaic_art.clone();
        let mosaic_art3 = mosaic_art.clone();
        let mut position_finder = GrayscalePositionFinder::new(origin_image);

        // The reason why I spawn a new thread is because `tokio::timer` does not work well
        // under multi-threaded environment.
        // https://github.com/tokio-rs/tokio/issues/305
        ::std::thread::spawn(move || {
            let f = insta_feeder
                .run(
                    hashtags,
                    mosaic_art,
                    block_users,
                    Duration::new(REFRESH_INTERVAL, 0),
                )
                .for_each(move |post| {
                    let pos = position_finder.find_position(&post.image);
                    mosaic_art2.apply_post(post, pos);
                    Ok(())
                })
                .map_err(|e| error!("{:?}", e));

            ::tokio::run(f)
        });

        mosaic_art3
    }
}
