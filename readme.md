# dl_api
A small, simple, safe-ish dynamic loading library for loading C libraries from Rust.

[Cargo](https://crates.io/crates/dl_api) /
[Documentation](https://docs.rs/dl_api) /
[Change Log](http://plopgrizzly.com/dl_api/changelog.html)

## Supports
* Windows
* Unix

## Roadmap to 1.0
* Make it easier to load parts of API at any time.
* Support Nintendo Switch (for the asi_vulkan crate).
* Probably support some other OS's.
* Different loading macro for Rust ABI / other ABIS (possible ABI parameter to the macro?)
* Make sure it's perfect.

# Contributing
If you'd like to help implement functions for unsupported platforms, fix bugs,
improve the API or improve the Documentation, then contact me at
jeron.lau@plopgrizzly.com. I'll appreciate any help.