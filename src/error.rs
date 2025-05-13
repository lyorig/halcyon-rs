use std::ffi::{CStr, CString};

use sdl3_sys::error::SDL_GetError;

/// Returned an owned string with the contents of `SDL_GetError()`.
pub fn get() -> CString {
    unsafe { CStr::from_ptr(SDL_GetError()) }.into()
}
