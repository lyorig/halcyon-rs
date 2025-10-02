use std::{ffi::CString, fmt::Display, mem::ManuallyDrop, ops::Deref};

use sdl3_sys::stdinc::SDL_free;

/// An owned `String`, but it gets dropped via SDL's
/// custom `SDL_free()` function.
pub struct SdlString {
    inner: ManuallyDrop<String>,
}

impl SdlString {
    pub(crate) unsafe fn from_ptr(ptr: *mut i8) -> Self {
        Self {
            inner: ManuallyDrop::new(
                unsafe { CString::from_raw(ptr) }
                    .into_string()
                    .expect("SDL string contains non-valid UTF-8"),
            ),
        }
    }
}

impl Deref for SdlString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Display for SdlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl Drop for SdlString {
    fn drop(&mut self) {
        unsafe { SDL_free(self.inner.as_mut_ptr().cast()) };
    }
}
