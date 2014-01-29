#!/bin/sh
mkdir -p build
gcc -Ofast -march=native -std=c99 -o build/libxxhash.a -c cbits/xxhash.c
rustc -Lbuild --opt-level=3 --out-dir=build lib.rs --test
