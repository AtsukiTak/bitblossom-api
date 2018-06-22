#![allow(renamed_and_removed_lints)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate percent_encoding;
extern crate tokio;

extern crate base64;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate image;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

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
pub mod worker;
pub mod error;
pub mod db;

use self::db::Mongodb;

fn main() {
    env_logger::init();
    let mongodb_host = get_env_str("MONGODB_HOST");
    let mongodb_port = get_env_u16("MONGODB_PORT");
    let mongodb_db = get_env_str("MONGODB_DB");
    let mongodb = Mongodb::new(mongodb_host.as_str(), mongodb_port, mongodb_db.as_str());
    api_server::run(mongodb);
}

fn get_env_str(key: &str) -> String {
    ::std::env::var(key).expect(format!("{} env var is not found", key).as_str())
}

fn get_env_u16(key: &str) -> u16 {
    let s = get_env_str(key);
    u16::from_str_radix(s.as_str(), 10).expect(format!("{} is not valid number", s).as_str())
}
