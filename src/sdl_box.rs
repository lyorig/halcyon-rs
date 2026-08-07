use std::{ops::Deref, ptr::NonNull};

use sdl3_sys::stdinc::SDL_free;

use crate::{Result, util::opt2res_map};

/// Wrapper for SDL allocations.
pub struct SdlBox<T> {
    pub handle: NonNull<T>,
}

impl<T> SdlBox<T> {
    /// Create an `SdlBox` from an owned pointer, most likely
    /// provided by SDL. This takes care of checking whether
    /// the pointer is null, and if so, returning the error.
    pub fn from_ptr(ptr: *mut T) -> Result<Self> {
        opt2res_map(NonNull::new(ptr), |handle| Self { handle })
    }

    pub fn from_nonnull(handle: NonNull<T>) -> Self {
        Self { handle }
    }
}

impl<T> Drop for SdlBox<T> {
    #[doc(alias = "SDL_free")]
    fn drop(&mut self) {
        unsafe { SDL_free(self.handle.as_ptr().cast()) };
    }
}

pub struct SdlBoxArr<T> {
    pub handle: SdlBox<T>,
    pub len: usize,
}

impl<T> SdlBoxArr<T> {
    pub unsafe fn new(handle: SdlBox<T>, len: usize) -> Self {
        Self { handle, len }
    }
}

impl<T> Deref for SdlBoxArr<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.handle.handle.as_ptr(), self.len) }
    }
}
