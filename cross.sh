#!/bin/bash
cargo build --release \
  -p catfish \
  --bin ${1:-catfish} \
  --target=x86_64-unknown-linux-gnu

STRIP=$(which x86_64-unknown-linux-gnu-strip 2>/dev/null)

if [[ -x $STRIP ]]; then
  $STRIP target/x86_64-unknown-linux-gnu/release/${1:-catfish}
fi
