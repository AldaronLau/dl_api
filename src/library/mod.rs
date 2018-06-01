// "dl_api" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

mod common;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

pub use self::common::Library;
