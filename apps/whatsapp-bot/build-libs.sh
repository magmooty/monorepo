#!/bin/bash

echo "Running build script for WhatsApp Bot..."

# Check if the argument is provided
if [ -z "$1" ]; then
  echo "Error: No argument provided. Please specify one of the following: darwin-arm64, windows-x86_64, linux-x86_64, linux-arm64, all."
  exit 1
fi

build() {
  case $1 in
    darwin-arm64)
      echo "Building WhatsApp Bot for darwin arm64..."
      mkdir -p target/lib/darwin-arm64
      CGO_ENABLED=1 GOOS=darwin GOARCH=arm64 go build -o target/lib/darwin-arm64/libwhatsapp.a -buildmode=c-archive lib.go
      ;;
    windows-x86_64)
      echo "Building WhatsApp Bot for Windows x86_64..."
      mkdir -p target/lib/windows-x86_64
      CGO_ENABLED=1 GOOS=windows GOARCH=amd64 CC=x86_64-w64-mingw32-gcc CXX=x86_64-w64-mingw32-g++ go build -o target/lib/windows-x86_64/libwhatsapp.a -buildmode=c-archive lib.go
      ;;
    linux-x86_64)
      echo "Building WhatsApp Bot for Linux x86_64..."
      mkdir -p target/lib/linux-x86_64
      CGO_ENABLED=1 GOOS=linux GOARCH=amd64 CC=x86_64-linux-musl-gcc go build -o target/lib/linux-x86_64/libwhatsapp.a -buildmode=c-archive lib.go
      ;;
    linux-arm64)
      echo "Building WhatsApp Bot for Linux arm64..."
      mkdir -p target/lib/linux-arm64
      CGO_ENABLED=1 GOOS=linux GOARCH=arm64 CC=aarch64-linux-musl-gcc go build -o target/lib/linux-arm64/libwhatsapp.a -buildmode=c-archive lib.go
      ;;
    *)
      echo "Error: Invalid argument. Please specify one of the following: darwin-arm64, windows-x86_64, linux-x86_64, linux-arm64."
      exit 1
      ;;
  esac
}

if [ "$1" = "all" ]; then
  build darwin-arm64
  build windows-x86_64
  build linux-x86_64
  build linux-arm64
else
  build $1
fi
