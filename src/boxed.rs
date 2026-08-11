use std::{ops::Deref, ptr::NonNull};

use sdl3_sys::stdinc::SDL_free;

use crate::{Result, util::opt2res_map};

/// [`std::boxed::Box`] but for SDL allocations, freed with [`SDL_free()`].
///
/// # Differences from [`std::boxed::Box`]
///
/// halcyon-rs' [`Box`] is tailored towards usage with SDL, so it's not exactly 1:1 in terms of
/// method availablity and signatures. The point is, first and foremost, to abstract the things
/// that SDL wants you to manually free, since it can sometimes be a hassle to figure out what
/// the library owns, as opposed to what it throws on the heap for fun & profit.
pub struct Box<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T: ?Sized> Box<T> {
    /// Create a [`Box`] from an owned pointer provided by SDL.
    /// Returns the current error if `ptr` is null.
    ///
    /// # Safety
    /// `ptr` must point to an SDL allocation of `T`.
    pub(crate) unsafe fn from_raw(ptr: *mut T) -> Result<Self> {
        opt2res_map(NonNull::new(ptr), |ptr| Self { ptr })
    }

    /// Convenience method to directly access the inner pointer.
    pub(crate) fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Consume the [`Box`] and return the underlying pointer, without freeing
    /// the allocation.
    pub(crate) fn into_raw(self) -> NonNull<T> {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }
}

impl<T> Box<[T]> {
    /// Create a [`Box`] from an SDL-provided pointer to `len` initialized elements.
    /// Returns the current error if `ptr` is null.
    ///
    /// # Safety
    /// `ptr` must point to an SDL allocation of at least `len` initialized elements of type `T`.
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
