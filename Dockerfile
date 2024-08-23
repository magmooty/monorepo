# Start from the official Rust image
FROM rust:latest AS build

# Install the necessary tools for cross-compilation
RUN apt-get update
RUN apt-get install -y libssl-dev pkg-config

# Install sccache
WORKDIR /usr/deps
RUN wget https://github.com/mozilla/sccache/releases/download/v0.8.1/sccache-v0.8.1-x86_64-unknown-linux-musl.tar.gz
RUN tar -xvf sccache-v0.8.1-x86_64-unknown-linux-musl.tar.gz
RUN mv sccache-v0.8.1-x86_64-unknown-linux-musl/sccache /usr/local/bin/sccache

ARG SCCACHE_GHA_ENABLED
ARG ACTIONS_CACHE_URL
ARG ACTIONS_RUNTIME_TOKEN

RUN echo $ACTIONS_CACHE_URL

ENV SCCACHE_LOG=debug

RUN touch /root/.config/sccache/config
RUN echo "[cache]" > /root/.config/sccache/config
RUN echo "cache_size=2" > /root/.config/sccache/config
RUN echo "dir=/usr/sccache" > /root/.config/sccache/config

RUN sccache --start-server
RUN sccache --show-stats

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
RUN cargo build --release --verbose

# Prepare output image with only the exectuable binary
FROM gcr.io/distroless/static-debian11

COPY --from=build /usr/src/target/release /

CMD ["./api"]
