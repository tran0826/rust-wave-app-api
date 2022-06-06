# build-stage
FROM rust:1.57.0 AS build-stage

WORKDIR /app

RUN USER=root cargo new wave-app
WORKDIR /app/wave-app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY . .
RUN rm ./target/release/deps/wave_app*
RUN cargo build --release
RUN cargo install diesel_cli

# production
FROM debian:buster-slim AS production
RUN apt-get update
RUN apt-get install libpq-dev -y
COPY --from=build-stage /app/wave-app/target/release/wave-app .
CMD ["./wave-app"]

# dev
FROM rust:1.57.0 AS develop
WORKDIR /app
RUN cargo install cargo-watch
RUN cargo install diesel_cli
COPY . .