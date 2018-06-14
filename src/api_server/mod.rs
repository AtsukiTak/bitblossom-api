mod worker;
mod start_worker;
mod get_art;

pub use self::worker::Worker;

use std::sync::{Arc, Mutex};
use images::size::{Size1500x1500, Size30x30};
use rocket::{Response, Request, response::Responder, http::{Status, hyper::header::AccessControlAllowOrigin}};

type MosaicArtSize = Size1500x1500;
type PieceImageSize = Size30x30;
type CurrentSharedMosaicArt = ::mosaic::SharedMosaicArt<MosaicArtSize, PieceImageSize>;
type CurrentMosaicArtContainer = ::mosaic::MosaicArtContainer<MosaicArtSize, PieceImageSize>;

pub struct SimpleCors<R>{
    res: R,
    allow_origin: AccessControlAllowOrigin,
}

impl<R> SimpleCors<R> {
    pub fn new(r: R) -> SimpleCors<R> {
        SimpleCors {
            res: r,
            allow_origin: AccessControlAllowOrigin::Any,
        }
    }
}

impl<'r, R: Responder<'r>> Responder<'r> for SimpleCors<R> {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        let mut res = self.res.respond_to(request)?;
        res.set_header(self.allow_origin);
        Ok(res)
    }
}

pub fn run(insta_api_host: String) {
    ::rocket::ignite()
        .manage(Arc::new(Mutex::new(CurrentMosaicArtContainer::new())))
        .manage(Worker::new(insta_api_host))
        .mount("/", routes![start_worker::handler, get_art::handler])
        .launch();
}
