//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_CreateGPUTexture
//! - [x] SDL_DownloadFromGPUTexture
//! - [x] SDL_ReleaseGPUTexture
//! - [x] SDL_SetGPUTextureName
//! - [x] SDL_UploadToGPUTexture

use std::ffi::CStr;

use bitmask_enum::bitmask;
use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{Result, rect::Point, resource_no_drop, traits::Ref};

use super::{copy_pass::GPUCopyPass, device::GPUDevice, transfer_buffer::GPUTransferBuffer};

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
