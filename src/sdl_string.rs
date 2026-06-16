use std::{
    ffi::{CStr, c_char},
    fmt::Display,
    ops::Deref,
    ptr::NonNull,
};

use sdl3_sys::stdinc::SDL_free;

/// Like an owned `String`, but it gets dropped via SDL's
/// custom `SDL_free()` function.
pub struct SdlString {
    handle: NonNull<c_char>,
    len: usize,
}

impl SdlString {
    pub(crate) unsafe fn from_ptr(handle: NonNull<c_char>) -> Self {
        let cs = unsafe { CStr::from_ptr(handle.as_ptr()) };
        Self {
            handle,
            len: cs.count_bytes(),
        }
    }
}

impl Deref for SdlString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe {
            str::from_utf8_unchecked(std::slice::from_raw_parts(
                self.handle.as_ptr().cast(),
                self.len,
            ))
        }
    }
}

impl Display for SdlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl Drop for SdlString {
    fn drop(&mut self) {
        unsafe { SDL_free(self.handle.as_ptr().cast()) };
    }
}
