# Start from the official Rust image
FROM rust:latest AS build

ENV DEBIAN_FRONTEND=noninteractive

# Install the necessary tools for cross-compilation
RUN apt-get update
RUN apt-get install -y libssl-dev pkg-config

# Install libssl1.1 for tdjson library
RUN echo "deb http://deb.debian.org/debian buster main" > /etc/apt/sources.list.d/buster.list && \
  apt-get update && \
  apt-get install -y --no-install-recommends libssl1.1 && \
  rm /etc/apt/sources.list.d/buster.list && \
  apt-get clean

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

# Start from the official Rust image
FROM rust:latest

ENV DEBIAN_FRONTEND=noninteractive

# Install the necessary tools for cross-compilation
RUN apt-get update
RUN apt-get install -y libssl-dev pkg-config

# Install libssl1.1 for tdjson library
RUN echo "deb http://deb.debian.org/debian buster main" > /etc/apt/sources.list.d/buster.list && \
  apt-get update && \
  apt-get install -y --no-install-recommends libssl1.1 && \
  rm /etc/apt/sources.list.d/buster.list && \
  apt-get clean

COPY --from=build /usr/src/target/release/api* /app/
COPY --from=build /usr/src/target/release/lib* /app/
COPY --from=build /usr/src/apps/api/binaries/linux-x86_64/libtdjson.so /lib/
COPY --from=build /usr/src/apps/api/binaries/linux-x86_64/libtdjson.so.1.8.35 /lib/

CMD ["/app/api"]
