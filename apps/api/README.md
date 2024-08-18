# Magmooty API

## Running on Windows

- Install MingW
- Override rustup to use windows gnu

```sh
rustup override set x86_64-pc-windows-gnu
```

- Install needed packages through Pacman
- Open MSYS64

Because we're building the API with MingW, we have to install OpenSSL the MingW way :) It's all documented in [openssl-sys](https://docs.rs/crate/openssl-sys/0.9.19).

```
pacman -S mingw-w64-x86_64-openssl pkgconf openssl-devel
```

Set environment variables in `.bashrc` or `.bash_profile`

```sh
export X86_64_PC_WINDOWS_GNU_OPENSSL_DIR="C:\\msys64\\mingw64"
```

Run through Git bash (don't use powershell)

```sh
cargo run
```

## Generating new keys

```sh
./generate-keys.sh
```

## Generating an admin token

You will need to have the private key in `keys` directory.

```sh
npm i -g jsonwebtoken
npm link jsonwebtoken
node generate-signed-token.js
```
