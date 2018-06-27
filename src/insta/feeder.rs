use std::sync::Arc;
use futures::{Future, IntoFuture, Stream, stream::iter_ok};

use images::{ImageFetcher, size::Size};
use insta::InstaApi;
use post::{HashtagList, InstaPost};
use db::Mongodb;
use error::Error;

pub struct InstaFeeder {
    insta_api: Arc<InstaApi>,
    image_fetcher: Arc<ImageFetcher>,
    db: Mongodb,
}

impl InstaFeeder {
    pub fn new(db: Mongodb) -> InstaFeeder {
        InstaFeeder {
            insta_api: Arc::new(InstaApi::new()),
            image_fetcher: Arc::new(ImageFetcher::new()),
            db: db,
        }
    }

    pub fn get_bunch_of_posts<SS: Size>(
        &self,
        hashtags: &HashtagList,
    ) -> impl Stream<Item = InstaPost<SS>, Error = Error> {
        let insta_api = self.insta_api.clone();
        let insta_api2 = self.insta_api.clone();
        let image_fetcher = self.image_fetcher.clone();
        let db = self.db.clone();
        let db2 = self.db.clone();

        iter_ok::<_, Error>(hashtags.iter())
            .map(move |hashtag| insta_api.get_bunch_posts_by_hashtag(&hashtag))
            .flatten()
            .filter(move |(_, p)| !db.contains_insta_post(&p.id))
            .and_then(move |(hashtag, p)| {
                info!("New post : {:?}", p);
                insta_api2.get_post_by_id(&p.id).map(|post| (hashtag, post))
            })
            .and_then(move |(hashtag, p)| {
                let db = db2.clone();
                image_fetcher
                    .fetch_image::<SS>(p.image_url.as_str())
                    .into_future()
                    .and_then(|img_fut| img_fut)
                    .map(move |img| InstaPost::new(p.id, img, p.user_name, hashtag))
                    .inspect(move |post| db.insert_one_insta_post(post))
            })
    }

    pub fn get_update_posts<SS: Size>(
        &self,
        hashtags: &HashtagList,
    ) -> impl Stream<Item = InstaPost<SS>, Error = Error> {
        let insta_api = self.insta_api.clone();
        let insta_api2 = self.insta_api.clone();
        let image_fetcher = self.image_fetcher.clone();
        let db = self.db.clone();
        let db2 = self.db.clone();

        iter_ok::<_, Error>(hashtags.iter().cycle())
            .map(move |hashtag| insta_api.get_posts_by_hashtag(&hashtag))
            .flatten()
            .filter(move |(_, p)| !db.contains_insta_post(&p.id))
            .and_then(move |(hashtag, p)| {
                info!("New post : {:?}", p);
                insta_api2.get_post_by_id(&p.id).map(|post| (hashtag, post))
            })
            .and_then(move |(hashtag, p)| {
                let db = db2.clone();
                image_fetcher
                    .fetch_image::<SS>(p.image_url.as_str())
                    .into_future()
                    .and_then(|img_fut| img_fut)
                    .map(move |img| InstaPost::new(p.id, img, p.user_name, hashtag))
                    .inspect(move |post| db.insert_one_insta_post(post))
            })
    }
}
