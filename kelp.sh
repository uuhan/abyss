#!/bin/bash
NAME=$(basename $0)
NAME=${NAME/.sh/}
pushd $(dirname $0) &>/dev/null

rm -f ${NAME}.node
cargo build -p ${NAME}

[[ -e target/debug/lib${NAME}.dylib ]] && cp target/debug/lib${NAME}.dylib ${NAME}.node
[[ -e target/debug/lib${NAME}.so ]] && cp target/debug/lib${NAME}.so ${NAME}.node

export RUST_LOG=${RUST_LOG-=info}
node --napi-modules ${NAME}.js

popd &>/dev/null
