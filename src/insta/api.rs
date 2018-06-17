use std::str::FromStr;
use futures::{Future, Stream};
use hyper::{Uri, client::{Client, HttpConnector}};
use serde::de::DeserializeOwned;
use percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};

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
        Uri::from_str(format!("http://{}/hashtags/HOGE", api_server_host.as_str()).as_str())
            .unwrap();

        InstaApi {
            api_server_host: api_server_host,
            client: client,
        }
    }

    pub fn get_posts_by_hashtag(
        &self,
        hashtag: &str,
    ) -> impl Future<Item = Vec<InstaHashtagResponse>, Error = Error> {
        #[derive(Deserialize)]
        struct RawResponse {
            res: Vec<InstaHashtagResponse>,
        }

        let url = {
            let encoded_hashtag =
                percent_encode(hashtag.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            Uri::from_str(
                format!(
                    "http://{}/hashtags/{}",
                    self.api_server_host, encoded_hashtag
                ).as_str(),
            ).unwrap()
        };

        let client = self.client.clone();

        call_api::<RawResponse>(&client, url.clone()).map(|res| res.res)
    }

    pub fn get_post_by_id(
        &self,
        post_id: &InstaPostId,
    ) -> impl Future<Item = InstaPostResponse, Error = Error> {
        let url = Uri::from_str(
            format!("http://{}/posts/{}", self.api_server_host, post_id.0).as_str(),
        ).unwrap();

        let client = self.client.clone();

        call_api::<InstaPostResponse>(&client, url.clone())
    }
}

fn call_api<R: DeserializeOwned>(
    client: &Client<HttpConnector>,
    url: Uri,
) -> impl Future<Item = R, Error = Error> {
    client
        .get(url)
        .and_then(|res| res.into_body().concat2())
        .map_err(Error::from)
        .and_then(|chunk| Ok(::serde_json::from_slice::<R>(&chunk)?))
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstaHashtagResponse {
    pub id: InstaPostId,
    pub image_url: String,
    pub hashtag: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstaPostResponse {
    pub id: InstaPostId,
    pub user_name: String,
    pub image_url: String,
}
