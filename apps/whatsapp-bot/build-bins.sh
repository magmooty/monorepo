#!/bin/bash

echo "Running build script for WhatsApp Bot..."

# Check if the argument is provided
if [ -z "$1" ]; then
  echo "Error: No argument provided. Please specify one of the following: darwin-arm64, windows-x86_64, windows-i686, all."
  exit 1
fi

# Define build function for each target
build() {
  case $1 in
    darwin-arm64)
      echo "Building WhatsApp Bot for darwin arm64..."
      CGO_ENABLED=1 GOOS=darwin GOARCH=arm64 go build -o target/bin/whatsapp-bot-aarch64-apple-darwin bin.go
      ;;
    windows-x86_64)
      echo "Building WhatsApp Bot for Windows x86_64..."
      CGO_ENABLED=1 GOOS=windows GOARCH=amd64 CC=x86_64-w64-mingw32-gcc CXX=x86_64-w64-mingw32-g++ go build -o target/bin/whatsapp-bot-x86_64-pc-windows-msvc.exe bin.go
      ;;
    windows-i686)
      echo "Building WhatsApp Bot for Windows i686..."
      CGO_ENABLED=1 GOOS=windows GOARCH=386 CC=i686-w64-mingw32-gcc CXX=i686-w64-mingw32-g++ go build -o target/bin/whatsapp-bot-i686-pc-windows-msvc.exe bin.go
      ;;
    *)
      echo "Error: Invalid argument. Please specify one of the following: darwin-arm64, windows-x86_64, windows-i686, all."
      exit 1
      ;;
  esac
}

# Handle the "all" argument
if [ "$1" == "all" ]; then
  build darwin-arm64
  build windows-x86_64
  build windows-i686
else
  build $1
fi
