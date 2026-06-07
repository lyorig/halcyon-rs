use std::ffi::{CStr, CString};

use sdl3_sys::error::SDL_GetError;

pub struct Error {
    error: CString,
}

impl Error {
    pub(crate) fn current() -> Self {
        let ptr = SDL_GetError();
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let error = cstr.to_owned();

        Self { error }
    }

    pub fn as_cstr(&self) -> &CStr {
        &self.error
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cow = self.error.to_string_lossy();
        f.write_str(&cow)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
