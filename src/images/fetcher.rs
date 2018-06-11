use std::{marker::PhantomData, str::FromStr};
use hyper::{Uri, client::{Client, HttpConnector}};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use image::{DynamicImage, RgbaImage};

use images::{Image, size::Size};
use error::Error;

#[derive(Debug)]
pub struct ImageFetcher {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl ImageFetcher {
    pub fn new() -> ImageFetcher {
        let https = HttpsConnector::new(1).unwrap();
        let client = Client::builder().build(https);
        ImageFetcher { client: client }
    }

    pub fn fetch_image<S: Size>(
        &self,
        url: &str,
    ) -> Result<impl Future<Item = FetchedImage<S>, Error = Error>, Error> {
        let url = Uri::from_str(url)?;
        let f = self.client
            .get(url)
            .and_then(|res| res.into_body().concat2())
            .map_err(|e| Error::from(e))
            .and_then(|data| Ok(FetchedImage::new(::image::load_from_memory(&data)?)));
        Ok(f)
    }
}

pub struct FetchedImage<S> {
    image: RgbaImage,
    phantom: PhantomData<S>,
}

impl<S: Size> Image for FetchedImage<S> {
    type Size = S;

    fn image(&self) -> &RgbaImage {
        &self.image
    }

    fn image_mut(&mut self) -> &mut RgbaImage {
        &mut self.image
    }
}

impl<S: Size> FetchedImage<S> {
    fn new(org: DynamicImage) -> FetchedImage<S> {
        let cropped = org.thumbnail_exact(S::WIDTH, S::HEIGHT).to_rgba();
        FetchedImage {
            image: cropped,
            phantom: PhantomData,
        }
    }
}

impl<S> ::std::fmt::Debug for FetchedImage<S> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "FetchedImage")
    }
}
