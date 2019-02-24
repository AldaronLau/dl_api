# [dl_api](https://crates.io/crates/dl_api)
A small, simple, safe-ish dynamic loading library for loading C libraries from
Rust.

## Features
* Macro to create a structure that dynamically loads a C API
* Works on Unix and Windows

## [Contributing](http://plopgrizzly.com/contributing/en#contributing)

## Roadmap to 1.0 (Future Features)
* Make it easier to load parts of API at any time.
* Support Nintendo Switch (for the asi_vulkan crate).
* Probably support some other OS's.
* Different loading macro for Rust ABI / other ABIS (possible ABI parameter to the macro?)
* Make sure it's perfect.

## Change Log
### 0.2
* `String` now implements `From<dl_api::Error>`.

### 0.1
* Initial version
* Have 1 API based on wrapper from `rust-dlopen`
* Use regular macros, rather than procedural macros
* Simplified Container API
* Allow non\_snake\_case function names in API structs.
* Function types are automatically prepended with `unsafe extern "system"` for
making type definitions platform-independant.

## Developed by [Plop Grizzly](http://plopgrizzly.com)
