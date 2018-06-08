use std::{marker::PhantomData, sync::{Arc, Mutex}};
use rocket::{State, response::status::{BadRequest, Created}};
use rocket_contrib::Json;
use image::{Rgba, RgbaImage};

use images::{Image, size::{Size, Size1500x1500}};
use mosaic::{ArtContainer, Worker};
use error::Error;

const HOST: &str = "hoge";

// =================================
// start worker API
// =================================

#[post("/start_worker", format = "application/json", data = "<json>")]
fn handler(
    json: Json<RawStartWorkerOption>,
    worker: State<Worker>,
    art_container: State<Arc<Mutex<ArtContainer<Size1500x1500>>>>,
) -> Result<Created<String>, BadRequest<()>> {
    let option = StartWorkerOption::from(json.into_inner()).map_err(|_| BadRequest(None))?;
    let art = worker.inner().run(option.hashtags, option.origin_img);
    let id = art_container.lock().unwrap().add(art);

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

type OriginImage = ProvidedImage<Size1500x1500>;

struct ProvidedImage<S> {
    image: RgbaImage,
    phantom: PhantomData<S>,
}

impl<S: Size> ProvidedImage<S> {
    fn from_base64_str(base64_str: &str) -> Result<ProvidedImage<S>, Error> {
        let bytes = ::base64::decode(base64_str)?;
        let image = ::image::load_from_memory(&bytes)?;
        Ok(ProvidedImage {
            image: image.thumbnail_exact(S::WIDTH, S::HEIGHT).to_rgba(),
            phantom: PhantomData,
        })
    }
}

impl<S: Size> Image for ProvidedImage<S> {
    type Size = S;
    type Image = RgbaImage;
    type Pixel = Rgba<u8>;

    fn image(&self) -> &Self::Image {
        &self.image
    }

    fn image_mut(&mut self) -> &mut Self::Image {
        &mut self.image
    }
}
