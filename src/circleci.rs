use reqwest::Response;
use reqwest::StatusCode;
use serde::de::{Deserialize, Deserializer};
use std::path::PathBuf;
use url::Url;
use utils::get_client;

#[derive(Debug, Deserialize, Clone)]
pub struct Artifact {
    pub path: PathBuf,
    pretty_path: PathBuf,
    node_index: u8,
    #[serde(deserialize_with = "deserialize_url")]
    pub url: Url,
}

fn deserialize_url<'a, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'a>,
{
    let raw_url = String::deserialize(deserializer)?;
    return Ok(Url::parse(&raw_url).expect(&format!("Found invalid URL: {}", raw_url)));
}

fn build_latest_asset_url(org: String, repo: String) -> Url {
    return Url::parse(&format!(
        "https://circleci.com/api/v1.1/project/github/{}/{}/latest/artifacts",
        org, repo
    )).expect("Failed to build URL");
}

pub fn get_artifacts(org: String, repo: String) -> Option<Vec<Artifact>> {
    let client = get_client();
    let url = build_latest_asset_url(org, repo);
    let mut response = client.get(url).send().expect("API Request failed");
    if response.status() == StatusCode::NotFound {
        return None;
    }
    return Some(response.json().expect("JSON parse error"));
}

pub fn fetch_artifact(artifact: Artifact) -> Response {
    return get_client()
        .get(artifact.url.as_str())
        .send()
        .expect("Failed to get artifact");
}
