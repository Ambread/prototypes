#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

cargo run

cd dev

gcc driver.c output.o

./a.out
