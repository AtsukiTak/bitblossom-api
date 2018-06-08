use std::sync::{Arc, Mutex};
use rocket::{State, response::status::NotFound};
use rocket_contrib::Json;

use mosaic::{ArtContainer, MosaicArt};
use images::{Image, size::Size1500x1500};

// =================================
// get mosaic art API
// =================================

#[get("/<id>")]
fn handler(
    id: u64,
    arts: State<Arc<Mutex<ArtContainer<Size1500x1500>>>>,
) -> Result<Json<MosaicArtResponse>, NotFound<&'static str>> {
    match arts.inner().lock().unwrap().get(id) {
        Some(art) => Ok(Json(construct_response(&art.lock().unwrap()))),
        None => Err(NotFound("Nothing is also art...")),
    }
}

fn construct_response(art: &MosaicArt<Size1500x1500>) -> MosaicArtResponse {
    let mosaic_art = {
        let raw_img = art.get_image().image();
        ::base64::encode(&raw_img.split_at(0).1)
    };
    let piece_posts = art.get_piece_posts()
        .map(|post| {
            let post = post.clone();
            InstaPostResponse {
                post_id: post.post_id.0,
                image_url: post.image_url,
                user_name: post.user_name,
            }
        })
        .collect();
    let hashtags = art.get_hashtags().clone();
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
    image_url: String,
    user_name: String,
}
