use std::ffi::{CStr, c_char};

use sdl3_sys::gpu::*;

use crate::{Result, gpu::Device, properties::Properties, resource::Ref};

use super::{ComputePipeline, ComputePipelineCreateInfo, ShaderFormat};

const CREATE_PROPERTIES: [*const c_char; 1] = [SDL_PROP_GPU_COMPUTEPIPELINE_CREATE_NAME_STRING];

/// Builder for [`ComputePipelineCreateInfo`] properties.
#[derive(Clone, Copy)]
pub struct ComputePipelineCreateInfoBuilder<'p> {
    props: Ref<'p, Properties>,
}

impl<'p> ComputePipelineCreateInfoBuilder<'p> {
    pub(super) fn new(props: Ref<'p, Properties>) -> Self {
        Self { props }
    }

    /// A name for the compute pipeline, used for debugging.
    #[doc(alias = "SDL_PROP_GPU_COMPUTEPIPELINE_CREATE_NAME_STRING")]
    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let key = unsafe { CStr::from_ptr(SDL_PROP_GPU_COMPUTEPIPELINE_CREATE_NAME_STRING) };
        _ = self.props.set_string(key, Some(value));
        self
    }

    /// Clear all GPU compute pipeline creation properties from a property group.
    pub fn clear_from(props: Ref<Properties>) {
        for key in CREATE_PROPERTIES {
            _ = props.clear(unsafe { CStr::from_ptr(key) });
        }
    }

    pub fn build<'bc, 'ep>(
        &self,
        code: &'bc [u8],
        entrypoint: &'ep CStr,
        fmt: ShaderFormat,
        counts: (u32, u32, u32, u32, u32, u32),
        thread_count: (u32, u32, u32),
    ) -> ComputePipelineCreateInfo<'bc, 'ep, 'p> {
        ComputePipelineCreateInfo::new_with_props(
            code,
            entrypoint,
            fmt,
            counts,
            thread_count,
            self.props,
        )
    }

    /// Creates a [`ComputePipeline`] using [`ComputePipelineCreateInfo`],
    /// then removes all compute pipeline creation properties from the group.
    pub fn build_cleanup<'bc, 'ep>(
        &self,
        device: Ref<Device>,
        create_info: &ComputePipelineCreateInfo<'bc, 'ep, 'p>,
    ) -> Result<ComputePipeline> {
        let result = ComputePipeline::new(device, create_info);
        Self::clear_from(self.props);
        result
    }
}
