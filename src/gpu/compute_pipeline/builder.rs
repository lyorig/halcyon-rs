use std::ffi::{CStr, c_char};

use sdl3_sys::gpu::*;

use crate::{Result, gpu::Device, properties::Properties, resource::Ref};

use super::{ComputePipeline, ComputePipelineCreateInfo};

const CREATE_PROPERTIES: [*const c_char; 1] = [SDL_PROP_GPU_COMPUTEPIPELINE_CREATE_NAME_STRING];

/// Builder for [`ComputePipelineCreateInfo`] properties.
#[derive(Clone, Copy)]
pub struct ComputePipelineBuilder<'p> {
    props: Ref<'p, Properties>,
}

impl<'p> ComputePipelineBuilder<'p> {
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
        device: Ref<Device>,
        mut create_info: ComputePipelineCreateInfo<'bc, 'ep>,
    ) -> Result<ComputePipeline> {
        create_info.0.props = self.props.id();
        ComputePipeline::new(device, &create_info)
    }

    /// Creates a [`ComputePipeline`] using [`ComputePipelineCreateInfo`],
    /// then removes all compute pipeline creation properties from the attached property group.
    pub fn build_cleanup<'bc, 'ep>(
        &self,
        device: Ref<Device>,
        create_info: ComputePipelineCreateInfo<'bc, 'ep>,
    ) -> Result<ComputePipeline> {
        let res = self.build(device, create_info);
        Self::clear_from(self.props);
        res
    }
}
