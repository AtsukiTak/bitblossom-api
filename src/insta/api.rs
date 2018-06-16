use std::str::FromStr;
use futures::{stream, Future, Stream};
use hyper::{Uri, client::{Client, HttpConnector}};
use serde_json::error::Error as SerdeError;
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

        let url = {
            let encoded_hashtag =
                percent_encode(hashtag.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            Uri::from_str(
                format!(
                    "http://{}/posts?hashtag={}",
                    self.api_server_host, encoded_hashtag
                ).as_str(),
            ).unwrap()
        };

        let client = self.client.clone();

        // Return first response successed to be parsed
        first_ok(chained_fut_stream(move || {
            call_api::<RawResponse>(&client, url.clone())
        })).map(|raw| raw.posts)
    }

    pub fn get_post_by_id(
        &self,
        post_id: &InstaPostId,
    ) -> impl Future<Item = InstaPostResponse, Error = Error> {
        let url = Uri::from_str(
            format!("http://{}/posts/{}", self.api_server_host, post_id.0).as_str(),
        ).unwrap();

        let client = self.client.clone();

        // Return first response successed to be parsed
        first_ok(chained_fut_stream(move || {
            call_api::<InstaPostResponse>(&client, url.clone())
        }))
    }
}

fn call_api<R: DeserializeOwned>(
    client: &Client<HttpConnector>,
    url: Uri,
) -> impl Future<Item = Result<R, SerdeError>, Error = Error> {
    client
        .get(url)
        .and_then(|res| res.into_body().concat2())
        .map_err(Error::from)
        .map(|chunk| ::serde_json::from_slice::<R>(&chunk))
}

fn chained_fut_stream<F, Fut, T, E>(mut f: F) -> impl Stream<Item = T, Error = E>
where
    F: FnMut() -> Fut,
    Fut: Future<Item = T, Error = E>,
{
    stream::unfold((), move |()| Some(f().map(|t| (t, ()))))
}

fn first_ok<S, T, E, E2>(stream: S) -> impl Future<Item = T, Error = E>
where
    S: Stream<Item = Result<T, E2>, Error = E>,
{
    stream
        .filter_map(|res| res.ok())
        .into_future()
        .map_err(|(e, _s)| e)
        .map(|(opt, _s)| opt.unwrap())
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
