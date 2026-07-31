//! Minimal SDL_gpu wrapper, plus some convenience functions.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_AcquireGPUCommandBuffer
//! - [ ] SDL_AcquireGPUSwapchainTexture
//! - [x] SDL_BeginGPUComputePass
//! - [x] SDL_BeginGPUCopyPass
//! - [x] SDL_BeginGPURenderPass
//! - [x] SDL_BindGPUComputePipeline
//! - [ ] SDL_BindGPUComputeSamplers
//! - [ ] SDL_BindGPUComputeStorageBuffers
//! - [ ] SDL_BindGPUComputeStorageTextures
//! - [ ] SDL_BindGPUFragmentSamplers
//! - [ ] SDL_BindGPUFragmentStorageBuffers
//! - [ ] SDL_BindGPUFragmentStorageTextures
//! - [x] SDL_BindGPUGraphicsPipeline
//! - [ ] SDL_BindGPUIndexBuffer
//! - [ ] SDL_BindGPUVertexBuffers
//! - [ ] SDL_BindGPUVertexSamplers
//! - [ ] SDL_BindGPUVertexStorageBuffers
//! - [ ] SDL_BindGPUVertexStorageTextures
//! - [ ] SDL_BlitGPUTexture
//! - [ ] SDL_CalculateGPUTextureFormatSize
//! - [ ] SDL_CancelGPUCommandBuffer
//! - [x] SDL_ClaimWindowForGPUDevice
//! - [ ] SDL_CopyGPUBufferToBuffer
//! - [ ] SDL_CopyGPUTextureToTexture
//! - [x] SDL_CreateGPUBuffer
//! - [x] SDL_CreateGPUComputePipeline
//! - [x] SDL_CreateGPUDevice
//! - [ ] SDL_CreateGPUDeviceWithProperties
//! - [x] SDL_CreateGPUGraphicsPipeline
//! - [ ] SDL_CreateGPUSampler
//! - [x] SDL_CreateGPUShader
//! - [x] SDL_CreateGPUTexture
//! - [x] SDL_CreateGPUTransferBuffer
//! - [x] SDL_DestroyGPUDevice
//! - [x] SDL_DispatchGPUCompute
//! - [ ] SDL_DispatchGPUComputeIndirect
//! - [x] SDL_DownloadFromGPUBuffer
//! - [x] SDL_DownloadFromGPUTexture
//! - [ ] SDL_DrawGPUIndexedPrimitives
//! - [ ] SDL_DrawGPUIndexedPrimitivesIndirect
//! - [ ] SDL_DrawGPUPrimitives
//! - [ ] SDL_DrawGPUPrimitivesIndirect
//! - [x] SDL_EndGPUComputePass
//! - [x] SDL_EndGPUCopyPass
//! - [x] SDL_EndGPURenderPass
//! - [ ] SDL_GDKResumeGPU
//! - [ ] SDL_GDKSuspendGPU
//! - [ ] SDL_GenerateMipmapsForGPUTexture
//! - [x] SDL_GetGPUDeviceDriver
//! - [ ] SDL_GetGPUDeviceProperties
//! - [ ] SDL_GetGPUDriver
//! - [ ] SDL_GetGPUShaderFormats
//! - [ ] SDL_GetGPUSwapchainTextureFormat
//! - [ ] SDL_GetGPUTextureFormatFromPixelFormat
//! - [ ] SDL_GetNumGPUDrivers
//! - [ ] SDL_GetPixelFormatFromGPUTextureFormat
//! - [ ] SDL_GPUSupportsProperties
//! - [x] SDL_GPUSupportsShaderFormats
//! - [ ] SDL_GPUTextureFormatTexelBlockSize
//! - [ ] SDL_GPUTextureSupportsFormat
//! - [ ] SDL_GPUTextureSupportsSampleCount
//! - [ ] SDL_InsertGPUDebugLabel
//! - [x] SDL_MapGPUTransferBuffer
//! - [ ] SDL_PopGPUDebugGroup
//! - [ ] SDL_PushGPUComputeUniformData
//! - [ ] SDL_PushGPUDebugGroup
//! - [ ] SDL_PushGPUFragmentUniformData
//! - [ ] SDL_PushGPUVertexUniformData
//! - [x] SDL_QueryGPUFence
//! - [x] SDL_ReleaseGPUBuffer
//! - [x] SDL_ReleaseGPUComputePipeline
//! - [x] SDL_ReleaseGPUFence
//! - [x] SDL_ReleaseGPUGraphicsPipeline
//! - [ ] SDL_ReleaseGPUSampler
//! - [x] SDL_ReleaseGPUShader
//! - [x] SDL_ReleaseGPUTexture
//! - [x] SDL_ReleaseGPUTransferBuffer
//! - [x] SDL_ReleaseWindowFromGPUDevice
//! - [x] SDL_SetGPUAllowedFramesInFlight
//! - [ ] SDL_SetGPUBlendConstants
//! - [x] SDL_SetGPUBufferName
//! - [x] SDL_SetGPUScissor
//! - [ ] SDL_SetGPUStencilReference
//! - [ ] SDL_SetGPUSwapchainParameters
//! - [x] SDL_SetGPUTextureName
//! - [ ] SDL_SetGPUViewport
//! - [x] SDL_SubmitGPUCommandBuffer
//! - [x] SDL_SubmitGPUCommandBufferAndAcquireFence
//! - [x] SDL_UnmapGPUTransferBuffer
//! - [x] SDL_UploadToGPUBuffer
//! - [x] SDL_UploadToGPUTexture
//! - [x] SDL_WaitAndAcquireGPUSwapchainTexture
//! - [x] SDL_WaitForGPUFences
//! - [x] SDL_WaitForGPUIdle
//! - [x] SDL_WaitForGPUSwapchain
//! - [x] SDL_WindowSupportsGPUPresentMode
//! - [x] SDL_WindowSupportsGPUSwapchainComposition

