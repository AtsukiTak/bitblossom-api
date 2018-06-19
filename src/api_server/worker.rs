use std::{sync::Arc, time::Duration};
use futures::{Future, Stream};

use mosaic::{GrayscalePositionFinder, SharedMosaicArt};
use insta::InstaFeeder;
use images::{SizedImage, size::{MultipleOf, Size, SmallerThan}};
use db::Mongodb;

const REFRESH_INTERVAL: u64 = 3;

pub struct Worker {
    insta_feeder: Arc<InstaFeeder>,
    db: Mongodb,
}

impl Worker {
    pub fn new(insta_api_server_host: String, db: Mongodb) -> Worker {
        Worker {
            insta_feeder: Arc::new(InstaFeeder::new(insta_api_server_host, db.clone())),
            db: db,
        }
    }

    pub fn run<S, SS>(
        &self,
        hashtags: Vec<String>,
        origin_image: SizedImage<S>,
    ) -> SharedMosaicArt<S, SS>
    where
        S: Size + MultipleOf<SS>,
        SS: Size + SmallerThan<S>,
    {
        let insta_feeder = self.insta_feeder.clone();
        let mosaic_art = SharedMosaicArt::new(hashtags.clone());
        let mosaic_art2 = mosaic_art.clone();
        let mut position_finder = GrayscalePositionFinder::new(origin_image);
        let mongodb = self.db.clone();

        // Initialize mosaic art
        let mut init_posts = self.db.find_posts_by_hashtags(hashtags.as_slice(), 1000);
        for post in init_posts.drain(..) {
            let pos = position_finder.find_position(post.get_image());
            mosaic_art.apply_post(post, pos);
        }

        // The reason why I spawn a new thread is because `tokio::timer` does not work well
        // under multi-threaded environment.
        // https://github.com/tokio-rs/tokio/issues/305
        ::std::thread::spawn(move || {
            let f = insta_feeder
                .run(hashtags, Duration::new(REFRESH_INTERVAL, 0))
                .for_each(move |post| {
                    mongodb.insert_one_post(&post);

                    let pos = position_finder.find_position(post.get_image());
                    mosaic_art.apply_post(post, pos);
                    Ok(())
                })
                .map_err(|e| error!("{:?}", e));

            ::tokio::run(f)
        });

        mosaic_art2
    }
}
