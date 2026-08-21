//! SDL properties API wrapper.
//! A property group ([`Properties`]) is essentially a map, where (in Rust terms):
//! - the key is a `&CStr`
//! - the value is one of {`CString`, `i64`, `f32`, `bool`, `*mut c_void`}.
//!
//! SDL has begun using this API in release 3.2.0, and many of its objects are built
//! by setting certain values on a property group, then calling `SDL_Create*WithProperties()`.
//! This enables extensibility, and is an interesting case for wrapping in an intuitive API.
//!
//! # Builders
//! Since each [`Properties`]-constructible SDL object has a finite well-documented set of properties,
//! Halcyon exposes an intuitive builder for each such object via the associated `builder()` function.
//! Each builder "attaches" to an existing property group, enabling efficient memory usage.
//!
//! For example:
//!
//! ```rust
//! use halcyon::{window::Window, rect::Point, resource::Resource, properties::Properties};
//!
//! // you can also obtain a 'static reference to an existing
//! // global property group via `Properties::global()`
//! let props = Properties::new().unwrap();
//! let wnd = Window::builder(props.as_ref())
//!     .title(c"My Super Amazing Window")
//!     .size(Point::new(640, 480))
//!     .build()
//!     .unwrap();
//! ```
//!
//! # Build-with-cleanup
//! Alongside the usual `.build()` method, builders also expose `build_cleanup()`, which
//! additionally removes all relevant properties from the property group it is attached to.
//!
//! This is useful when using a longer-lived property group, specifically:
//! - you don't want to keep the builder properties in memory, since they won't be used anymore
//! - you intend to re-use it to build something else, and don't want the earlier configuration
//!   to influence future builds
//!
//! # `[Object]CreateInfo` builders
//!
//! Many objects in the GPU submodule use a separate structure in place of constructor arguments,
//! e.g. [`Texture`](crate::gpu::Texture) uses [`TextureCreateInfo`](crate::gpu::TextureCreateInfo).
//! These internally hold a [`SDL_PropertiesID`](sdl3_sys::properties::SDL_PropertiesID), enabling
//! setting further options (SDL calls them "extensions"). I considered a few designs wrapping this
//! behavior in a builder, and finally settled on builders for the `[Object]CreateInfo` structs
//! themselves. Two build options are provided:
//!
//! ```rust
//! fn build(&self, /* required struct fields */) -> FooCreateInfo<'p>;
//! fn build_cleanup(&self, ci: &FooCreateInfo) -> Result<Foo>;
//! ```
//!
//! `build` constructs the `CreateInfo` struct with the builder's properties attached.
//! `build_cleanup` uses an existing `CreateInfo` to create an object, then clears any properties from itself.
//!
//! All `CreateInfo` structs expose an associated `new` function that enables them to be created
//! with only the required components (no properties). In this case, the lifetime is inferred.
//!
//! # API checklist ([source](https://wiki.libsdl.org/SDL3/CategoryProperties))
//! - [x] SDL_ClearProperty
//! - [x] SDL_CopyProperties
//! - [x] SDL_CreateProperties
//! - [x] SDL_DestroyProperties
//! - [x] SDL_EnumerateProperties
//! - [x] SDL_GetBooleanProperty
//! - [x] SDL_GetFloatProperty
//! - [x] SDL_GetGlobalProperties
//! - [x] SDL_GetNumberProperty
//! - [x] SDL_GetPointerProperty
//! - [x] SDL_GetPropertyType
//! - [x] SDL_GetStringProperty
//! - [x] SDL_HasProperty
//! - [x] SDL_SetBooleanProperty
//! - [x] SDL_SetFloatProperty
//! - [x] SDL_SetNumberProperty
//! - [x] SDL_SetPointerProperty
//! - [x] SDL_SetStringProperty
//!
//! Not planned/unavailable for implementation:
//! - SDL_GetNumProperties (since SDL 3.6.0)
//! - SDL_LockProperties
//! - SDL_SetPointerPropertyWithCleanup
//! - SDL_UnlockProperties

