use crate::{
    Result,
    gpu::{
        DepthStencilState, Device, GraphicsPipelineTargetInfo, MultisampleState, PrimitiveType,
        RasterizerState, Shader, VertexInputState,
    },
    properties::Properties,
    resource::Ref,
};

use super::{GraphicsPipeline, GraphicsPipelineCreateInfo};

use sdl3_sys::gpu::*;
use std::ffi::{CStr, c_char};

const CREATE_PROPERTIES: [*const c_char; 1] = [SDL_PROP_GPU_GRAPHICSPIPELINE_CREATE_NAME_STRING];

/// Builder for [`GraphicsPipelineCreateInfo`] properties.
#[derive(Clone, Copy)]
pub struct GraphicsPipelineCreateInfoBuilder<'p> {
    props: Ref<'p, Properties>,
}

impl<'p> GraphicsPipelineCreateInfoBuilder<'p> {
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
        vertex_shader: Ref<'vs, Shader>,
        fragment_shader: Ref<'fs, Shader>,
        vertex_input_state: VertexInputState<'vbd, 'va>,
        primitive_type: PrimitiveType,
        rasterizer_state: RasterizerState,
        multisample_state: MultisampleState,
        depth_stencil_state: DepthStencilState,
        target_info: GraphicsPipelineTargetInfo<'ctd>,
    ) -> GraphicsPipelineCreateInfo<'vs, 'fs, 'vbd, 'va, 'ctd, 'p> {
        GraphicsPipelineCreateInfo::new_with_props(
            vertex_shader,
            fragment_shader,
            vertex_input_state,
            primitive_type,
            rasterizer_state,
            multisample_state,
            depth_stencil_state,
            target_info,
            self.props,
        )
    }

    /// Creates a [`GraphicsPipeline`] using [`GraphicsPipelineCreateInfo`],
    /// then removes all graphics pipeline creation properties from the group.
    #[allow(clippy::too_many_arguments)]
    pub fn build_cleanup<'vs, 'fs, 'vbd, 'va, 'ctd>(
        &self,
        device: Ref<Device>,
        create_info: &GraphicsPipelineCreateInfo<'vs, 'fs, 'vbd, 'va, 'ctd, 'p>,
    ) -> Result<GraphicsPipeline> {
        let res = GraphicsPipeline::new(device, create_info);
        Self::clear_from(self.props);
        res
    }
}