use std::{ffi::CStr, mem::MaybeUninit, ptr::NonNull};

use bitmask_enum::bitmask;
use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{
    Result, boolenum,
    error::Error,
    rect::{Point, RectI32},
    resource, resource_no_drop,
    traits::Ref,
    util::{opt2ptr, opt2ptr_mut, to_result},
    window::Window,
};

/// Non-bitmask variant of [`SDL_GPUShaderFormat`].
#[repr(u32)]
#[doc(alias = "SDL_GPUShaderFormat")]
pub enum ShaderFormat {
    SpirV = SDL_GPUShaderFormat::SPIRV.0,
    Dxbc = SDL_GPUShaderFormat::DXBC.0,
    Dxil = SDL_GPUShaderFormat::DXIL.0,
    Msl = SDL_GPUShaderFormat::MSL.0,
    Metallib = SDL_GPUShaderFormat::METALLIB.0,
}

#[bitmask(u32)]
#[doc(alias = "SDL_GPUShaderFormat")]
pub enum ShaderFormats {
    SpirV = SDL_GPUShaderFormat::SPIRV.0,
    Dxbc = SDL_GPUShaderFormat::DXBC.0,
    Dxil = SDL_GPUShaderFormat::DXIL.0,
    Msl = SDL_GPUShaderFormat::MSL.0,
    Metallib = SDL_GPUShaderFormat::METALLIB.0,
}

#[doc(alias = "SDL_GPUSupportsShaderFormats")]
pub fn are_formats_supported(fmts: ShaderFormats) -> bool {
    let fmts = SDL_GPUShaderFormat::new(fmts.bits());
    unsafe { SDL_GPUSupportsShaderFormats(fmts, std::ptr::null()) }
}

boolenum!(DeviceDebug);
boolenum!(WaitAll);

resource!(GPUDevice);
impl GPUDevice {
    #[doc(alias = "SDL_CreateGPUDevice")]
    pub fn new(formats: ShaderFormats, debug: DeviceDebug) -> Result<Self> {
        let fmts = SDL_GPUShaderFormat::new(formats.bits());
        let handle = unsafe { SDL_CreateGPUDevice(fmts, debug.into(), std::ptr::null()) };
        Self::from_ptr(handle)
    }
}

impl GPUDeviceHandle {
    #[doc(alias = "SDL_ClaimWindowForGPUDevice")]
    pub fn claim_window(&self, window: Ref<Window>) -> Result {
        to_result(unsafe {
            SDL_ClaimWindowForGPUDevice(self.handle.as_ptr(), window.handle.as_ptr())
        })
    }

