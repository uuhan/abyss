#!/bin/bash
pushd "$(dirname $0)" &>/dev/null || exit

RELEASE=
BUILD_IMAGE_NAME="abyss/builder"
BUILD_IMAGE=
FORCE_RE_CREATE_BUILDER=

usage() {
  echo "build.sh: [-e] [-r] [-f] [-b] [-h | --help]"
  exit 0
}

build_image() {
  if [[ -n "${FORCE_RE_CREATE_BUILDER}" ]]; then
    docker rmi "$BUILD_IMAGE_NAME" &>/dev/null
  fi

  docker build --rm -t "${BUILD_IMAGE_NAME}" docker/builder
}

ARGS=$(getopt erfb "$@")
eval set -- "${ARGS}"

while :; do
  case $1 in
    -r)
      RELEASE=1
      shift
      ;;
    -e)
      EXEC_IMAGE=1
      shift
      ;;
    -f)
      FORCE_RE_CREATE_BUILDER=1
      shift
      ;;
    -b)
      BUILD_IMAGE=1
      echo "build image"
      shift
      ;;
    -h|--help)
      usage
      ;;
    --)
      shift
      break
      ;;
    *) usage;
  esac
done

main() {
  if [[ -n "${BUILD_IMAGE}" ]]; then
    build_image
    exit
  fi

  if [[ -n "${RELEASE}" ]]; then
    docker run --rm \
      -v "$PWD":/data \
      -v cargo-cache:/root/.cargo/registry \
      -it "${BUILD_IMAGE_NAME}" \
      cargo build --release --bin catfish --bin database
  elif [[ -n "${EXEC_IMAGE}" ]]; then
    docker run --rm \
      -v "$PWD":/data \
      -v cargo-cache:/root/.cargo/registry \
      -it "${BUILD_IMAGE_NAME}" \
      bash
  else
    docker run --rm \
      -v "$PWD":/data \
      -v cargo-cache:/root/.cargo/registry \
      -it "${BUILD_IMAGE_NAME}" \
      cargo build --bin catfish --bin database
  fi
}

main

popd &>/dev/null || exit
