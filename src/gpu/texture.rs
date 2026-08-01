//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_CreateGPUTexture
//! - [x] SDL_DownloadFromGPUTexture
//! - [x] SDL_ReleaseGPUTexture
//! - [x] SDL_SetGPUTextureName
//! - [x] SDL_UploadToGPUTexture

use std::ffi::CStr;

use bitmask_enum::bitmask;
use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{Result, rect::Point, resource::Ref, resource_new_no_drop};

use super::{
    copy_pass::CopyPass, device::Device, sampler::Sampler, transfer_buffer::TransferBuffer,
};

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

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_GPUTextureFormat")]
pub enum TextureFormat {
    Invalid = SDL_GPUTextureFormat::INVALID.0,
    A8Unorm = SDL_GPUTextureFormat::A8_UNORM.0,
    R8Unorm = SDL_GPUTextureFormat::R8_UNORM.0,
    R8g8Unorm = SDL_GPUTextureFormat::R8G8_UNORM.0,
    R8g8b8a8Unorm = SDL_GPUTextureFormat::R8G8B8A8_UNORM.0,
    R16Unorm = SDL_GPUTextureFormat::R16_UNORM.0,
    R16g16Unorm = SDL_GPUTextureFormat::R16G16_UNORM.0,
    R16g16b16a16Unorm = SDL_GPUTextureFormat::R16G16B16A16_UNORM.0,
    R10g10b10a2Unorm = SDL_GPUTextureFormat::R10G10B10A2_UNORM.0,
    B5g6r5Unorm = SDL_GPUTextureFormat::B5G6R5_UNORM.0,
    B5g5r5a1Unorm = SDL_GPUTextureFormat::B5G5R5A1_UNORM.0,
    B4g4r4a4Unorm = SDL_GPUTextureFormat::B4G4R4A4_UNORM.0,
    B8g8r8a8Unorm = SDL_GPUTextureFormat::B8G8R8A8_UNORM.0,
    Bc1RgbaUnorm = SDL_GPUTextureFormat::BC1_RGBA_UNORM.0,
    Bc2RgbaUnorm = SDL_GPUTextureFormat::BC2_RGBA_UNORM.0,
    Bc3RgbaUnorm = SDL_GPUTextureFormat::BC3_RGBA_UNORM.0,
    Bc4RUnorm = SDL_GPUTextureFormat::BC4_R_UNORM.0,
    Bc5RgUnorm = SDL_GPUTextureFormat::BC5_RG_UNORM.0,
    Bc7RgbaUnorm = SDL_GPUTextureFormat::BC7_RGBA_UNORM.0,
    Bc6hRgbFloat = SDL_GPUTextureFormat::BC6H_RGB_FLOAT.0,
    Bc6hRgbUfloat = SDL_GPUTextureFormat::BC6H_RGB_UFLOAT.0,
    R8Snorm = SDL_GPUTextureFormat::R8_SNORM.0,
    R8g8Snorm = SDL_GPUTextureFormat::R8G8_SNORM.0,
    R8g8b8a8Snorm = SDL_GPUTextureFormat::R8G8B8A8_SNORM.0,
    R16Snorm = SDL_GPUTextureFormat::R16_SNORM.0,
    R16g16Snorm = SDL_GPUTextureFormat::R16G16_SNORM.0,
    R16g16b16a16Snorm = SDL_GPUTextureFormat::R16G16B16A16_SNORM.0,
    R16Float = SDL_GPUTextureFormat::R16_FLOAT.0,
    R16g16Float = SDL_GPUTextureFormat::R16G16_FLOAT.0,
    R16g16b16a16Float = SDL_GPUTextureFormat::R16G16B16A16_FLOAT.0,
    R32Float = SDL_GPUTextureFormat::R32_FLOAT.0,
    R32g32Float = SDL_GPUTextureFormat::R32G32_FLOAT.0,
    R32g32b32a32Float = SDL_GPUTextureFormat::R32G32B32A32_FLOAT.0,
    R11g11b10Ufloat = SDL_GPUTextureFormat::R11G11B10_UFLOAT.0,
    R8Uint = SDL_GPUTextureFormat::R8_UINT.0,
    R8g8Uint = SDL_GPUTextureFormat::R8G8_UINT.0,
    R8g8b8a8Uint = SDL_GPUTextureFormat::R8G8B8A8_UINT.0,
    R16Uint = SDL_GPUTextureFormat::R16_UINT.0,
    R16g16Uint = SDL_GPUTextureFormat::R16G16_UINT.0,
    R16g16b16a16Uint = SDL_GPUTextureFormat::R16G16B16A16_UINT.0,
    R32Uint = SDL_GPUTextureFormat::R32_UINT.0,
    R32g32Uint = SDL_GPUTextureFormat::R32G32_UINT.0,
    R32g32b32a32Uint = SDL_GPUTextureFormat::R32G32B32A32_UINT.0,
    R8Int = SDL_GPUTextureFormat::R8_INT.0,
    R8g8Int = SDL_GPUTextureFormat::R8G8_INT.0,
    R8g8b8a8Int = SDL_GPUTextureFormat::R8G8B8A8_INT.0,
    R16Int = SDL_GPUTextureFormat::R16_INT.0,
    R16g16Int = SDL_GPUTextureFormat::R16G16_INT.0,
    R16g16b16a16Int = SDL_GPUTextureFormat::R16G16B16A16_INT.0,
    R32Int = SDL_GPUTextureFormat::R32_INT.0,
    R32g32Int = SDL_GPUTextureFormat::R32G32_INT.0,
    R32g32b32a32Int = SDL_GPUTextureFormat::R32G32B32A32_INT.0,
    R8g8b8a8UnormSrgb = SDL_GPUTextureFormat::R8G8B8A8_UNORM_SRGB.0,
    B8g8r8a8UnormSrgb = SDL_GPUTextureFormat::B8G8R8A8_UNORM_SRGB.0,
    Bc1RgbaUnormSrgb = SDL_GPUTextureFormat::BC1_RGBA_UNORM_SRGB.0,
    Bc2RgbaUnormSrgb = SDL_GPUTextureFormat::BC2_RGBA_UNORM_SRGB.0,
    Bc3RgbaUnormSrgb = SDL_GPUTextureFormat::BC3_RGBA_UNORM_SRGB.0,
    Bc7RgbaUnormSrgb = SDL_GPUTextureFormat::BC7_RGBA_UNORM_SRGB.0,
    D16Unorm = SDL_GPUTextureFormat::D16_UNORM.0,
    D24Unorm = SDL_GPUTextureFormat::D24_UNORM.0,
    D32Float = SDL_GPUTextureFormat::D32_FLOAT.0,
    D24UnormS8Uint = SDL_GPUTextureFormat::D24_UNORM_S8_UINT.0,
    D32FloatS8Uint = SDL_GPUTextureFormat::D32_FLOAT_S8_UINT.0,
    Astc4x4Unorm = SDL_GPUTextureFormat::ASTC_4x4_UNORM.0,
    Astc5x4Unorm = SDL_GPUTextureFormat::ASTC_5x4_UNORM.0,
    Astc5x5Unorm = SDL_GPUTextureFormat::ASTC_5x5_UNORM.0,
    Astc6x5Unorm = SDL_GPUTextureFormat::ASTC_6x5_UNORM.0,
    Astc6x6Unorm = SDL_GPUTextureFormat::ASTC_6x6_UNORM.0,
    Astc8x5Unorm = SDL_GPUTextureFormat::ASTC_8x5_UNORM.0,
    Astc8x6Unorm = SDL_GPUTextureFormat::ASTC_8x6_UNORM.0,
    Astc8x8Unorm = SDL_GPUTextureFormat::ASTC_8x8_UNORM.0,
    Astc10x5Unorm = SDL_GPUTextureFormat::ASTC_10x5_UNORM.0,
    Astc10x6Unorm = SDL_GPUTextureFormat::ASTC_10x6_UNORM.0,
    Astc10x8Unorm = SDL_GPUTextureFormat::ASTC_10x8_UNORM.0,
    Astc10x10Unorm = SDL_GPUTextureFormat::ASTC_10x10_UNORM.0,
    Astc12x10Unorm = SDL_GPUTextureFormat::ASTC_12x10_UNORM.0,
    Astc12x12Unorm = SDL_GPUTextureFormat::ASTC_12x12_UNORM.0,
    Astc4x4UnormSrgb = SDL_GPUTextureFormat::ASTC_4x4_UNORM_SRGB.0,
    Astc5x4UnormSrgb = SDL_GPUTextureFormat::ASTC_5x4_UNORM_SRGB.0,
    Astc5x5UnormSrgb = SDL_GPUTextureFormat::ASTC_5x5_UNORM_SRGB.0,
    Astc6x5UnormSrgb = SDL_GPUTextureFormat::ASTC_6x5_UNORM_SRGB.0,
    Astc6x6UnormSrgb = SDL_GPUTextureFormat::ASTC_6x6_UNORM_SRGB.0,
    Astc8x5UnormSrgb = SDL_GPUTextureFormat::ASTC_8x5_UNORM_SRGB.0,
    Astc8x6UnormSrgb = SDL_GPUTextureFormat::ASTC_8x6_UNORM_SRGB.0,
    Astc8x8UnormSrgb = SDL_GPUTextureFormat::ASTC_8x8_UNORM_SRGB.0,
    Astc10x5UnormSrgb = SDL_GPUTextureFormat::ASTC_10x5_UNORM_SRGB.0,
    Astc10x6UnormSrgb = SDL_GPUTextureFormat::ASTC_10x6_UNORM_SRGB.0,
    Astc10x8UnormSrgb = SDL_GPUTextureFormat::ASTC_10x8_UNORM_SRGB.0,
    Astc10x10UnormSrgb = SDL_GPUTextureFormat::ASTC_10x10_UNORM_SRGB.0,
    Astc12x10UnormSrgb = SDL_GPUTextureFormat::ASTC_12x10_UNORM_SRGB.0,
    Astc12x12UnormSrgb = SDL_GPUTextureFormat::ASTC_12x12_UNORM_SRGB.0,
    Astc4x4Float = SDL_GPUTextureFormat::ASTC_4x4_FLOAT.0,
    Astc5x4Float = SDL_GPUTextureFormat::ASTC_5x4_FLOAT.0,
    Astc5x5Float = SDL_GPUTextureFormat::ASTC_5x5_FLOAT.0,
    Astc6x5Float = SDL_GPUTextureFormat::ASTC_6x5_FLOAT.0,
    Astc6x6Float = SDL_GPUTextureFormat::ASTC_6x6_FLOAT.0,
    Astc8x5Float = SDL_GPUTextureFormat::ASTC_8x5_FLOAT.0,
    Astc8x6Float = SDL_GPUTextureFormat::ASTC_8x6_FLOAT.0,
    Astc8x8Float = SDL_GPUTextureFormat::ASTC_8x8_FLOAT.0,
    Astc10x5Float = SDL_GPUTextureFormat::ASTC_10x5_FLOAT.0,
    Astc10x6Float = SDL_GPUTextureFormat::ASTC_10x6_FLOAT.0,
    Astc10x8Float = SDL_GPUTextureFormat::ASTC_10x8_FLOAT.0,
    Astc10x10Float = SDL_GPUTextureFormat::ASTC_10x10_FLOAT.0,
    Astc12x10Float = SDL_GPUTextureFormat::ASTC_12x10_FLOAT.0,
    Astc12x12Float = SDL_GPUTextureFormat::ASTC_12x12_FLOAT.0,
}

