//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_BindGPUGraphicsPipeline
//! - [x] SDL_CreateGPUGraphicsPipeline
//! - [x] SDL_ReleaseGPUGraphicsPipeline

use sdl3_sys::gpu::*;

use crate::{Result, resource_no_drop, traits::Ref};

use super::{device::GPUDevice, render_pass::GPURenderPass};

resource_no_drop!(GPUGraphicsPipeline);
impl GPUGraphicsPipeline {
    #[doc(alias = "SDL_CreateGPUGraphicsPipeline")]
    pub fn new(
        device: Ref<GPUDevice>,
        create_info: &SDL_GPUGraphicsPipelineCreateInfo,
    ) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUGraphicsPipeline(device.handle.as_ptr(), create_info) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUGraphicsPipeline")]
    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUGraphicsPipeline(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl GPUGraphicsPipelineHandle {
    #[doc(alias = "SDL_BindGPUGraphicsPipeline")]
    pub fn bind(&self, render_pass: Ref<GPURenderPass>) {
        unsafe { SDL_BindGPUGraphicsPipeline(render_pass.handle.as_ptr(), self.handle.as_ptr()) };
    }
}
