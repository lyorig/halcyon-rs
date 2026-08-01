//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_BeginGPUCopyPass
//! - [x] SDL_CopyGPUBufferToBuffer
//! - [x] SDL_CopyGPUTextureToTexture
//! - [x] SDL_EndGPUCopyPass

use sdl3_sys::gpu::*;

use crate::{Result, gpu::Cycle, resource::Ref, resource_new};

use super::{buffer::BufferLocation, command_buffer::CommandBuffer, texture::TextureLocation};

resource_new!(SDL_GPUCopyPass, CopyPass, SDL_EndGPUCopyPass);
impl CopyPass {
    #[doc(alias = "SDL_BeginGPUCopyPass")]
    pub fn new(cmdbuf: Ref<CommandBuffer>) -> Result<Self> {
        let handle = unsafe { SDL_BeginGPUCopyPass(cmdbuf.handle.as_ptr()) };
        Self::from_ptr(handle)
    }
}

impl CopyPassHandle {
    #[doc(alias = "SDL_CopyGPUTextureToTexture")]
    pub fn copy_texture_to_texture(
        &self,
        source: &TextureLocation,
        destination: &TextureLocation,
        (w, h, d): (u32, u32, u32),
        cycle: Cycle,
    ) {
        unsafe {
            SDL_CopyGPUTextureToTexture(
                self.handle.as_ptr(),
                &source.0,
                &destination.0,
                w,
                h,
                d,
                cycle.into(),
            )
        }
    }

    #[doc(alias = "SDL_CopyGPUBufferToBuffer")]
    pub fn copy_buffer_to_buffer(
        &self,
        source: &BufferLocation,
        destination: &BufferLocation,
        size: u32,
        cycle: Cycle,
    ) {
        unsafe {
            SDL_CopyGPUBufferToBuffer(
                self.handle.as_ptr(),
                &source.0,
                &destination.0,
                size,
                cycle.into(),
            )
        }
    }
}
