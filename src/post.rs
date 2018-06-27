use std::sync::Arc;
use serde::ser::{Serialize, Serializer};

use images::{Size, SizedImage};

pub trait Post {
    type ImageSize: Size;
    fn image(&self) -> &SizedImage<Self::ImageSize>;
    fn user_name(&self) -> &str;
    fn hashtag(&self) -> &Hashtag;
}

#[derive(Debug, Clone)]
pub struct BluummPost<S> {
    image: Arc<SizedImage<S>>,
    user_name: Arc<String>,
    hashtag: Hashtag,
}

impl<S: Size> BluummPost<S> {
    pub fn new<T: Into<String>>(
        image: SizedImage<S>,
        user_name: T,
        hashtag: Hashtag,
    ) -> BluummPost<S> {
        BluummPost {
            image: Arc::new(image),
            user_name: Arc::new(user_name.into()),
            hashtag: hashtag,
        }
    }
}

impl<S: Size> Post for BluummPost<S> {
    type ImageSize = S;

    fn image(&self) -> &SizedImage<S> {
        &self.image
    }

    fn user_name(&self) -> &str {
        self.user_name.as_str()
    }

    fn hashtag(&self) -> &Hashtag {
        &self.hashtag
    }
}

#[derive(Debug, Clone)]
pub struct InstaPost<S> {
    pub post_id: InstaPostId,
    image: Arc<SizedImage<S>>,
    user_name: Arc<String>,
    hashtag: Hashtag,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
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

impl<S: Size> InstaPost<S> {
    pub fn new<T: Into<String>>(
        id: InstaPostId,
        image: SizedImage<S>,
        user_name: T,
        hashtag: Hashtag,
    ) -> InstaPost<S> {
        InstaPost {
            post_id: id,
            image: Arc::new(image),
            user_name: Arc::new(user_name.into()),
            hashtag: hashtag,
        }
    }
}

impl<S: Size> Post for InstaPost<S> {
    type ImageSize = S;
    fn image(&self) -> &SizedImage<S> {
        &self.image
    }

    fn user_name(&self) -> &str {
        self.user_name.as_str()
    }

    fn hashtag(&self) -> &Hashtag {
        &self.hashtag
    }
}

#[derive(Debug, Clone)]
pub enum GenericPost<S> {
    BluummPost(BluummPost<S>),
    InstaPost(InstaPost<S>),
}

impl<S: Size> Post for GenericPost<S> {
    type ImageSize = S;
    fn image(&self) -> &SizedImage<S> {
        match self {
            &GenericPost::BluummPost(ref p) => p.image(),
            &GenericPost::InstaPost(ref p) => p.image(),
        }
    }
    fn user_name(&self) -> &str {
        match self {
            &GenericPost::BluummPost(ref p) => p.user_name(),
            &GenericPost::InstaPost(ref p) => p.user_name(),
        }
    }
    fn hashtag(&self) -> &Hashtag {
        match self {
            &GenericPost::BluummPost(ref p) => p.hashtag(),
            &GenericPost::InstaPost(ref p) => p.hashtag(),
        }
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
