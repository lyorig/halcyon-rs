//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryStdinc)):
//! - [x] SDL_free
//! - [x] SDL_malloc

use std::{
    borrow::Borrow,
    fmt::{self, Debug, Display},
    hash::{Hash, Hasher},
    iter::FusedIterator,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use sdl3_sys::stdinc::{SDL_free, SDL_malloc};

use crate::{Result, util::opt2res_map};

/// Mirror of [`std::boxed::Box`], with (de)allocation performed via
/// [`SDL_malloc`]/[`SDL_free`] instead of the global allocator.
///
/// The pointee's alignment must not exceed the fundamental alignment
/// (16 bytes on 64-bit platforms), which is what SDL's allocator
/// guarantees. This holds for all standard types; only over-aligned types
/// (e.g. SIMD vectors, `#[repr(align(32))]` structs) are excluded.
pub struct Box<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T> Box<T> {
    /// Allocate memory for `T` via [`SDL_malloc`] and move `x` into it.
    /// Panics if the allocation fails.
    #[doc(alias = "SDL_malloc")]
    pub fn new(x: T) -> Self {
        match Self::try_new(x) {
            Some(b) => b,
            None => panic!("allocation failed"),
        }
    }

    /// Like [`Self::new`], but returns `None` instead of panicking if the
    /// allocation fails.
    #[doc(alias = "SDL_malloc")]
    pub fn try_new(x: T) -> Option<Self> {
        // `SDL_malloc` only guarantees the fundamental alignment.
        debug_assert!(align_of::<T>() <= align_of::<usize>() * 2);

        // SAFETY: `SDL_malloc` returns NULL on failure (handled by
        // `NonNull::new`), and otherwise returns at least `size_of::<T>()`
        // bytes of memory with the alignment asserted above.
        let ptr = NonNull::new(unsafe { SDL_malloc(size_of::<T>()) }.cast())?;

        // SAFETY: `ptr` is a fresh, uninitialized allocation for `T`.
        unsafe { ptr.cast::<T>().as_ptr().write(x) };

        Some(Self { ptr })
    }
}

impl<T: ?Sized> Box<T> {
    /// Consume the [`Box`], returning the underlying pointer without freeing
    /// the allocation.
    pub fn into_raw(self) -> *mut T {
        let ptr = self.ptr.as_ptr();
        std::mem::forget(self);
        ptr
    }

    /// Like [`Self::into_raw`], but preserves the non-null invariant.
    pub fn into_raw_non_null(self) -> NonNull<T> {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }

    /// Reconstruct a [`Box`] from a pointer obtained via [`Self::into_raw`]
    /// (or [`Self::into_raw_non_null`]).
    ///
    /// # Safety
    /// `raw` must have been obtained from a [`Box`] of this module and must
    /// not have been freed.
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        // SAFETY: Per the contract, `raw` is a valid, aligned, non-null
        // pointer obtained from `into_raw`.
        Self {
            ptr: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    /// Create a [`Box`] from an SDL allocation.
    /// Returns the current error if `raw` is null.
    ///
    /// # Safety
    /// `raw` must be allocated via SDL.
    pub(crate) unsafe fn from_raw_nullchk(raw: *mut T) -> Result<Self> {
        opt2res_map(NonNull::new(raw), |ptr| Self { ptr })
    }

    /// Consume the [`Box`] and leak the allocation, returning a mutable
    /// reference valid for the rest of the program's life.
    pub fn leak(self) -> &'static mut T {
        let ptr = self.ptr.as_ptr();
        std::mem::forget(self);

        // SAFETY: The allocation is intentionally leaked, so the reference
        // is valid for `'static`.
        unsafe { ptr.as_mut_unchecked() }
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T> Box<[T]> {
    /// Create a [`Box`] from an SDL allocation of `len` elements.
    ///
    /// # Safety
    /// `ptr` must have been obtained from an SDL allocation and must be valid
    /// for `len` * `size_of::<T>()` bytes.
    pub unsafe fn from_raw_parts(ptr: *mut T, len: usize) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(ptr) };
        let slice = NonNull::slice_from_raw_parts(ptr, len);
        Self { ptr: slice }
    }

    /// Create a [`Box`] from an SDL allocation of `len` elements.
    /// Returns the current error if `ptr` is null.
    ///
    /// # Safety
    /// `ptr` must have been obtained from an SDL allocation and must be valid
    /// for `len` * `size_of::<T>()` bytes.
    pub(crate) unsafe fn from_raw_parts_nullchk(ptr: *mut T, len: usize) -> Result<Self> {
        opt2res_map(NonNull::new(ptr), |nn| {
            let slice = NonNull::slice_from_raw_parts(nn, len);
            Self { ptr: slice }
        })
    }

    /// Allocate space for `len` uninitialized `T`s via [`SDL_malloc`],
    /// panicking on failure.
    #[doc(alias = "SDL_malloc")]
    fn alloc(len: usize) -> NonNull<u8> {
        // `SDL_malloc` only guarantees the fundamental alignment.
        debug_assert!(align_of::<T>() <= align_of::<usize>() * 2);

        let size = size_of::<T>().checked_mul(len).expect("size overflow");

        // SAFETY: `SDL_malloc` returns NULL on failure (handled by
        // `NonNull::new`), and otherwise returns `size` bytes with the
        // alignment asserted above. A size of 0 is handled by SDL, which
        // allocates at least 1 byte in that case.
        NonNull::new(unsafe { SDL_malloc(size) }.cast()).expect("allocation failed")
    }

    /// Allocate a boxed slice and clone each element of `slice` into it.
    #[doc(alias = "SDL_malloc")]
    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        let len = slice.len();
        let ptr = Self::alloc(len);
        let dst = ptr.cast::<T>().as_ptr();

        for (i, x) in slice.iter().enumerate() {
            // SAFETY: `i` is below `len`, so the slot is in-bounds and
            // uninitialized.
            unsafe { dst.add(i).write(x.clone()) };
        }

        Self {
            ptr: NonNull::slice_from_raw_parts(ptr.cast(), len),
        }
    }
}

