#!/bin/bash

DAY=$1

if [[ -z "${DAY}" ]]; then
  echo "NO TRADING DAY!"
  exit -1
fi

echo "MERGE DAY:" ${DAY}

zcat "$DAY".bin.gz |psql abyss abyss \
  -c "copy ticks from stdin with (format binary)"

