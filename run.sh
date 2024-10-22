#!/bin/bash
pushd "$(dirname $0)" &>/dev/null || exit
BUILD_IMAGE_NAME="abyss/builder"

docker run --rm \
  -v "$PWD":/data \
  -v cargo-cache:/root/.cargo/registry \
  -it "${BUILD_IMAGE_NAME}" \
  env RUST_LOG="${RUST_LOG:-debug}" MOCK="${MOCK}" cargo run --bin "${@}"

popd &>/dev/null || exit
