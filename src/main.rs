#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate url;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;

use reqwest::Response;
use rocket::config::{Config, Environment};
use rocket::response::Stream;

mod circleci;
mod utils;

const CHUNK_SIZE: u64 = 4096;

#[cfg(not(debug_assertions))]
const ROCKET_ENVIRONMENT: Environment = Environment::Production;

#[cfg(debug_assertions)]
const ROCKET_ENVIRONMENT: Environment = Environment::Development;

#[get("/<org>/<repo>/<path>")]
pub fn get_latest_asset(org: String, repo: String, path: String) -> Option<Stream<Response>> {
    let url = circleci::build_asset_url(org, repo);
    let artifacts = circleci::get_artifacts_from(url)?;
    let artifact = utils::filter_artifacts(artifacts, path);
    return match artifact {
        None => None,
        Some(a) => Some(Stream::chunked(circleci::fetch_artifact(a), CHUNK_SIZE)),
    };
}

fn main() {
    let config = Config::build(ROCKET_ENVIRONMENT)
        .port(utils::get_port())
        .unwrap();
    rocket::custom(config, true)
        .mount("/", routes![get_latest_asset])
        .launch();
}
