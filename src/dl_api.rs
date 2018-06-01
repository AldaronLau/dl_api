// "dl_api" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

/// Create a struct laying out the api:
///
/// ```
/// #[macro_use]
/// extern crate dl_api;
///
/// dl_api!(MyApi, "libmylibrary.so.1" // or "mylibrary-1.dll" or "libMyLibrary.dylib"
/// 	fn cFunction(FirstParamType) -> ReturnType
/// );
///
/// fn main() {
/// 	let api = MyApi::new().unwrap(); // unwrap the `Result`.
///
/// 	let rtn: ReturnType = unsafe {
/// 		(api.cFunction)(0);
/// 	};
/// }
/// ```
#[macro_export] macro_rules! dl_api(
	($sname: ident, $l: expr, $(fn $fname: ident($($farg: ty),*) -> $fret:ty),*) =>
	(
		#[allow(non_snake_case)]
		pub struct $sname {
			#[allow(dead_code)]
			// this is not dead code because destructor of Library
			// deallocates the library
			__lib: $crate::Library,
			$(
				$fname: unsafe extern "system" fn($($farg),*)
					-> $fret,
			)*
		}

		impl $sname {
			unsafe fn new()
				-> ::std::result::Result<Self, $crate::Error>
			{
				let __lib = $crate::Library::new($l)?;
				Ok(Self{
					$($fname: {
						match __lib.symbol(stringify!($fname)) {
							Ok(s) => s,
							Err(e) => return Err(e),
						}
					},
					)*
					__lib,
				})
			}
		}
	);
);
