use std::{
    ffi::{CStr, c_char},
    fmt::{Debug, Display},
    ptr::NonNull,
};

use sdl3_sys::stdinc::SDL_free;

use crate::{Result, error::Error};

/// Like an owned [`String`], but it gets dropped via SDL's
/// custom [`SDL_free()`] function.
#[derive(Debug)]
pub struct SdlString {
    handle: NonNull<c_char>,
}

impl SdlString {
    pub(crate) fn from_ptr(handle: *mut c_char) -> Result<Self> {
        match NonNull::new(handle) {
            Some(handle) => Ok(Self { handle }),
            None => Err(Error::current()),
        }
    }

    /// Convert this SDL string to a string slice. This can be done,
    /// since all strings originating from SDL are guaranteed UTF-8.
    /// This involves calculating its length via [`Self::count_bytes`].
    pub fn to_str(&self) -> &str {
        use core::slice::from_raw_parts;
        unsafe {
            let slice = from_raw_parts(self.handle.as_ptr().cast(), self.count_bytes());
            str::from_utf8_unchecked(slice)
        }
    }

    /// Analogous to [`CStr::count_bytes`].
    pub fn count_bytes(&self) -> usize {
        let cs = unsafe { CStr::from_ptr(self.handle.as_ptr()) };
        cs.count_bytes()
    }
}

impl Display for SdlString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = self.to_str();
        <str as Display>::fmt(str, f)
    }
}

impl PartialEq for SdlString {
    fn eq(&self, other: &Self) -> bool {
        self.to_str() == other.to_str()
    }
}

impl Drop for SdlString {
    #[doc(alias = "SDL_free")]
    fn drop(&mut self) {
        unsafe { SDL_free(self.handle.as_ptr().cast()) };
    }
}
