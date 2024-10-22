#!/bin/bash
ls csv/* |xargs -P 4 -n 1 bash scripts/import.sh
