use std::{
    ffi::{CStr, c_char},
    fmt::Display,
};

use crate::{Result, boxed};

/// An SDL-allocated string. In the spirit of zero-cost abstraction, its length
/// is not pre-computed (since it's null-terminated). The [`String::count_bytes()`]
/// method is available to signal that it's not an O(1) operation.
pub struct String {
    handle: boxed::Box<c_char>,
}

impl String {
    /// # Safety
    /// See the safety requirements of [`boxed::Box::from_ptr()`].
    pub(crate) unsafe fn from_ptr(handle: *mut c_char) -> Result<Self> {
        unsafe { boxed::Box::from_ptr(handle).map(|handle| Self { handle }) }
    }

    /// Convert this SDL string to a string slice. This can be done,
    /// since all strings originating from SDL are guaranteed UTF-8.
    /// This involves calculating its length via [`Self::count_bytes`].
    pub fn to_str(&self) -> &str {
        use core::slice::from_raw_parts;
        unsafe {
            let slice = from_raw_parts(self.handle.as_ptr().cast(), self.count_bytes());
            str::from_utf8_unchecked(slice)
        }
    }

    /// Analogous to [`CStr::count_bytes()`].
    pub fn count_bytes(&self) -> usize {
        let cs = unsafe { CStr::from_ptr(self.handle.as_ptr()) };
        cs.count_bytes()
    }
}

impl Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = self.to_str();
        <str as Display>::fmt(str, f)
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.to_str() == other.to_str()
    }
}
