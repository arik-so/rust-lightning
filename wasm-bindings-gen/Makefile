DIRECTORY := $(CURDIR)
PROJECT_ROOT := "$(DIRECTORY)/.."
WASM_SOURCE_ROOT := "$(PROJECT_ROOT)/lightning-wasm-bindings"
MANUAL_CACHE := "$(WASM_SOURCE_ROOT)/manual_cache"


SRC:="$(PROJECT_ROOT)/lightning/src"
OUT:="$(WASM_SOURCE_ROOT)/src"
OUT_TEMPL:="$(WASM_SOURCE_ROOT)/src/c_types/derived.rs"
OUT_F:="$(WASM_SOURCE_ROOT)/include/rust_types.h"
OUT_CPP:="$(WASM_SOURCE_ROOT)/include/lightningpp.hpp"

test-assignment:
	EXPO="123"; \
	TUXPO="${WASM_SOURCE_ROOT} 345"; \
	echo "$${EXPO} $${TUXPO}"

preserve-manual-generation:
	mv "$(WASM_SOURCE_ROOT)/src/c_types/mod.rs" $(MANUAL_CACHE); \
    mv "$(WASM_SOURCE_ROOT)/src/bitcoin" $(MANUAL_CACHE)

restore-manual-generation:
	mkdir -p "$(WASM_SOURCE_ROOT)/src/c_types/" ;\
	mv "$(MANUAL_CACHE)/mod.rs" "$(WASM_SOURCE_ROOT)/src/c_types"; \
    mv "$(MANUAL_CACHE)/bitcoin" "$(WASM_SOURCE_ROOT)/src"

clean_wasm_source:
	make preserve-manual-generation; \
	rm -rf "$(WASM_SOURCE_ROOT)/src" ;\
	make restore-manual-generation

generate-wasm-source: clean_wasm_source
	set -x; \
	set -e; \
    export RUST_BACKTRACE="full"; \
    ./target/debug/wasm-bindings-gen ${SRC}/ ${OUT}/ lightning ${OUT_TEMPL} ${OUT_F} ${OUT_CPP}
