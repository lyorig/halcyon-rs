//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_AcquireGPUCommandBuffer
//! - [x] SDL_SubmitGPUCommandBuffer
//! - [x] SDL_SubmitGPUCommandBufferAndAcquireFence
//! - [x] SDL_WaitAndAcquireGPUSwapchainTexture
//! - [x] SDL_CancelGPUCommandBuffer
//! - [x] SDL_BlitGPUTexture

use std::{mem::MaybeUninit, ptr::NonNull};

use sdl3_sys::gpu::*;

use crate::{
    Result, resource_no_drop,
    traits::Ref,
    util::{opt2ptr_mut, to_result},
    window::Window,
};

use super::{
    device::GPUDevice,
    fence::GPUFence,
    texture::{GPUTexture, GPUTextureHandle},
};

resource_no_drop!(GPUCommandBuffer);
impl GPUCommandBuffer {
    #[doc(alias = "SDL_AcquireGPUCommandBuffer")]
    pub fn new(device: Ref<GPUDevice>) -> Result<Self> {
        let handle = unsafe { SDL_AcquireGPUCommandBuffer(device.handle.as_ptr()) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_SubmitGPUCommandBuffer")]
    pub fn submit(self) -> Result {
        to_result(unsafe { SDL_SubmitGPUCommandBuffer(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_SubmitGPUCommandBufferAndAcquireFence")]
    pub fn submit_fence(self) -> Result<GPUFence> {
        let fence = unsafe { SDL_SubmitGPUCommandBufferAndAcquireFence(self.handle.as_ptr()) };
        GPUFence::from_ptr(fence)
    }

    #[doc(alias = "SDL_CancelGPUCommandBuffer")]
    pub fn cancel(self) -> Result {
        to_result(unsafe { SDL_CancelGPUCommandBuffer(self.handle.as_ptr()) })
    }
}

impl GPUCommandBufferHandle {
    #[doc(alias = "SDL_WaitAndAcquireGPUSwapchainTexture")]
    pub fn wait_for_swapchain_texture(
        &self,
        wnd: Ref<Window>,
        (tex_x, tex_y): (Option<&mut u32>, Option<&mut u32>),
    ) -> Result<Option<Ref<'_, GPUTexture>>> {
        let mut tex = MaybeUninit::uninit();
        let res = unsafe {
            SDL_WaitAndAcquireGPUSwapchainTexture(
                self.handle.as_ptr(),
                wnd.handle.as_ptr(),
                tex.as_mut_ptr(),
                opt2ptr_mut(tex_x),
                opt2ptr_mut(tex_y),
            )
        };

        fn m<'a>(ptr: *mut SDL_GPUTexture) -> Option<Ref<'a, GPUTexture>> {
            let handle = NonNull::new(ptr)?;
            let inner = GPUTextureHandle { handle };
            Some(unsafe { Ref::from_handle(inner) })
        }

        to_result(res).map(|()| m(unsafe { tex.assume_init() }))
    }

    #[doc(alias = "SDL_BlitGPUTexture")]
    pub fn blit(&self, info: &SDL_GPUBlitInfo) {
        unsafe { SDL_BlitGPUTexture(self.handle.as_ptr(), info) }
    }
}
