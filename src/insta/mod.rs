pub mod feeder;
pub mod api;

pub use self::feeder::InstaFeeder;
pub use self::api::{InstaApi, InstaHashtagResponse, InstaPostResponse};

use std::string::ToString;
use hyper::Uri;

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
    post_id: InstaPostId,
    username: String,
    image: FetchedImage<S>,
    hashtag: String,
}

impl<S: Size> InstaPost<S> {
    pub fn new(
        id: InstaPostId,
        username: String,
        img: FetchedImage<S>,
        hashtag: String,
    ) -> InstaPost<S> {
        InstaPost {
            post_id: id,
            username: username,
            image: img,
            hashtag: hashtag,
        }
    }

    pub fn get_id(&self) -> &InstaPostId {
        &self.post_id
    }

    pub fn get_id_str(&self) -> &str {
        self.post_id.0.as_str()
    }

    pub fn get_username(&self) -> &str {
        self.username.as_str()
    }

    pub fn get_image(&self) -> &FetchedImage<S> {
        &self.image
    }

    pub fn get_image_source(&self) -> &Uri {
        self.image.get_source()
    }

    pub fn get_image_source_str(&self) -> String {
        self.get_image_source().to_string()
    }

    pub fn get_hashtag(&self) -> &str {
        &self.hashtag.as_str()
    }
}
