# Start from the official Rust image
FROM rust:latest AS build

# Install the necessary tools for cross-compilation
RUN apt-get update
RUN apt-get install -y libssl-dev pkg-config
RUN cargo install sccache

WORKDIR /usr/src

COPY ./api ./api
COPY ./telegram-bot ./telegram-bot
COPY ./telegram-macros ./telegram-macros

WORKDIR /usr/src/api

ENV RUSTC_WRAPPER=sccache
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR=/usr/lib/aarch64-linux-gnu

# Build the Rust project
RUN cargo build --release --verbose

# Prepare output image with only the exectuable binary
FROM gcr.io/distroless/static-debian11

COPY --from=build /usr/src/api/target/release /

CMD ["/api"]
