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

#[cfg(not(debug_assertions))]
const ROCKET_ENVIRONMENT: Environment = Environment::Production;

#[cfg(debug_assertions)]
const ROCKET_ENVIRONMENT: Environment = Environment::Development;

fn get_port() -> u16 {
    return env::var("PORT")
        .unwrap_or("5000".into())
        .parse::<u16>()
        .expect("Invalid port number");
}

fn main() {
    let config = Config::build(ROCKET_ENVIRONMENT).port(get_port()).unwrap();
    rocket::custom(config, true)
        .mount("/", routes![view::handle])
        .launch();
}
