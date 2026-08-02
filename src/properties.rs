use std::{
    ffi::{CStr, c_char, c_void},
    num::NonZero,
};

use sdl3_sys::properties::*;

use crate::{Result, error::Error, resource::Resource, util::to_result};

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
    pub fn number(&self, key: *const c_char, default: i64) -> i64 {
        unsafe { SDL_GetNumberProperty(self.id(), key, default) }
    }

    #[doc(alias = "SDL_SetNumberProperty")]
    pub fn set_number(&mut self, key: *const c_char, value: i64) -> Result {
        to_result(unsafe { SDL_SetNumberProperty(self.id(), key, value) })
    }

    #[doc(alias = "SDL_GetFloatProperty")]
    pub fn float(&self, key: *const c_char, default: f32) -> f32 {
        unsafe { SDL_GetFloatProperty(self.id(), key, default) }
    }

    #[doc(alias = "SDL_SetFloatProperty")]
    pub fn set_float(&mut self, key: *const c_char, value: f32) -> Result {
        to_result(unsafe { SDL_SetFloatProperty(self.id(), key, value) })
    }

    #[doc(alias = "SDL_GetPointerProperty")]
    pub fn pointer(&self, key: *const c_char, default: *mut c_void) -> *mut c_void {
        unsafe { SDL_GetPointerProperty(self.id(), key, default) }
    }

    #[doc(alias = "SDL_SetPointerProperty")]
    pub fn set_pointer(&mut self, key: *const c_char, value: *mut c_void) -> Result {
        to_result(unsafe { SDL_SetPointerProperty(self.id(), key, value) })
    }

    #[doc(alias = "SDL_GetStringProperty")]
    pub fn string(&self, key: *const c_char, default: &CStr) -> &CStr {
        unsafe { CStr::from_ptr(SDL_GetStringProperty(self.id(), key, default.as_ptr())) }
    }

    #[doc(alias = "SDL_SetStringProperty")]
    pub fn set_string(&mut self, key: *const c_char, value: &CStr) -> Result {
        to_result(unsafe { SDL_SetStringProperty(self.id(), key, value.as_ptr()) })
    }

    #[doc(alias = "SDL_GetBooleanProperty")]
    pub fn bool(&self, key: *const c_char, default: bool) -> bool {
        unsafe { SDL_GetBooleanProperty(self.id(), key, default) }
    }

    #[doc(alias = "SDL_SetBooleanProperty")]
    pub fn set_bool(&mut self, key: *const c_char, value: bool) -> Result {
        to_result(unsafe { SDL_SetBooleanProperty(self.id(), key, value) })
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
    pub(crate) fn new() -> Result<Self> {
        match PropertiesHandle::from_id(unsafe { SDL_CreateProperties() }) {
            Some(inner) => Ok(Self { inner }),
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