impl From<SDL_GPUTextureFormat> for TextureFormat {
    fn from(value: SDL_GPUTextureFormat) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[doc(alias = "SDL_GPUTextureCreateInfo")]
#[derive(Clone, Copy)]
pub struct TextureCreateInfo(SDL_GPUTextureCreateInfo);
impl TextureCreateInfo {
    pub const fn new(
        kind: TextureType,
        format: TextureFormat,
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
        let format = SDL_GPUTextureFormat::new(format as _);

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
#[derive(Clone, Copy)]
pub struct TextureTransferInfo(SDL_GPUTextureTransferInfo);
impl TextureTransferInfo {
    pub fn new(
        tb: Ref<TransferBuffer>,
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
#[derive(Clone, Copy)]
pub struct TextureRegion(SDL_GPUTextureRegion);
impl TextureRegion {
    pub fn new(
        tex: Ref<Texture>,
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

#[doc(alias = "SDL_GPUTextureLocation")]
#[derive(Clone, Copy)]
pub struct TextureLocation(pub(crate) SDL_GPUTextureLocation);
impl TextureLocation {
    pub fn new(tex: Ref<Texture>, mip_level: u32, layer: u32, (x, y, z): (u32, u32, u32)) -> Self {
        let texture = tex.handle.as_ptr();
        let inner = SDL_GPUTextureLocation {
            texture,
            mip_level,
            layer,
            x,
            y,
            z,
        };
        Self(inner)
    }
}

#[doc(alias = "SDL_GPUTextureSamplerBinding")]
#[derive(Clone, Copy)]
pub struct TextureSamplerBinding(SDL_GPUTextureSamplerBinding);
impl TextureSamplerBinding {
    pub fn new(texture: Ref<Texture>, sampler: Ref<Sampler>) -> Self {
        Self(SDL_GPUTextureSamplerBinding {
            texture: texture.handle.as_ptr(),
            sampler: sampler.handle.as_ptr(),
        })
    }
}

#[doc(alias = "SDL_GPUStorageTextureReadWriteBinding")]
#[derive(Clone, Copy)]
pub struct StorageTextureReadWriteBinding(SDL_GPUStorageTextureReadWriteBinding);
impl StorageTextureReadWriteBinding {
    pub fn new(texture: Ref<Texture>, mip_level: u32, layer: u32, cycle: bool) -> Self {
        Self(SDL_GPUStorageTextureReadWriteBinding {
            texture: texture.handle.as_ptr(),
            mip_level,
            layer,
            cycle,
            ..Default::default()
        })
    }
}

#[doc(alias = "SDL_GPUBlitRegion")]
#[derive(Clone, Copy)]
pub struct BlitRegion(pub(crate) SDL_GPUBlitRegion);
impl BlitRegion {
    pub fn new(
        tex: Ref<Texture>,
        mip_level: u32,
        layer_or_depth_plane: u32,
        (x, y, w, h): (u32, u32, u32, u32),
    ) -> Self {
        let texture = tex.handle.as_ptr();
        Self(SDL_GPUBlitRegion {
            texture,
            mip_level,
            layer_or_depth_plane,
            x,
            y,
            w,
            h,
        })
    }
}

resource_new_no_drop!(SDL_GPUTexture, Texture);
impl Texture {
    #[doc(alias = "SDL_CreateGPUTexture")]
    pub fn new(device: Ref<Device>, create_info: &TextureCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUTexture(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUTexture")]
    pub fn drop(self, device: Ref<Device>) {
        unsafe { SDL_ReleaseGPUTexture(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl TextureHandle {
    #[doc(alias = "SDL_DownloadFromGPUTexture")]
    pub fn download(
        &self,
        copy_pass: Ref<CopyPass>,
        src: &TextureRegion,
        dst: &TextureTransferInfo,
    ) {
        unsafe { SDL_DownloadFromGPUTexture(copy_pass.handle.as_ptr(), &src.0, &dst.0) };
    }

    #[doc(alias = "SDL_UploadToGPUTexture")]
    pub fn upload(
        &self,
        copy_pass: Ref<CopyPass>,
        src: &TextureTransferInfo,
        dst: &TextureRegion,
        cycle: bool,
    ) {
        unsafe {
            SDL_UploadToGPUTexture(copy_pass.handle.as_ptr(), &src.0, &dst.0, cycle);
        }
    }

    #[doc(alias = "SDL_SetGPUTextureName")]
    pub fn set_name(&self, device: Ref<Device>, name: &CStr) {
        unsafe {
            SDL_SetGPUTextureName(device.handle.as_ptr(), self.handle.as_ptr(), name.as_ptr())
        }
    }
}
