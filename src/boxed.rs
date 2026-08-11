use std::{ops::Deref, ptr::NonNull};

use sdl3_sys::stdinc::SDL_free;

use crate::{Result, util::opt2res_map};

/// [`std::boxed::Box`] but for SDL allocations, freed with [`SDL_free()`].
pub struct Box<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T: ?Sized> Box<T> {
    /// Create a [`Box`] from an owned pointer provided by SDL.
    /// Returns [`Err`] if `ptr` is null.
    ///
    /// # Safety
    /// `ptr` must point to an SDL allocation of `T`.
    pub unsafe fn from_raw(ptr: *mut T) -> Result<Self> {
        opt2res_map(NonNull::new(ptr), |ptr| Self { ptr })
    }

    /// Convenience method to directly access the inner pointer.
    pub(crate) fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Consume the [`Box`] and return the underlying pointer, without freeing
    /// the allocation.
    pub fn into_raw(self) -> NonNull<T> {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }
}

impl<T> Box<[T]> {
    /// Create a [`Box`] from an SDL-provided pointer to `len` initialized elements.
    /// Returns [`Err`] if `ptr` is null.
    ///
    /// # Safety
    /// `ptr` must point to an SDL allocation of at least `len` initialized
    /// elements of type `T`.
    pub unsafe fn from_raw_parts(ptr: *mut T, len: usize) -> Result<Self> {
        opt2res_map(NonNull::new(ptr), |ptr| Self {
            ptr: NonNull::slice_from_raw_parts(ptr, len),
        })
    }
}

impl<T: ?Sized> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ptr().as_ref_unchecked() }
    }
}

impl<T: ?Sized> Drop for Box<T> {
    #[doc(alias = "SDL_free")]
    fn drop(&mut self) {
        unsafe { SDL_free(self.ptr.as_ptr().cast()) };
    }
}
