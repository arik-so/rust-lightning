gen-headers:
	cbindgen -v --crate bindings -c cbindgen.toml -o ldk_ffi.h

gen-lib:
	cargo lipo --release

gen-wasm:
	wasm-pack build -t nodejs -d $(CURRENT_DIR)/wasm $(CURRENT_DIR)