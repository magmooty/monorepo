#!/bin/bash

echo "Running build script for WhatsApp Bot..."

# Check if the argument is provided
if [ -z "$1" ]; then
  echo "Error: No argument provided. Please specify one of the following: darwin-arm64, windows-x86_64, windows-i686."
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
    cd target/windows-x86_64
    # Generate a .def file from MingW library (.a file)
    dlltool -z whatsapplib.def --export-all-symbols libwhatsapp.a
    # Generate a .lib file from the .def file
    dlltool -m i386:x86-64 -d whatsapplib.def -l libwhatsapp.lib -A libwhatsapp.a
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
