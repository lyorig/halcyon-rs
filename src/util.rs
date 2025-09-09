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
