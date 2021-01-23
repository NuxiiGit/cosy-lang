#!/bin/bash
FILE="$1"
CFILE="./bin/a.c"
BIN="./bin/a.out"
cargo run -- "$1" > "$CFILE"
clang -o "$BIN" "$CFILE"
"$BIN"
