# dl_api 0.3.0
* Macro `dl_api!()` renamed to `link!()`.
* The `link!()` macro now takes in the same syntax as an `extern` block.
* Uses 2018 edition now.

# dl_api 0.2.0
* `String` now implements `From<dl_api::Error>`.

# dl_api 0.1.0
* Initial version
* Have 1 API based on wrapper from `rust-dlopen`
* Use regular macros, rather than procedural macros
* Simplified Container API
* Allow non\_snake\_case function names in API structs.
* Function types are automatically prepended with `unsafe extern "system"` for
making type definitions platform-independant.
