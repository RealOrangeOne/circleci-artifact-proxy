# circleci-asset-downloader

[![CircleCI](https://circleci.com/gh/RealOrangeOne/circleci-asset-downloader.svg?style=svg)](https://circleci.com/gh/RealOrangeOne/circleci-asset-downloader)

Download the latest asset available from a CircleCI build. The files are streamed from CircleCI's servers, through this, to the client, rather than redirecting, in an attempt to keep cleaner urls.

## Installation
### Requirements
- `rust`
- `cargo`


    cargo run --release

The default is to listen on port `5000`, unless `$PORT` is defined.

## Usage
The URLs take the structure `<org>/<repo>/<path>`, meaning `RealOrangeOne/circleci-asset-downloader/subdir/file.rs` will serve a file from this repo, saved to `subdir/file.rs` in the assets.

