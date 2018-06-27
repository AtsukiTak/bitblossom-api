pub mod feeder;
pub mod api;

pub use self::feeder::InstaFeeder;
pub use self::api::{InstaApi, InstaHashtagResponse, InstaPostResponse};

use std::sync::Arc;
use serde::ser::{Serialize, Serializer};
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hashtag(pub Arc<String>);

impl Hashtag {
    pub fn new(s: &str) -> Hashtag {
        Hashtag(Arc::new(s.into()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Serialize for Hashtag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        str::serialize(self.as_str(), serializer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashtagList(pub Arc<Vec<Hashtag>>);

impl HashtagList {
    pub fn new(vec: Vec<String>) -> HashtagList {
        let hashtags = vec.iter().map(|s| Hashtag::new(s)).collect();
        HashtagList(Arc::new(hashtags))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> HashtagIterator {
        HashtagIterator {
            list: self.clone(),
            idx: 0,
        }
    }
}

impl Serialize for HashtagList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <[Hashtag]>::serialize(self.0.as_slice(), serializer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashtagIterator {
    list: HashtagList,
    idx: usize,
}

impl Iterator for HashtagIterator {
    type Item = Hashtag;
    fn next(&mut self) -> Option<Hashtag> {
        if self.idx < self.list.len() {
            self.idx += 1;
            Some(self.list.0[self.idx].clone())
        } else {
            None
        }
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
    pub hashtag: Hashtag,
}

impl<S: Size> InstaPost<S> {
    pub fn new(
        id: InstaPostId,
        username: String,
        img: SizedImage<S>,
        hashtag: Hashtag,
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
