gen-headers:
	cbindgen -v --crate lightning-c-bindings -c cbindgen.toml -o ldk_ffi.h

gen-lib:
	cargo lipo --release

gen-wasm:
	wasm-pack build -t nodejs -d $(CURRENT_DIR)/wasm $(CURRENT_DIR)

export:
	cp ./ldk_ffi.h "/Users/arik/Developer/rust-swift-ffi/ios/Swift Rust FFI/"
	cp ./include/rust_types.h "/Users/arik/Developer/rust-swift-ffi/ios/Swift Rust FFI/"
	cp ../target/universal/release/liblightning.a "/Users/arik/Developer/rust-swift-ffi/ios/Swift Rust FFI/libbindings.a"
