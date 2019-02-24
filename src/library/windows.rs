// "dl_api" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::ffi::{CStr, OsStr};
use std::io::{Error as IoError, ErrorKind};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::sync::Mutex;
use winapi;
use Error;

static USE_ERRORMODE: AtomicBool = ATOMIC_BOOL_INIT;

struct SetErrorModeData {
    pub count: u32,
    pub previous: winapi::shared::minwindef::DWORD,
}

lazy_static! {
    static ref SET_ERR_MODE_DATA: Mutex<SetErrorModeData> = Mutex::new(SetErrorModeData {
        count: 0,
        previous: 0
    });
}

pub type Handle = winapi::shared::minwindef::HMODULE;

/*
Windows has an ugly feature: by default not finding the given library opens a window
and passes control to the user.
To fix this wee need to change thread/process error mode for the moment when the function is called
and then revert it to the previous value.

Since Windows 7 the SetThreadErrorMode function is supported. It sets error mode for the given
thread. Older systems require calling SetErrorMode. This function sets error mode for the whole
process.

https://msdn.microsoft.com/pl-pl/library/windows/desktop/dd553630(v=vs.85).aspx
*/

const ERROR_MODE: winapi::shared::minwindef::DWORD = 1; //app handles everything

enum ErrorModeGuard {
    ThreadPreviousValue(winapi::shared::minwindef::DWORD),
    DoNothing,
    Process,
}

impl ErrorModeGuard {
    fn new() -> Result<ErrorModeGuard, IoError> {
        if !USE_ERRORMODE.load(Ordering::Acquire) {
            let mut previous: winapi::shared::minwindef::DWORD = 0;
            if unsafe { winapi::um::errhandlingapi::SetThreadErrorMode(ERROR_MODE, &mut previous) }
                == 0
            {
                //error. On some systems SetThreadErrorMode may not be implemented
                let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
                if error == winapi::shared::winerror::ERROR_CALL_NOT_IMPLEMENTED {
                    USE_ERRORMODE.store(true, Ordering::Release);
                } else {
                    //this is an actual error
                    //SetErrorMode never fails. Shouldn't we use it now?
                    return Err(IoError::from_raw_os_error(error as i32));
                }
            } else {
                return Ok(if previous == ERROR_MODE {
                    ErrorModeGuard::DoNothing
                } else {
                    ErrorModeGuard::ThreadPreviousValue(previous)
                });
            }
        }
        //several threads may be opening libraries at the same time.
        //we need to make sure that only the first one sets the erro mode
        //and only the last reverts it to the original value

        //poisoning should never happen
        let mut lock = SET_ERR_MODE_DATA.lock().expect("Mutex got poisoned");
        if lock.count == 0 {
            lock.previous = unsafe { winapi::um::errhandlingapi::SetErrorMode(ERROR_MODE) };
            if lock.previous == ERROR_MODE {
                return Ok(ErrorModeGuard::DoNothing);
            }
        }
        lock.count += 1;
        Ok(ErrorModeGuard::Process)
    }
}

impl Drop for ErrorModeGuard {
    fn drop(&mut self) {
        match self {
            &mut ErrorModeGuard::DoNothing => (),
            &mut ErrorModeGuard::Process => {
                //poisoning should never happen
                let mut lock = SET_ERR_MODE_DATA.lock().expect("Mutex got poisoned");
                lock.count -= 1;
                if lock.count == 0 {
                    unsafe { winapi::um::errhandlingapi::SetErrorMode(lock.previous) };
                }
            }
            &mut ErrorModeGuard::ThreadPreviousValue(previous) => unsafe {
                winapi::um::errhandlingapi::SetThreadErrorMode(previous, null_mut());
            },
        }
    }
}

unsafe fn get_win_error() -> IoError {
    let error = winapi::um::errhandlingapi::GetLastError();
    if error == 0 {
        IoError::new(
            ErrorKind::Other,
            "Could not obtain information about the error",
        )
    } else {
        IoError::from_raw_os_error(error as i32)
    }
}

#[inline]
pub unsafe fn get_sym(handle: Handle, name: &CStr) -> Result<*mut (), Error> {
    let symbol = winapi::um::libloaderapi::GetProcAddress(handle, name.as_ptr());
    if symbol.is_null() {
        Err(Error::SymbolGettingError(get_win_error()))
    } else {
        Ok(symbol as *mut ())
    }
}

#[inline]
pub unsafe fn open_lib(name: &OsStr) -> Result<Handle, Error> {
    let wide_name: Vec<u16> = name.encode_wide().chain(Some(0)).collect();
    let _guard = match ErrorModeGuard::new() {
        Ok(val) => val,
        Err(err) => return Err(Error::OpeningLibraryError(err)),
    };
    let handle = winapi::um::libloaderapi::LoadLibraryW(wide_name.as_ptr());
    if handle.is_null() {
        Err(Error::OpeningLibraryError(get_win_error()))
    } else {
        Ok(handle)
    }
}

#[inline]
pub fn close_lib(handle: Handle) -> Handle {
    if unsafe { winapi::um::libloaderapi::FreeLibrary(handle) } == 0 {
        //this should not happen
        panic!("FreeLibrary() failed, the error is {}", unsafe {
            get_win_error()
        });
    }
    null_mut()
}
