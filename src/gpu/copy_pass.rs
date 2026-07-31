//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_BeginGPUCopyPass
//! - [x] SDL_CopyGPUBufferToBuffer
//! - [x] SDL_CopyGPUTextureToTexture
//! - [x] SDL_EndGPUCopyPass

use sdl3_sys::gpu::*;

use crate::{Result, resource, traits::Ref};

use super::command_buffer::GPUCommandBuffer;

resource!(GPUCopyPass, SDL, End);
impl GPUCopyPass {
    #[doc(alias = "SDL_BeginGPUCopyPass")]
    pub fn new(cmdbuf: Ref<GPUCommandBuffer>) -> Result<Self> {
        let handle = unsafe { SDL_BeginGPUCopyPass(cmdbuf.handle.as_ptr()) };
        Self::from_ptr(handle)
    }
}

impl GPUCopyPassHandle {
    #[doc(alias = "SDL_CopyGPUTextureToTexture")]
    pub fn copy_texture_to_texture(
        &self,
        source: &SDL_GPUTextureLocation,
        destination: &SDL_GPUTextureLocation,
        (w, h, d): (u32, u32, u32),
        cycle: bool,
    ) {
        unsafe {
            SDL_CopyGPUTextureToTexture(self.handle.as_ptr(), source, destination, w, h, d, cycle)
        }
    }

    #[doc(alias = "SDL_CopyGPUBufferToBuffer")]
    pub fn copy_buffer_to_buffer(
        &self,
        source: &SDL_GPUBufferLocation,
        destination: &SDL_GPUBufferLocation,
        size: u32,
        cycle: bool,
    ) {
        unsafe { SDL_CopyGPUBufferToBuffer(self.handle.as_ptr(), source, destination, size, cycle) }
    }
}
