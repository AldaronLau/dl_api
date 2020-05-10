// DL API
//
// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr::NonNull;

/// Dynamically loaded library API.
#[derive(Debug)]
pub(super) struct DlApi;

impl DlApi {
    /// Load a Dynamic Library API
    pub(super) fn new(filename: &CStr) -> Option<Self> {
        let _ = filename;
        Some(DlApi)
    }

    /// Get a function or global from the library.
    pub(super) fn get(&self, symbol: &CStr) -> Option<NonNull<c_void>> {
        let _ = symbol;
        None
    }
}
