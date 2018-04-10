use url::Url;

pub fn build_asset_url(org: String, repo: String) -> Url {
    return Url::parse(&format!(
        "https://circleci.com/api/v1.1/project/github/{}/{}/latest/artifacts",
        org, repo
    )).expect("Failed to build URL");
}
