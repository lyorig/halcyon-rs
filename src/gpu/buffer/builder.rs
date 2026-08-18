use std::ffi::{CStr, c_char};

use sdl3_sys::gpu::*;

use crate::{Result, gpu::Device, properties::Properties, resource::Ref};

use super::{Buffer, BufferCreateInfo, BufferUsageFlags};

const CREATE_PROPERTIES: [*const c_char; 1] = [SDL_PROP_GPU_BUFFER_CREATE_NAME_STRING];

/// Builder for [`BufferCreateInfo`] properties.
#[derive(Clone, Copy)]
pub struct BufferCreateInfoBuilder<'p> {
    props: Ref<'p, Properties>,
}

impl<'p> BufferCreateInfoBuilder<'p> {
    pub(super) fn new(props: Ref<'p, Properties>) -> Self {
        Self { props }
    }

    /// A name for the buffer, used for debugging.
    #[doc(alias = "SDL_PROP_GPU_BUFFER_CREATE_NAME_STRING")]
    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let key = unsafe { CStr::from_ptr(SDL_PROP_GPU_BUFFER_CREATE_NAME_STRING) };
        _ = self.props.set_string(key, Some(value));
        self
    }

    /// Clear all GPU buffer creation properties from a property group.
    pub fn clear_from(props: Ref<Properties>) {
        for key in CREATE_PROPERTIES {
            _ = props.clear(unsafe { CStr::from_ptr(key) });
        }
    }

    pub fn build(&self, usage: BufferUsageFlags, size: u32) -> BufferCreateInfo<'p> {
        BufferCreateInfo::new_with_props(usage, size, self.props)
    }

    /// Creates a [`Buffer`] using [`BufferCreateInfo`], then removes all
    /// buffer creation properties from the attached property group.
    pub fn build_cleanup(
        &self,
        device: Ref<Device>,
        create_info: &BufferCreateInfo<'p>,
    ) -> Result<Buffer> {
        let result = Buffer::new(device, create_info);
        Self::clear_from(self.props);
        result
    }
}
