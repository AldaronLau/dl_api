// "dl_api" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use Error;
use std::ffi::{CStr, CString, OsStr};

//choose the right platform implementation here
#[cfg(unix)]
use super::unix::{close_lib, get_sym, open_lib, Handle};
#[cfg(windows)]
use super::windows::{close_lib, get_sym, open_lib, Handle};

use std::mem::{size_of, transmute_copy};

#[derive(Debug)]
pub struct Library {
	handle: Handle,
}

impl Library {
	pub fn new<S>(name: S) -> Result<Library, Error>
	where
		S: AsRef<OsStr>,
	{
		Ok(Self {
			handle: unsafe { open_lib(name.as_ref()) }?,
		})
	}

	pub unsafe fn symbol<T>(&self, name: &str) -> Result<T, Error> {
		let cname = CString::new(name)?;
		self.symbol_cstr(cname.as_ref())
	}

	pub unsafe fn symbol_cstr<T>(&self, name: &CStr) -> Result<T, Error> {
		//TODO: convert it to some kind of static assertion (not yet supported in Rust)
		//this comparison should be calculated by compiler at compilation time - zero cost
		if size_of::<T>() != size_of::<*mut ()>() {
			panic!(
				"The type passed to dlopen::Library::symbol() function has a different size than a \
				 pointer - cannot transmute"
			);
		}
		let raw = get_sym(self.handle, name)?;
		if raw.is_null() {
			return Err(Error::NullSymbol);
		} else {
			Ok(transmute_copy(&raw))
		}
	}
}

impl Drop for Library {
	fn drop(&mut self) {
		self.handle = close_lib(self.handle);
	}
}

unsafe impl Sync for Library {}
unsafe impl Send for Library {}
