//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_QueryGPUFence
//! - [x] SDL_ReleaseGPUFence

use sdl3_sys::gpu::*;

use crate::{resource::Ref, resource_new_no_drop};

use super::device::GPUDevice;

resource_new_no_drop!(GPUFence);
impl GPUFence {
    #[doc(alias = "SDL_ReleaseGPUFence")]
    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUFence(device.handle.as_ptr(), self.handle.as_ptr()) }
    }
}

impl GPUFenceHandle {
    #[doc(alias = "SDL_QueryGPUFence")]
    pub fn is_signaled(&self, device: Ref<GPUDevice>) -> bool {
        unsafe { SDL_QueryGPUFence(device.handle.as_ptr(), self.handle.as_ptr()) }
    }
}
