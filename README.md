# ![DL API](https://raw.githubusercontent.com/AldaronLau/dl_api/master/res/icon.svg)

#### The easiest, simplest and safest way to load dynamic (shared object) libraries from Rust!

[![Build Status](https://api.travis-ci.org/AldaronLau/dl_api.svg?branch=master)](https://travis-ci.org/AldaronLau/dl_api)
[![Docs](https://docs.rs/dl_api/badge.svg)](https://docs.rs/dl_api)
[![crates.io](https://img.shields.io/crates/v/dl_api.svg)](https://crates.io/crates/dl_api)

- Macro to create a structure that dynamically loads a C API
- Works on Linux (and probably other unix) and Windows

### Roadmap to 1.0 (Future Features)
- Make it easier to load parts of API at any time (modular loading from same .so
  file).
- Support some other obscure OS's.
- Different loading macro for Rust ABI / other ABIS (possible ABI parameter to
  the macro?)
- Make sure it's perfect.

## Table of Contents
- [Getting Started](#getting-started)
   - [Example](#example)
   - [API](#api)
   - [Features](#features)
- [Upgrade](#upgrade)
- [License](#license)
   - [Contribution](#contribution)


## Getting Started
Add the following to your `Cargo.toml`.

```toml
[dependencies]
dl_api = "0.4"
```

### Example
The code inside of the curly braces for `link!()` matches exactly with code
inside of the curly braces for `extern "C"`.  This makes it easy for you to turn
your `extern "C"`s into `link!()`s.

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

### API
API documentation can be found on [docs.rs](https://docs.rs/dl_api).

### Features
There are no optional features.

## Upgrade
You can use the
[changelog](https://github.com/AldaronLau/dl_api/blob/master/CHANGELOG.md)
to facilitate upgrading this crate as a dependency.

## License
Licensed under either of
 - Apache License, Version 2.0,
   ([LICENSE-APACHE](https://github.com/AldaronLau/dl_api/blob/master/LICENSE-APACHE) or
   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
 - Zlib License,
   ([LICENSE-ZLIB](https://github.com/AldaronLau/dl_api/blob/master/LICENSE-ZLIB) or
   [https://opensource.org/licenses/Zlib](https://opensource.org/licenses/Zlib))

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Contributors are always welcome (thank you for being interested!), whether it
be a bug report, bug fix, feature request, feature implementation or whatever.
Don't be shy about getting involved.  I always make time to fix bugs, so usually
a patched version of the library will be out a few days after a report.
Features requests will not complete as fast.  If you have any questions, design
critques, or want me to find you something to work on based on your skill level,
you can email me at [jeronlau@plopgrizzly.com](mailto:jeronlau@plopgrizzly.com).
Otherwise,
[here's a link to the issues on GitHub](https://github.com/AldaronLau/dl_api/issues).
Before contributing, check out the
[contribution guidelines](https://github.com/AldaronLau/dl_api/blob/master/CONTRIBUTING.md),
and, as always, make sure to follow the
[code of conduct](https://github.com/AldaronLau/dl_api/blob/master/CODE_OF_CONDUCT.md).
