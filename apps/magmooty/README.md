# Magmooty Desktop App

## Development

### Installing dependencies

- [Install Rust](https://www.rust-lang.org/)

Install using PNPM

```sh
pnpm install
```

### Running the app for development

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
