# CircleCI Artifact Proxy

[![CircleCI](https://circleci.com/gh/RealOrangeOne/circleci-artifact-proxy.svg?style=svg)](https://circleci.com/gh/RealOrangeOne/circleci-artifact-proxy)

Download artifacts available from a CircleCI build.

One of CircleCI's greatest features is artifacts, files saved from builds which can then be downloaded after the build. However, the URLs aren't very friendly or discoverable, and there's no easy way to 'just get the latest'.

## Installation
### Requirements
- `rust`
- `cargo`

```bash
cargo run  # For local development
cargo run --release  # For deployment
```

The default is to listen on port `5000`, unless `$PORT` is defined.

## Usage
The URLs take the structure `<org>/<repo>/<build>/<path>`, meaning `RealOrangeOne/circleci-artifact-proxy/subdir/file.rs` will serve a file from this repo, saved to `subdir/file.rs` in the artifacts, of the 9th build. You can automatically serve the latest build by using `latest` as a build number.

The files are streamed from CircleCI's servers, through this, to the client, rather than redirecting, in an attempt to keep cleaner urls. If any of the values are invalid (no organisation, no repo, invalid build id, missing build, missing path), a `404` is returned.

