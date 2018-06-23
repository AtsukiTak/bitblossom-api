use std::str::FromStr;
use hyper::{Uri, client::{Client, HttpConnector}};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};

use images::{Size, SizedImage, Image};
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
    ) -> Result<impl Future<Item = SizedImage<S>, Error = Error>, Error> {
        let url = Uri::from_str(url)?;
        let f = self.client
            .get(url)
            .and_then(|res| res.into_body().concat2())
            .map_err(|e| Error::from(e))
            .and_then(|data| Ok(SizedImage::with_resize(Image::from_bytes(&data)?)));
        Ok(f)
    }
}
