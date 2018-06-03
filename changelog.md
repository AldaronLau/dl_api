## 0.1
* Initial version
* Have 1 API based on wrapper from `rust-dlopen`
* Use regular macros, rather than procedural macros
* Simplified Container API
* Allow non\_snake\_case function names in API structs.
* Function types are automatically prepended with `unsafe extern "system"` for
making type definitions platform-independant.