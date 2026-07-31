//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_BindGPUGraphicsPipeline
//! - [x] SDL_CreateGPUGraphicsPipeline
//! - [x] SDL_ReleaseGPUGraphicsPipeline

use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{Result, resource_no_drop, traits::Ref};

use super::{
    device::GPUDevice,
    pipeline_state::{
        DepthStencilState, GraphicsPipelineTargetInfo, MultisampleState, RasterizerState,
        VertexInputState,
    },
    render_pass::GPURenderPass,
    shader::GPUShader,
};

#[repr(i32)]
#[doc(alias = "SDL_GPUPrimitiveType")]
pub enum PrimitiveType {
    TriangleList = SDL_GPUPrimitiveType::TRIANGLELIST.0,
    TriangleStrip = SDL_GPUPrimitiveType::TRIANGLESTRIP.0,
    LineList = SDL_GPUPrimitiveType::LINELIST.0,
    LineStrip = SDL_GPUPrimitiveType::LINESTRIP.0,
    PointList = SDL_GPUPrimitiveType::POINTLIST.0,
}

#[doc(alias = "SDL_GPUGraphicsPipelineCreateInfo")]
#[derive(Clone, Copy)]
pub struct GraphicsPipelineCreateInfo(SDL_GPUGraphicsPipelineCreateInfo);
impl GraphicsPipelineCreateInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        vertex_shader: Ref<GPUShader>,
        fragment_shader: Ref<GPUShader>,
        vertex_input_state: VertexInputState,
        primitive_type: PrimitiveType,
        rasterizer_state: RasterizerState,
        multisample_state: MultisampleState,
        depth_stencil_state: DepthStencilState,
        target_info: GraphicsPipelineTargetInfo,
    ) -> Self {
        Self(SDL_GPUGraphicsPipelineCreateInfo {
            vertex_shader: vertex_shader.handle.as_ptr(),
            fragment_shader: fragment_shader.handle.as_ptr(),
            vertex_input_state: vertex_input_state.0,
            primitive_type: SDL_GPUPrimitiveType::new(primitive_type as _),
            rasterizer_state: rasterizer_state.0,
            multisample_state: multisample_state.0,
            depth_stencil_state: depth_stencil_state.0,
            target_info: target_info.0,
            props: SDL_PropertiesID::new(0),
        })
    }
}

resource_no_drop!(GPUGraphicsPipeline);
impl GPUGraphicsPipeline {
    #[doc(alias = "SDL_CreateGPUGraphicsPipeline")]
    pub fn new(device: Ref<GPUDevice>, create_info: &GraphicsPipelineCreateInfo) -> Result<Self> {
        let handle =
            unsafe { SDL_CreateGPUGraphicsPipeline(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUGraphicsPipeline")]
    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUGraphicsPipeline(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl GPUGraphicsPipelineHandle {
    #[doc(alias = "SDL_BindGPUGraphicsPipeline")]
    pub fn bind(&self, render_pass: Ref<GPURenderPass>) {
        unsafe { SDL_BindGPUGraphicsPipeline(render_pass.handle.as_ptr(), self.handle.as_ptr()) };
    }
}
