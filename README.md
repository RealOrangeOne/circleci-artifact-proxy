# CircleCI Asset Downloader

[![CircleCI](https://circleci.com/gh/RealOrangeOne/circleci-asset-downloader.svg?style=svg)](https://circleci.com/gh/RealOrangeOne/circleci-asset-downloader)

Download the latest asset available from a CircleCI build. 

One of CircleCI's greatest features is artifacts, files saved from builds which can then be downloaded after the build. However, the URLs aren't very friendly or discoverable, and there's no easy way to 'just get the latest'.

## Installation
### Requirements
- `rust`
- `cargo`

```bash
cargo run --release
```

The default is to listen on port `5000`, unless `$PORT` is defined.

## Usage
The URLs take the structure `<org>/<repo>/<path>`, meaning `RealOrangeOne/circleci-asset-downloader/subdir/file.rs` will serve a file from this repo, saved to `subdir/file.rs` in the assets. 

The files are streamed from CircleCI's servers, through this, to the client, rather than redirecting, in an attempt to keep cleaner urls.

