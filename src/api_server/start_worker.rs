use std::sync::Mutex;
use rocket::{State, response::status::{BadRequest, Created}};
use rocket_contrib::Json;

use images::{Image, MultipleOf, Size, SizedImage, SmallerThan,
             size::{Size100x100, Size1500x1500, Size3000x3000, Size30x30, Size50x50}};
use worker::{WorkerId, WorkerManager};
use mosaic::MosaicArtGenerator;
use error::Error;

const HOST: &str = "";

// =================================
// start worker API
// =================================

#[post("/start_worker", format = "application/json", data = "<json>")]
fn handler(
    json: Json<RawStartWorkerOption>,
    worker_manager: State<Mutex<WorkerManager>>,
) -> Result<Created<String>, BadRequest<()>> {
    let option = StartWorkerOption::from(json.into_inner()).map_err(|_| BadRequest(None))?;

    debug!(
        "Accept start_worker request. hashtags = {:?}",
        option.hashtags
    );

    let origin_size = (option.origin.width(), option.origin.height());

    let id = match (origin_size, option.piece_size) {
        ((1500, 1500), Some((30, 30))) => start_worker::<Size1500x1500, Size30x30>(
            SizedImage::new(option.origin).unwrap(),
            option.hashtags,
            worker_manager,
        ),
        ((1500, 1500), Some((50, 50))) => start_worker::<Size1500x1500, Size50x50>(
            SizedImage::new(option.origin).unwrap(),
            option.hashtags,
            worker_manager,
        ),
        ((1500, 1500), None) => start_worker::<Size1500x1500, Size30x30>(
            SizedImage::new(option.origin).unwrap(),
            option.hashtags,
            worker_manager,
        ),
        ((3000, 3000), Some((30, 30))) => start_worker::<Size3000x3000, Size30x30>(
            SizedImage::new(option.origin).unwrap(),
            option.hashtags,
            worker_manager,
        ),
        ((3000, 3000), Some((50, 50))) => start_worker::<Size3000x3000, Size50x50>(
            SizedImage::new(option.origin).unwrap(),
            option.hashtags,
            worker_manager,
        ),
        ((3000, 3000), Some((100, 100))) => start_worker::<Size3000x3000, Size100x100>(
            SizedImage::new(option.origin).unwrap(),
            option.hashtags,
            worker_manager,
        ),
        ((3000, 3000), None) => start_worker::<Size3000x3000, Size50x50>(
            SizedImage::new(option.origin).unwrap(),
            option.hashtags,
            worker_manager,
        ),
        _ => return Err(BadRequest(None)),
    };

    let created_url = format!("{}/{}", HOST, id);
    Ok(Created(created_url, Some(format!("{}", id))))
}

fn start_worker<S, SS>(
    origin: SizedImage<S>,
    hashtags: Vec<String>,
    worker_manager: State<Mutex<WorkerManager>>,
) -> WorkerId
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    let generator = MosaicArtGenerator::new(origin, hashtags);

    let id = worker_manager
        .inner()
        .lock()
        .unwrap()
        .start_worker(generator);
    info!("Run a new worker");

    id
}

#[derive(Deserialize)]
struct RawStartWorkerOption {
    origin: String, // base64 encoded
    hashtags: Vec<String>,
    piece_size: Option<(u32, u32)>,
}

struct StartWorkerOption {
    origin: Image,
    hashtags: Vec<String>,
    piece_size: Option<(u32, u32)>,
}

impl StartWorkerOption {
    fn from(raw: RawStartWorkerOption) -> Result<StartWorkerOption, Error> {
        Ok(StartWorkerOption {
            origin: encode_image(raw.origin.as_str())?,
            hashtags: raw.hashtags,
            piece_size: raw.piece_size,
        })
    }
}

fn encode_image(base64_str: &str) -> Result<Image, Error> {
    let bytes = ::base64::decode(base64_str)?;
    Image::from_bytes(bytes.as_slice())
}
