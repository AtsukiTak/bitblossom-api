use std::sync::{Arc, Mutex};
use rocket::{State, response::status::NotFound};
use rocket_contrib::Json;

use mosaic::MosaicArtId;
use super::{CurrentMosaicArtContainer, CurrentSharedMosaicArt};

// =================================
// get mosaic art API
// =================================

#[get("/<id>")]
fn handler(
    id: u64,
    arts: State<Arc<Mutex<CurrentMosaicArtContainer>>>,
) -> Result<Json<MosaicArtResponse>, NotFound<&'static str>> {
    match arts.inner().lock().unwrap().get(MosaicArtId(id)) {
        Some(ref art) => Ok(Json(construct_response(art))),
        None => Err(NotFound("Nothing is also art...")),
    }
}

fn construct_response(art: &CurrentSharedMosaicArt) -> MosaicArtResponse {
    let mosaic_art = {
        let png_img = art.get_image().to_png_bytes();
        ::base64::encode(png_img.as_slice())
    };
    let piece_posts = art.get_piece_posts()
        .iter()
        .map(|post| InstaPostResponse {
            post_id: post.post_id.0.clone(),
            user_name: post.user_name.clone(),
        })
        .collect();
    let hashtags = art.get_hashtags();
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
