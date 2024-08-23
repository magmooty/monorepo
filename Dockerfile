# Start from the official Rust image
FROM rust:latest AS build

ENV DEBIAN_FRONTEND=noninteractive

# Install the necessary tools for cross-compilation
RUN apt-get update
RUN apt-get install -y libssl-dev pkg-config

# Install sccache
WORKDIR /usr/deps
RUN wget https://github.com/mozilla/sccache/releases/download/v0.8.1/sccache-v0.8.1-$(arch)-unknown-linux-musl.tar.gz
RUN tar -xvf sccache-v0.8.1-$(arch)-unknown-linux-musl.tar.gz
RUN mv sccache-v0.8.1-$(arch)-unknown-linux-musl/sccache /usr/local/bin/sccache

WORKDIR /usr/src

# Copy the Cargo files
COPY api-cargo.toml ./Cargo.toml
COPY Cargo.lock .
COPY ./apps/api/Cargo.toml ./apps/api/
COPY ./apps/telegram-bot/Cargo.toml ./apps/telegram-bot/
COPY ./apps/telegram-macros/Cargo.toml ./apps/telegram-macros/

# Fetch the dependencies
RUN cargo fetch

# Copy the source code
COPY ./apps/api ./apps/api
COPY ./apps/telegram-bot ./apps/telegram-bot
COPY ./apps/telegram-macros ./apps/telegram-macros

WORKDIR /usr/src/api

# Set the environment variables
ENV RUSTC_WRAPPER=sccache
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR=/usr/lib/aarch64-linux-gnu

# Build the Rust project
RUN --mount=type=cache,target=/root/.cache/sccache cargo build --release

# Prepare output image with only the exectuable binary
FROM debian:buster-slim

COPY --from=build /usr/src/target/release /app
COPY ./apps/api/binaries/linux-x86_64/libtdjson.so.1.8.29 /app/libtdjson.so.1.8.29

CMD ["app/api"]
