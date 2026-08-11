use std::ffi::c_char;

/// A string that the `TTF_RenderText*` functions can safely read.
///
/// The `TTF_RenderText*` functions take a pointer and a byte length.
/// A length of zero means that the pointer refers to a nul-terminated string.
/// As Rust strings do not have a nul terminator, an empty `&str` would make the
/// function read past the end of the string. This struct prevents that error.
///
/// Construct a "TTF-ready" string from a `&str`:
/// - [`RtStr::new()`] checks for an empty string. In that case, the pointer it set to `c""`.
/// - [`RtStr::new_unchecked`] skips the check. Use when you know that the string is not empty.
#[derive(Clone, Copy)]
pub struct RtStr<'a> {
    ptr: *const c_char,
    len: usize,
    marker: std::marker::PhantomData<&'a str>,
}

impl RtStr<'_> {
    pub const fn new<'a>(s: &'a str) -> RtStr<'a> {
        let len = s.len();
        let ptr = if len == 0 {
            c"".as_ptr()
        } else {
            s.as_ptr().cast()
        };

        RtStr {
            ptr,
            len,
            marker: std::marker::PhantomData,
        }
    }

    /// # Safety
    /// `s` must not be empty.
    pub const unsafe fn new_unchecked<'a>(s: &'a str) -> RtStr<'a> {
        unsafe { std::mem::transmute(s) }
    }

    pub const fn as_ptr(&self) -> *const c_char {
        self.ptr
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}
