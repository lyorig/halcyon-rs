use std::ffi::{CStr, c_void};

use sdl3_sys::properties::*;

use crate::{defs::SdlResult, util::to_result};

/// Owned SDL properties.
pub struct Properties {
    id: SDL_PropertiesID,
}

impl Properties {
    #[doc(alias = "SDL_CreateProperties")]
    pub(crate) fn new() -> Self {
        Self {
            id: unsafe { SDL_CreateProperties() },
        }
    }

    pub fn id(&self) -> SDL_PropertiesID {
        self.id
    }

    #[doc(alias = "SDL_GetNumberProperty")]
    pub fn number(&self, key: *const i8, default: i64) -> i64 {
        unsafe { SDL_GetNumberProperty(self.id, key, default) }
    }

    #[doc(alias = "SDL_SetNumberProperty")]
    pub fn set_number(&mut self, key: *const i8, value: i64) -> SdlResult {
        to_result(unsafe { SDL_SetNumberProperty(self.id, key, value) })
    }

    #[doc(alias = "SDL_GetFloatProperty")]
    pub fn float(&self, key: *const i8, default: f32) -> f32 {
        unsafe { SDL_GetFloatProperty(self.id, key, default) }
    }

    #[doc(alias = "SDL_SetFloatProperty")]
    pub fn set_float(&mut self, key: *const i8, value: f32) -> SdlResult {
        to_result(unsafe { SDL_SetFloatProperty(self.id, key, value) })
    }

    #[doc(alias = "SDL_GetPointerProperty")]
    pub fn pointer(&self, key: *const i8, default: *mut c_void) -> *mut c_void {
        unsafe { SDL_GetPointerProperty(self.id, key, default) }
    }

    #[doc(alias = "SDL_SetPointerProperty")]
    pub fn set_pointer(&mut self, key: *const i8, value: *mut c_void) -> SdlResult {
        to_result(unsafe { SDL_SetPointerProperty(self.id, key, value) })
    }

    #[doc(alias = "SDL_GetStringProperty")]
    pub fn string(&self, key: *const i8, default: &CStr) -> &CStr {
        unsafe { CStr::from_ptr(SDL_GetStringProperty(self.id, key, default.as_ptr())) }
    }

    #[doc(alias = "SDL_SetStringProperty")]
    pub fn set_string(&mut self, key: *const i8, value: &CStr) -> SdlResult {
        to_result(unsafe { SDL_SetStringProperty(self.id, key, value.as_ptr()) })
    }

    #[doc(alias = "SDL_GetBooleanProperty")]
    pub fn bool(&self, key: *const i8, default: bool) -> bool {
        unsafe { SDL_GetBooleanProperty(self.id, key, default) }
    }

    #[doc(alias = "SDL_SetBooleanProperty")]
    pub fn set_bool(&mut self, key: *const i8, value: bool) -> SdlResult {
        to_result(unsafe { SDL_SetBooleanProperty(self.id, key, value) })
    }
}

impl Drop for Properties {
    #[doc(alias = "SDL_DestroyProperties")]
    fn drop(&mut self) {
        unsafe { SDL_DestroyProperties(self.id) }
    }
}
