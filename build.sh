#!/bin/sh
mkdir -p build
gcc   -Ofast -march=native -std=c99 -o build/libxxhash-gcc.a   -c cbits/xxhash.c
clang -Ofast -march=native -std=c99 -o build/libxxhash-clang.a -c cbits/xxhash.c

rustc -Lbuild --cfg clang --opt-level=3 --out-dir=build lib.rs --test
