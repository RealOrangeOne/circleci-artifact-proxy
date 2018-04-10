use circleci::build_asset_url;

#[get("/<org>/<repo>/<path>")]
pub fn handle(org: String, repo: String, path: String) -> String {
    return build_asset_url(org, repo).as_str().into();
}
