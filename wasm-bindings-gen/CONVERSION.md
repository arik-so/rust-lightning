# Conversion Approaches

## Closures

- Discard `this_arg`
- Create constructors to pass `js_sys::Function` instances
- Create unexposed implementations to function as adapters for calling js_sys::Function instances

### Example

```rust
#[wasm_bindgen]
pub struct WasmLogger {
	// cannot be public because js_sys::Function does not implement Copy
	wasm_log: js_sys::Function
}

#[wasm_bindgen]
impl WasmLogger {
	// We therefore need a constructor
	pub fn new(wasm_log: js_sys::Function) -> Self {
		WasmLogger { wasm_log }
	}

	// instead, log gets called from Rust
	fn log(&self, a: u32, b: u32) {
		let this = JsValue::null();
		let x = JsValue::from(a);
		let y = JsValue::from(b);
		// and log calls self.wasm_log
		let sum: std::result::Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> = self.wasm_log.call2(&this, &x, &y);
	}
}
```

Note that the context (`this`) always needs to be passed. The number in the `call2` method signals the number of
arguments (not counting `this`).

### Testing only

```rust
#[wasm_bindgen]
pub fn trigger_logger(logger: &WasmLogger) {
	logger.log(23, 34)
}
```

## External methods returning instances

- Mark returned types as pointers (*const Value)
- Wrap returns in Box::into_raw(Box::new(value))

## Structs with exposed public fields of unsupported types

- Create wrapper struct with unexposed inner field
- Create getters and setters that do some conversion work

## Generics

- Completely unsupported in WASM
- Copy generic types used in derived.rs into completely concretized versions that do not rely on generics

### Example

Insead of using

```rust
pub struct GenericTemplate<A, B> {
	first: A,
	second: B
}

pub type ConcretizedGeneric = GenericTemplate<String, String>;
```

Use

```rust
#[wasm_bindgen]
pub struct ConcreteTemplate {
	first: String,
	second: String
}
```

## Results and Options

In principle, `Result<T, JsValue>` should be supported, but it doesn't really work. The approach that works is wrapping
the result in a struct that contains it (unexposed), and a flag whether it's good. A getter is implemented for the `Ok`
and `Err` values each. The `Ok` and `Err` types both need to implement `Copy` and `Clone`.

### Example A

```rust
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct SomeGoodStruct {}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct SomeBadStruct {}

#[wasm_bindgen]
pub struct WasmResultExample {
	contents: Result<SomeGoodStruct, SomeBadStruct>,
	pub is_result_ok: bool,
}

#[wasm_bindgen]
impl WasmResultExample {
	pub fn get_ok(&self) -> SomeGoodStruct {
		self.contents.ok().unwrap()
	}

	pub fn get_err(&self) -> SomeBadStruct {
		self.contents.err().unwrap()
	}
}
```

### Example B

Alternatively, we can use pointers to the values and retrieve them thus.

```rust
#[wasm_bindgen]
pub struct WasmResultExample {
	ok_value: *mut SomeGoodStruct,
	err_value: *mut SomeBadStruct,
	pub is_result_ok: bool,
}

#[wasm_bindgen]
impl WasmResultExample {
	pub fn get_ok(&self) -> SomeGoodStruct {
		let ok_value = unsafe { Box::from_raw(self.ok_value) };
		*ok_value
	}

	pub fn get_err(&self) -> SomeBadStruct {
		let err_value = unsafe { Box::from_raw(self.err_value) };
		*err_value
	}
}
```

With this approach, the types don't need to derive the `Clone` and `Copy` traits.

## Structs containing Strings

`String` fields cannot be public. If a struct contains a string that needs to be publicly accessible, the string needs
to be private, and a getter needs to be set up.

Further, the getter cannot return a `String` type. Instead it must return a `JsValue`.

### Example

```rust
#[wasm_bindgen]
pub struct SomeStructWithAString {
	string: String
}

#[wasm_bindgen]
impl SomeStructWithAString {
	pub fn get_string(&self) -> JsValue {
		JsValue::from(self.string.clone())
	}
}
```

## Instance Methods

Unlike C, WASM support instance methods, so a bunch of work such as keeping track of instances is simplified (especially
with closures). Therefore, rather than creating top-level methods, `impl` can be used instead with a `#[wasm_bindgen]`
annotation.

## Complex JsValues

For complex data structures that are purely meant for the transfer of data, and where
no keeping track of instances is necessary (e. g. arguments to callbacks), we can
use Serde. We need to require the necessary functionality:

```rust
extern crate serde;
use serde::{Serialize, Deserialize};
```

The data structure to be passed needs to have some methods derived:

```rust
#[derive(Serialize, Deserialize)]
struct ComplexValue {
	first: u32,
	second: u16,
	int_array: Vec<u8>,
	string_array: Vec<String>
}
```

And then we can can instantiate `JsValue` instances per the following example.

### Example A

```rust
#[wasm_bindgen]
pub fn create_complex_value() -> JsValue {
	let value = ComplexValue {
		first: 2,
		second: 13,
		int_array: vec![7, 10, 12, 0, 5],
		string_array: vec!["first".to_string(), "second".to_string()]
	};
	JsValue::from_serde(&value).unwrap()
}
```

### Example B

```rust
#[wasm_bindgen]
impl WasmLogger {
	fn log(&self) {
		let this = JsValue::null();
		let value = ComplexValue {
			first: 2,
			second: 13,
			int_array: vec![7, 10, 12, 0, 5],
			string_array: vec!["first".to_string(), "second".to_string()]
		};
		let js_value = JsValue::from_serde(&value).unwrap();
		self.wasm_log.call1(&this, &js_value);
	}
}
```

Note that this serialization approach is rather circuitous and can therefore be suboptimal.

## Byte Arrays

Returning byte arrays that JavaScript sees as `UInt8Array`s is a simple conversion
easily supported by both integer slices and `Vec<u8>`s.

### Example

```rust
#[wasm_bindgen]
pub fn create_array() -> js_sys::Uint8Array {
	let byte_array = vec![1, 2, 3];
	byte_array.as_slice().into()
}
```

## Nested Struct Data

`wasm_bindgen` doesn't take kindly to anything but primitives in exposed structs.
For example, if you would like to have a struct with the following fields:

```rust
#[wasm_bindgen]
pub struct NestedDataContainer {
	pub small_number: u8,
	pub big_number: f64,
	enum_value: SomeEnumType,
	nested_data: SomeNestedDataType
}
```

`SomeEnumType` and `SomeNestedDataType` cannot be public. Instead, they require getters.

### Example: Enum

To return the enum type, it is sufficient to have a `#[wasm_bindgen]` annotation
and a clone derivation:

```rust
#[wasm_bindgen]
#[derive(Clone)]
pub enum SomeEnumType {
	Foo,
	Bar,
}

#[wasm_bindgen]
impl NestedDataContainer {
	pub fn get_enum_value(&self) -> SomeEnumType {
		self.enum_value.clone()
	}
}
```

### Example: Struct

For the nested struct, multiple approaches can be pursued. One potential approach,
if keeping track of the instance is not necessary, and just the data needs to be exposed,
is using the approach outlined above for [Complex JsValues](#complex-jsvalues).

```rust
#[derive(Serialize, Deserialize)]
pub struct NestedData {
	pub nested_number: u8
}

#[wasm_bindgen]
impl NestedDataContainer {
	pub fn get_nested_data(&self) -> JsValue {
		JsValue::from_serde(&self.nest).unwrap()
	}
}
```

Alternatively, if the actual instance needs to be returned, please refer to the examples
outlined in [Results and Options](#results-and-options), especially its Example B.
