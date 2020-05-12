#!/bin/bash

rm lightning-c-bindings/src/{ln,util,chain,routing}/*

set -e
cd c-bindings-gen && cargo build && cd ..
GEN="$(pwd)/c-bindings-gen/target/debug/c-bindings-gen"
SRC="$(pwd)/lightning/src"
OUT="$(pwd)/lightning-c-bindings/src"
OUT_TEMPL="$(pwd)/lightning-c-bindings/src/c_types/derived.rs"
OUT_F="$(pwd)/lightning-c-bindings/include/rust_types.h"
OUT_CPP="$(pwd)/lightning-c-bindings/include/lightningpp.hpp"
echo > $OUT_F

RUST_BACKTRACE=1 $GEN $SRC/ $OUT/ lightning $OUT_TEMPL $OUT_F $OUT_CPP

PATH="$PATH:~/.cargo/bin"

cd lightning-c-bindings
cargo build
cbindgen -v --config cbindgen.toml -o include/lightning.h

# cbindgen is relatively braindead when exporting typedefs -
# it happily exports all our typedefs for private types, even with the
# generics we specified in C mode! So we drop all those types manually here.
sed -i 's/typedef LDKln.*Import.*LDKln.*;//g' include/lightning.h

gcc -Wall -g -pthread demo.c ../target/debug/liblightning.a -ldl
./a.out

g++ -Wall -g -pthread demo.cpp -L../target/debug/ -llightning -ldl
LD_LIBRARY_PATH=../target/debug/ valgrind --memcheck:leak-check=full ./a.out

clang++ -Wall -pthread demo.cpp ../target/debug/liblightning.a -ldl
./a.out
echo " C++ Bin size w/o optimization:"
ls -lha a.out
time ./a.out > /dev/null

cargo rustc -v --release -- -C lto
clang++ -Wall -flto -O2 -pthread demo.cpp ../target/release/liblightning.a -ldl
echo "C++ Bin size with only RL optimized:"
ls -lha a.out
time ./a.out > /dev/null

cargo rustc -v --release -- -C linker-plugin-lto -C lto -C link-arg=-fuse-ld=lld
clang++ -Wall -flto -O2 -pthread demo.cpp ../target/release/liblightning.a -ldl
echo "C++ Bin size with cross-language LTO:"
ls -lha a.out
time ./a.out > /dev/null
