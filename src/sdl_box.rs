use std::{mem::MaybeUninit, ops::Deref, ptr::NonNull};

use sdl3_sys::stdinc::SDL_free;

use crate::{defs::SdlResult, error::get_error};

/// Wrapper for SDL allocations.
pub struct SdlBox<T> {
    pub handle: NonNull<T>,
}

impl<T> SdlBox<T> {
    /// Create an `SdlBox` from an owned pointer, most likely
    /// provided by SDL. This takes care of checking whether
    /// the pointer is null, and if so, returning the error.
    pub unsafe fn from_ptr(ptr: *mut T) -> SdlResult<Self> {
        match NonNull::new(ptr) {
            Some(handle) => Ok(Self { handle }),
            None => Err(get_error()),
        }
    }
}

impl<T> Drop for SdlBox<T> {
    fn drop(&mut self) {
        unsafe { SDL_free(self.handle.as_ptr().cast()) };
    }
}

pub struct SdlBoxArr<T> {
    pub handle: SdlBox<T>,
    pub len: usize,
}

impl<T> SdlBoxArr<T> {
    /// Create an `SdlBoxArr` from an owned pointer, most likely provided by SDL,
    /// and potentially uninitialized size, since those work via out-parameters.
    /// This takes care of checking whether the pointer is null, and if so, returning the error.
    pub unsafe fn from_ptr(ptr: *mut T, len: MaybeUninit<i32>) -> SdlResult<Self> {
        unsafe {
            SdlBox::from_ptr(ptr).map(|handle| Self {
                handle,
                // If SDL returned a valid pointer, assume the length has also
                // been provided.
                len: len.assume_init() as _,
            })
        }
    }
}

impl<T> Deref for SdlBoxArr<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.handle.handle.as_ptr(), self.len) }
    }
}
