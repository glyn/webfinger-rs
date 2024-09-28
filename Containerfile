FROM docker.io/rust:latest AS build

WORKDIR /usr/src/webfinger-rs
COPY . .

RUN cargo build --release

FROM docker.io/debian:bookworm-slim AS app

COPY --from=build /usr/src/webfinger-rs/target/release/webfinger-rs /usr/local/bin/

CMD ["webfinger-rs"]
