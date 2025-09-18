use std::ffi::CStr;

use sdl3_sys::error::SDL_GetError;

/// Returned an owned string with the contents of `SDL_GetError()`.
#[doc(alias = "SDL_GetError")]
pub fn get() -> &'static CStr {
    unsafe { CStr::from_ptr(SDL_GetError()) }
}
