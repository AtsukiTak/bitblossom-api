pub mod feeder;
pub mod insta_api;

pub use self::feeder::InstaFeeder;
pub use self::insta_api::{InstaApi, InstaPartialPostResponse, InstaPostResponse};

use images::{FetchedImage, Size};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct InstaPostId(pub String);

impl ::std::fmt::Display for InstaPostId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct InstaPost<S> {
    pub post_id: InstaPostId,
    pub user_name: String,
    pub image: FetchedImage<S>,
}

impl<S: Size> InstaPost<S> {
    pub fn new(id: InstaPostId, user_name: String, img: FetchedImage<S>) -> InstaPost<S> {
        InstaPost {
            post_id: id,
            user_name: user_name,
            image: img,
        }
    }
}
