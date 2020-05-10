// DL API
//
// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ffi::CStr;
use std::os::raw::{c_void, c_char, c_int};
use std::ptr::NonNull;

#[repr(transparent)]
struct DlObj(c_void);

/// Dynamically loaded library API.
#[derive(Debug)]
pub(super) struct DlApi(NonNull<DlObj>);

impl DlApi {
    /// Load a Dynamic Library API.
    pub(super) fn new(filename: &CStr) -> Option<Self> {
        extern "C" {
            fn dlopen(name: *const c_char, flags: c_int) -> *mut DlObj;
        }
        Some(DlApi(unsafe {
            NonNull::new(dlopen(filename.as_ptr(), 0x00002 /*NOW*/))?
        }))
    }

    /// Get a function or global from the library.
    pub(super) fn get(&self, symbol: &CStr) -> Option<NonNull<c_void>> {
        extern "C" {
            fn dlsym(dlobj: *mut DlObj, symbol: *const c_char) -> *mut c_void;
        }
        unsafe { NonNull::new(dlsym(self.0.as_ptr(), symbol.as_ptr())) }
    }
}
