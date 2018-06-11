use std::{marker::PhantomData, sync::{Arc, Mutex}};
use rocket::{State, response::status::{BadRequest, Created}};
use rocket_contrib::Json;
use image::{GenericImage, RgbaImage};

use images::{Image, size::Size};
use api_server::Worker;
use error::Error;
use super::{CurrentMosaicArtContainer, MosaicArtSize};

const HOST: &str = "hoge";

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

struct StartWorkerOption {
    origin_img: OriginImage,
    hashtags: Vec<String>,
}

impl StartWorkerOption {
    fn from(raw: RawStartWorkerOption) -> Result<StartWorkerOption, Error> {
        Ok(StartWorkerOption {
            origin_img: OriginImage::from_base64_str(raw.origin_img.as_str())?,
            hashtags: raw.hashtags,
        })
    }
}

type OriginImage = ProvidedImage<MosaicArtSize>;

struct ProvidedImage<S> {
    image: RgbaImage,
    phantom: PhantomData<S>,
}

impl<S: Size> ProvidedImage<S> {
    fn from_base64_str(base64_str: &str) -> Result<ProvidedImage<S>, Error> {
        let bytes = ::base64::decode(base64_str)?;
        let image = ::image::load_from_memory(&bytes)?;
        if image.width() != S::WIDTH || image.height() != S::HEIGHT {
            bail!(::error::ErrorKind::InvalidImageSize("1500 x 1500"));
        }
        Ok(ProvidedImage {
            image: image.to_rgba(),
            phantom: PhantomData,
        })
    }
}

impl<S: Size> Image for ProvidedImage<S> {
    type Size = S;

    fn image(&self) -> &RgbaImage {
        &self.image
    }

    fn image_mut(&mut self) -> &mut RgbaImage {
        &mut self.image
    }
}
