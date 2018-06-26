// dl_api
//
// Copyright (c) 2018 Jeron A. Lau
// Copyright (c) 2017 Szymon Wieloch
// Distributed under the MIT LICENSE (See accompanying file LICENSE.txt)

mod common;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

pub use self::common::Library;
