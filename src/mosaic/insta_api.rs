use std::str::FromStr;
use futures::{Future, Stream};
use hyper::{Uri, client::{Client, HttpConnector}};

use error::Error;

pub struct InstaApi {
    api_server_host: String,
    client: Client<HttpConnector>,
}

impl InstaApi {
    pub fn new(api_server_host: String) -> InstaApi {
        let client = Client::new();

        InstaApi {
            api_server_host: api_server_host,
            client: client,
        }
    }

    pub fn get_posts_by_hashtag(
        &self,
        hashtag: &str,
    ) -> impl Future<Item = Vec<InstaPartialPost>, Error = Error> {
        #[derive(Deserialize)]
        struct RawResponse {
            posts: Vec<InstaPartialPost>,
        }

        let url = Uri::from_str(
            format!("{}/posts?hashtag={}", self.api_server_host, hashtag).as_str(),
        ).unwrap();

        self.client
            .get(url)
            .and_then(|res| res.into_body().concat2())
            .map_err(|hyper_err| Error::from(hyper_err))
            .and_then(|chunk| Ok(::serde_json::from_slice::<RawResponse>(&chunk)?.posts))
    }

    pub fn get_post_by_id(
        &self,
        post_id: &InstaPostId,
    ) -> impl Future<Item = InstaPost, Error = Error> {
        let url = Uri::from_str(format!("{}/posts/{}", self.api_server_host, post_id.0).as_str())
            .unwrap();

        self.client
            .get(url)
            .and_then(|res| res.into_body().concat2())
            .map_err(|hyper_err| Error::from(hyper_err))
            .and_then(|chunk| Ok(::serde_json::from_slice::<InstaPost>(&chunk)?))
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstaPostId(pub String);

#[derive(Deserialize, Debug, Clone)]
pub struct InstaPartialPost {
    pub post_id: InstaPostId,
    pub image_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstaPost {
    pub post_id: InstaPostId,
    pub image_url: String,
    pub user_name: String,
}
