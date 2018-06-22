use std::sync::{Arc, Mutex};
use rocket::{State, response::status::NotFound};
use rocket_contrib::Json;

use mosaic::MosaicArt;
use worker::{WorkerId, WorkerManager};
use super::{MosaicArtSize, PieceImageSize};

// =================================
// get mosaic art API
// =================================

#[get("/<id>")]
fn handler(
    id: u64,
    worker_manager: State<Mutex<WorkerManager<MosaicArtSize, PieceImageSize>>>,
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

fn construct_response(art: Arc<MosaicArt<MosaicArtSize, PieceImageSize>>) -> MosaicArtResponse {
    let mosaic_art = {
        let png_img = art.image.to_png_bytes();
        ::base64::encode(png_img.as_slice())
    };
    let piece_posts = art.pieces
        .iter()
        .map(|post| InstaPostResponse {
            post_id: post.post_id.0.clone(),
            user_name: post.user_name.clone(),
        })
        .collect();
    let hashtags = art.hashtags.as_ref().clone();
    MosaicArtResponse {
        mosaic_art: mosaic_art,
        piece_posts: piece_posts,
        insta_hashtags: hashtags,
    }
}

#[derive(Serialize)]
pub struct MosaicArtResponse {
    mosaic_art: String, // base64 encoded,
    piece_posts: Vec<InstaPostResponse>,
    insta_hashtags: Vec<String>,
}

#[derive(Serialize)]
pub struct InstaPostResponse {
    post_id: String,
    user_name: String,
}
