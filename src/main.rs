#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::config::{Config, Environment};


#[get("/<org>/<repo>/<path>")]
fn handle(org: String, repo: String, path: String) -> String {
    return "woo".into();
}


fn main() {
    let config = Config::build(Environment::Development)
        .port(5000)
        .finalize().unwrap();
    rocket::custom(config, true).mount("/", routes![handle]).launch();
}
