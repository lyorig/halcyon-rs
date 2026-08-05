//! API checklist [source](https://wiki.libsdl.org/SDL3/CategoryProperties):
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
    ffi::{CStr, c_void},
    fmt::Display,
    hint::unreachable_unchecked,
    num::NonZero,
};

use sdl3_sys::properties::*;

use crate::{
    Result,
    error::Error,
    resource::{Ref, Resource},
    util::to_result,
};

#[derive(Clone, Copy)]
pub enum Property {
    Pointer(*mut c_void),
    String(*const i8),
    Number(i64),
    Float(f32),
    Bool(bool),
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Pointer(p) => write!(f, "{p:p}"),
            Property::String(s) => {
                let str = if s.is_null() {
                    "<null string>"
                } else {
                    &unsafe { CStr::from_ptr(*s) }.to_string_lossy()
                };
                write!(f, "{str}")
            }
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

    pub fn id(&self) -> SDL_PropertiesID {
        SDL_PropertiesID::new(self.handle.get())
    }

    #[doc(alias = "SDL_GetNumberProperty")]
    pub fn number(&self, key: &CStr, default: i64) -> i64 {
        unsafe { SDL_GetNumberProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_SetNumberProperty")]
    pub fn set_number(&self, key: &CStr, value: i64) -> Result {
        to_result(unsafe { SDL_SetNumberProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_GetFloatProperty")]
    pub fn float(&self, key: &CStr, default: f32) -> f32 {
        unsafe { SDL_GetFloatProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_SetFloatProperty")]
    pub fn set_float(&self, key: &CStr, value: f32) -> Result {
        to_result(unsafe { SDL_SetFloatProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_GetPointerProperty")]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn pointer(&self, key: &CStr, default: *mut c_void) -> *mut c_void {
        unsafe { SDL_GetPointerProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_SetPointerProperty")]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn set_pointer(&self, key: &CStr, value: *mut c_void) -> Result {
        to_result(unsafe { SDL_SetPointerProperty(self.id(), key.as_ptr(), value) })
    }

    pub fn set(&self, key: &CStr, value: Property) -> Result {
        use Property::*;

        match value {
            Pointer(p) => self.set_pointer(key, p),
            String(s) => self.set_string(key, s),
            Number(n) => self.set_number(key, n),
            Float(f) => self.set_float(key, f),
            Bool(b) => self.set_bool(key, b),
        }
    }

    pub fn get(&self, key: &CStr) -> Option<Property> {
        use Property::*;

        match self.type_of(key) {
            SDL_PropertyType::INVALID => None,
            SDL_PropertyType::POINTER => Some(Pointer(self.pointer(key, std::ptr::null_mut()))),
            SDL_PropertyType::STRING => Some(String(self.string(key, std::ptr::null()))),
            SDL_PropertyType::NUMBER => Some(Number(self.number(key, 0))),
            SDL_PropertyType::FLOAT => Some(Float(self.float(key, 0.))),
            SDL_PropertyType::BOOLEAN => Some(Bool(self.bool(key, false))),
            _ => unsafe { unreachable_unchecked() },
        }
    }

    #[doc(alias = "SDL_GetStringProperty")]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn string(&self, key: &CStr, default: *const i8) -> *const i8 {
        unsafe { SDL_GetStringProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_SetStringProperty")]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn set_string(&self, key: &CStr, value: *const i8) -> Result {
        to_result(unsafe { SDL_SetStringProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_GetBooleanProperty")]
    pub fn bool(&self, key: &CStr, default: bool) -> bool {
        unsafe { SDL_GetBooleanProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_SetBooleanProperty")]
    pub fn set_bool(&self, key: &CStr, value: bool) -> Result {
        to_result(unsafe { SDL_SetBooleanProperty(self.id(), key.as_ptr(), value) })
    }

    /// Enumerate all properties. Accepts a function with a key-value pair as parameters.
    #[doc(alias = "SDL_EnumerateProperties")]
    pub fn enumerate<F: FnMut(&CStr, Property)>(&self, f: F) -> Result {
        type DynCbk<'a> = dyn FnMut(&CStr, Property) + 'a;

        // SDL invokes the callback synchronously inside `SDL_EnumerateProperties`,
        // so the closure can live in a `Box` on the stack for the duration of the
        // call, with the `Box` itself handed to SDL as the opaque `userdata`
        // pointer. This only involves thin pointer casts, unlike the previous
        // version which transmuted between function and data pointers.
        unsafe extern "C" fn wrap(userdata: *mut c_void, props: SDL_PropertiesID, name: *const i8) {
            // SAFETY: We are enumerating a valid property group.
            let handle = unsafe { PropertiesHandle::from_id(props).unwrap_unchecked() };

            // SAFETY: This `Ref` is only used inside the body of `enumerate()`.
            let r: Ref<'_, Properties> = unsafe { Ref::from_handle(handle) };

            // SAFETY: SDL property names are null-terminated.
            let key = unsafe { CStr::from_ptr(name) };

            // SAFETY: Existing properties always have an associated value.
            let value = unsafe { r.get(key).unwrap_unchecked() };

            // SAFETY: Smuggled inside `userdata`, see body of `enumerate`.
            let f = unsafe { userdata.cast::<Box<DynCbk<'static>>>().as_mut_unchecked() };

            f(key, value);
        }

        let mut f: Box<DynCbk<'_>> = Box::new(f);
        let userdata = std::ptr::from_mut(&mut f).cast::<c_void>();

        to_result(unsafe { SDL_EnumerateProperties(self.id(), Some(wrap), userdata) })
    }

    #[doc(alias = "SDL_ClearProperty")]
    fn clear(&self, key: &CStr) -> Result {
        to_result(unsafe { SDL_ClearProperty(self.id(), key.as_ptr()) })
    }

    #[doc(alias = "SDL_CopyProperties")]
    fn copy_to(&self, dst: Ref<'_, Properties>) -> Result {
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
}

#[doc(alias = "SDL_PropertiesID")]
pub struct Properties {
    pub(crate) inner: PropertiesHandle,
}

impl Properties {
    pub(crate) fn from_id(handle: SDL_PropertiesID) -> Result<Self> {
        match NonZero::new(handle.0) {
            Some(handle) => Ok(Self {
                inner: PropertiesHandle { handle },
            }),
            None => Err(Error::current()),
        }
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
