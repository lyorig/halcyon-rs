use std::ffi::CStr;

use crate::{defs::SdlResult, error};

pub fn opt2ptr<T>(opt: Option<&T>) -> *const T {
    match opt {
        Some(s) => s as *const T,
        None => std::ptr::null(),
    }
}

pub fn to_result(result: bool) -> SdlResult {
    if result { Ok(()) } else { Err(error::get()) }
}

/// Convert a `*const i8` (commonly used in FFI) to a `&str`.
/// This function is unsafe, and all of the usual pointer pitfalls apply,
/// plus the string pointed to by `ptr` must be valid UTF-8.
pub unsafe fn c_to_str(ptr: *const i8) -> &'static str {
    unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) }
}