use std::{
    ffi::{CStr, c_char, c_void},
    fmt::Display,
    hint::unreachable_unchecked,
    num::NonZero,
};

use sdl3_sys::properties::*;

use crate::{
    Result,
    error::Error,
    resource::{Ref, Resource},
    util::{opt2res_map, to_result},
};

fn ptrify(c: Option<&CStr>) -> *const c_char {
    match c {
        Some(c) => c.as_ptr(),
        None => std::ptr::null(),
    }
}

fn unptrify<'a>(ptr: *const c_char) -> Option<&'a CStr> {
    if ptr.is_null() {
        None
    } else {
        let cs = unsafe { CStr::from_ptr(ptr) };
        Some(cs)
    }
}

pub enum Property<'p> {
    Pointer(*mut c_void),
    String(Option<&'p CStr>),
    Number(i64),
    Float(f32),
    Bool(bool),
}

impl Display for Property<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Property::Pointer(p) => write!(f, "{p:p}"),
            Property::String(s) => match s {
                Some(s) => write!(f, "\"{}\"", s.to_string_lossy()),
                None => write!(f, "[null string]"),
            },
            Property::Number(n) => write!(f, "{n}"),
            Property::Float(fl) => write!(f, "{fl}"),
            Property::Bool(b) => write!(f, "{b}"),
        }
    }
}

#[derive(Clone, Copy)]
#[doc(alias = "SDL_PropertiesID")]
pub struct PropertiesHandle {
    pub(crate) handle: NonZero<u32>,
}

impl PropertiesHandle {
    pub(crate) fn from_id(handle: SDL_PropertiesID) -> Option<Self> {
        NonZero::new(handle.0).map(|handle| Self { handle })
    }

