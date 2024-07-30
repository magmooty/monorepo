# Start from the official Rust image
FROM rust:latest

WORKDIR /usr/local

# Install the necessary tools for cross-compilation
RUN apt-get update
RUN apt-get install -y gcc-x86-64-linux-gnu

RUN rustup target add x86_64-unknown-linux-gnu

# Set the working directory inside the container
WORKDIR /usr/src/api

# Copy the current directory contents into the container
COPY . .

# Build the Rust project
RUN cargo build --release --verbose --target x86_64-unknown-linux-gnu

EXPOSE 4000

# Set the entry point to run the compiled binary
CMD ["./target/x86_64-unknown-linux-gnu/release/api"]
