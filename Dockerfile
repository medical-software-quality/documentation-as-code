FROM rust:1.73 as build

RUN rustup target add x86_64-unknown-linux-musl

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN echo "fn main() {}" > dummy.rs && sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN mkdir tests && echo "fn main() {}" > tests/it.rs

RUN cargo build --target x86_64-unknown-linux-musl --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY ./src ./src

# build for release
RUN rm ./target/release/deps/*
RUN cargo build --target x86_64-unknown-linux-musl --release

RUN ls target/x86_64-unknown-linux-musl/release

FROM alpine:3.16.0

COPY --from=build target/x86_64-unknown-linux-musl/release/quality /usr/local/bin

ENTRYPOINT ["/usr/local/bin/quality"]
