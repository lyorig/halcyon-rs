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

use crate::{
    Result,
    color::RgbaF32,
    gpu::GPUBuffer,
    rect::{Point, RectI32},
    resource,
    traits::Ref,
    util::opt2ptr,
};

use super::{
    buffer::BufferBinding,
    command_buffer::GPUCommandBuffer,
    texture::{GPUTexture, TextureSamplerBinding},
};

#[repr(i32)]
#[doc(alias = "SDL_GPUIndexElementSize")]
pub enum IndexElementSize {
    Bits16 = SDL_GPUIndexElementSize::_16BIT.0,
    Bits32 = SDL_GPUIndexElementSize::_32BIT.0,
}

#[doc(alias = "SDL_GPUViewport")]
#[derive(Clone, Copy)]
pub struct Viewport(SDL_GPUViewport);
impl Viewport {
    pub fn new(pos: Point<f32>, size: Point<f32>, (min_depth, max_depth): (f32, f32)) -> Self {
        Self(SDL_GPUViewport {
            x: pos.x,
            y: pos.y,
            w: size.x,
            h: size.y,
            min_depth,
            max_depth,
        })
    }
}

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_GPULoadOp")]
pub enum LoadOp {
    Load = SDL_GPULoadOp::LOAD.0,
    Clear = SDL_GPULoadOp::CLEAR.0,
    DontCare = SDL_GPULoadOp::DONT_CARE.0,
}

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_GPUStoreOp")]
pub enum StoreOp {
    Store = SDL_GPUStoreOp::STORE.0,
    DontCare = SDL_GPUStoreOp::DONT_CARE.0,
    Resolve = SDL_GPUStoreOp::RESOLVE.0,
    ResolveAndStore = SDL_GPUStoreOp::RESOLVE_AND_STORE.0,
}

#[doc(alias = "SDL_GPUColorTargetInfo")]
#[derive(Clone, Copy)]
pub struct ColorTargetInfo(SDL_GPUColorTargetInfo);
impl ColorTargetInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tex: Ref<GPUTexture>,
        mip_level: u32,
        layer_or_depth_plane: u32,
        clear_color: RgbaF32,
        load_op: LoadOp,
        store_op: StoreOp,
        resolve_texture: Option<Ref<GPUTexture>>,
        (resolve_mip_level, resolve_layer): (u32, u32),
        cycle: bool,
        cycle_resolve_texture: bool,
    ) -> Self {
        // SAFETY: `RgbaF32` is `#[repr(C)]` and layout-identical to `SDL_FColor`.
        let clear_color: SDL_FColor = unsafe { std::mem::transmute(clear_color) };
        let resolve_texture = resolve_texture.map_or(std::ptr::null_mut(), |t| t.handle.as_ptr());
        Self(SDL_GPUColorTargetInfo {
            texture: tex.handle.as_ptr(),
            mip_level,
            layer_or_depth_plane,
            clear_color,
            load_op: SDL_GPULoadOp::new(load_op as _),
            store_op: SDL_GPUStoreOp::new(store_op as _),
            resolve_texture,
            resolve_mip_level,
            resolve_layer,
            cycle,
            cycle_resolve_texture,
            ..Default::default()
        })
    }
}

#[doc(alias = "SDL_GPUDepthStencilTargetInfo")]
#[derive(Clone, Copy)]
pub struct DepthStencilTargetInfo(SDL_GPUDepthStencilTargetInfo);
impl DepthStencilTargetInfo {
    pub fn new(
        tex: Ref<GPUTexture>,
        clear_depth: f32,
        (load_op, store_op): (LoadOp, StoreOp),
        (stencil_load_op, stencil_store_op): (LoadOp, StoreOp),
        cycle: bool,
        clear_stencil: u8,
        (mip_level, layer): (u8, u8),
    ) -> Self {
        let texture = tex.handle.as_ptr();
        Self(SDL_GPUDepthStencilTargetInfo {
            texture,
            clear_depth,
            load_op: SDL_GPULoadOp::new(load_op as _),
            store_op: SDL_GPUStoreOp::new(store_op as _),
            stencil_load_op: SDL_GPULoadOp::new(stencil_load_op as _),
            stencil_store_op: SDL_GPUStoreOp::new(stencil_store_op as _),
            cycle,
            clear_stencil,
            mip_level,
            layer,
        })
    }
}

resource!(GPURenderPass, SDL, End);
impl GPURenderPass {
    #[doc(alias = "SDL_BeginGPURenderPass")]
    pub fn new(
        cmdbuf: Ref<GPUCommandBuffer>,
        color_targets: &[ColorTargetInfo],
        depth_stencil_target: Option<&DepthStencilTargetInfo>,
    ) -> Result<Self> {
        let handle = unsafe {
            SDL_BeginGPURenderPass(
                cmdbuf.handle.as_ptr(),
                color_targets.as_ptr().cast(),
                color_targets.len() as _,
                opt2ptr(depth_stencil_target).cast(),
            )
        };

        Self::from_ptr(handle)
    }
}

impl GPURenderPassHandle {
    #[doc(alias = "SDL_SetGPUViewport")]
    pub fn set_viewport(&self, viewport: &Viewport) {
        unsafe { SDL_SetGPUViewport(self.handle.as_ptr(), &viewport.0) }
    }

    #[doc(alias = "SDL_SetGPUBlendConstants")]
    pub fn set_blend_constants(&self, blend_constants: RgbaF32) {
        unsafe { SDL_SetGPUBlendConstants(self.handle.as_ptr(), blend_constants.into()) }
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
    pub fn bind_vertex_buffers(&self, first_slot: u32, bindings: &[BufferBinding]) {
        unsafe {
            SDL_BindGPUVertexBuffers(
                self.handle.as_ptr(),
                first_slot,
                bindings.as_ptr().cast(),
                bindings.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_BindGPUIndexBuffer")]
    pub fn bind_index_buffer(&self, binding: &BufferBinding, index_element_size: IndexElementSize) {
        unsafe {
            SDL_BindGPUIndexBuffer(
                self.handle.as_ptr(),
                &binding.0,
                SDL_GPUIndexElementSize::new(index_element_size as _),
            )
        }
    }

    #[doc(alias = "SDL_BindGPUVertexSamplers")]
    pub fn bind_vertex_samplers(&self, first_slot: u32, bindings: &[TextureSamplerBinding]) {
        unsafe {
            SDL_BindGPUVertexSamplers(
                self.handle.as_ptr(),
                first_slot,
                bindings.as_ptr().cast(),
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
    pub fn bind_fragment_samplers(&self, first_slot: u32, bindings: &[TextureSamplerBinding]) {
        unsafe {
            SDL_BindGPUFragmentSamplers(
                self.handle.as_ptr(),
                first_slot,
                bindings.as_ptr().cast(),
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
