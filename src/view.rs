use circleci::{get_artifacts, Artifact};

fn filter_artifacts(artifacts: Vec<Artifact>, path: String) -> Option<Artifact> {
    if artifacts.is_empty() {
        return None;
    }
    let filtered_artifacts: Vec<&Artifact> = artifacts.iter().filter(|artifact| artifact.path.to_string_lossy() == path).collect();
    if filtered_artifacts.is_empty() {
        return None;
    }
    return Some(filtered_artifacts[0].clone());



}

#[get("/<org>/<repo>/<path>")]
pub fn handle(org: String, repo: String, path: String) -> Option<String> {
    let artifacts = get_artifacts(org, repo);
    let artifact = filter_artifacts(artifacts, path);
    return match artifact {
        None => None,
        Some(a) => Some(format!("{:?}", a))
    };
}
