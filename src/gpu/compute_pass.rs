//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_BeginGPUComputePass
//! - [x] SDL_BindGPUComputePipeline
//! - [x] SDL_BindGPUComputeSamplers
//! - [x] SDL_BindGPUComputeStorageBuffers
//! - [x] SDL_BindGPUComputeStorageTextures
//! - [x] SDL_DispatchGPUCompute
//! - [x] SDL_DispatchGPUComputeIndirect
//! - [x] SDL_EndGPUComputePass

use sdl3_sys::gpu::*;

use crate::{Result, resource::Ref, resource_new};

use super::{
    buffer::{Buffer, StorageBufferReadWriteBinding},
    command_buffer::CommandBuffer,
    compute_pipeline::GPUComputePipeline,
    texture::{StorageTextureReadWriteBinding, Texture, TextureSamplerBinding},
};

resource_new!(SDL_GPUComputePass, ComputePass, SDL_EndGPUComputePass);
impl ComputePass {
    #[doc(alias = "SDL_BeginGPUComputePass")]
    pub fn new(
        cmdbuf: Ref<CommandBuffer>,
        storage_texture_bindings: &[StorageTextureReadWriteBinding],
        storage_buffer_bindings: &[StorageBufferReadWriteBinding],
    ) -> Result<Self> {
        let handle = unsafe {
            SDL_BeginGPUComputePass(
                cmdbuf.handle.as_ptr(),
                storage_texture_bindings.as_ptr().cast(),
                storage_texture_bindings.len() as _,
                storage_buffer_bindings.as_ptr().cast(),
                storage_buffer_bindings.len() as _,
            )
        };
        Self::from_ptr(handle)
    }
}

impl ComputePassHandle {
    #[doc(alias = "SDL_BindGPUComputePipeline")]
    pub fn bind(&self, pipeline: Ref<GPUComputePipeline>) {
        unsafe { SDL_BindGPUComputePipeline(self.handle.as_ptr(), pipeline.handle.as_ptr()) };
    }

    #[doc(alias = "SDL_BindGPUComputeSamplers")]
    pub fn bind_samplers(&self, first_slot: u32, bindings: &[TextureSamplerBinding]) {
        unsafe {
            SDL_BindGPUComputeSamplers(
                self.handle.as_ptr(),
                first_slot,
                bindings.as_ptr().cast(),
                bindings.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_BindGPUComputeStorageTextures")]
    pub fn bind_storage_textures(&self, first_slot: u32, textures: &[Ref<Texture>]) {
        unsafe {
            SDL_BindGPUComputeStorageTextures(
                self.handle.as_ptr(),
                first_slot,
                textures.as_ptr().cast(),
                textures.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_BindGPUComputeStorageBuffers")]
    pub fn bind_storage_buffers(&self, first_slot: u32, buffers: &[Ref<Buffer>]) {
        unsafe {
            SDL_BindGPUComputeStorageBuffers(
                self.handle.as_ptr(),
                first_slot,
                buffers.as_ptr().cast(),
                buffers.len() as _,
            )
        }
    }

    #[doc(alias = "SDL_DispatchGPUCompute")]
    pub fn dispatch(&self, (x, y, z): (u32, u32, u32)) {
        unsafe { SDL_DispatchGPUCompute(self.handle.as_ptr(), x, y, z) }
    }

    #[doc(alias = "SDL_DispatchGPUComputeIndirect")]
    pub fn dispatch_indirect(&self, buffer: Ref<Buffer>, offset: u32) {
        unsafe {
            SDL_DispatchGPUComputeIndirect(self.handle.as_ptr(), buffer.handle.as_ptr(), offset)
        }
    }
}
