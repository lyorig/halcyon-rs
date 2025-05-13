use std::ffi::CString;

use crate::error;

pub fn btur(test: bool) -> Result<(), CString> {
    if test { Ok(()) } else { Err(error::get()) }
}

pub fn opt2ptr<T>(opt: Option<&T>) -> *const T {
    match opt {
        Some(s) => s as *const T,
        None => std::ptr::null(),
    }
}
