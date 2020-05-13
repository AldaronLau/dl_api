// DL API
//
// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Macro to define the dynamic library API runtime linker struct.
///
/// ```no_run
/// // Shared object: either "libmylibrary.so.1", "mylibrary-1.dll"
/// // or "libMyLibrary.dylib"
/// dl_api::linker!(extern "C" MyApi "libmylibrary.so.1" {
///     fn cFunction(param_name: *mut u32) -> u32;
/// });
///
/// fn main() {
///     let api = MyApi::new().unwrap(); // unwrap the `Result`.
///
///     let rtn: u32 = unsafe {
///         (api.cFunction)(std::ptr::null_mut())
///     };
/// }
/// ```
#[macro_export]
macro_rules! linker(
	(extern $abi: literal/*item*/ $sname: ident $filename: literal {
      $(static $data:ident : $darg:ty;)*
      $(valist fn $vafn:ident ($($varg:ident : $fvrg:ty),* , ...) -> $frvt:ty;)*
      $(fn $name:ident ($($sarg:ident : $farg:ty),* $(,)?) -> $fret:ty;)*
    }) => {
		#[allow(non_snake_case)]
		struct $sname {
		    $( $data: $darg, )*
			$( $vafn: unsafe extern $abi fn($($fvrg),*, ...) -> $frvt, )*
			$( $name: unsafe extern $abi fn($($farg),*) -> $fret, )*
		}

		impl $sname {
			fn new() -> ::std::result::Result<Self, $crate::Error> {
				unsafe {
    				const FILENAME: &str = concat!($filename, "\0");
	    		    let dl_api = $crate::manual::DlApi::new(
    	    		    ::std::ffi::CStr::from_bytes_with_nul_unchecked(
    	    		        FILENAME.as_bytes()
	    		        )
	    		    ).ok_or($crate::Error::NotInstalled)?;
                    ::std::result::Result::<Self, $crate::Error>::Ok(Self {
                        $(
                            $data: {
                                const NAME: &str = concat!(stringify!($data), "\0");
                				::std::mem::transmute(dl_api.get(
                    				::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    				    NAME.as_bytes()
                    				)
                				).ok_or($crate::Error::DoesntExist(stringify!($data)))?)
                            },
                        )*
                        $(
                            $vafn: {
                				const NAME: &str = concat!(stringify!($vafn), "\0");
                				::std::mem::transmute(dl_api.get(
                    				::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    				    NAME.as_bytes()
                    				)
                				).ok_or($crate::Error::DoesntExist(stringify!($vafn)))?)
            				},
        				)*
                        $(
                            $name: {
                				const NAME: &str = concat!(stringify!($name), "\0");
                				::std::mem::transmute(dl_api.get(
                    				::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    				    NAME.as_bytes()
                    				)
                				).ok_or($crate::Error::DoesntExist(stringify!($name)))?)
            				},
        				)*
    				})
				}
			}
		}
	};
);
