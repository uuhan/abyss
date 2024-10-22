#!/bin/bash

DAY=$1

if [[ -z "${DAY}" ]]; then
  echo "未指定交易日!"
  exit -1
fi

echo "DUMP DAY:" ${DAY}

psql abyss abyss \
  -c "copy (select * from ticks where time >= '$DAY 00:00:00' and time < '$DAY 24:00:00') to stdout with (format binary);" \
  | gzip > "$DAY".bin.gz

