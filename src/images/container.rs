use std::{collections::{HashMap, LinkedList}, hash::Hash, str::FromStr};

use hyper::{Uri, client::{Client, HttpConnector}};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};

use image::DynamicImage;

use error::Error;

pub trait ContainableImage: 'static {
    type Key: Clone + Eq + Hash + 'static;

    fn key(&self) -> &Self::Key;
}

pub struct ImageContainer<I: ContainableImage> {
    images: HashMap<I::Key, I>,
    images_order: LinkedList<I::Key>,
    max_contains: usize,
}

impl<I: ContainableImage> ImageContainer<I> {
    pub fn new(max_contains: usize) -> ImageContainer<I> {
        ImageContainer {
            images: HashMap::new(),
            images_order: LinkedList::new(),
            max_contains: max_contains,
        }
    }

    pub fn contains_key(&self, key: &I::Key) -> bool {
        self.images.contains_key(key)
    }

    pub fn append(&mut self, mut images: Vec<I>) {
        for image in images.drain(..) {
            self.apply_one(image);
        }
    }

    pub fn apply_one(&mut self, image: I) {
        if self.contains_key(image.key()) {
            return;
        }
        self.images_order.push_back(image.key().clone());
        self.images.insert(image.key().clone(), image);

        if self.images_order.len() > self.max_contains {
            self.delete_oldest()
        }
    }

    fn delete_oldest(&mut self) {
        if let Some(oldest_key) = self.images_order.pop_front() {
            self.images.remove(&oldest_key);
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InstaImageSource(Uri);

pub struct InstaImageFetcher {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl InstaImageFetcher {
    pub fn new() -> InstaImageFetcher {
        let https = HttpsConnector::new(1).unwrap();
        let client = Client::builder().build(https);
        InstaImageFetcher { client: client }
    }

    pub fn fetch_image(
        &self,
        source: &InstaImageSource,
    ) -> impl Future<Item = DynamicImage, Error = Error> {
        self.client
            .get(source.0.clone())
            .and_then(|res| res.into_body().concat2())
            .map_err(|e| Error::from(e))
            .and_then(|chunk| Ok(::image::load_from_memory(&chunk)?))
    }
}
