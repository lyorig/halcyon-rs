use std::{ffi::CStr, fmt::Display, ops::Deref, ptr::NonNull};

use sdl3_sys::stdinc::SDL_free;

/// Like an owned `String`, but it gets dropped via SDL's
/// custom `SDL_free()` function.
pub struct SdlString {
    handle: NonNull<i8>,
    len: usize,
}

impl SdlString {
    pub(crate) fn from_ptr(handle: NonNull<i8>) -> Self {
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
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
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
