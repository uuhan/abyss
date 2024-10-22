#!/bin/bash
pushd "$(dirname $0)" &>/dev/null || exit
BUILD_IMAGE_NAME="abyss/builder"

docker run --rm \
  -v "$PWD":/data \
  -v cargo-cache:/root/.cargo/registry \
  -it "${BUILD_IMAGE_NAME}" \
  cargo test "${@}"

popd &>/dev/null || exit
