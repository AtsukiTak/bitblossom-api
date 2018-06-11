use std::str::FromStr;
use futures::{Future, Stream};
use hyper::{Uri, client::{Client, HttpConnector}};

use insta::InstaPostId;
use error::Error;

pub struct InstaApi {
    api_server_host: String,
    client: Client<HttpConnector>,
}

impl InstaApi {
    pub fn new(api_server_host: String) -> InstaApi {
        let client = Client::new();

        // check whether api_server_host is valid
        Uri::from_str(format!("http://{}/posts?hashtag=HOGE", api_server_host.as_str()).as_str())
            .unwrap();

        InstaApi {
            api_server_host: api_server_host,
            client: client,
        }
    }

    pub fn get_posts_by_hashtag(
        &self,
        hashtag: &str,
    ) -> impl Future<Item = Vec<InstaPartialPostResponse>, Error = Error> {
        #[derive(Deserialize)]
        struct RawResponse {
            posts: Vec<InstaPartialPostResponse>,
        }

        let url = Uri::from_str(
            format!("http://{}/posts?hashtag={}", self.api_server_host, hashtag).as_str(),
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
    ) -> impl Future<Item = InstaPostResponse, Error = Error> {
        let url = Uri::from_str(
            format!("http://{}/posts/{}", self.api_server_host, post_id.0).as_str(),
        ).unwrap();

        self.client
            .get(url)
            .and_then(|res| res.into_body().concat2())
            .map_err(|hyper_err| Error::from(hyper_err))
            .and_then(|chunk| Ok(::serde_json::from_slice::<InstaPostResponse>(&chunk)?))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstaPartialPostResponse {
    pub post_id: InstaPostId,
    pub image_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstaPostResponse {
    pub post_id: InstaPostId,
    pub user_name: String,
    pub image_url: String,
}
