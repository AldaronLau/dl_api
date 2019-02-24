// dl_api
//
// Copyright (c) 2018 Jeron A. Lau
// Copyright (c) 2017 Szymon Wieloch
// Distributed under the MIT LICENSE (See accompanying file LICENSE.txt)

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

mod dl_api;
mod error;
mod library;

pub use crate::error::Error;
#[doc(hidden)]
pub use crate::library::Library; // Use in dl_api only.
