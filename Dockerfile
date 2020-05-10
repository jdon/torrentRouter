# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/torrentrouter

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/torrent_router*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 torrentrouter

RUN adduser -D -s /bin/sh -u 1000 -G torrentrouter torrentrouter

WORKDIR /home/torrentrouter/bin/

COPY --from=cargo-build /usr/src/torrentrouter/target/x86_64-unknown-linux-musl/release/torrent_router .

RUN chown torrentrouter:torrentrouter torrent_router

USER torrentrouter

CMD ["./torrent_router"]