    #[doc(alias = "SDL_GetNumberProperty")]
    pub fn number(&self, key: &CStr, default: i64) -> i64 {
        unsafe { SDL_GetNumberProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_GetFloatProperty")]
    pub fn float(&self, key: &CStr, default: f32) -> f32 {
        unsafe { SDL_GetFloatProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_GetPointerProperty")]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn pointer(&self, key: &CStr, default: *mut c_void) -> *mut c_void {
        unsafe { SDL_GetPointerProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_GetStringProperty")]
    pub fn string(&self, key: &CStr, default: Option<&CStr>) -> Option<&CStr> {
        let ptr = unsafe { SDL_GetStringProperty(self.id(), key.as_ptr(), ptrify(default)) };
        unptrify(ptr)
    }

    #[doc(alias = "SDL_GetBooleanProperty")]
    pub fn bool(&self, key: &CStr, default: bool) -> bool {
        unsafe { SDL_GetBooleanProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_SetNumberProperty")]
    pub fn set_number(&self, key: &CStr, value: i64) -> Result {
        to_result(unsafe { SDL_SetNumberProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_SetFloatProperty")]
    pub fn set_float(&self, key: &CStr, value: f32) -> Result {
        to_result(unsafe { SDL_SetFloatProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_SetPointerProperty")]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn set_pointer(&self, key: &CStr, value: *mut c_void) -> Result {
        to_result(unsafe { SDL_SetPointerProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_SetStringProperty")]
    pub fn set_string(&self, key: &CStr, value: Option<&CStr>) -> Result {
        to_result(unsafe { SDL_SetStringProperty(self.id(), key.as_ptr(), ptrify(value)) })
    }

    #[doc(alias = "SDL_SetBooleanProperty")]
    pub fn set_bool(&self, key: &CStr, value: bool) -> Result {
        to_result(unsafe { SDL_SetBooleanProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_ClearProperty")]
    pub fn clear(&self, key: &CStr) -> Result {
        to_result(unsafe { SDL_ClearProperty(self.id(), key.as_ptr()) })
    }

    #[doc(alias = "SDL_CopyProperties")]
    fn copy_to(&self, dst: Ref<Properties>) -> Result {
        to_result(unsafe { SDL_CopyProperties(self.id(), dst.id()) })
    }

    #[doc(alias = "SDL_HasProperty")]
    pub fn has(&self, key: &CStr) -> bool {
        unsafe { SDL_HasProperty(self.id(), key.as_ptr()) }
    }

    #[doc(alias = "SDL_GetPropertyType")]
    fn type_of(&self, key: &CStr) -> SDL_PropertyType {
        unsafe { SDL_GetPropertyType(self.id(), key.as_ptr()) }
    }

    pub fn id(&self) -> SDL_PropertiesID {
        SDL_PropertiesID::new(self.handle.get())
    }

    fn get(&self, key: &CStr) -> Option<Property<'_>> {
        use Property::*;

        let tp = self.type_of(key);
        if tp == SDL_PropertyType::INVALID {
            None
        } else {
            let ret = match tp {
                SDL_PropertyType::POINTER => Pointer(self.pointer(key, std::ptr::null_mut())),
                SDL_PropertyType::STRING => {
                    let cstr = self.string(key, None);
                    String(cstr)
                }
                SDL_PropertyType::NUMBER => Number(self.number(key, 0)),
                SDL_PropertyType::FLOAT => Float(self.float(key, 0.)),
                SDL_PropertyType::BOOLEAN => Bool(self.bool(key, false)),
                _ => unsafe { unreachable_unchecked() },
            };

            Some(ret)
        }
    }

    /// Enumerate all properties in a group. Accepts a function with a key-value pair as parameters.
    #[doc(alias = "SDL_EnumerateProperties")]
    pub fn enumerate<F: FnMut(&str, Property)>(&self, f: F) -> Result {
        type DynCbk<'a> = dyn FnMut(&str, Property) + 'a;

        unsafe extern "C" fn wrap(
            userdata: *mut c_void,
            props: SDL_PropertiesID,
            name: *const c_char,
        ) {
            unsafe {
                let handle = PropertiesHandle::from_id(props).unwrap_unchecked();
                let key = CStr::from_ptr(name);
                let value = handle.get(key).unwrap_unchecked();
                let f = userdata.cast::<Box<DynCbk<'static>>>().as_mut_unchecked();
                let key_str = str::from_utf8_unchecked(key.to_bytes());

                f(key_str, value);
            }
        }

        let mut f: Box<DynCbk> = Box::new(f);
        let userdata = std::ptr::from_mut(&mut f).cast::<c_void>();

        to_result(unsafe { SDL_EnumerateProperties(self.id(), Some(wrap), userdata) })
    }
}

#[doc(alias = "SDL_PropertiesID")]
pub struct Properties {
    pub(crate) inner: PropertiesHandle,
}

impl Properties {
    pub(crate) fn from_id(handle: SDL_PropertiesID) -> Result<Self> {
        opt2res_map(NonZero::new(handle.0), |handle| Self {
            inner: PropertiesHandle { handle },
        })
    }

    #[doc(alias = "SDL_CreateProperties")]
    pub fn new() -> Result<Self> {
        let id = unsafe { SDL_CreateProperties() };
        match PropertiesHandle::from_id(id) {
            Some(inner) => Ok(Self { inner }),
            None => Err(Error::current()),
        }
    }

    #[doc(alias = "SDL_GetGlobalProperties")]
    pub fn global() -> Result<Ref<'static, Properties>> {
        let id = unsafe { SDL_GetGlobalProperties() };
        match PropertiesHandle::from_id(id) {
            Some(p) => Ok(unsafe { Ref::from_handle(p) }),
            None => Err(Error::current()),
        }
    }
}

impl std::ops::Deref for Properties {
    type Target = PropertiesHandle;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Resource for Properties {
    type Handle = PropertiesHandle;
    unsafe fn as_handle(&self) -> Self::Handle {
        self.inner
    }
}

impl Drop for Properties {
    #[doc(alias = "SDL_DestroyProperties")]
    fn drop(&mut self) {
        unsafe { SDL_DestroyProperties(self.id()) }
    }
}
