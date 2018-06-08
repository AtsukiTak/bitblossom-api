#![allow(renamed_and_removed_lints)]
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
    let insta_api_host = get_env("INSTA_API_SERVER_HOST").unwrap();
    api_server::run(insta_api_host);
}

fn get_env(key: &str) -> Option<String> {
    ::std::env::var(key).ok()
}
