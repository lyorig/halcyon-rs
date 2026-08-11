use std::{iter::FusedIterator, ops::Deref, ptr::NonNull};

use sdl3_sys::stdinc::SDL_free;

use crate::{Result, util::opt2res_map};

/// Wrapper for SDL allocations, freed with [`SDL_free()`].
pub struct Box<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T> Box<T> {
    /// Create an `SdlBox` from an owned pointer, most likely
    /// provided by SDL. This takes care of checking whether
    /// the pointer is null, and if so, returning the error.
    pub fn from_ptr(ptr: *mut T) -> Result<Self> {
        opt2res_map(NonNull::new(ptr), |ptr| Self { ptr })
    }

    pub fn from_nonnull(handle: NonNull<T>) -> Self {
        Self { ptr: handle }
    }
}

impl<T> Box<[T]> {
    /// Create an `SdlBox` from an owned pointer to `len` initialized elements,
    /// most likely provided by SDL. This takes care of checking whether the
    /// pointer is null, and if so, returning the error.
    ///
    /// # Safety
    /// `ptr` must point to an SDL allocation of at least `len` initialized
    /// elements.
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

/// Owned iterator over the elements of an [`SdlBox<[T]>`], moving them out by
/// value. Yields each element exactly once; elements not yet yielded when the
/// iterator is dropped still get their destructors run.
///
/// Like [`SdlBox::from_raw_parts`], this assumes the allocation holds its
/// slice length of initialized elements.
pub struct IntoIter<T> {
    arr: Box<[T]>,
    index: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index == self.arr.len() {
            return None;
        }
        // SAFETY: The allocation holds `len` initialized elements and `index`
        // is below `len`, so the read is in-bounds. Each element is yielded
        // at most once, since `index` only increases.
        let elem = unsafe { self.arr.as_ptr().add(self.index).read() };
        self.index += 1;
        Some(elem)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.arr.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // Elements that were never yielded still need their destructors run;
        // the allocation itself is freed by the inner `SdlBox`.
        let ptr = self.arr.as_ptr();
        for i in self.index..self.arr.len() {
            // SAFETY: `i` is within the allocation and the element at `i` has
            // not been moved out, so it is still initialized.
            unsafe { std::ptr::drop_in_place(ptr.add(i) as *mut T) }
        }
    }
}

impl<T> IntoIterator for Box<[T]> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            arr: self,
            index: 0,
        }
    }
}

impl<'a, T> IntoIterator for &'a Box<[T]> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Box<[T]> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: The fat pointer's metadata is the slice length, and `self`
        // borrows the allocation mutably for `'a`.
        unsafe { &mut *self.ptr.as_ptr() }.iter_mut()
    }
}
