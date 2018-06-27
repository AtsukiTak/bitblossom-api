use std::sync::Arc;
use futures::{Future, IntoFuture, Stream, stream::iter_ok};

use images::{ImageFetcher, size::{MultipleOf, Size, SmallerThan}};
use insta::{HashtagList, InstaApi, InstaPost};
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

    pub fn run<S, SS>(
        &self,
        hashtags: &HashtagList,
    ) -> impl Stream<Item = InstaPost<SS>, Error = Error>
    where
        S: Size + MultipleOf<SS>,
        SS: Size + SmallerThan<S>,
    {
        let insta_api = self.insta_api.clone();
        let init_posts = iter_ok::<_, Error>(hashtags.iter())
            .map(move |hashtag| insta_api.get_bunch_posts_by_hashtag(&hashtag).take(1000))
            .flatten();

        let insta_api = self.insta_api.clone();
        let update_posts = iter_ok::<_, Error>(hashtags.iter().cycle())
            .map(move |hashtag| insta_api.get_posts_by_hashtag(&hashtag))
            .flatten();

        let post_stream = init_posts.chain(update_posts);
        let db = self.db.clone();
        let image_fetcher = self.image_fetcher.clone();
        let insta_api = self.insta_api.clone();

        post_stream
            .filter(move |(_, p)| !db.contains_post(&p.id))
            .and_then(move |(hashtag, p)| {
                info!("New post : {:?}", p);
                insta_api.get_post_by_id(&p.id).map(|post| (hashtag, post))
            })
            .and_then(move |(hashtag, p)| {
                image_fetcher
                    .fetch_image::<SS>(p.image_url.as_str())
                    .into_future()
                    .and_then(|img_fut| img_fut)
                    .map(move |img| InstaPost::new(p.id, p.user_name, img, hashtag))
            })
    }
}
