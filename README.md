# ![DL_API](https://free.plopgrizzly.com/dl_api/icon.svg)
[![Build Status](https://travis-ci.com/plopgrizzly/dl_api.svg?branch=master)](https://travis-ci.com/plopgrizzly/dl_api)

The easiest, simplest and safest way to load dynamic (shared object) libraries from Rust!

## Features
* Macro to create a structure that dynamically loads a C API
* Works on Unix and Windows

## Roadmap to 1.0 (Future Features)
* Make it easier to load parts of API at any time (modular loading from same .so file).
* Support some other obscure OS's.
* Different loading macro for Rust ABI / other ABIS (possible ABI parameter to the macro?)
* Make sure it's perfect.

## Getting Started: Example
The code inside of the curly braces for `link!()` matches exactly with code inside of the curly
braces for `extern "C"`.  This makes it easy for you to turn your `extern "C"`s into `link!()`s.
```rust
// Shared object: either "libmylibrary.so.1", "mylibrary-1.dll" or "libMyLibrary.dylib"
dl_api::link!(MyApi, "libmylibrary.so.1", {
	fn cFunction(param_name: ParamType) -> ReturnType;
});

fn main() {
	let api = MyApi::new().unwrap(); // unwrap the `Result`.

	let rtn: ReturnType = unsafe {
		(api.cFunction)(0);
	};
}
```

## Links
* [Website](https://free.plopgrizzly.com/dl_api)
* [Cargo](https://crates.io/crates/dl_api)
* [Documentation](https://docs.rs/dl_api)
* [Change Log](https://free.plopgrizzly.com/dl_api/changelog)
* [Contributing](https://plopgrizzly.com/contributing)
* [Code of Conduct](https://free.plopgrizzly.com/dl_api/codeofconduct)

---

[![Plop Grizzly](https://plopgrizzly.com/images/logo-bar.png)](https://plopgrizzly.com)
