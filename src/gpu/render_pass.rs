//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_BeginGPURenderPass
//! - [x] SDL_EndGPURenderPass
//! - [x] SDL_BindGPUFragmentSamplers
//! - [x] SDL_BindGPUFragmentStorageBuffers
//! - [x] SDL_BindGPUFragmentStorageTextures
//! - [x] SDL_BindGPUIndexBuffer
//! - [x] SDL_BindGPUVertexBuffers
//! - [x] SDL_BindGPUVertexSamplers
//! - [x] SDL_BindGPUVertexStorageBuffers
//! - [x] SDL_BindGPUVertexStorageTextures
//! - [x] SDL_DrawGPUIndexedPrimitives
//! - [x] SDL_DrawGPUIndexedPrimitivesIndirect
//! - [x] SDL_DrawGPUPrimitives
//! - [x] SDL_DrawGPUPrimitivesIndirect
//! - [x] SDL_SetGPUBlendConstants
//! - [x] SDL_SetGPUScissor
//! - [x] SDL_SetGPUStencilReference
//! - [x] SDL_SetGPUViewport

use sdl3_sys::{gpu::*, pixels::SDL_FColor};

use crate::{Result, gpu::GPUBuffer, rect::RectI32, resource, traits::Ref, util::opt2ptr};

use super::{command_buffer::GPUCommandBuffer, texture::GPUTexture};

resource!(GPURenderPass, SDL, End);
impl GPURenderPass {
    #[doc(alias = "SDL_BeginGPURenderPass")]
    pub fn new(
        cmdbuf: Ref<GPUCommandBuffer>,
        color_targets: &[SDL_GPUColorTargetInfo],
        depth_stencil_target: Option<&SDL_GPUDepthStencilTargetInfo>,
    ) -> Result<Self> {
        let handle = unsafe {
            SDL_BeginGPURenderPass(
                cmdbuf.handle.as_ptr(),
                color_targets.as_ptr(),
                color_targets.len() as _,
                opt2ptr(depth_stencil_target),
            )
        };

        Self::from_ptr(handle)
    }
}

impl GPURenderPassHandle {
    #[doc(alias = "SDL_SetGPUViewport")]
    pub fn set_viewport(&self, viewport: &SDL_GPUViewport) {
        unsafe { SDL_SetGPUViewport(self.handle.as_ptr(), viewport) }
    }

    #[doc(alias = "SDL_SetGPUBlendConstants")]
    pub fn set_blend_constants(&self, blend_constants: SDL_FColor) {
        unsafe { SDL_SetGPUBlendConstants(self.handle.as_ptr(), blend_constants) }
    }

    #[doc(alias = "SDL_SetGPUStencilReference")]
    pub fn set_stencil_reference(&self, reference: u8) {
        unsafe { SDL_SetGPUStencilReference(self.handle.as_ptr(), reference) }
    }

    #[doc(alias = "SDL_SetGPUScissor")]
    pub fn set_scissor(&self, scissor: &RectI32) {
        unsafe { SDL_SetGPUScissor(self.handle.as_ptr(), scissor.as_sdl_ptr()) };
    }

    #[doc(alias = "SDL_BindGPUVertexBuffers")]
    pub fn bind_vertex_buffers(&self, first_slot: u32, bindings: &[SDL_GPUBufferBinding]) {
        unsafe {
            SDL_BindGPUVertexBuffers(
                self.handle.as_ptr(),
                first_slot,
                bindings.as_ptr(),
                bindings.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_BindGPUIndexBuffer")]
    pub fn bind_index_buffer(
        &self,
        binding: &SDL_GPUBufferBinding,
        index_element_size: SDL_GPUIndexElementSize,
    ) {
        unsafe { SDL_BindGPUIndexBuffer(self.handle.as_ptr(), binding, index_element_size) }
    }

    #[doc(alias = "SDL_BindGPUVertexSamplers")]
    pub fn bind_vertex_samplers(&self, first_slot: u32, bindings: &[SDL_GPUTextureSamplerBinding]) {
        unsafe {
            SDL_BindGPUVertexSamplers(
                self.handle.as_ptr(),
                first_slot,
                bindings.as_ptr(),
                bindings.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_BindGPUVertexStorageTextures")]
    pub fn bind_vertex_storage_textures(&self, first_slot: u32, textures: &[Ref<GPUTexture>]) {
        unsafe {
            SDL_BindGPUVertexStorageTextures(
                self.handle.as_ptr(),
                first_slot,
                textures.as_ptr().cast(),
                textures.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_BindGPUVertexStorageBuffers")]
    pub fn bind_vertex_storage_buffers(&self, first_slot: u32, buffers: &[Ref<GPUBuffer>]) {
        unsafe {
            SDL_BindGPUVertexStorageBuffers(
                self.handle.as_ptr(),
                first_slot,
                buffers.as_ptr().cast(),
                buffers.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_BindGPUFragmentSamplers")]
    pub fn bind_fragment_samplers(
        &self,
        first_slot: u32,
        bindings: &[SDL_GPUTextureSamplerBinding],
    ) {
        unsafe {
            SDL_BindGPUFragmentSamplers(
                self.handle.as_ptr(),
                first_slot,
                bindings.as_ptr(),
                bindings.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_BindGPUFragmentStorageTextures")]
    pub fn bind_fragment_storage_textures(&self, first_slot: u32, textures: &[Ref<GPUTexture>]) {
        unsafe {
            SDL_BindGPUFragmentStorageTextures(
                self.handle.as_ptr(),
                first_slot,
                textures.as_ptr().cast(),
                textures.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_BindGPUFragmentStorageBuffers")]
    pub fn bind_fragment_storage_buffers(&self, first_slot: u32, buffers: &[Ref<GPUBuffer>]) {
        unsafe {
            SDL_BindGPUFragmentStorageBuffers(
                self.handle.as_ptr(),
                first_slot,
                buffers.as_ptr().cast(),
                buffers.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_DrawGPUPrimitives")]
    pub fn draw_primitives(&self, n_verts: u32, n_insts: u32, first_vert: u32, first_inst: u32) {
        unsafe {
            SDL_DrawGPUPrimitives(
                self.handle.as_ptr(),
                n_verts,
                n_insts,
                first_vert,
                first_inst,
            )
        }
    }

    #[doc(alias = "SDL_DrawGPUPrimitivesIndirect")]
    pub fn draw_primitives_indirect(&self, buffer: Ref<GPUBuffer>, offset: u32, draw_count: u32) {
        unsafe {
            SDL_DrawGPUPrimitivesIndirect(
                self.handle.as_ptr(),
                buffer.handle.as_ptr(),
                offset,
                draw_count,
            )
        }
    }

    #[doc(alias = "SDL_DrawGPUIndexedPrimitives")]
    pub fn draw_indexed_primitives(
        &self,
        num_indices: u32,
        num_instances: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            SDL_DrawGPUIndexedPrimitives(
                self.handle.as_ptr(),
                num_indices,
                num_instances,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
    }

    #[doc(alias = "SDL_DrawGPUIndexedPrimitivesIndirect")]
    pub fn draw_indexed_primitives_indirect(
        &self,
        buffer: Ref<GPUBuffer>,
        offset: u32,
        draw_count: u32,
    ) {
        unsafe {
            SDL_DrawGPUIndexedPrimitivesIndirect(
                self.handle.as_ptr(),
                buffer.handle.as_ptr(),
                offset,
                draw_count,
            )
        }
    }
}
