use std::{
    ffi::{CStr, c_char},
    fmt::Display,
    ptr::NonNull,
};

use sdl3_sys::stdinc::SDL_free;

use crate::{defs::SdlResult, error::Error};

/// Like an owned `String`, but it gets dropped via SDL's
/// custom `SDL_free()` function.
pub struct SdlString {
    handle: NonNull<c_char>,
}

impl SdlString {
    pub(crate) fn from_ptr(handle: *const c_char) -> SdlResult<Self> {
        match NonNull::new(handle.cast_mut()) {
            Some(handle) => Ok(Self { handle }),
            None => Err(Error::current()),
        }
    }

    pub fn to_str(&self) -> &str {
        unsafe {
            let cs = CStr::from_ptr(self.handle.as_ptr());
            let slice = core::slice::from_raw_parts(self.handle.as_ptr().cast(), cs.count_bytes());

            str::from_utf8_unchecked(slice)
        }
    }
}

impl Display for SdlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_str().fmt(f)
    }
}

impl Drop for SdlString {
    fn drop(&mut self) {
        unsafe { SDL_free(self.handle.as_ptr().cast()) };
    }
}
