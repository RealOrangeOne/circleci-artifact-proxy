FROM rust:latest

COPY . /app
WORKDIR /app

RUN cargo build --release

CMD target/release/circleci-artifact-proxy

EXPOSE 5000
