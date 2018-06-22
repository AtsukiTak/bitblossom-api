mod worker;
mod start_worker;
mod get_art;

pub use self::worker::Worker;

use std::sync::{Arc, Mutex};
use images::size::{Size1500x1500, Size30x30};
use db::Mongodb;

type MosaicArtSize = Size1500x1500;
type PieceImageSize = Size30x30;
type CurrentSharedMosaicArt = ::mosaic::SharedMosaicArt<MosaicArtSize, PieceImageSize>;
type CurrentMosaicArtContainer = ::mosaic::MosaicArtContainer<MosaicArtSize, PieceImageSize>;

pub fn run(mongodb: Mongodb) {
    let cors = ::rocket_cors::Cors::default();
    ::rocket::ignite()
        .manage(Arc::new(Mutex::new(CurrentMosaicArtContainer::new())))
        .manage(Worker::new(mongodb))
        .mount("/", routes![start_worker::handler, get_art::handler])
        .attach(cors)
        .launch();
}
