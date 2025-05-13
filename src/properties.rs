use core::ffi;
use std::ffi::{CStr, CString};

use sdl3_sys::properties::*;

pub struct Properties {
    id: SDL_PropertiesID,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            id: unsafe { SDL_CreateProperties() },
        }
    }

    pub fn ptr(&self, name: &CStr, value: *mut ffi::c_void) -> Result<(), CString> {
        crate::util::btur(unsafe { SDL_SetPointerProperty(self.id, name.as_ptr(), value) })
    }

    pub fn f32(&self, name: &CStr, value: f32) -> Result<(), CString> {
        crate::util::btur(unsafe { SDL_SetFloatProperty(self.id, name.as_ptr(), value) })
    }

    pub fn i64(&self, name: &CStr, value: i64) -> Result<(), CString> {
        crate::util::btur(unsafe { SDL_SetNumberProperty(self.id, name.as_ptr(), value) })
    }

    pub fn cstr(&self, name: &CStr, value: &CStr) -> Result<(), CString> {
        crate::util::btur(unsafe { SDL_SetStringProperty(self.id, name.as_ptr(), value.as_ptr()) })
    }

    pub fn bool(&self, name: &CStr, value: bool) -> Result<(), CString> {
        crate::util::btur(unsafe { SDL_SetBooleanProperty(self.id, name.as_ptr(), value) })
    }

    pub fn id(&self) -> SDL_PropertiesID {
        self.id
    }
}

impl Drop for Properties {
    fn drop(&mut self) {
        unsafe { SDL_DestroyProperties(self.id) }
    }
}
