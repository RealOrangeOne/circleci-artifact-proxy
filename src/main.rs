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
use std::env;

mod circleci;
mod utils;

use circleci::{fetch_artifact, get_artifacts, Artifact};

const CHUNK_SIZE: u64 = 4096;

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

fn filter_artifacts(artifacts: Vec<Artifact>, path: String) -> Option<Artifact> {
    if artifacts.is_empty() {
        return None;
    }
    let filtered_artifacts: Vec<&Artifact> = artifacts
        .iter()
        .filter(|artifact| artifact.path.to_string_lossy() == path)
        .collect();
    if filtered_artifacts.is_empty() {
        return None;
    }
    return Some(filtered_artifacts[0].clone());
}

#[get("/<org>/<repo>/<path>")]
pub fn handle(org: String, repo: String, path: String) -> Option<Stream<Response>> {
    let artifacts = get_artifacts(org, repo)?;
    let artifact = filter_artifacts(artifacts, path);
    return match artifact {
        None => None,
        Some(a) => Some(Stream::chunked(fetch_artifact(a), CHUNK_SIZE)),
    };
}

fn main() {
    let config = Config::build(ROCKET_ENVIRONMENT).port(get_port()).unwrap();
    rocket::custom(config, true)
        .mount("/", routes![handle])
        .launch();
}
