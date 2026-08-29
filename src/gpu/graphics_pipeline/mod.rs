//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_BindGPUGraphicsPipeline
//! - [x] SDL_CreateGPUGraphicsPipeline
//! - [x] SDL_ReleaseGPUGraphicsPipeline

use std::marker::PhantomData;

use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{
    Result,
    gpu::{ColorTargetDescription, VertexAttribute, VertexBufferDescription},
    impl_enum_transmute, mod_reexport,
    properties::Properties,
    resource::Ref,
    resource_new_no_drop,
};

use super::{
    device::Device,
    pipeline_state::{
        DepthStencilState, GraphicsPipelineTargetInfo, MultisampleState, RasterizerState,
        VertexInputState,
    },
    render_pass::RenderPass,
    shader::Shader,
};

mod_reexport!(builder);

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_GPUPrimitiveType")]
pub enum PrimitiveType {
    TriangleList = SDL_GPUPrimitiveType::TRIANGLELIST.0,
    TriangleStrip = SDL_GPUPrimitiveType::TRIANGLESTRIP.0,
    LineList = SDL_GPUPrimitiveType::LINELIST.0,
    LineStrip = SDL_GPUPrimitiveType::LINESTRIP.0,
    PointList = SDL_GPUPrimitiveType::POINTLIST.0,
}

impl_enum_transmute!(SDL_GPUPrimitiveType, PrimitiveType);

#[doc(alias = "SDL_GPUGraphicsPipelineCreateInfo")]
#[derive(Clone, Copy)]
pub struct GraphicsPipelineCreateInfo<'vs, 'fs, 'vbd, 'va, 'ctd>(
    SDL_GPUGraphicsPipelineCreateInfo,
    PhantomData<Ref<'vs, Shader>>,
    PhantomData<Ref<'fs, Shader>>,
    PhantomData<&'vbd [VertexBufferDescription]>,
    PhantomData<&'va [VertexAttribute]>,
    PhantomData<&'ctd [ColorTargetDescription]>,
);
impl<'vs, 'fs, 'vbd, 'va, 'ctd> GraphicsPipelineCreateInfo<'vs, 'fs, 'vbd, 'va, 'ctd> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        vertex_shader: Ref<'vs, Shader>,
        fragment_shader: Ref<'fs, Shader>,
        vertex_input_state: VertexInputState<'vbd, 'va>,
        primitive_type: PrimitiveType,
        rasterizer_state: RasterizerState,
        multisample_state: MultisampleState,
        depth_stencil_state: DepthStencilState,
        target_info: GraphicsPipelineTargetInfo<'ctd>,
    ) -> Self {
        Self(
            SDL_GPUGraphicsPipelineCreateInfo {
                vertex_shader: vertex_shader.handle.as_ptr(),
                fragment_shader: fragment_shader.handle.as_ptr(),
                vertex_input_state: vertex_input_state.0,
                primitive_type: SDL_GPUPrimitiveType::new(primitive_type as _),
                rasterizer_state: rasterizer_state.0,
                multisample_state: multisample_state.0,
                depth_stencil_state: depth_stencil_state.0,
                target_info: target_info.0,
                props: SDL_PropertiesID::new(0),
            },
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

resource_new_no_drop!(SDL_GPUGraphicsPipeline, GraphicsPipeline);
impl GraphicsPipeline {
    /// Bind a builder to a property group.
    pub fn builder<'p>(props: Ref<'p, Properties>) -> GraphicsPipelineBuilder<'p> {
        GraphicsPipelineBuilder::new(props)
    }

    #[doc(alias = "SDL_CreateGPUGraphicsPipeline")]
    pub fn new(device: Ref<Device>, create_info: &GraphicsPipelineCreateInfo) -> Result<Self> {
        let handle =
            unsafe { SDL_CreateGPUGraphicsPipeline(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUGraphicsPipeline")]
    pub fn drop(self, device: Ref<Device>) {
        unsafe { SDL_ReleaseGPUGraphicsPipeline(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl GraphicsPipelineHandle {
    #[doc(alias = "SDL_BindGPUGraphicsPipeline")]
    pub fn bind(&self, render_pass: Ref<RenderPass>) {
        unsafe { SDL_BindGPUGraphicsPipeline(render_pass.handle.as_ptr(), self.handle.as_ptr()) };
    }
}
