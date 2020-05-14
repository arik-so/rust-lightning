#!/bin/bash

rm lightning-c-bindings/src/{ln,util,chain,routing}/*
set -e
cd c-bindings-gen && cargo build && cd ..
GEN="$(pwd)/c-bindings-gen/target/debug/c-bindings-gen"
SRC="$(pwd)/lightning/src"
OUT="$(pwd)/lightning-c-bindings/src"
OUT_F="$(pwd)/lightning-c-bindings/include/rust_types.h"
echo > $OUT_F

RUST_BACKTRACE=1 $GEN $SRC/ $OUT/ lightning $OUT_F

cd lightning-c-bindings
cargo build
PATH="$PATH:~/.cargo/bin"
cbindgen -v --config cbindgen.toml -o include/lightning.h
gcc -static -pthread demo.c ../target/debug/liblightning.a -ldl
echo "Bin size w/o optimization:"
ls -lha a.out

cargo rustc -v --release -- -C lto
clang -flto -O2 -static -pthread demo.c ../target/release/liblightning.a -ldl
echo "Bin size with only RL optimized:"
ls -lha a.out

cargo rustc -v --release -- -C linker-plugin-lto -C lto
clang -flto -O2 -static -pthread demo.c ../target/release/liblightning.a -ldl
echo "Bin size with cross-language LTO:"
ls -lha a.out
