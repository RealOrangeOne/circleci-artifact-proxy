use circleci::Artifact;
use reqwest::header;
use reqwest::{Client, ClientBuilder};
use std::env;

pub fn get_port() -> u16 {
    return env::var("PORT")
        .unwrap_or("5000".into())
        .parse::<u16>()
        .expect("Invalid port number");
}

pub fn get_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );

    return ClientBuilder::new()
        .gzip(true)
        .default_headers(headers)
        .build()
        .expect("Failed to build client");
}

pub fn filter_artifacts(artifacts: Vec<Artifact>, path: String) -> Option<Artifact> {
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

pub fn is_valid_build_num(build: &String) -> bool {
    if build == "latest" {
        return true;
    }
    return build.parse::<u32>().is_ok();
}
