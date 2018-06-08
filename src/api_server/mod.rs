mod start_worker;
mod get_art;

use std::sync::{Arc, Mutex};

use images::size::Size1500x1500;
use mosaic::{ArtContainer, Worker};

pub fn run(insta_api_host: String) {
    ::rocket::ignite()
        .manage(Arc::new(Mutex::new(ArtContainer::<Size1500x1500>::new())))
        .manage(Worker::new(insta_api_host))
        .mount("/", routes![start_worker::handler, get_art::handler])
        .launch();
}
