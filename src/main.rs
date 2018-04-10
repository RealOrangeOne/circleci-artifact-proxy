#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate url;

use rocket::config::{Config, Environment};

mod circleci;
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
