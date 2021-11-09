FROM rust:latest as builder
RUN cargo install cargo-build-deps

WORKDIR /usr/app

RUN USER=root cargo init --bin --vcs none data
WORKDIR /usr/app/data

COPY Cargo.toml ./
COPY Cargo.lock ./
RUN cargo build-deps --release

COPY src/ src/
RUN cargo build --release

# ---

FROM debian:buster-slim

RUN apt-get update && apt-get install --yes openssl dumb-init && rm -rf /var/lib/apt/lists/*

ARG USER=rustuser
RUN useradd --create-home --shell /bin/bash $USER
USER $USER

ARG RUST_LOG=debug
ENV RUST_LOG=$RUST_LOG

WORKDIR /home/$USER/app

COPY --from=builder /usr/app/data/target/release/data ./

ENTRYPOINT ["dumb-init"]
CMD ["./data"]
