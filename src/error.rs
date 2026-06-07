use std::ffi::CStr;

use sdl3_sys::error::SDL_GetError;

pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = SDL_GetError();
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let err = cstr.to_string_lossy();

        f.write_str(&err)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
