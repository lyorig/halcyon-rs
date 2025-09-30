use std::ffi::CStr;

use crate::{defs::SdlResult, error::get_error};

/// Define a resource and implement shared traits and member functions.
#[macro_export]
macro_rules! resource {
    ($owned:ident) => {
        resource!($owned, SDL);
    };

    ($owned:ident, $library:ident) => {
        resource!($owned, $library, Destroy);
    };

    ($owned:ident, $library:ident, $dtor: ident) => {
        paste::paste! {
            #[derive(Clone, Copy)]
            pub struct [<$owned Ref>] {
                pub(crate) handle: std::ptr::NonNull<[<$library _ $owned>]>,
            }

        impl [<$owned Ref>] {
            pub(crate) fn from_ptr(handle: *mut [<$library _ $owned>]) -> Option<Self> {
                std::ptr::NonNull::new(handle).map(|handle| Self { handle })
            }
        }

        pub struct $owned {
            inner: [<$owned Ref>],
        }

        impl $owned {
            pub(crate) fn from_ptr(handle: *mut [<$library _ $owned>]) -> crate::defs::SdlResult<Self> {
                match std::ptr::NonNull::new(handle) {
                    Some(handle) => Ok(Self {
                        inner: [<$owned Ref>] { handle },
                    }),
                    None => Err(crate::error::get_error()),
                }
            }
        }

        impl std::ops::Deref for $owned {
            type Target = [<$owned Ref>];
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $owned {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl From<&$owned> for [<$owned Ref>] {
            fn from(value: &$owned) -> Self {
                value.inner
            }
        }

        impl Drop for $owned {
            #[doc(alias = "[<$library _Destroy $owned>]")]
            fn drop(&mut self) {
                unsafe { [<$library _ $dtor $owned>](self.inner.handle.as_ptr()) }
            }
        }
        }
    };
}

pub fn opt2ptr<T>(opt: Option<&T>) -> *const T {
    opt.map_or(std::ptr::null(), |s| s as *const T)
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
