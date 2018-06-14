pub mod models;

pub use self::models::{InstaPost, NewInstaPost};

#[derive(Clone)]
pub struct DB(());

impl DB {
    pub fn new(_db_url: &str) -> DB {
        unimplemented!();
    }

    pub fn insert_new_post(&self, _new_post: NewInstaPost) {
        unimplemented!();
    }

    pub fn get_posts_by_hashtag(&self, _hashtag: &str) -> Vec<InstaPost> {
        unimplemented!();
    }
}
