//! Minimal SDL_gpu wrapper, plus some convenience functions.
//! TODO: Somehow put `#[must_use]` onto structs whose drop methods
//! are implemented separately (e.g. [`GPUBuffer`]).

use std::{ffi::CStr, mem::MaybeUninit, ptr::NonNull};

use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{
    defs::SdlResult,
    resource, resource_no_drop,
    traits::Ref,
    util::{opt2ptr_mut, to_result},
    window::Window,
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

    pub fn driver(&self) -> &str {
        let raw = unsafe { SDL_GetGPUDeviceDriver(self.handle.as_ptr()) };
        let cstr = unsafe { CStr::from_ptr(raw) };
        unsafe { std::str::from_utf8_unchecked(cstr.to_bytes()) }
    }
}

pub struct BufferCreateInfo {
    inner: SDL_GPUBufferCreateInfo,
}

impl BufferCreateInfo {
    pub const fn new(usage: SDL_GPUBufferUsageFlags, size: u32) -> Self {
        let inner = SDL_GPUBufferCreateInfo {
            usage,
            size,
            props: SDL_PropertiesID::new(0),
        };
        Self { inner }
    }
}

resource_no_drop!(GPUBuffer);
impl GPUBuffer {
    pub fn new(device: Ref<GPUDevice>, create_info: &BufferCreateInfo) -> SdlResult<Self> {
        let handle = unsafe { SDL_CreateGPUBuffer(device.handle.as_ptr(), &create_info.inner) };
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
    #[doc(alias = "SDL_AcquireGPUCommandBuffer")]
    pub fn new(device: Ref<GPUDevice>) -> SdlResult<Self> {
        let handle = unsafe { SDL_AcquireGPUCommandBuffer(device.handle.as_ptr()) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_SubmitGPUCommandBuffer")]
    pub fn submit(&self) -> SdlResult {
        to_result(unsafe { SDL_SubmitGPUCommandBuffer(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_SubmitGPUCommandBufferAndAcquireFence")]
    pub fn submit_fence(&self) -> SdlResult<GPUFence> {
        let fence = unsafe { SDL_SubmitGPUCommandBufferAndAcquireFence(self.handle.as_ptr()) };
        GPUFence::from_ptr(fence)
    }

    #[doc(alias = "SDL_WaitAndAcquireGPUSwapchainTexture")]
    pub fn wait_for_swapchain_texture(
        &self,
        wnd: Ref<Window>,
        (tex_x, tex_y): (Option<&mut u32>, Option<&mut u32>),
    ) -> SdlResult<Option<GPUTexture>> {
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

        fn m(ptr: *mut SDL_GPUTexture) -> Option<GPUTexture> {
            let handle = NonNull::new(ptr)?;
            let inner = GPUTextureHandle { handle };
            Some(GPUTexture { inner })
        }

        to_result(res).map(|()| m(unsafe { tex.assume_init() }))
    }
}

resource!(GPURenderPass, SDL, End);
impl GPURenderPass {
    pub fn new(cmdbuf: Ref<GPUCommandBuffer>, g: &[SDL_GPUColorTargetInfo]) -> SdlResult<Self> {
        let handle = unsafe {
            SDL_BeginGPURenderPass(
                cmdbuf.handle.as_ptr(),
                g.as_ptr(),
                g.len() as _,
                std::ptr::null(),
            )
        };
        Self::from_ptr(handle)
    }
}

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
