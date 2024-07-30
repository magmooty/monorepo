# Magmooty API

## Running on Windows

- Install MingW
- Override rustup to use windows gnu

```sh
rustup override set x86_64-pc-windows-gnu
```

Run

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
