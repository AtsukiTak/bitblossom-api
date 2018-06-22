use std::sync::Mutex;
use rocket::{State, response::status::{BadRequest, Created}};
use rocket_contrib::Json;

use images::{Size, SizedImage};
use mosaic::MosaicArtGenerator;
use worker::WorkerManager;
use error::Error;
use super::{MosaicArtSize, PieceImageSize};

const HOST: &str = "";

// =================================
// start worker API
// =================================

#[post("/start_worker", format = "application/json", data = "<json>")]
fn handler(
    json: Json<RawStartWorkerOption>,
    worker_manager: State<Mutex<WorkerManager<MosaicArtSize, PieceImageSize>>>,
) -> Result<Created<String>, BadRequest<()>> {
    let option = StartWorkerOption::from(json.into_inner()).map_err(|_| BadRequest(None))?;

    debug!(
        "Accept start_worker request. hashtags = {:?}",
        option.hashtags
    );

    let generator = MosaicArtGenerator::new(option.origin_img, option.hashtags);

    let id = worker_manager
        .inner()
        .lock()
        .unwrap()
        .start_worker(generator);
    info!("Run a new worker");

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