impl<T> FromIterator<T> for Box<[T]> {
    /// Allocate a boxed slice and move the iterator's elements into it.
    #[doc(alias = "SDL_malloc")]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let items: Vec<T> = iter.into_iter().collect();
        let len = items.len();
        let ptr = Self::alloc(len);
        let dst = ptr.cast::<T>().as_ptr();

        for (i, item) in items.into_iter().enumerate() {
            // SAFETY: `i` is below `len`, so the slot is in-bounds and
            // uninitialized.
            unsafe { dst.add(i).write(item) };
        }

        Self {
            ptr: NonNull::slice_from_raw_parts(ptr.cast(), len),
        }
    }
}

impl<T, const N: usize> From<Box<[T; N]>> for Box<[T]> {
    fn from(b: Box<[T; N]>) -> Self {
        let raw = b.into_raw();

        // SAFETY: `[T; N]` and `[T]` share the same layout for a length of
        // `N`, so the allocation is valid as a slice without resizing.
        unsafe { Self::from_raw(std::ptr::slice_from_raw_parts_mut(raw as *mut T, N)) }
    }
}

impl<T: ?Sized> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The `Box` owns a valid allocation of `T`.
        unsafe { self.ptr.as_ptr().as_ref_unchecked() }
    }
}

impl<T: ?Sized> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The `Box` owns a valid allocation of `T`.
        unsafe { self.ptr.as_ptr().as_mut_unchecked() }
    }
}

impl<T: ?Sized> Drop for Box<T> {
    #[doc(alias = "SDL_free")]
    fn drop(&mut self) {
        unsafe { SDL_free(self.ptr.as_ptr().cast()) };
    }
}

impl<T: ?Sized> AsRef<T> for Box<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: ?Sized> AsMut<T> for Box<T> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: ?Sized> Borrow<T> for Box<T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ?Sized> fmt::Pointer for Box<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

impl<T: Debug + ?Sized> Debug for Box<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        T::fmt(self, f)
    }
}

impl<T: Display + ?Sized> Display for Box<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        T::fmt(self, f)
    }
}

impl<T: Clone> Clone for Box<T> {
    fn clone(&self) -> Self {
        Self::new(T::clone(self))
    }
}

impl<T: Clone> Clone for Box<[T]> {
    fn clone(&self) -> Self {
        Self::from_slice(self)
    }
}

impl<T: Default> Default for Box<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Default for Box<[T]> {
    fn default() -> Self {
        Self::from_iter(std::iter::empty())
    }
}

impl<T> From<T> for Box<T> {
    fn from(x: T) -> Self {
        Self::new(x)
    }
}

impl<T: Copy> From<&T> for Box<T> {
    fn from(x: &T) -> Self {
        Self::new(*x)
    }
}

impl<T: Clone> From<&[T]> for Box<[T]> {
    fn from(slice: &[T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T: PartialEq + ?Sized> PartialEq for Box<T> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: Eq + ?Sized> Eq for Box<T> {}

impl<T: PartialOrd + ?Sized> PartialOrd for Box<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }

    fn lt(&self, other: &Self) -> bool {
        **self < **other
    }

    fn le(&self, other: &Self) -> bool {
        **self <= **other
    }

    fn gt(&self, other: &Self) -> bool {
        **self > **other
    }

    fn ge(&self, other: &Self) -> bool {
        **self >= **other
    }
}

impl<T: Ord + ?Sized> Ord for Box<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: Hash + ?Sized> Hash for Box<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

/// Owned iterator over the elements of a [`Box<[T]>`], moving them out by
/// value. Yields each element exactly once; elements not yet yielded when the
/// iterator is dropped still get their destructors run.
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
        let ptr = self.arr.ptr.as_ptr().cast::<T>();
        let elem = unsafe { ptr.add(self.index).read() };
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
        // the allocation itself is freed by the inner [`Box`].
        let ptr = self.arr.ptr.as_ptr().cast::<T>();
        for i in self.index..self.arr.len() {
            // SAFETY: `i` is within the allocation and the element at `i` has
            // not been moved out, so it is still initialized.
            unsafe { std::ptr::drop_in_place(ptr.add(i)) }
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

impl Box<str> {
    pub fn as_str(&self) -> &str {
        &self
    }
}
