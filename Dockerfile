FROM termoshtt/rust:v1.19.0
MAINTAINER termoshtt <toshiki.teramura@gmail.com>
COPY Cargo.toml /source/
COPY src/ /source/src/
RUN cargo build --release
ENTRYPOINT cargo run --release --bin worker
