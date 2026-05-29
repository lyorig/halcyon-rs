use std::ffi::{CStr, c_char};

use crate::{defs::SdlResult, error};

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
            pub struct [<$owned Handle>] {
                pub(crate) handle: std::ptr::NonNull<[<$library _ $owned>]>,
            }

            impl [<$owned Handle>] {
                pub(crate) fn from_ptr(handle: *mut [<$library _ $owned>]) -> Option<Self> {
                    std::ptr::NonNull::new(handle).map(|handle| Self { handle })
                }
            }

            pub struct $owned {
                pub(crate) inner: [<$owned Handle>],
            }

            impl $owned {
                pub(crate) fn from_ptr(handle: *mut [<$library _ $owned>]) -> $crate::defs::SdlResult<Self> {
                    match std::ptr::NonNull::new(handle) {
                        Some(handle) => Ok(Self {
                            inner: [<$owned Handle>] { handle },
                        }),
                        None => Err($crate::error::get()),
                    }
                }
            }

            impl std::ops::Deref for $owned {
                type Target = [<$owned Handle>];
                fn deref(&self) -> &Self::Target {
                    &self.inner
                }
            }

            impl std::ops::DerefMut for $owned {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.inner
                }
            }

            impl $crate::traits::Resource for $owned {
                type Handle = [<$owned Handle>];

                unsafe fn as_handle(&self) -> Self::Handle {
                    self.inner
                }
            }

            impl Drop for $owned {
                #[doc(alias = $library "_Destroy" $owned)]
                fn drop(&mut self) {
                    unsafe { [<$library _ $dtor $owned>](self.inner.handle.as_ptr()) }
                }
            }
        }
    };
}

/// Converts an [`Option`] to a pointer.
/// This is a convenience function that also casts the resulting pointer.
///
/// # Output
/// `None` => `std::ptr::null()`
/// `Some(&T)` => `*const T as *const Dst`
///
/// # Safety
/// This is only meant for interfacing with C libraries.
/// All the usual pointer lifetime pitfalls apply.
pub unsafe fn opt2ptr<T, Dst>(opt: Option<&T>) -> *const Dst {
    opt.map_or(std::ptr::null(), |s| s as *const T as *const Dst)
}

pub fn to_result(result: bool) -> SdlResult {
    if result { Ok(()) } else { Err(error::get()) }
}

/// Convert a `NonNull<c_char>` (commonly used in FFI) to a `&str`.
///
/// # Safety
/// This function is VERY unsafe, a non-exhaustive list of assumptions:
/// - `ptr` points to a valid null-terminated C string
/// - the string pointed to by `ptr` is valid UTF-8
///
/// The returned slice's lifetime is `'static`, which is EVEN MORE unsound
/// and I recommend only using it as a one-off temporary value, i.e. to
/// construct a [`String`], or for printing (unless you know for sure that the
/// foreign string won't change its location and/or size).
pub unsafe fn c_ptr_to_str(ptr: *const c_char) -> &'static str {
    unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) }
}
