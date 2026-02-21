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
            pub struct [<$owned Ref>]<'a> {
                pub(crate) handle: std::ptr::NonNull<[<$library _ $owned>]>,
                _data: std::marker::PhantomData<&'a ()>
            }

            impl [<$owned Ref>]<'_> {
                pub(crate) fn from_ptr<'a>(handle: *mut [<$library _ $owned>]) -> Option<[<$owned Ref>]<'a>> {
                    std::ptr::NonNull::new(handle).map(|handle| [<$owned Ref>] { handle, _data: std::marker::PhantomData })
                }
            }

            pub struct $owned<'a> {
                inner: [<$owned Ref>]<'a>,
            }

            impl $owned<'_> {
                pub(crate) fn from_ptr(handle: *mut [<$library _ $owned>]) -> crate::defs::SdlResult<Self> {
                    match std::ptr::NonNull::new(handle) {
                        Some(handle) => Ok(Self {
                            inner: [<$owned Ref>] { handle, _data: std::marker::PhantomData },
                        }),
                        None => Err(crate::error::get()),
                    }
                }
            }

            impl<'a> std::ops::Deref for $owned<'a> {
                type Target = [<$owned Ref>]<'a>;
                fn deref(&self) -> &Self::Target {
                    &self.inner
                }
            }

            impl std::ops::DerefMut for $owned<'_> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.inner
                }
            }

            impl<'a> From<&$owned<'a>> for [<$owned Ref>]<'a> {
                fn from(value: &$owned<'a>) -> Self {
                    value.inner
                }
            }

            impl Drop for $owned<'_> {
                #[doc(alias = $library "_Destroy" $owned)]
                fn drop(&mut self) {
                    unsafe { [<$library _ $dtor $owned>](self.inner.handle.as_ptr()) }
                }
            }
        }
    };
}

/// Converts a Halcyon `Option` to an SDL pointer.
/// This is a convenience function that also casts the resulting pointer.
pub unsafe fn opt2ptr<T, Dst>(opt: Option<&T>) -> *const Dst {
    opt.map_or(std::ptr::null(), |s| s as *const T as *const Dst)
}

pub fn to_result(result: bool) -> SdlResult {
    if result { Ok(()) } else { Err(error::get()) }
}

/// Convert a `NonNull<c_char>` (commonly used in FFI) to a `&str`.
/// This function is VERY unsafe, a non-exhaustive list of assumptions:
/// - `ptr` points to a valid null-terminated C string
/// - the string pointed to by `ptr` is valid UTF-8
///
/// The returned slice's lifetime is `'static`, which is EVEN MORE unsound
/// and I recommend only using it as a one-off temporary value, i.e. to
/// construct a `String`, or for printing (unless you know for sure that the
/// foreign string won't change its location and/or size).
pub unsafe fn c_ptr_to_str(ptr: *const c_char) -> &'static str {
    unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) }
}
