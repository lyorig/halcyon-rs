//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_BeginGPUCopyPass
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
