use std::sync::Mutex;
use rocket::{State, response::status::BadRequest};
use rocket_contrib::Json;

use images::{Image, Size, SizedImage};
use worker::{WorkerId, WorkerManager};
use post::{BluummPost, Hashtag};
use error::Error;
use super::{OriginImageSize, PieceImageSize};

#[post("/worker/<id>/bluumm_post", format = "application/json", data = "<json>")]
fn handler(
    id: u64,
    json: Json<RawAddBluummPostArg>,
    worker_manager: State<Mutex<WorkerManager<OriginImageSize, PieceImageSize>>>,
) -> Result<&'static str, BadRequest<()>> {
    match worker_manager
        .inner()
        .lock()
        .unwrap()
        .get_worker(WorkerId(id))
    {
        Some(worker) => match encode_arg(json.into_inner()) {
            Ok(post) => {
                worker.add_bluumm_post(post);
                Ok("Success")
            }
            Err(_e) => Err(BadRequest(None)),
        },
        None => Err(BadRequest(None)),
    }
}

#[derive(Deserialize)]
struct RawAddBluummPostArg {
    image: String,
    user_name: String,
    hashtag: String,
}

fn encode_arg<SS: Size>(arg: RawAddBluummPostArg) -> Result<BluummPost<SS>, Error> {
    let image = encode_image(arg.image.as_str())?;
    let sized_image = SizedImage::new(image)?;
    Ok(BluummPost::new(
        sized_image,
        arg.user_name,
        Hashtag::new(arg.hashtag),
    ))
}

fn encode_image(base64_str: &str) -> Result<Image, Error> {
    let bytes = ::base64::decode(base64_str)?;
    Image::from_bytes(bytes.as_slice())
}
