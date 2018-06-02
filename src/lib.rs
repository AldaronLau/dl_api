// "dl_api" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

//! `dl_api` is a library for dynamically loading API's from .dll/.so/.dylib
//! files.  It's based off of `rust-dlopen`.  A lot of simplifications have
//! been made.
//!
//! It's the easiest, simplest and safest way to load dynamic libraries!

#[macro_use]
extern crate lazy_static;
#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate winapi;

mod library;
mod error;
mod dl_api;

#[doc(hidden)] pub use library::Library; // Use in dl_api only.
pub use error::Error;
