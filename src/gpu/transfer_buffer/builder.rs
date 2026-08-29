use std::ffi::{CStr, c_char};

use sdl3_sys::gpu::*;

use crate::{Result, gpu::Device, properties::Properties, resource::Ref};

use super::{TransferBuffer, TransferBufferCreateInfo};

const CREATE_PROPERTIES: [*const c_char; 1] = [SDL_PROP_GPU_TRANSFERBUFFER_CREATE_NAME_STRING];

/// Builder for [`TransferBufferCreateInfo`] properties.
#[derive(Clone, Copy)]
pub struct TransferBufferBuilder<'p> {
    props: Ref<'p, Properties>,
}

impl<'p> TransferBufferBuilder<'p> {
    pub(super) fn new(props: Ref<'p, Properties>) -> Self {
        Self { props }
    }

    /// A name for the transfer buffer, used for debugging.
    #[doc(alias = "SDL_PROP_GPU_TRANSFERBUFFER_CREATE_NAME_STRING")]
    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let key = unsafe { CStr::from_ptr(SDL_PROP_GPU_TRANSFERBUFFER_CREATE_NAME_STRING) };
        _ = self.props.set_string(key, Some(value));
        self
    }

    /// Clear all GPU transfer buffer creation properties from a property group.
    pub fn clear_from(props: Ref<Properties>) {
        for key in CREATE_PROPERTIES {
            _ = props.clear(unsafe { CStr::from_ptr(key) });
        }
    }

    pub fn build(
        &self,
        device: Ref<Device>,
        mut create_info: TransferBufferCreateInfo,
    ) -> Result<TransferBuffer> {
        create_info.0.props = self.props.id();
        TransferBuffer::new(device, &create_info)
    }

    /// Creates a [`TransferBuffer`] using [`TransferBufferCreateInfo`],
    /// then removes all transfer buffer creation properties from the attached group.
    pub fn build_cleanup(
        &self,
        device: Ref<Device>,
        create_info: TransferBufferCreateInfo,
    ) -> Result<TransferBuffer> {
        let res = self.build(device, create_info);
        Self::clear_from(self.props);
        res
    }
}
