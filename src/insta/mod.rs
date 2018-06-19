pub mod feeder;
pub mod api;

pub use self::feeder::InstaFeeder;
pub use self::api::{InstaApi, InstaHashtagResponse, InstaPostResponse};


use images::{SizedImage, Size};

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

#[derive(Debug)]
pub struct InstaPost<S> {
    post_id: InstaPostId,
    username: String,
    image: SizedImage<S>,
    hashtag: String,
}

impl<S: Size> InstaPost<S> {
    pub fn new(
        id: InstaPostId,
        username: String,
        img: SizedImage<S>,
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

    pub fn get_image(&self) -> &SizedImage<S> {
        &self.image
    }

    pub fn get_hashtag(&self) -> &str {
        &self.hashtag.as_str()
    }
}
