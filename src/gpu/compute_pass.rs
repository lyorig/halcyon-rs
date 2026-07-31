//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_BeginGPUComputePass
//! - [x] SDL_BindGPUComputePipeline
//! - [x] SDL_DispatchGPUCompute
//! - [x] SDL_EndGPUComputePass

use sdl3_sys::gpu::*;

use crate::{Result, resource, traits::Ref};

use super::{command_buffer::GPUCommandBuffer, compute_pipeline::GPUComputePipeline};

resource!(GPUComputePass, SDL, End);
impl GPUComputePass {
    #[doc(alias = "SDL_BeginGPUComputePass")]
    pub fn new(
        cmdbuf: Ref<GPUCommandBuffer>,
        storage_texture_bindings: &[SDL_GPUStorageTextureReadWriteBinding],
        storage_buffer_bindings: &[SDL_GPUStorageBufferReadWriteBinding],
    ) -> Result<Self> {
        let handle = unsafe {
            SDL_BeginGPUComputePass(
                cmdbuf.handle.as_ptr(),
                storage_texture_bindings.as_ptr(),
                storage_texture_bindings.len() as _,
                storage_buffer_bindings.as_ptr(),
                storage_buffer_bindings.len() as _,
            )
        };
        Self::from_ptr(handle)
    }
}

impl GPUComputePassHandle {
    #[doc(alias = "SDL_BindGPUComputePipeline")]
    pub fn bind(&self, pipeline: Ref<GPUComputePipeline>) {
        unsafe { SDL_BindGPUComputePipeline(self.handle.as_ptr(), pipeline.handle.as_ptr()) };
    }

    #[doc(alias = "SDL_DispatchGPUCompute")]
    pub fn dispatch(&self, (x, y, z): (u32, u32, u32)) {
        unsafe { SDL_DispatchGPUCompute(self.handle.as_ptr(), x, y, z) }
    }
}
