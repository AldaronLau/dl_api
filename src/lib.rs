//! `dl_api` is a library for dynamically loading API's from .dll/.so/.dylib
//! files.  It's based off of `rust-dlopen`.  A lot of simplifications have
//! been made.
//!
//! It's the easiest, simplest and safest way to load dynamic (shared object) libraries!
//!
//! # Getting Started: Example
//! The code inside of the curly braces for `link!()` matches exactly with code inside of the curly
//! braces for `extern "C"`.  This makes it easy for you to turn your `extern "C"`s into `link!()`s.
//! ```no_run
//! // Shared object: either "libmylibrary.so.1", "mylibrary-1.dll" or "libMyLibrary.dylib"
//! dl_api::link!(MyApi, "libmylibrary.so.1", {
//! 	fn cFunction(param_name: *mut u32) -> u32;
//! });
//!
//! fn main() {
//! 	let api = MyApi::new().unwrap(); // unwrap the `Result`.
//!
//! 	let rtn: u32 = unsafe {
//! 		(api.cFunction)(std::ptr::null_mut())
//! 	};
//! }
//! ```
//! 

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
