#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate url;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;

use rocket::config::{Config, Environment};
use std::env;

mod circleci;
mod utils;
mod view;

fn get_port() -> u16 {
    return env::var("PORT")
        .unwrap_or("5000".into())
        .parse::<u16>()
        .expect("Invalid port number");
}

fn main() {
    let config = Config::build(Environment::Production)
        .port(get_port())
        .finalize()
        .unwrap();
    rocket::custom(config, true)
        .mount("/", routes![view::handle])
        .launch();
}
