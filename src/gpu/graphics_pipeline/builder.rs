use crate::{Result, gpu::Device, properties::Properties, resource::Ref};

use super::{GraphicsPipeline, GraphicsPipelineCreateInfo};

use sdl3_sys::gpu::*;
use std::ffi::{CStr, c_char};

const CREATE_PROPERTIES: [*const c_char; 1] = [SDL_PROP_GPU_GRAPHICSPIPELINE_CREATE_NAME_STRING];

/// Builder for [`GraphicsPipelineCreateInfo`] properties.
#[derive(Clone, Copy)]
pub struct GraphicsPipelineBuilder<'p> {
    props: Ref<'p, Properties>,
}

impl<'p> GraphicsPipelineBuilder<'p> {
    pub(super) fn new(props: Ref<'p, Properties>) -> Self {
        Self { props }
    }

    /// A name for the graphics pipeline, used for debugging.
    #[doc(alias = "SDL_PROP_GPU_GRAPHICSPIPELINE_CREATE_NAME_STRING")]
    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let key = unsafe { CStr::from_ptr(SDL_PROP_GPU_GRAPHICSPIPELINE_CREATE_NAME_STRING) };
        _ = self.props.set_string(key, Some(value));
        self
    }

    /// Clear all GPU graphics pipeline creation properties from a property group.
    pub fn clear_from(props: Ref<Properties>) {
        for key in CREATE_PROPERTIES {
            _ = props.clear(unsafe { CStr::from_ptr(key) });
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn build<'vs, 'fs, 'vbd, 'va, 'ctd>(
        &self,
        device: Ref<Device>,
        mut create_info: GraphicsPipelineCreateInfo<'vs, 'fs, 'vbd, 'va, 'ctd>,
    ) -> Result<GraphicsPipeline> {
        create_info.0.props = self.props.id();
        GraphicsPipeline::new(device, &create_info)
    }

    /// Creates a [`GraphicsPipeline`] using [`GraphicsPipelineCreateInfo`],
    /// then removes all graphics pipeline creation properties from the attached property group.
    #[allow(clippy::too_many_arguments)]
    pub fn build_cleanup<'vs, 'fs, 'vbd, 'va, 'ctd>(
        &self,
        device: Ref<Device>,
        create_info: GraphicsPipelineCreateInfo<'vs, 'fs, 'vbd, 'va, 'ctd>,
    ) -> Result<GraphicsPipeline> {
        let res = self.build(device, create_info);
        Self::clear_from(self.props);
        res
    }
}
