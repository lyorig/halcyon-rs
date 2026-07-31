//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_BeginGPURenderPass
//! - [x] SDL_EndGPURenderPass
//! - [x] SDL_SetGPUScissor

use sdl3_sys::gpu::*;

use crate::{Result, rect::RectI32, resource, traits::Ref, util::opt2ptr};

use super::command_buffer::GPUCommandBuffer;

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
    #[doc(alias = "SDL_SetGPUScissor")]
    pub fn set_scissor(&self, scissor: &RectI32) {
        unsafe { SDL_SetGPUScissor(self.handle.as_ptr(), scissor.as_sdl_ptr()) };
    }
}
