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

/// Owned iterator over the elements of an [`SdlBoxArr`], moving them out by
/// value. Yields each element exactly once; elements not yet yielded when the
/// iterator is dropped still get their destructors run.
///
/// Like [`SdlBoxArr::new`], this assumes the allocation holds `len`
/// initialized elements.
pub struct IntoIter<T> {
    arr: SdlBoxArr<T>,
    index: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index == self.arr.len {
            return None;
        }
        // SAFETY: The allocation holds `len` initialized elements and `index`
        // is below `len`, so the read is in-bounds. Each element is yielded
        // at most once, since `index` only increases.
        let elem = unsafe { self.arr.handle.handle.as_ptr().add(self.index).read() };
        self.index += 1;
        Some(elem)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.arr.len - self.index;
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> std::iter::FusedIterator for IntoIter<T> {}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // Elements that were never yielded still need their destructors run;
        // the allocation itself is freed by the inner `SdlBox`.
        let ptr = self.arr.handle.handle.as_ptr();
        for i in self.index..self.arr.len {
            // SAFETY: `i` is within the allocation and the element at `i` has
            // not been moved out, so it is still initialized.
            unsafe { ptr.add(i).drop_in_place() }
        }
    }
}

impl<T> IntoIterator for SdlBoxArr<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            arr: self,
            index: 0,
        }
    }
}

impl<'a, T> IntoIterator for &'a SdlBoxArr<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SdlBoxArr<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: The allocation holds `len` initialized elements, and `self`
        // borrows it mutably for `'a`.
        unsafe { std::slice::from_raw_parts_mut(self.handle.handle.as_ptr(), self.len) }.iter_mut()
    }
}
