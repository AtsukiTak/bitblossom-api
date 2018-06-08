#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate tokio;
extern crate tokio_core;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate http;

extern crate rocket;
extern crate rocket_contrib;
extern crate base64;

extern crate image;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate error_chain;
extern crate rand;

pub mod mosaic;
pub mod images;
pub mod error;
pub mod api_server;

fn main() {
    let insta_api_host = "".into();
    api_server::run(insta_api_host);
}
