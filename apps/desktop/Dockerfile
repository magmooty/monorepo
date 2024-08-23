FROM ubuntu:22.04

# Set environment variables to avoid user interaction during installation
ENV DEBIAN_FRONTEND=noninteractive

# Install deps
RUN apt update
RUN apt install -y libwebkit2gtk-4.0-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
RUN apt install -y lld llvm
RUN apt install -y curl wget bash build-essential
RUN apt install -y gnupg ca-certificates
RUN apt install -y build-essential

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get install -y nodejs
RUN node -v
RUN npm -v
RUN npm i -g pnpm
RUN npm i -g yarn

# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# Install toolchain
RUN rustup install stable
RUN rustup target add x86_64-pc-windows-msvc
RUN cargo install --locked cargo-xwin

# Copy the app
WORKDIR /desktop
COPY . .

# Build
RUN pnpm i
RUN yarn tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc