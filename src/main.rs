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

mod circleci;
mod utils;
mod view;

fn main() {
    let config = Config::build(Environment::Development)
        .port(5000)
        .finalize()
        .unwrap();
    rocket::custom(config, true)
        .mount("/", routes![view::handle])
        .launch();
}
