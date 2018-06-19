use std::sync::{Arc, Mutex};
use rocket::{State, response::status::{BadRequest, Created}};
use rocket_contrib::Json;

use images::{Size, SizedImage};
use api_server::Worker;
use error::Error;
use super::CurrentMosaicArtContainer;

const HOST: &str = "";

// =================================
// start worker API
// =================================

#[post("/start_worker", format = "application/json", data = "<json>")]
fn handler(
    json: Json<RawStartWorkerOption>,
    worker: State<Worker>,
    art_container: State<Arc<Mutex<CurrentMosaicArtContainer>>>,
) -> Result<Created<String>, BadRequest<()>> {
    let option = StartWorkerOption::from(json.into_inner()).map_err(|_| BadRequest(None))?;

    debug!(
        "Accept start_worker request. hashtags = {:?}",
        option.hashtags
    );

    let art = worker.inner().run(option.hashtags, option.origin_img);
    info!("Run a new worker");

    let id = art_container.lock().unwrap().add(art);
    info!("New mosaic art's id : {}", id);

    let created_url = format!("{}/{}", HOST, id);
    Ok(Created(created_url, Some(format!("{}", id))))
}

#[derive(Deserialize)]
struct RawStartWorkerOption {
    origin_img: String, // base64 encoded
    hashtags: Vec<String>,
}

struct StartWorkerOption<S> {
    origin_img: SizedImage<S>,
    hashtags: Vec<String>,
}

impl<S: Size> StartWorkerOption<S> {
    fn from(raw: RawStartWorkerOption) -> Result<StartWorkerOption<S>, Error> {
        Ok(StartWorkerOption {
            origin_img: encode_image(raw.origin_img.as_str())?,
            hashtags: raw.hashtags,
        })
    }
}

fn encode_image<S: Size>(base64_str: &str) -> Result<SizedImage<S>, Error> {
    let bytes = ::base64::decode(base64_str)?;
    SizedImage::from_raw_bytes(&bytes)
}
