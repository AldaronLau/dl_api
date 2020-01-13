//! Generated by DL API

#![allow(unused)]
#![allow(unsafe_code)]
// #![rustfmt::skip]

const LM_ID_NEWLM: std::os::raw::c_long = -1;
const RTLD_NOW: std::os::raw::c_int = 0x00002;

extern {
    fn dlmopen(
        lmid: std::os::raw::c_long,
        filename: *const std::os::raw::c_char,
        flags: std::os::raw::c_int
    ) -> *mut std::ffi::c_void;
    fn dlclose(handle: *mut std::ffi::c_void) -> std::os::raw::c_int;
    fn dlsym(handle: *mut std::ffi::c_void, symbol: *const std::os::raw::c_char)
        -> *mut std::ffi::c_void;
}

unsafe fn new(name: &[u8]) -> Option<std::ptr::NonNull<std::ffi::c_void>> {
    std::ptr::NonNull::new(dlmopen(LM_ID_NEWLM, name.as_ptr().cast(), RTLD_NOW))
}

unsafe fn old(dll: std::ptr::NonNull<std::ffi::c_void>) {
    assert_eq!(dlclose(dll.as_ptr()), 0);
}

unsafe fn sym(dll: &std::ptr::NonNull<std::ffi::c_void>, name: &[u8])
    -> Option<std::ptr::NonNull<std::ffi::c_void>>
{
    std::ptr::NonNull::new(dlsym(dll.as_ptr(), name.as_ptr().cast()))
}

static mut THREAD_ID: std::mem::MaybeUninit<std::thread::ThreadId>
    = std::mem::MaybeUninit::uninit();
static mut DLL: std::mem::MaybeUninit<std::ptr::NonNull<std::ffi::c_void>>
    = std::mem::MaybeUninit::uninit();
static mut START_FFI: std::sync::Once = std::sync::Once::new();
static mut SUCCESS: bool = false;

unsafe fn check_thread() -> Option<std::ptr::NonNull<std::ffi::c_void>> {
    assert_eq!(THREAD_ID.assume_init(), std::thread::current().id());

    START_FFI.call_once(|| {
        THREAD_ID = std::mem::MaybeUninit::new(std::thread::current().id());
        if let Some(dll) = new(DL_API_SHARED_OBJECT_NAME) {
            DLL = std::mem::MaybeUninit::new(dll);
        } else {
            SUCCESS = true;
        }
    });

    if SUCCESS {
        Some(DLL.assume_init())
    } else {
        None
    }
}

