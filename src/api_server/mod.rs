mod start_worker;
mod get_art;

use std::sync::Mutex;
use images::size::{Size100x100, Size3000x3000};
use worker::WorkerManager;
use db::Mongodb;

type MosaicArtSize = Size3000x3000;
type PieceImageSize = Size100x100;

pub fn run(mongodb: Mongodb) {
    let cors = ::rocket_cors::Cors::default();
    ::rocket::ignite()
        .manage(Mutex::new(
            WorkerManager::<MosaicArtSize, PieceImageSize>::new(mongodb),
        ))
        .mount("/", routes![start_worker::handler, get_art::handler])
        .attach(cors)
        .launch();
}
