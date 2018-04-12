use circleci::get_artifacts;

#[get("/<org>/<repo>/<path>")]
pub fn handle(org: String, repo: String, path: String) -> String {
    return format!("{:?}", get_artifacts(org, repo));
}
