use std::ffi::{CStr, CString};

use sdl3_sys::clipboard::*;

use crate::{defs::SdlResult, error::get_error, util::c_to_str};

pub fn has_text() -> bool {
    unsafe { SDL_HasClipboardText() }
}

pub fn text() -> Box<CStr> {
    let cs = unsafe { CStr::from_ptr(SDL_GetClipboardText()) };
    if cs.is_empty() {
        drop(cs);
        return Err(get_error());
    } else {
        return Ok(String::from);
    }
}
