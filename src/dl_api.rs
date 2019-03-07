/// Macro to generate the API struct.
///
/// ```no_run
/// // Shared object: either "libmylibrary.so.1", "mylibrary-1.dll" or "libMyLibrary.dylib"
/// dl_api::link!(MyApi, "libmylibrary.so.1", {
/// 	fn cFunction(param_name: *mut u32) -> u32;
/// });
///
/// fn main() {
/// 	let api = MyApi::new().unwrap(); // unwrap the `Result`.
///
/// 	let rtn: u32 = unsafe {
/// 		(api.cFunction)(std::ptr::null_mut())
/// 	};
/// }
/// ```
#[macro_export]
macro_rules! link(
	($sname: ident, $l: expr, { $(fn $fname: ident($($sarg: ident: $farg: ty),* $(,)?) -> $fret:ty);* $(;)? }) =>
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
			fn new() -> ::std::result::Result<Self, $crate::Error> {
				unsafe {
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
		}
	)
);
