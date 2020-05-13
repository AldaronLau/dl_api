// DL API
//
// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

///
#[derive(Debug, Copy, Clone)]
pub enum Error {
    /// Library could not be found.
    NotInstalled,
    /// Function or global static doesn't exist in this library.
    DoesntExist(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotInstalled => write!(f, "Not Installed"),
            Error::DoesntExist(details) => write!(f, "Symbol \"{}\" doesn't exist", details),
        }
    }
}
