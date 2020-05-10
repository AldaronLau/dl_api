# Changelog
All notable changes to DL API will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://jeronlau.tk/semver/).

## [0.4.0] - Unreleased

## [0.3.1] - 2019-03-07
### Fixed
* Breaking from edition changes for Windows (Fix not compiling on Windows).

## [0.3.0] - 2019-02-23
### Changed
* Macro `dl_api!()` renamed to `link!()`.
* The `link!()` macro now takes in the same syntax as an `extern` block.
* Uses 2018 edition now.

## [0.2.0] - 2018-06-26
### Added
* Implement `From<dl_api::Error>` for `String`.

## [0.1.0] - 2018-06-02
### Added
- An API based on wrapper from `rust-dlopen`
- Allow non\_snake\_case function names in API structs.
- Function types are automatically prepended with `unsafe extern "system"` for
making type definitions platform-independant.

### Changed
- Use regular macros, rather than procedural macros
- Simplified Container API
