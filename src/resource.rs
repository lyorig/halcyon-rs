use std::{marker::PhantomData, ops::Deref};

pub trait Resource: Sized {
    type Handle: Copy;

    /// Return the raw underlying handle of this object.
    ///
    /// # Safety
    /// Handles are only valid as long as their owning objects.
    /// Using them outside of said lifetime == use-after-free.
    unsafe fn as_handle(&self) -> Self::Handle;

    fn as_ref<'a>(&'a self) -> Ref<'a, Self> {
        Ref {
            handle: unsafe { self.as_handle() },
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct Ref<'a, T: Resource> {
    handle: T::Handle,
    _marker: PhantomData<&'a T>,
}

impl<T: Resource> Ref<'_, T> {
    pub(crate) unsafe fn from_handle(handle: T::Handle) -> Self {
        Self {
            handle,
            _marker: PhantomData,
        }
    }
}

impl<T: Resource> Clone for Ref<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Resource> Copy for Ref<'_, T> {}

impl<T: Resource> Deref for Ref<'_, T> {
    type Target = T::Handle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

#[macro_export]
macro_rules! resource_new_impl {
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

            impl $crate::resource::Resource for $owned {
                type Handle = [<$owned Handle>];

                unsafe fn as_handle(&self) -> Self::Handle {
                    self.inner
                }
            }
        }
    };
}

#[macro_export]
macro_rules! resource_new_no_drop {
    ($owned:ident) => {
        resource_new_no_drop!($owned, SDL);
    };

    ($owned:ident, $library:ident) => {
        paste::paste! {
            #[must_use = "This struct has to be manually dropped via an associated `drop()` method."]
            pub struct $owned {
                pub(crate) inner: [<$owned Handle>],
            }

        }

        $crate::resource_new_impl!($owned, $library);
    };
}

/// Define a resource and implement shared traits and member functions.
#[macro_export]
macro_rules! resource_new {
    ($owned:ident) => {
        resource_new!($owned, SDL);
    };

    ($owned:ident, $library:ident) => {
        resource_new!($owned, $library, Destroy);
    };

    ($owned:ident, $library:ident, $dtor: ident) => {
        paste::paste! {
            pub struct $owned {
                pub(crate) inner: [<$owned Handle>],
            }
        }

        $crate::resource_new_impl!($owned, $library);

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
macro_rules! resource_new_tied {
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

            impl $crate::resource::Resource for $owned<'_> {
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
