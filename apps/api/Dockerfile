# Start from the official Rust image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/api

# Copy the current directory contents into the container
COPY . .

# Build the Rust project
RUN cargo build --release

EXPOSE 4000

# Set the entry point to run the compiled binary
CMD ["./target/release/api"]
