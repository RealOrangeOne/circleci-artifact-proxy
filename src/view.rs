use circleci::{fetch_artifact, get_artifacts, Artifact};
use reqwest::Response;
use rocket::response::Stream;

const CHUNK_SIZE: u64 = 4096;

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
    let artifacts = get_artifacts(org, repo);
    if artifacts.is_none() {
        return None;
    }
    let artifact = filter_artifacts(artifacts.unwrap(), path);
    return match artifact {
        None => None,
        Some(a) => Some(Stream::chunked(fetch_artifact(a), CHUNK_SIZE)),
    };
}
