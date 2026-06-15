//! Minimal SDL_gpu wrapper, plus some convenience functions.
//! TODO: Somehow put `#[must_use]` onto structs whose drop methods
//! are implemented separately (e.g. [`GPUBuffer`]).

use std::{ffi::CStr, mem::MaybeUninit, ptr::NonNull};

use bitmask_enum::bitmask;
use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{
    defs::SdlResult,
    error::Error,
    rect::Point,
    resource, resource_no_drop,
    traits::Ref,
    util::{opt2ptr, opt2ptr_mut, to_result},
    window::Window,
};

/// Non-bitmask variant of [`SDL_GPUShaderFormat`].
#[repr(u32)]
pub enum ShaderFormat {
    SpirV = SDL_GPUShaderFormat::SPIRV.0,
    Dxbc = SDL_GPUShaderFormat::DXBC.0,
    Dxil = SDL_GPUShaderFormat::DXIL.0,
    Msl = SDL_GPUShaderFormat::MSL.0,
    Metallib = SDL_GPUShaderFormat::METALLIB.0,
}

#[bitmask(u32)]
pub enum ShaderFormats {
    SpirV = SDL_GPUShaderFormat::SPIRV.0,
    Dxbc = SDL_GPUShaderFormat::DXBC.0,
    Dxil = SDL_GPUShaderFormat::DXIL.0,
    Msl = SDL_GPUShaderFormat::MSL.0,
    Metallib = SDL_GPUShaderFormat::METALLIB.0,
}

pub fn are_formats_supported(fmts: ShaderFormats) -> bool {
    let fmts = SDL_GPUShaderFormat::new(fmts.bits());
    unsafe { SDL_GPUSupportsShaderFormats(fmts, std::ptr::null()) }
}

resource!(GPUDevice);
impl GPUDevice {
    pub fn new(formats: ShaderFormats, debug_mode: bool) -> SdlResult<Self> {
        let fmts = SDL_GPUShaderFormat::new(formats.bits());
        let handle = unsafe { SDL_CreateGPUDevice(fmts, debug_mode, std::ptr::null()) };
        Self::from_ptr(handle)
    }
}

impl GPUDeviceHandle {
    pub fn claim_window(&self, window: Ref<Window>) -> SdlResult {
        to_result(unsafe {
            SDL_ClaimWindowForGPUDevice(self.handle.as_ptr(), window.handle.as_ptr())
        })
    }

    pub fn driver(&self) -> SdlResult<&str> {
        let raw = unsafe { SDL_GetGPUDeviceDriver(self.handle.as_ptr()) };
        if raw.is_null() {
            Err(Error::current())
        } else {
            let cstr = unsafe { CStr::from_ptr(raw) };
            Ok(unsafe { std::str::from_utf8_unchecked(cstr.to_bytes()) })
        }
    }
}

#[bitmask(u32)]
pub enum BufferUsageFlags {
    Vertex = SDL_GPUBufferUsageFlags::VERTEX.0,
    Index = SDL_GPUBufferUsageFlags::INDEX.0,
    Indirect = SDL_GPUBufferUsageFlags::INDIRECT.0,
    GraphicsStorageRead = SDL_GPUBufferUsageFlags::GRAPHICS_STORAGE_READ.0,
    ComputeStorageRead = SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_READ.0,
    ComputeStorageWrite = SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_WRITE.0,
}

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
    pub fn new(device: Ref<GPUDevice>, create_info: &BufferCreateInfo) -> SdlResult<Self> {
        let handle = unsafe { SDL_CreateGPUBuffer(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUBuffer(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl GPUBufferHandle {
    pub fn upload(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &TransferBufferLocation,
        dst: &BufferRegion,
        cycle: bool,
    ) {
        unsafe { SDL_UploadToGPUBuffer(copy_pass.handle.as_ptr(), &src.0, &dst.0, cycle) }
    }

    pub fn download(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &BufferRegion,
        dst: &TransferBufferLocation,
    ) {
        unsafe { SDL_DownloadFromGPUBuffer(copy_pass.handle.as_ptr(), &src.0, &dst.0) };
    }
}

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
    pub fn new(device: Ref<GPUDevice>, create_info: &ComputePipelineCreateInfo) -> SdlResult<Self> {
        let handle =
            unsafe { SDL_CreateGPUComputePipeline(device.handle.as_ptr(), &create_info.0) };
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

    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUGraphicsPipeline(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl GPUGraphicsPipelineHandle {
    pub fn bind(&self, render_pass: Ref<GPURenderPass>) {
        unsafe { SDL_BindGPUGraphicsPipeline(render_pass.handle.as_ptr(), self.handle.as_ptr()) };
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
    pub fn submit(self) -> SdlResult {
        to_result(unsafe { SDL_SubmitGPUCommandBuffer(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_SubmitGPUCommandBufferAndAcquireFence")]
    pub fn submit_fence(self) -> SdlResult<GPUFence> {
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
    ) -> SdlResult<Option<Ref<'_, GPUTexture>>> {
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
    pub fn new(
        cmdbuf: Ref<GPUCommandBuffer>,
        color_targets: &[SDL_GPUColorTargetInfo],
        depth_stencil_target: Option<&SDL_GPUDepthStencilTargetInfo>,
    ) -> SdlResult<Self> {
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

resource!(GPUComputePass, SDL, End);
impl GPUComputePass {
    // TODO: `SDL_BeginGPUComputePass`
}

impl GPUComputePassHandle {
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

#[repr(i32)]
pub enum ShaderStage {
    Vertex = SDL_GPUShaderStage::VERTEX.0,
    Fragment = SDL_GPUShaderStage::FRAGMENT.0,
}

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
    pub fn new(device: Ref<GPUDevice>, create_info: &ShaderCreateInfo) -> SdlResult<Self> {
        let handle = unsafe { SDL_CreateGPUShader(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe {
            SDL_ReleaseGPUShader(device.handle.as_ptr(), self.handle.as_ptr());
        }
    }
}

#[repr(i32)]
pub enum TextureType {
    _2d = SDL_GPUTextureType::_2D.0,
    _2dArray = SDL_GPUTextureType::_2D_ARRAY.0,
    _3d = SDL_GPUTextureType::_3D.0,
    Cube = SDL_GPUTextureType::CUBE.0,
    CubeArray = SDL_GPUTextureType::CUBE_ARRAY.0,
}

#[bitmask(u32)]
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
pub enum SampleCount {
    One = SDL_GPUSampleCount::_1.0,
    Two = SDL_GPUSampleCount::_2.0,
    Four = SDL_GPUSampleCount::_4.0,
    Eight = SDL_GPUSampleCount::_8.0,
}

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
    pub fn new(device: Ref<GPUDevice>, create_info: &TextureCreateInfo) -> SdlResult<Self> {
        let handle = unsafe { SDL_CreateGPUTexture(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUTexture(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl GPUTextureHandle {
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
}

resource_no_drop!(GPUTransferBuffer, SDL);
impl GPUTransferBuffer {
    pub fn drop(self, dev: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUTransferBuffer(dev.handle.as_ptr(), self.handle.as_ptr()) };
    }

    // TODO:
    // - SDL_CreateGPUTransferBuffer
    // - SDL_MapGPUTransferBuffer
    // - SDL_UnmapGPUTransferBuffer
}
