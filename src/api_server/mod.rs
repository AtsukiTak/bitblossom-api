mod start_worker;
mod get_art;

use std::sync::Mutex;
use worker::WorkerManager;
use db::Mongodb;

pub fn run(mongodb: Mongodb) {
    let cors = ::rocket_cors::Cors::default();
    ::rocket::ignite()
        .manage(Mutex::new(
            WorkerManager::new(mongodb),
        ))
        .mount("/", routes![start_worker::handler, get_art::handler])
        .attach(cors)
        .launch();
}
