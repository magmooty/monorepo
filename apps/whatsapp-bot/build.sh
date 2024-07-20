#!/bin/bash

echo "Running build script for WhatsApp Bot..."

# Check if the argument is provided
if [ -z "$1" ]; then
  echo "Error: No argument provided. Please specify one of the following: darwin-arm64, darwin-amd64, windows-amd64, windows-i686."
  exit 1
fi

# Define commands for each case
case $1 in
  darwin-arm64)
    echo "Building WhatsApp Bot for darwin arm64..."
    mkdir -p target/darwin-arm64
    CGO_ENABLED=1 GOOS=darwin GOARCH=arm64 go build -o target/darwin-arm64/libwhatsapp.a -buildmode=c-archive main.go
    ;;
  darwin-amd64)
    echo "Building WhatsApp Bot for darwin amd64..."
    ;;
  windows-amd64)
    echo "Building WhatsApp Bot for Windows amd64..."
    # Add your Windows AMD64 specific commands here
    ;;
  windows-i686)
    echo "Building WhatsApp Bot for Windows i686..."
    # Add your Windows i686 specific commands here
    ;;
  *)
    echo "Error: Invalid argument. Please specify one of the following: macos-arm64, macos-amd64, windows-amd64, windows-i686."
    exit 1
    ;;
esac
