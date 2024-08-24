#!/bin/bash

echo "Running build script for TDLib..."

# Check if the argument is provided
if [ -z "$1" ]; then
  echo "Error: No argument provided. Please specify one of the following: linux-x86_64, linux-arm64, all."
  exit 1
fi

build() {
  case $1 in
  linux-x86_64)
    echo "Building TDLib for Linux x86_64..."
    docker build --platform=linux/amd64 -t magmooty-tdlib-builder:linux-x86_64 -f linux-x86_64-dockerfile .
    mkdir -p target/linux-x86_64
    docker run -it --name magmooty-tdlib-builder-linux-x86_64 --rm -v $(pwd)/target/linux-x86_64:/usr/tdlib/td/tdlib magmooty-tdlib-builder:linux-x86_64
    ;;
  linux-arm64)
    echo "Building TDLib for Linux arm64..."
    docker build -t magmooty-tdlib-builder:linux-arm64 -f linux-arm64-dockerfile .
    mkdir -p target/linux-arm64
    docker run -it --name magmooty-tdlib-builder-linux-arm64 --rm -v $(pwd)/target/linux-arm64:/usr/tdlib/td/tdlib magmooty-tdlib-builder:linux-arm64
    ;;
  *)
    echo "Error: Invalid argument. Please specify one of the following: linux-x86_64, linux-arm64."
    exit 1
    ;;
  esac
}

if [ "$1" = "all" ]; then
  build linux-x86_64
  build linux-arm64
else
  build $1
fi
