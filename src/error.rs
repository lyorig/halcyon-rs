use std::ffi::{CStr, CString};

use sdl3_sys::error::SDL_GetError;

#[derive(Debug)]
pub struct Error {
    reason: String,
}

impl Error {
    pub fn current() -> Self {
        // SAFETY: SDL's error strings are UTF-8.
        let cstr = unsafe { CStr::from_ptr(SDL_GetError()) };
        let str = unsafe { str::from_utf8_unchecked(cstr.to_bytes()) };
        let reason = String::from(str);

        Self { reason }
    }

    pub fn as_str(&self) -> &str {
        self.reason.as_str()
    }

    /// Consume the [`Error`], turning it into a [`CString`].
    /// This is useful when interfacing with C APIs which
    /// expect nul-terminated strings.
    pub fn into_cstring(self) -> CString {
        // SAFETY: The stored SDL string contains no nul bytes.
        let vec = self.reason.into_bytes();
        unsafe { CString::from_vec_unchecked(vec) }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.reason)
    }
}

impl std::error::Error for Error {}
