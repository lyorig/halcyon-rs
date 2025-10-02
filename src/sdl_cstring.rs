use std::{
    ffi::{CStr, CString},
    ops::Deref,
};

use sdl3_sys::stdinc::SDL_free;

/// An owned `CString`, but it gets dropped via SDL's
/// custom `SDL_free()` function.
pub struct SdlCString {
    inner: CString,
}

impl SdlCString {
    pub(crate) unsafe fn from_ptr(ptr: *mut i8) -> Self {
        Self {
            inner: unsafe { CString::from_raw(ptr) },
        }
    }
}

impl Deref for SdlCString {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        self.inner.as_c_str()
    }
}

impl Drop for SdlCString {
    fn drop(&mut self) {
        let inner = std::mem::take(&mut self.inner);
        unsafe { SDL_free(inner.into_raw().cast()) };
    }
}
