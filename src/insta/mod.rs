pub mod feeder;
pub mod api;

pub use self::feeder::InstaFeeder;
pub use self::api::{InstaApi, InstaHashtagResponse, InstaPostResponse};

use images::{Size, SizedImage};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct InstaPostId(pub String);

impl InstaPostId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ::std::fmt::Display for InstaPostId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct InstaPost<S> {
    pub meta: InstaPostInfo,
    pub image: SizedImage<S>,
}

#[derive(Debug, Clone)]
pub struct InstaPostInfo {
    pub post_id: InstaPostId,
    pub user_name: String,
    pub hashtag: String,
}

impl<S: Size> InstaPost<S> {
    pub fn new(
        id: InstaPostId,
        username: String,
        img: SizedImage<S>,
        hashtag: String,
    ) -> InstaPost<S> {
        InstaPost {
            meta: InstaPostInfo {
                post_id: id,
                user_name: username,
                hashtag: hashtag,
            },
            image: img,
        }
    }
}
