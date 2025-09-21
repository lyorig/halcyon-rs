use std::ffi::CStr;

use crate::{defs::SdlResult, error::get_error};

/// Implement shared traits for resources.
/// Currently, this implements:
/// - std::ops::Deref
/// - std::ops::DerefMut
/// - From
#[macro_export]
macro_rules! impl_resource_traits {
    ($owned:ty, $ref: ty, $dtor:expr) => {
        impl std::ops::Deref for $owned {
            type Target = $ref;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $owned {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl From<&$owned> for $ref {
            fn from(value: &$owned) -> Self {
                value.inner
            }
        }

        impl Drop for $owned {
            fn drop(&mut self) {
                unsafe { $dtor(self.inner.handle.as_ptr()) }
            }
        }
    };
}

/// Defines `from_ptr(*mut $opaque) -> SdlResult<Self>`, to be
/// used by constructors calling SDL's pointer-returning functions.
///
/// Must be called in an `impl` block.
#[macro_export]
macro_rules! fn_from_ptr {
    ($ref:tt, $opaque:ty) => {
        fn from_ptr(handle: *mut $opaque) -> crate::defs::SdlResult<Self> {
            match std::ptr::NonNull::new(handle) {
                Some(handle) => Ok(Self {
                    inner: $ref { handle },
                }),
                None => Err(crate::error::get_error()),
            }
        }
    };
}

/// Define a resource and implement shared traits and member functions.
#[macro_export]
macro_rules! resource {
    ($owned:ident, $ref:ident, $opaque:ty, $dtor:expr) => {
        #[derive(Clone, Copy)]
        pub struct $ref {
            pub(crate) handle: std::ptr::NonNull<$opaque>,
        }

        pub struct $owned {
            inner: $ref,
        }

        impl $owned {
            crate::fn_from_ptr!($ref, $opaque);
        }

        crate::impl_resource_traits!($owned, $ref, $dtor);
    };
}

pub fn opt2ptr<T>(opt: Option<&T>) -> *const T {
    match opt {
        Some(s) => s as *const T,
        None => std::ptr::null(),
    }
}

pub fn to_result(result: bool) -> SdlResult {
    if result { Ok(()) } else { Err(get_error()) }
}

/// Convert a `*const i8` (commonly used in FFI) to a `&str`.
/// This function is unsafe, and all of the usual pointer pitfalls apply,
/// plus the string pointed to by `ptr` must be valid UTF-8.
pub unsafe fn c_to_str(ptr: *const i8) -> &'static str {
    unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) }
}