    #[doc(alias = "SDL_ReleaseWindowFromGPUDevice")]
    pub fn release_window(&self, window: Ref<Window>) {
        unsafe { SDL_ReleaseWindowFromGPUDevice(self.handle.as_ptr(), window.handle.as_ptr()) };
    }

    #[doc(alias = "SDL_WindowSupportsGPUPresentMode")]
    pub fn window_supports_gpu_present_mode(
        &self,
        window: Ref<Window>,
        pm: SDL_GPUPresentMode,
    ) -> bool {
        unsafe {
            SDL_WindowSupportsGPUPresentMode(self.handle.as_ptr(), window.handle.as_ptr(), pm)
        }
    }

    #[doc(alias = "SDL_WindowSupportsGPUSwapchainComposition")]
    pub fn window_supports_gpu_swapchain_composition(
        &self,
        window: Ref<Window>,
        sc: SDL_GPUSwapchainComposition,
    ) -> bool {
        unsafe {
            SDL_WindowSupportsGPUSwapchainComposition(
                self.handle.as_ptr(),
                window.handle.as_ptr(),
                sc,
            )
        }
    }

    #[doc(alias = "SDL_WaitForGPUIdle")]
    pub fn wait_idle(&self) -> Result {
        to_result(unsafe { SDL_WaitForGPUIdle(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_WaitForGPUSwapchain")]
    pub fn wait_swapchain(&self, window: Ref<Window>) -> Result {
        to_result(unsafe { SDL_WaitForGPUSwapchain(self.handle.as_ptr(), window.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_WaitForGPUFences")]
    pub fn wait_fences(&self, wait_all: WaitAll, fences: &[Ref<GPUFence>]) -> Result {
        to_result(unsafe {
            SDL_WaitForGPUFences(
                self.handle.as_ptr(),
                wait_all.into(),
                fences.as_ptr().cast(),
                fences.len() as _,
            )
        })
    }

    #[doc(alias = "SDL_GetGPUDeviceDriver")]
    pub fn driver(&self) -> Result<&str> {
        let raw = unsafe { SDL_GetGPUDeviceDriver(self.handle.as_ptr()) };
        if raw.is_null() {
            Err(Error::current())
        } else {
            let cstr = unsafe { CStr::from_ptr(raw) };
            Ok(unsafe { str::from_utf8_unchecked(cstr.to_bytes()) })
        }
    }

    #[doc(alias = "SDL_SetGPUAllowedFramesInFlight")]
    pub fn set_allowed_frames_in_flight(&self, n: u32) -> Result {
        to_result(unsafe { SDL_SetGPUAllowedFramesInFlight(self.handle.as_ptr(), n) })
    }
}

#[bitmask(u32)]
#[doc(alias = "SDL_GPUBufferUsageFlags")]
pub enum BufferUsageFlags {
    Vertex = SDL_GPUBufferUsageFlags::VERTEX.0,
    Index = SDL_GPUBufferUsageFlags::INDEX.0,
    Indirect = SDL_GPUBufferUsageFlags::INDIRECT.0,
    GraphicsStorageRead = SDL_GPUBufferUsageFlags::GRAPHICS_STORAGE_READ.0,
    ComputeStorageRead = SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_READ.0,
    ComputeStorageWrite = SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_WRITE.0,
}

#[doc(alias = "SDL_GPUBufferCreateInfo")]
pub struct BufferCreateInfo(SDL_GPUBufferCreateInfo);
impl BufferCreateInfo {
    pub const fn new(usage: BufferUsageFlags, size: u32) -> Self {
        let usage = SDL_GPUBufferUsageFlags::new(usage.bits());
        let inner = SDL_GPUBufferCreateInfo {
            usage,
            size,
            props: SDL_PropertiesID::new(0),
        };
        Self(inner)
    }
}

#[doc(alias = "SDL_GPUBufferRegion")]
pub struct BufferRegion(SDL_GPUBufferRegion);
impl BufferRegion {
    pub fn new(buffer: Ref<GPUBuffer>, offset: u32, size: u32) -> Self {
        let buffer = buffer.handle.as_ptr();
        let inner = SDL_GPUBufferRegion {
            buffer,
            offset,
            size,
        };
        Self(inner)
    }
}

#[doc(alias = "SDL_GPUTransferBufferLocation")]
pub struct TransferBufferLocation(SDL_GPUTransferBufferLocation);
impl TransferBufferLocation {
    pub fn new(tb: Ref<GPUTransferBuffer>, offset: u32) -> Self {
        let transfer_buffer = tb.handle.as_ptr();
        let inner = SDL_GPUTransferBufferLocation {
            transfer_buffer,
            offset,
        };
        Self(inner)
    }
}

resource_no_drop!(GPUBuffer);
impl GPUBuffer {
    #[doc(alias = "SDL_CreateGPUBuffer")]
    pub fn new(device: Ref<GPUDevice>, create_info: &BufferCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUBuffer(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUBuffer")]
    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUBuffer(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl GPUBufferHandle {
    #[doc(alias = "SDL_UploadToGPUBuffer")]
    pub fn upload(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &TransferBufferLocation,
        dst: &BufferRegion,
        cycle: bool,
    ) {
        unsafe { SDL_UploadToGPUBuffer(copy_pass.handle.as_ptr(), &src.0, &dst.0, cycle) }
    }

    #[doc(alias = "SDL_DownloadFromGPUBuffer")]
    pub fn download(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &BufferRegion,
        dst: &TransferBufferLocation,
    ) {
        unsafe { SDL_DownloadFromGPUBuffer(copy_pass.handle.as_ptr(), &src.0, &dst.0) };
    }

    #[doc(alias = "SDL_SetGPUBufferName")]
    pub fn set_name(&self, device: Ref<GPUDevice>, name: &CStr) {
        unsafe {
            SDL_SetGPUBufferName(device.handle.as_ptr(), self.handle.as_ptr(), name.as_ptr())
        };
    }
}

#[doc(alias = "SDL_GPUComputePipelineCreateInfo")]
pub struct ComputePipelineCreateInfo(SDL_GPUComputePipelineCreateInfo);
impl ComputePipelineCreateInfo {
    pub const fn new(
        code: &[u8],
        entrypoint: &CStr,
        fmt: ShaderFormat,
        (samplers, ro_stor_tex, ro_stor_buf, rw_stor_tex, rw_stor_buf, unif_buf): (
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        ),
        (thr_x, thr_y, thr_z): (u32, u32, u32),
    ) -> Self {
        let inner = SDL_GPUComputePipelineCreateInfo {
            code_size: code.len(),
            code: code.as_ptr(),
            entrypoint: entrypoint.as_ptr(),
            format: SDL_GPUShaderFormat::new(fmt as _),
            num_samplers: samplers,
            num_readonly_storage_textures: ro_stor_tex,
            num_readonly_storage_buffers: ro_stor_buf,
            num_readwrite_storage_textures: rw_stor_tex,
            num_readwrite_storage_buffers: rw_stor_buf,
            num_uniform_buffers: unif_buf,
            threadcount_x: thr_x,
            threadcount_y: thr_y,
            threadcount_z: thr_z,
            props: SDL_PropertiesID::new(0),
        };

        Self(inner)
    }
}

resource_no_drop!(GPUComputePipeline);
impl GPUComputePipeline {
    #[doc(alias = "SDL_CreateGPUComputePipeline")]
    pub fn new(device: Ref<GPUDevice>, create_info: &ComputePipelineCreateInfo) -> Result<Self> {
        let handle =
            unsafe { SDL_CreateGPUComputePipeline(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUComputePipeline")]
    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUComputePipeline(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

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

resource_no_drop!(GPUFence);
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
}

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

resource!(GPUCopyPass, SDL, End);
impl GPUCopyPass {
    #[doc(alias = "SDL_BeginGPUCopyPass")]
    pub fn new(cmdbuf: Ref<GPUCommandBuffer>) -> Result<Self> {
        let handle = unsafe { SDL_BeginGPUCopyPass(cmdbuf.handle.as_ptr()) };
        Self::from_ptr(handle)
    }
}

#[repr(i32)]
#[doc(alias = "SDL_GPUShaderStage")]
pub enum ShaderStage {
    Vertex = SDL_GPUShaderStage::VERTEX.0,
    Fragment = SDL_GPUShaderStage::FRAGMENT.0,
}

#[doc(alias = "SDL_GPUShaderCreateInfo")]
pub struct ShaderCreateInfo(SDL_GPUShaderCreateInfo);
impl ShaderCreateInfo {
    pub const fn new(
        code: &[u8],
        entrypoint: &CStr,
        fmt: ShaderFormat,
        stage: ShaderStage,
        num_samplers: u32,
        (num_storage_textures, num_storage_buffers, num_uniform_buffers): (u32, u32, u32),
    ) -> Self {
        let inner = SDL_GPUShaderCreateInfo {
            code_size: code.len(),
            code: code.as_ptr(),
            entrypoint: entrypoint.as_ptr(),
            format: SDL_GPUShaderFormat::new(fmt as _),
            stage: SDL_GPUShaderStage::new(stage as _),
            num_samplers,
            num_storage_textures,
            num_storage_buffers,
            num_uniform_buffers,
            props: SDL_PropertiesID::new(0),
        };
        Self(inner)
    }
}

resource_no_drop!(GPUShader);
impl GPUShader {
    #[doc(alias = "SDL_CreateGPUShader")]
    pub fn new(device: Ref<GPUDevice>, create_info: &ShaderCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUShader(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUShader")]
    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe {
            SDL_ReleaseGPUShader(device.handle.as_ptr(), self.handle.as_ptr());
        }
    }
}

#[repr(i32)]
#[doc(alias = "SDL_GPUTextureType")]
pub enum TextureType {
    _2d = SDL_GPUTextureType::_2D.0,
    _2dArray = SDL_GPUTextureType::_2D_ARRAY.0,
    _3d = SDL_GPUTextureType::_3D.0,
    Cube = SDL_GPUTextureType::CUBE.0,
    CubeArray = SDL_GPUTextureType::CUBE_ARRAY.0,
}

#[bitmask(u32)]
#[doc(alias = "SDL_GPUTextureUsageFlags")]
pub enum TextureUsageFlags {
    Sampler = SDL_GPUTextureUsageFlags::SAMPLER.0,
    ColorTarget = SDL_GPUTextureUsageFlags::COLOR_TARGET.0,
    DepthStencilTarget = SDL_GPUTextureUsageFlags::DEPTH_STENCIL_TARGET.0,
    GraphicsStorageRead = SDL_GPUTextureUsageFlags::GRAPHICS_STORAGE_READ.0,
    ComputeStorageRead = SDL_GPUTextureUsageFlags::COMPUTE_STORAGE_READ.0,
    ComputeStorageWrite = SDL_GPUTextureUsageFlags::COMPUTE_STORAGE_WRITE.0,
    ComputeStorageReadWrite = SDL_GPUTextureUsageFlags::COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUSampleCount")]
pub enum SampleCount {
    One = SDL_GPUSampleCount::_1.0,
    Two = SDL_GPUSampleCount::_2.0,
    Four = SDL_GPUSampleCount::_4.0,
    Eight = SDL_GPUSampleCount::_8.0,
}

#[doc(alias = "SDL_GPUTextureCreateInfo")]
pub struct TextureCreateInfo(SDL_GPUTextureCreateInfo);
impl TextureCreateInfo {
    pub const fn new(
        kind: TextureType,
        format: SDL_GPUTextureFormat,
        usage: TextureUsageFlags,
        size: Point<u32>,
        layer_count_or_depth: u32,
        num_levels: u32,
        samples: SampleCount,
    ) -> Self {
        let r#type = SDL_GPUTextureType::new(kind as _);
        let usage = SDL_GPUTextureUsageFlags::new(usage.bits());
        let props = SDL_PropertiesID::new(0);
        let sample_count = SDL_GPUSampleCount::new(samples as _);

        let inner = SDL_GPUTextureCreateInfo {
            r#type,
            format,
            usage,
            width: size.x,
            height: size.y,
            layer_count_or_depth,
            num_levels,
            sample_count,
            props,
        };
        Self(inner)
    }
}

#[doc(alias = "SDL_GPUTextureTransferInfo")]
pub struct TextureTransferInfo(SDL_GPUTextureTransferInfo);
impl TextureTransferInfo {
    pub fn new(
        tb: Ref<GPUTransferBuffer>,
        offset: u32,
        pixels_per_row: u32,
        rows_per_layer: u32,
    ) -> Self {
        let transfer_buffer = tb.handle.as_ptr();
        let inner = SDL_GPUTextureTransferInfo {
            transfer_buffer,
            offset,
            pixels_per_row,
            rows_per_layer,
        };
        Self(inner)
    }
}

#[doc(alias = "SDL_GPUTextureRegion")]
pub struct TextureRegion(SDL_GPUTextureRegion);
impl TextureRegion {
    pub fn new(
        tex: Ref<GPUTexture>,
        mip_level: u32,
        layer: u32,
        (x, y, z): (u32, u32, u32),
        (w, h, d): (u32, u32, u32),
    ) -> Self {
        let texture = tex.handle.as_ptr();
        let inner = SDL_GPUTextureRegion {
            texture,
            mip_level,
            layer,
            x,
            y,
            z,
            w,
            h,
            d,
        };
        Self(inner)
    }
}

resource_no_drop!(GPUTexture);
impl GPUTexture {
    #[doc(alias = "SDL_CreateGPUTexture")]
    pub fn new(device: Ref<GPUDevice>, create_info: &TextureCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUTexture(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUTexture")]
    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUTexture(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl GPUTextureHandle {
    #[doc(alias = "SDL_DownloadFromGPUTexture")]
    pub fn download(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &TextureRegion,
        dst: &TextureTransferInfo,
    ) {
        unsafe { SDL_DownloadFromGPUTexture(copy_pass.handle.as_ptr(), &src.0, &dst.0) };
    }

    #[doc(alias = "SDL_UploadToGPUTexture")]
    pub fn upload(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &TextureTransferInfo,
        dst: &TextureRegion,
        cycle: bool,
    ) {
        unsafe {
            SDL_UploadToGPUTexture(copy_pass.handle.as_ptr(), &src.0, &dst.0, cycle);
        }
    }

    #[doc(alias = "SDL_SetGPUTextureName")]
    pub fn set_name(&self, device: Ref<GPUDevice>, name: &CStr) {
        unsafe {
            SDL_SetGPUTextureName(device.handle.as_ptr(), self.handle.as_ptr(), name.as_ptr())
        }
    }
}

#[repr(i32)]
#[doc(alias = "SDL_GPUTransferBufferUsage")]
pub enum TransferBufferUsage {
    Upload = SDL_GPUTransferBufferUsage::UPLOAD.0,
    Download = SDL_GPUTransferBufferUsage::DOWNLOAD.0,
}

pub struct TransferBufferCreateInfo(SDL_GPUTransferBufferCreateInfo);
impl TransferBufferCreateInfo {
    pub const fn new(usage: TransferBufferUsage, size: u32) -> Self {
        Self(SDL_GPUTransferBufferCreateInfo {
            usage: SDL_GPUTransferBufferUsage::new(usage as _),
            size,
            props: SDL_PropertiesID::new(0),
        })
    }
}

resource_no_drop!(GPUTransferBuffer, SDL);
impl GPUTransferBuffer {
    #[doc(alias = "SDL_CreateGPUTransferBuffer")]
    pub fn new(device: Ref<GPUDevice>, create_info: &TransferBufferCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUTransferBuffer(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUTransferBuffer")]
    pub fn drop(self, dev: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUTransferBuffer(dev.handle.as_ptr(), self.handle.as_ptr()) };
    }

    #[doc(alias = "SDL_MapGPUTransferBuffer")]
    pub fn map(&self, device: Ref<GPUDevice>, cycle: bool) -> Result<NonNull<u8>> {
        let ptr = unsafe {
            SDL_MapGPUTransferBuffer(device.handle.as_ptr(), self.handle.as_ptr(), cycle)
        };
        NonNull::new(ptr.cast()).ok_or_else(Error::current)
    }

    #[doc(alias = "SDL_UnmapGPUTransferBuffer")]
    pub fn unmap(&self, device: Ref<GPUDevice>) {
        unsafe { SDL_UnmapGPUTransferBuffer(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}
