mod worker;
mod start_worker;
mod get_art;

pub use self::worker::Worker;

use std::sync::{Arc, Mutex};
use images::size::{Size1500x1500, Size30x30};

type MosaicArtSize = Size1500x1500;
type PieceImageSize = Size30x30;
type CurrentSharedMosaicArt = ::mosaic::SharedMosaicArt<MosaicArtSize, PieceImageSize>;
type CurrentMosaicArtContainer = ::mosaic::MosaicArtContainer<MosaicArtSize, PieceImageSize>;


pub fn run(insta_api_host: String) {
    let cors = ::rocket_cors::Cors::default();
    ::rocket::ignite()
        .manage(Arc::new(Mutex::new(CurrentMosaicArtContainer::new())))
        .manage(Worker::new(insta_api_host))
        .mount("/", routes![start_worker::handler, get_art::handler])
        .attach(cors)
        .launch();
}
