use std::sync::Arc;
use futures::{Future, Stream};

use mosaic::{DistanceCalculator, SharedMosaicArt};
use insta::InstaFeeder;
use images::{SizedImage, size::{MultipleOf, Size, SmallerThan}};
use db::Mongodb;

pub struct Worker {
    insta_feeder: Arc<InstaFeeder>,
    db: Mongodb,
}

impl Worker {
    pub fn new(db: Mongodb) -> Worker {
        Worker {
            insta_feeder: Arc::new(InstaFeeder::new(db.clone())),
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
        let mosaic_art = SharedMosaicArt::new(hashtags.clone(), origin_image.clone());
        let mosaic_art2 = mosaic_art.clone();
        let distance_calc = DistanceCalculator::new(origin_image);
        let mongodb = self.db.clone();

        // Initialize mosaic art
        info!("Initialize mosaic art with database data");
        let mut init_posts = self.db.find_posts_by_hashtags(hashtags.as_slice(), 1000);
        for post in init_posts.drain(..) {
            let piece = distance_calc.calc_post(post);
            mosaic_art.apply_piece(piece);
        }

        // The reason why I spawn a new thread is because `tokio::timer` does not work well
        // under multi-threaded environment.
        // https://github.com/tokio-rs/tokio/issues/305
        ::std::thread::spawn(move || {
            let f = insta_feeder
                .run(hashtags)
                .for_each(move |post| {
                    mongodb.insert_one_post(&post);

                    let piece = distance_calc.calc_post(post);
                    mosaic_art.apply_piece(piece);
                    Ok(())
                })
                .map_err(|e| error!("{:?}", e));

            ::tokio::run(f)
        });

        mosaic_art2
    }
}
