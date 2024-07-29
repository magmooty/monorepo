#!/bin/bash

echo "Running build script for WhatsApp Bot..."

# Check if the argument is provided
if [ -z "$1" ]; then
  echo "Error: No argument provided. Please specify one of the following: darwin-arm64, windows-x86_64, linux-x86_64."
  exit 1
fi

# Define commands for each case
case $1 in
  darwin-arm64)
    echo "Building WhatsApp Bot for darwin arm64..."
    mkdir -p target/darwin-arm64
    CGO_ENABLED=1 GOOS=darwin GOARCH=arm64 go build -o target/darwin-arm64/libwhatsapp.a -buildmode=c-archive main.go
    ;;
  windows-x86_64)
    echo "Building WhatsApp Bot for Windows x86_64..."
    mkdir -p target/windows-x86_64
    CGO_ENABLED=1 GOOS=windows GOARCH=amd64 go build -o target/windows-x86_64/libwhatsapp.a -buildmode=c-archive main.go
    ;;
  linux-x86_64)
    echo "Building WhatsApp Bot for Linux x86_64..."
    mkdir -p target/linux-x86_64
    CGO_ENABLED=1 GOOS=linux GOARCH=amd64 CC=x86_64-linux-musl-gcc go build -o target/linux-x86_64/libwhatsapp.a -buildmode=c-archive main.go
    ;;
  linux-arm64)
    echo "Building WhatsApp Bot for Linux arm64..."
    mkdir -p target/linux-arm64
    CGO_ENABLED=1 GOOS=linux GOARCH=arm64 CC=aarch64-linux-musl-gcc go build -o target/linux-arm64/libwhatsapp.a -buildmode=c-archive main.go
    ;;
  *)
    echo "Error: Invalid argument. Please specify one of the following: darwn-arm64, windows-x86_64, linux-x86_64, linux-arm64."
    exit 1
    ;;
esac
