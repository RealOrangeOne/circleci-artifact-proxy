use serde::de::{Deserialize, Deserializer};
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Artifact {
    path: PathBuf,
    pretty_path: PathBuf,
    node_index: u8,
    #[serde(deserialize_with = "deserialize_url")]
    image: Url,
}

fn deserialize_url<'a, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'a>,
{
    let raw_url = String::deserialize(deserializer)?;
    return Ok(Url::parse(&raw_url).expect(&format!("Found invalid URL: {}", raw_url)));
}

pub fn build_asset_url(org: String, repo: String) -> Url {
    return Url::parse(&format!(
        "https://circleci.com/api/v1.1/project/github/{}/{}/latest/artifacts",
        org, repo
    )).expect("Failed to build URL");
}
