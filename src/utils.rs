use reqwest::{ClientBuilder, Client};
use reqwest::header;
use reqwest::mime;

pub fn get_client() -> Client {
    let mut headers = header::Headers::new();
    headers.set(header::Accept(vec![
        header::qitem(mime::APPLICATION_JSON)
    ]));

    return ClientBuilder::new()
        .gzip(true)
        .default_headers(headers)
        .build()
        .expect("Failed to build client");
}
