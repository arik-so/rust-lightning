xgen-headers:
	cbindgen -v ./ -c cbindgen.toml -o ffi.h

gen-headers:
	cbindgen -v --crate bindings -c cbindgen.toml -o ffi.h

gen-lib:
	cargo lipo --release