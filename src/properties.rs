use std::{
    ffi::{CStr, c_void},
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
    pub fn set_number(&mut self, key: &CStr, value: i64) -> Result {
        to_result(unsafe { SDL_SetNumberProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_GetFloatProperty")]
    pub fn float(&self, key: &CStr, default: f32) -> f32 {
        unsafe { SDL_GetFloatProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_SetFloatProperty")]
    pub fn set_float(&mut self, key: &CStr, value: f32) -> Result {
        to_result(unsafe { SDL_SetFloatProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_GetPointerProperty")]
    pub fn pointer(&self, key: &CStr, default: *mut c_void) -> *mut c_void {
        unsafe { SDL_GetPointerProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_SetPointerProperty")]
    pub fn set_pointer(&mut self, key: &CStr, value: *mut c_void) -> Result {
        to_result(unsafe { SDL_SetPointerProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_GetStringProperty")]
    pub fn string(&self, key: &CStr, default: &CStr) -> &CStr {
        unsafe {
            CStr::from_ptr(SDL_GetStringProperty(
                self.id(),
                key.as_ptr(),
                default.as_ptr(),
            ))
        }
    }

    #[doc(alias = "SDL_SetStringProperty")]
    pub fn set_string(&mut self, key: &CStr, value: &CStr) -> Result {
        to_result(unsafe { SDL_SetStringProperty(self.id(), key.as_ptr(), value.as_ptr()) })
    }

    #[doc(alias = "SDL_GetBooleanProperty")]
    pub fn bool(&self, key: &CStr, default: bool) -> bool {
        unsafe { SDL_GetBooleanProperty(self.id(), key.as_ptr(), default) }
    }

    #[doc(alias = "SDL_SetBooleanProperty")]
    pub fn set_bool(&mut self, key: &CStr, value: bool) -> Result {
        to_result(unsafe { SDL_SetBooleanProperty(self.id(), key.as_ptr(), value) })
    }

    #[doc(alias = "SDL_EnumerateProperties")]
    pub fn enumerate<F: FnMut(Ref<'_, Properties>, &CStr)>(&self, f: F) -> Result {
        use std::ffi::c_void;

        // SDL invokes the callback synchronously inside `SDL_EnumerateProperties`,
        // so the closure can live in a `Box` on the stack for the duration of the
        // call, with the `Box` itself handed to SDL as the opaque `userdata`
        // pointer. This only involves thin pointer casts, unlike the previous
        // version which transmuted between function and data pointers.
        unsafe extern "C" fn wrap(userdata: *mut c_void, props: SDL_PropertiesID, name: *const i8) {
            let f = unsafe { &mut *userdata.cast::<Box<dyn FnMut(Ref<'_, Properties>, &CStr)>>() };
            let handle = unsafe { PropertiesHandle::from_id(props).unwrap_unchecked() };
            let r: Ref<'_, Properties> = unsafe { Ref::from_handle(handle) };

            f(r, unsafe { CStr::from_ptr(name) });
        }

        let mut f: Box<dyn FnMut(Ref<'_, Properties>, &CStr)> = Box::new(f);
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
        match NonZero::new(handle.0) {
            Some(handle) => Ok(Self {
                inner: PropertiesHandle { handle },
            }),
            None => Err(Error::current()),
        }
    }

    #[doc(alias = "SDL_CreateProperties")]
    pub fn new() -> Result<Self> {
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
