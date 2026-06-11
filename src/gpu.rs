//! Minimal SDL_gpu wrapper, plus some convenience functions.
//! TODO: Somehow put `#[must_use]` onto structs whose drop methods
//! are implemented separately (e.g. [`GPUBuffer`]).

use sdl3_sys::gpu::*;

use crate::{
    defs::SdlResult, resource, resource_no_drop, traits::Ref, util::to_result, window::Window,
};

pub fn are_formats_supported(fmts: SDL_GPUShaderFormat) -> bool {
    unsafe { SDL_GPUSupportsShaderFormats(fmts, std::ptr::null()) }
}

resource!(GPUDevice);
impl GPUDevice {
    pub fn new(formats: SDL_GPUShaderFormat, debug_mode: bool) -> SdlResult<Self> {
        let handle = unsafe { SDL_CreateGPUDevice(formats, debug_mode, std::ptr::null()) };
        Self::from_ptr(handle)
    }

    pub fn claim_window(&self, window: Ref<Window>) -> SdlResult {
        to_result(unsafe {
            SDL_ClaimWindowForGPUDevice(self.handle.as_ptr(), window.handle.as_ptr())
        })
    }
}

resource_no_drop!(GPUBuffer);
impl GPUBuffer {
    pub fn new(device: Ref<GPUDevice>, create_info: &SDL_GPUBufferCreateInfo) -> SdlResult<Self> {
        let handle = unsafe { SDL_CreateGPUBuffer(device.handle.as_ptr(), create_info) };
        Self::from_ptr(handle)
    }

    pub fn upload(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &SDL_GPUTransferBufferLocation,
        dst: &SDL_GPUBufferRegion,
        cycle: bool,
    ) {
        unsafe { SDL_UploadToGPUBuffer(copy_pass.handle.as_ptr(), src, dst, cycle) }
    }

    pub fn download(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &SDL_GPUBufferRegion,
        dst: &SDL_GPUTransferBufferLocation,
    ) {
        unsafe { SDL_DownloadFromGPUBuffer(copy_pass.handle.as_ptr(), src, dst) };
    }

    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUBuffer(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

resource_no_drop!(GPUComputePipeline);
impl GPUComputePipeline {
    pub fn new(
        device: Ref<GPUDevice>,
        create_info: &SDL_GPUComputePipelineCreateInfo,
    ) -> SdlResult<Self> {
        let handle = unsafe { SDL_CreateGPUComputePipeline(device.handle.as_ptr(), create_info) };
        Self::from_ptr(handle)
    }

    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUComputePipeline(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

resource_no_drop!(GPUGraphicsPipeline);
impl GPUGraphicsPipeline {
    pub fn new(
        device: Ref<GPUDevice>,
        create_info: &SDL_GPUGraphicsPipelineCreateInfo,
    ) -> SdlResult<Self> {
        let handle = unsafe { SDL_CreateGPUGraphicsPipeline(device.handle.as_ptr(), create_info) };
        Self::from_ptr(handle)
    }

    pub fn bind(&self, render_pass: Ref<GPURenderPass>) {
        unsafe { SDL_BindGPUGraphicsPipeline(render_pass.handle.as_ptr(), self.handle.as_ptr()) };
    }

    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUGraphicsPipeline(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

resource_no_drop!(GPUFence);
impl GPUFence {
    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUFence(device.handle.as_ptr(), self.handle.as_ptr()) }
    }
}

resource_no_drop!(GPUCommandBuffer);
impl GPUCommandBuffer {
    pub fn new(device: Ref<GPUDevice>) -> SdlResult<Self> {
        let handle = unsafe { SDL_AcquireGPUCommandBuffer(device.handle.as_ptr()) };
        Self::from_ptr(handle)
    }

    pub fn submit(&self) -> SdlResult {
        to_result(unsafe { SDL_SubmitGPUCommandBuffer(self.handle.as_ptr()) })
    }

    pub fn submit_fence(&self) -> SdlResult<GPUFence> {
        let fence = unsafe { SDL_SubmitGPUCommandBufferAndAcquireFence(self.handle.as_ptr()) };
        GPUFence::from_ptr(fence)
    }
}

resource!(GPURenderPass, SDL, End);

resource!(GPUComputePass, SDL, End);
impl GPUComputePass {
    pub fn bind(&self, pipeline: Ref<GPUComputePipeline>) {
        unsafe { SDL_BindGPUComputePipeline(self.handle.as_ptr(), pipeline.handle.as_ptr()) };
    }

    pub fn dispatch(&self, (x, y, z): (u32, u32, u32)) {
        unsafe { SDL_DispatchGPUCompute(self.handle.as_ptr(), x, y, z) }
    }
}

resource!(GPUCopyPass, SDL, End);
impl GPUCopyPass {
    pub fn new(cmdbuf: Ref<GPUCommandBuffer>) -> SdlResult<Self> {
        let handle = unsafe { SDL_BeginGPUCopyPass(cmdbuf.handle.as_ptr()) };
        Self::from_ptr(handle)
    }
}

resource_no_drop!(GPUShader);
impl GPUShader {
    pub fn new(device: Ref<GPUDevice>, create_info: &SDL_GPUShaderCreateInfo) -> SdlResult<Self> {
        let handle = unsafe { SDL_CreateGPUShader(device.handle.as_ptr(), create_info) };
        Self::from_ptr(handle)
    }

    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe {
            SDL_ReleaseGPUShader(device.handle.as_ptr(), self.handle.as_ptr());
        }
    }
}

resource_no_drop!(GPUTexture);
impl GPUTexture {
    pub fn new(device: Ref<GPUDevice>, create_info: &SDL_GPUTextureCreateInfo) -> SdlResult<Self> {
        let handle = unsafe { SDL_CreateGPUTexture(device.handle.as_ptr(), create_info) };
        Self::from_ptr(handle)
    }

    pub fn upload(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &SDL_GPUTextureTransferInfo,
        dst: &SDL_GPUTextureRegion,
        cycle: bool,
    ) {
        unsafe {
            SDL_UploadToGPUTexture(copy_pass.handle.as_ptr(), src, dst, cycle);
        }
    }

    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUTexture(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

resource_no_drop!(GPUTransferBuffer, SDL);
impl GPUTransferBuffer {
    pub fn drop(self, dev: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUTransferBuffer(dev.handle.as_ptr(), self.handle.as_ptr()) };
    }
}
