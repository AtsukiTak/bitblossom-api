use std::sync::{Arc, Mutex};
use rocket::{State, response::status::NotFound};
use rocket_contrib::Json;

use mosaic::MosaicArt;
use post::{BluummPost, GenericPost, Hashtag, HashtagList, InstaPost, InstaPostId, Post};
use images::Size;
use worker::{WorkerId, WorkerManager};

use super::{OriginImageSize, PieceImageSize};

// =================================
// get mosaic art API
// =================================

#[get("/<id>")]
fn handler(
    id: u64,
    worker_manager: State<Mutex<WorkerManager<OriginImageSize, PieceImageSize>>>,
) -> Result<Json<MosaicArtResponse>, NotFound<&'static str>> {
    match worker_manager
        .inner()
        .lock()
        .unwrap()
        .get_worker(WorkerId(id))
    {
        Some(ref worker) => Ok(Json(construct_response(worker.get_art()))),
        None => Err(NotFound("Nothing is also art...")),
    }
}

fn construct_response<S, SS>(art: Arc<MosaicArt<S, SS>>) -> MosaicArtResponse
where
    S: Size,
    SS: Size,
{
    let mosaic_art = {
        let png_img = art.image.to_png_bytes();
        ::base64::encode(png_img.as_slice())
    };
    let piece_posts = art.posts
        .iter()
        .map(|post| PostResponse::from(post))
        .collect();
    let hashtags = art.hashtags.clone();
    MosaicArtResponse {
        mosaic_art: mosaic_art,
        piece_posts: piece_posts,
        insta_hashtags: hashtags,
    }
}

#[derive(Serialize)]
pub struct MosaicArtResponse {
    mosaic_art: String, // base64 encoded,
    piece_posts: Vec<PostResponse>,
    insta_hashtags: HashtagList,
}

#[derive(Serialize)]
pub enum PostResponse {
    BluummPost(BluummPostResponse),
    InstaPost(InstaPostResponse),
}

impl PostResponse {
    fn from<SS: Size>(post: &GenericPost<SS>) -> PostResponse {
        match post {
            &GenericPost::BluummPost(ref post) => {
                PostResponse::BluummPost(BluummPostResponse::from(post))
            }
            &GenericPost::InstaPost(ref post) => {
                PostResponse::InstaPost(InstaPostResponse::from(post))
            }
        }
    }
}

#[derive(Serialize)]
pub struct BluummPostResponse {
    image: String,
    user_name: String,
    hashtag: Hashtag,
}

impl BluummPostResponse {
    fn from<SS: Size>(post: &BluummPost<SS>) -> BluummPostResponse {
        let image = ::base64::encode(post.image().to_png_bytes().as_slice());
        let user_name = post.user_name().into();
        let hashtag = post.hashtag().clone();
        BluummPostResponse {
            image: image,
            user_name: user_name,
            hashtag: hashtag,
        }
    }
}

#[derive(Serialize)]
pub struct InstaPostResponse {
    post_id: InstaPostId,
    image: String,
    user_name: String,
    hashtag: Hashtag,
}

impl InstaPostResponse {
    fn from<SS: Size>(post: &InstaPost<SS>) -> InstaPostResponse {
        let image = ::base64::encode(post.image().to_png_bytes().as_slice());
        let user_name = post.user_name().into();
        let hashtag = post.hashtag().clone();
        InstaPostResponse {
            post_id: post.post_id.clone(),
            image: image,
            user_name: user_name,
            hashtag: hashtag,
        }
    }
}
