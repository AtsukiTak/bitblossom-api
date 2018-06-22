use std::sync::Arc;
use futures::{Future, IntoFuture, Stream, stream::{iter_ok, repeat}};

use images::{ImageFetcher, size::{MultipleOf, Size, SmallerThan}};
use insta::{InstaApi, InstaPost};
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
        hashtags: Arc<Vec<String>>,
    ) -> impl Stream<Item = InstaPost<SS>, Error = Error>
    where
        S: Size + MultipleOf<SS>,
        SS: Size + SmallerThan<S>,
    {
        let mut hashtags_cycle = HashtagCycle::new(hashtags);
        let insta_api = self.insta_api.clone();
        let insta_api2 = self.insta_api.clone();
        let db = self.db.clone();
        let image_fetcher = self.image_fetcher.clone();

        repeat::<_, Error>(0)
            .and_then(move |_| {
                let hashtag = hashtags_cycle.next();
                debug!("Search instagram by hashtag : {}", hashtag);
                insta_api.get_posts_by_hashtag(hashtag)
            })
            .map(|res| {
                let (tag, posts) = (res.hashtag, res.posts);
                trace!("Get posts : {:?}", posts);
                iter_ok::<_, Error>(posts).map(move |post| (tag.clone(), post))
            })
            .flatten()
            .filter(move |(_, p)| !db.contains_post(&p.id))
            .and_then(move |(hashtag, p)| {
                debug!("New post : {:?}", p);
                insta_api2.get_post_by_id(&p.id).map(|post| (hashtag, post))
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

struct HashtagCycle {
    hashtags: Arc<Vec<String>>,
    next_idx: usize,
}

impl HashtagCycle {
    pub fn new(hashtags: Arc<Vec<String>>) -> HashtagCycle {
        HashtagCycle {
            hashtags: hashtags,
            next_idx: 0,
        }
    }

    pub fn next(&mut self) -> &str {
        let hashtag = &self.hashtags[self.next_idx];
        self.next_idx += 1;
        if !(self.next_idx < self.hashtags.len()) {
            self.next_idx = 0;
        }
        hashtag
    }
}
