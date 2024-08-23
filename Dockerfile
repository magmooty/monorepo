# Start from the official Rust image
FROM rust:latest AS build

# Install the necessary tools for cross-compilation
RUN apt-get update
RUN apt-get install -y libssl-dev pkg-config
RUN cargo install sccache

# Set up sccache caching directory
ENV SCCACHE_DIR=/usr/local/sccache

# Create a volume for sccache cache
VOLUME ["/usr/local/sccache"]

WORKDIR /usr/src

# Copy the Cargo files
COPY api-cargo.toml ./Cargo.toml
COPY Cargo.lock .
COPY ./apps/api/Cargo.toml ./apps/api/
COPY ./apps/telegram-bot/Cargo.toml ./apps/telegram-bot/
COPY ./apps/telegram-macros/Cargo.toml ./apps/telegram-macros/
COPY ./dummy.rs ./apps/api/src/main.rs
COPY ./dummy.rs ./apps/telegram-bot/src/lib.rs
COPY ./dummy.rs ./apps/telegram-macros/src/lib.rs

# Fetch the dependencies
RUN cargo fetch

# Build dependencies
RUN cargo build --release --verbose

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

CMD ["/api"]
