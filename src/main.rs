#![allow(renamed_and_removed_lints)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate tokio;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate http;

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate base64;

extern crate image;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate error_chain;
extern crate rand;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod mosaic;
pub mod images;
pub mod insta;
pub mod api_server;
pub mod error;

fn main() {
    env_logger::init();
    let insta_api_host = get_env("INSTA_API_SERVER_HOST");
    api_server::run(insta_api_host);
}

fn get_env(key: &str) -> String {
    ::std::env::var(key).expect(format!("{} env var is not found", key).as_str())
}
