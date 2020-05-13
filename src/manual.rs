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

use crate::ffi::DlApi as DlApiNative;

/// Dynamically loaded library API.
#[derive(Debug)]
pub struct DlApi(DlApiNative);

impl DlApi {
    /// Load a Dynamic Library API
    pub fn new(filename: &CStr) -> Option<Self> {
        Some(DlApi(DlApiNative::new(filename)?))
    }

    /// Get a function pointer or pointer to global static from the library.
    pub fn get(&self, symbol: &CStr) -> Option<NonNull<c_void>> {
        self.0.get(symbol)
    }
}
