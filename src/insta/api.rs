use std::{str::FromStr, time::{Duration, Instant}};
use futures::{Future, Stream, stream::iter_ok};
use hyper::{Uri, client::{Client, HttpConnector}};
use hyper_tls::HttpsConnector;
use tokio::timer::Delay;
use percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};

use insta::InstaPostId;
use error::Error;

pub struct InstaApi {
    delay: Duration,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl InstaApi {
    pub fn new() -> InstaApi {
        InstaApi {
            delay: Duration::new(3, 0),
            client: Client::builder().build(HttpsConnector::new(1).unwrap()),
        }
    }

    pub fn get_posts_by_hashtag(
        &self,
        hashtag: &str,
    ) -> impl Future<Item = InstaHashtagResponse, Error = Error> {
        let delay = Delay::new(Instant::now() + self.delay.clone()).map_err(Error::from);
        let res = get_posts_by_hashtag(&self.client, hashtag, None);
        delay.and_then(|_| res)
    }

    pub fn get_post_by_id(
        &self,
        id: &InstaPostId,
    ) -> impl Future<Item = InstaPostResponse, Error = Error> {
        let delay = Delay::new(Instant::now() + self.delay.clone()).map_err(Error::from);
        let res = get_post_by_id(&self.client, id);
        delay.and_then(|_| res)
    }

    pub fn get_bunch_posts_by_hashtag(
        &self,
        hashtag: &str,
    ) -> impl Stream<Item = InstaPartialPost, Error = Error> {
        let interval = self.delay.clone();
        let client = self.client.clone();
        let hashtag: String = hashtag.into();
        let posts_stream = ::futures::stream::unfold((None, true), move |(max_id, has_next)| {
            if has_next == false {
                None
            } else {
                let delay = Delay::new(Instant::now() + interval.clone()).map_err(Error::from);
                let res = get_posts_by_hashtag(&client, hashtag.as_str(), max_id);
                Some(delay.and_then(move |_| {
                    res.map(|res| (res.posts, (res.end_cursor, res.has_next_page)))
                }))
            }
        });
        posts_stream
            .map(|posts| iter_ok::<_, Error>(posts))
            .flatten()
    }
}

// internal api caller functions
fn get_posts_by_hashtag(
    client: &Client<HttpsConnector<HttpConnector>>,
    hashtag: &str,
    max_id: Option<String>,
) -> impl Future<Item = InstaHashtagResponse, Error = Error> {
    #[derive(Deserialize)]
    struct Response {
        graphql: Graphql,
    }
    #[derive(Deserialize)]
    struct Graphql {
        hashtag: Hashtag,
    }
    #[derive(Deserialize)]
    struct Hashtag {
        name: String,
        edge_hashtag_to_media: EdgeToMedia,
    }
    #[derive(Deserialize)]
    struct EdgeToMedia {
        edges: Vec<Edge>,
        page_info: PageInfo,
    }
    #[derive(Deserialize)]
    struct PageInfo {
        end_cursor: Option<String>,
        has_next_page: bool,
    }
    #[derive(Deserialize)]
    struct Edge {
        node: Node,
    }
    #[derive(Deserialize)]
    struct Node {
        #[serde(rename = "shortcode")]
        id: InstaPostId,
        #[serde(rename = "display_url")]
        image_url: String,
    }

    fn parse_res(mut res: Response) -> InstaHashtagResponse {
        let hashtag = res.graphql.hashtag.name;
        let end_cursor = res.graphql
            .hashtag
            .edge_hashtag_to_media
            .page_info
            .end_cursor;
        let has_next_page = res.graphql
            .hashtag
            .edge_hashtag_to_media
            .page_info
            .has_next_page;
        let posts = res.graphql
            .hashtag
            .edge_hashtag_to_media
            .edges
            .drain(..)
            .map(|edge| InstaPartialPost {
                id: edge.node.id,
                image_url: edge.node.image_url,
            })
            .collect();
        InstaHashtagResponse {
            posts: posts,
            hashtag: hashtag,
            end_cursor: end_cursor,
            has_next_page: has_next_page,
        }
    }

    let url = {
        let encoded_hashtag = percent_encode(hashtag.as_bytes(), DEFAULT_ENCODE_SET).to_string();
        let url_str = match max_id {
            Some(id) => format!(
                "https://www.instagram.com/explore/tags/{}/?__a=1&max_id={}",
                encoded_hashtag, id
            ),
            None => format!(
                "https://www.instagram.com/explore/tags/{}/?__a=1",
                encoded_hashtag
            ),
        };
        Uri::from_str(url_str.as_str()).unwrap()
    };

    client
        .get(url)
        .and_then(|res| res.into_body().concat2())
        .map_err(Error::from)
        .and_then(|chunk| {
            trace!(
                "Response from Instagram : {}",
                ::std::str::from_utf8(&chunk).unwrap()
            );
            Ok(::serde_json::from_slice::<Response>(&chunk)?)
        })
        .map(parse_res)
}

pub fn get_post_by_id(
    client: &Client<HttpsConnector<HttpConnector>>,
    post_id: &InstaPostId,
) -> impl Future<Item = InstaPostResponse, Error = Error> {
    #[derive(Deserialize)]
    struct Response {
        graphql: Graphql,
    }
    #[derive(Deserialize)]
    struct Graphql {
        shortcode_media: Media,
    }
    #[derive(Deserialize)]
    struct Media {
        shortcode: InstaPostId,
        display_url: String,
        owner: Owner,
    }
    #[derive(Deserialize)]
    struct Owner {
        username: String,
    }

    fn parse_res(res: Response) -> InstaPostResponse {
        InstaPostResponse {
            id: res.graphql.shortcode_media.shortcode,
            image_url: res.graphql.shortcode_media.display_url,
            user_name: res.graphql.shortcode_media.owner.username,
        }
    }

    let url = Uri::from_str(
        format!("https://www.instagram.com/p/{}/?__a=1", post_id.as_str()).as_str(),
    ).unwrap();

    client
        .get(url)
        .and_then(|res| res.into_body().concat2())
        .map_err(Error::from)
        .and_then(|chunk| {
            trace!(
                "Response from Instagram : {}",
                ::std::str::from_utf8(&chunk).unwrap()
            );
            Ok(::serde_json::from_slice::<Response>(&chunk)?)
        })
        .map(parse_res)
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstaHashtagResponse {
    pub posts: Vec<InstaPartialPost>,
    pub hashtag: String,
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstaPartialPost {
    pub id: InstaPostId,
    pub image_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstaPostResponse {
    pub id: InstaPostId,
    pub user_name: String,
    pub image_url: String,
}
