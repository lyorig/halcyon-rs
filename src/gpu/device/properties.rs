use std::ffi::{CStr, c_char};

use sdl3_sys::gpu::*;

use crate::{
    properties::{Properties, PropertiesHandle},
    resource::Ref,
};

#[derive(Clone, Copy)]
pub struct DeviceProperties<'a> {
    inner: Ref<'a, Properties>,
}

impl<'a> DeviceProperties<'a> {
    pub(super) fn new(inner: Ref<'a, Properties>) -> Self {
        Self { inner }
    }

    fn get(&self, key: *const c_char) -> Option<&str> {
        let cstr = unsafe { CStr::from_ptr(key) };
        let s = self.inner.string(cstr, None);

        s.map(|c| unsafe { str::from_utf8_unchecked(c.to_bytes()) })
    }

    pub fn device_name(&self) -> Option<&str> {
        self.get(SDL_PROP_GPU_DEVICE_NAME_STRING)
    }

    pub fn driver_name(&self) -> Option<&str> {
        self.get(SDL_PROP_GPU_DEVICE_DRIVER_NAME_STRING)
    }

    pub fn driver_version(&self) -> Option<&str> {
        self.get(SDL_PROP_GPU_DEVICE_DRIVER_VERSION_STRING)
    }

    pub fn driver_info(&self) -> Option<&str> {
        self.get(SDL_PROP_GPU_DEVICE_DRIVER_INFO_STRING)
    }
}

impl std::ops::Deref for DeviceProperties<'_> {
    type Target = PropertiesHandle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
