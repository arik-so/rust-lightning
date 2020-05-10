#!/bin/bash

set -e
cd c-bindings-gen && cargo build && cd ..
GEN="$(pwd)/c-bindings-gen/target/debug/c-bindings-gen"
SRC="$(pwd)/lightning/src"
OUT="$(pwd)/lightning-c-bindings/src"
OUT_F="$(pwd)/lightning-c-bindings/include/rust_types.h"
echo > $OUT_F

gen_bindings() {
	MOD=""
	FULL_PATH=""
	for SEGMENT in "$@"; do
		[ "$MOD" != "" ] && MOD="$MOD::"
		MOD="$MOD$SEGMENT"
		FULL_PATH="$FULL_PATH/$SEGMENT"
	done
	for FILE in *; do
		if [ -d "$FILE" ]; then
			cd $FILE
			gen_bindings "$@" "$FILE"
			cd ..
		elif [ -f "$FILE" -a "$FILE" != "mod.rs" -a "$FILE" != "lib.rs" ]; then
			# For now just whitelist certain files for generation:
			WHITELIST=0
			[ "$FILE" = "features.rs" ] && WHITELIST=1
			[ "$FILE" = "config.rs" ] && WHITELIST=1
			[ "$FILE" = "events.rs" ] && WHITELIST=1
			[ "$FILE" = "logger.rs" ] && WHITELIST=1
			[ "$FILE" = "peer_handler.rs" ] && WHITELIST=1
			[ "$FILE" = "msgs.rs" ] && WHITELIST=1
			[ "$FILE" = "chaininterface.rs" ] && WHITELIST=1
			[ "$FILE" = "chan_utils.rs" ] && WHITELIST=1
			[ "$FILE" = "channelmanager.rs" ] && WHITELIST=1
			[ "$FILE" = "channelmonitor.rs" ] && WHITELIST=1
			[ "$FILE" = "keysinterface.rs" ] && WHITELIST=1
			if [ "$WHITELIST" = 1 ]; then
				echo "Generating $@ $FILE"
				FMOD=$(basename $FILE .rs)
				RUST_BACKTRACE=1 $GEN $SRC/$FULL_PATH/$FILE lightning $MOD::$FMOD $OUT_F > $OUT/$FULL_PATH/$FILE
			fi
		fi
	done
}

cd lightning/src
gen_bindings
cd ../..

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
