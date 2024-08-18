# Magmooty Desktop App

## Development

### Installing dependencies

- [Install Rust](https://www.rust-lang.org/)

Install using PNPM

```sh
pnpm install
```

### Running the app for development

Install sccache for faster builds

```sh
cargo install sccache
```

Modify your `.bashrc` or `.zshrc` to use sccache

```sh
export RUSTC_WRAPPER=sccache
```

### Windows and OpenSSL

Because we're building Tauri with MSVC, we have to install OpenSSL the Microsoft way with vcpkg :) It's all documented in [openssl-sys](https://docs.rs/crate/openssl-sys/0.9.19).

- Clone and install vcpkg somewhere on your computer. Let's say `C:/vcpkg`

```sh
git clone https://github.com/microsoft/vcpkg.git
./vcpkg.exe integrate install
./vcpkg.exe install openssl:x64-windows
./vcpkg.exe install openssl:x86-windows
```

Now, set your environment variables in `.bashrc` or `.bash_profile`:

```sh
export X86_64_PC_WINDOWS_MSVC_OPENSSL_DIR="C:\\vcpkg\\packages\\openssl_x64-windows"
export X86_PC_WINDOWS_MSVC_OPENSSL_DIR="C:\\vcpkg\\packages\\openssl_x86-windows"
```

Run the app

```sh
yarn tauri dev
```

## Building

### Building on Mac

> Make sure Wine is installed.

Install the MinGW-w64 toolchain

```sh
brew install mingw-w64
```

Install NSIS

```sh
brew install nsis
```

Linkers are defined in `.cargo/config` to use MinGW-w64 on MacOS

**Compiling for Windows 64-bit**

Add Windows 32-bit target

```
rustup target add i686-pc-windows-gnu
```

Run Tauri builder

```sh
TAURI_FIPS_COMPLIANT="true" yarn tauri build --target=i686-pc-windows-gnu
```

**Compiling for Windows 32-bit**

Add Windows 64-bit target

```
rustup target add x86_64-pc-windows-gnu
```

Run Tauri builder

```sh
TAURI_FIPS_COMPLIANT="true" yarn tauri build --target=x86_64-pc-windows-gnu
```
