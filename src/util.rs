use std::ffi::{CStr, c_char};

use crate::{Result, error::Error};

#[macro_export]
macro_rules! resource_impl {
    ($owned:ident, $library:ident) => {
        paste::paste! {
            #[derive(Clone, Copy)]
            #[doc(alias = $library "_" $owned)]
            pub struct [<$owned Handle>] {
                pub(crate) handle: std::ptr::NonNull<[<$library _ $owned>]>,
            }

            impl [<$owned Handle>] {
                pub(crate) fn from_ptr(handle: *mut [<$library _ $owned>]) -> Option<Self> {
                    std::ptr::NonNull::new(handle).map(|handle| Self { handle })
                }
            }

            impl $owned {
                pub(crate) fn from_ptr(handle: *mut [<$library _ $owned>]) -> $crate::Result<Self> {
                    match std::ptr::NonNull::new(handle) {
                        Some(handle) => Ok(Self {
                            inner: [<$owned Handle>] { handle },
                        }),
                        None => Err($crate::error::Error::current()),
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
        }
    };
}

#[macro_export]
macro_rules! resource_no_drop {
    ($owned:ident) => {
        resource_no_drop!($owned, SDL);
    };

    ($owned:ident, $library:ident) => {
        paste::paste! {
            #[must_use = "This struct has to be manually dropped via an associated `drop()` method."]
            pub struct $owned {
                pub(crate) inner: [<$owned Handle>],
            }

        }

        $crate::resource_impl!($owned, $library);
    };
}

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
            pub struct $owned {
                pub(crate) inner: [<$owned Handle>],
            }
        }

        $crate::resource_impl!($owned, $library);

        paste::paste! {
            impl Drop for $owned {
                #[doc(alias = $library "_" $dtor $owned)]
                fn drop(&mut self) {
                    unsafe { [<$library _ $dtor $owned>](self.inner.handle.as_ptr()) }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! resource_tied {
    ($owned:ident, $library:ident, $dtor:ident, $tied:ident) => {
            paste::paste! {
                #[doc(alias = $library "_" $owned)]
                pub struct $owned<'a> {
                    pub(crate) inner: [<$owned Handle>],
                    marker: PhantomData<&'a $tied>,
                }

                #[derive(Clone, Copy)]
                #[doc(alias = $library "_" $owned)]
                pub struct [<$owned Handle>] {
                    pub(crate) handle: std::ptr::NonNull<[<$library _ $owned>]>,
                }

                impl [<$owned Handle>] {
                    pub(crate) fn from_ptr(handle: *mut [<$library _ $owned>]) -> Option<Self> {
                        std::ptr::NonNull::new(handle).map(|handle| Self { handle })
                    }
                }

                impl $owned<'_> {
                    pub(crate) fn from_ptr<'a>(handle: *mut [<$library _ $owned>]) -> $crate::Result<$owned<'a>> {
                        match std::ptr::NonNull::new(handle) {
                            Some(handle) => Ok($owned {
                                inner: [<$owned Handle>] { handle },
                                marker: PhantomData,
                            }),
                            None => Err($crate::error::Error::current()),
                        }
                    }
                }

                impl std::ops::Deref for $owned<'_> {
                    type Target = [<$owned Handle>];
                    fn deref(&self) -> &Self::Target {
                        &self.inner
                    }
                }

                impl std::ops::DerefMut for $owned<'_> {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.inner
                    }
                }

                impl $crate::traits::Resource for $owned<'_> {
                    type Handle = [<$owned Handle>];

                    unsafe fn as_handle(&self) -> Self::Handle {
                        self.inner
                    }
                }

                impl Drop for $owned<'_> {
                    #[doc(alias = $library "_" $dtor $owned)]
                    fn drop(&mut self) {
                        unsafe { [<$library _ $dtor $owned>](self.inner.handle.as_ptr()) }
                    }
                }
            }
        }
}

#[macro_export]
macro_rules! boolenum {
    ($name:ident) => {
        #[repr(u8)]
        #[derive(Clone, Copy)]
        pub enum $name {
            No = 0,
            Yes = 1,
        }

        impl From<$name> for bool {
            fn from(value: $name) -> Self {
                match value {
                    $name::No => false,
                    $name::Yes => true,
                }
            }
        }
    };
}

/// Converts an [`Option`] holding a reference to a pointer.
/// As you would expect, `None` produces [`std::ptr::null()`], while
/// `Some` returns `&T` as a pointer.
///
/// This function's purpose is to facilitate interfacing with C FFI libraries.
pub fn opt2ptr<T>(opt: Option<&T>) -> *const T {
    opt.map_or(std::ptr::null(), |s| s)
}

/// Analogous to [`opt2ptr`], but for mutable references.
pub fn opt2ptr_mut<T>(opt: Option<&mut T>) -> *mut T {
    opt.map_or(std::ptr::null_mut(), |s| s)
}

pub fn to_result(result: bool) -> Result {
    if result {
        Ok(())
    } else {
        Err(Error::current())
    }
}

/// Convert a `NonNull<c_char>` (commonly used in FFI) to a `&str`.
///
/// # Safety
/// This function is VERY unsafe, a non-exhaustive list of assumptions:
/// - `ptr` points to a valid null-terminated C string
/// - the string pointed to by `ptr` is valid UTF-8
///
/// The returned value's lifetime is inferred from its usage (see [`CStr::from_ptr`]).
///
pub unsafe fn c_ptr_to_str<'a>(ptr: *const c_char) -> &'a str {
    unsafe { str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) }
}
