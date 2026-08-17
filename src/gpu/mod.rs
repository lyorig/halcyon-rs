//! SDL_gpu wrapper.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)).
//! Only free functions and functionality that fits no submodule belong here;
//! everything else is covered by the submodule checklists.
//! - [x] SDL_CalculateGPUTextureFormatSize
//! - [ ] SDL_GDKResumeGPU — GDK (Xbox) only, makes no sense to implement here
//! - [ ] SDL_GDKSuspendGPU — GDK (Xbox) only, makes no sense to implement here
//! - [x] SDL_GetGPUDriver
//! - [x] SDL_GetGPUTextureFormatFromPixelFormat
//! - [x] SDL_GetNumGPUDrivers
//! - [x] SDL_GetPixelFormatFromGPUTextureFormat
//! - [x] SDL_GPUSupportsProperties
//! - [x] SDL_GPUSupportsShaderFormats
//! - [x] SDL_GPUTextureFormatTexelBlockSize

mod_reexport!(buffer);
mod_reexport!(command_buffer);
mod_reexport!(compute_pass);
mod_reexport!(compute_pipeline);
mod_reexport!(copy_pass);
mod_reexport!(device);
mod_reexport!(enums);
mod_reexport!(fence);
mod_reexport!(graphics_pipeline);
mod_reexport!(pipeline_state);
mod_reexport!(render_pass);
mod_reexport!(render_state);
mod_reexport!(sampler);
mod_reexport!(shader);
mod_reexport!(texture);
mod_reexport!(transfer_buffer);

use bitmask_enum::bitmask;
use sdl3_sys::{gpu::*, pixels::SDL_PixelFormat, properties::SDL_PropertiesID};

use crate::{mod_reexport, util::c_ptr_to_str};

/// Non-bitmask variant of `SDL_GPUShaderFormat`.
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

impl ShaderFormat {
    /// Returns [`ShaderFormats`] with only this bit set.
    pub const fn as_mask(self) -> ShaderFormats {
        ShaderFormats { bits: self as u32 }
    }
}

impl From<ShaderFormat> for ShaderFormats {
    fn from(value: ShaderFormat) -> Self {
        value.as_mask()
    }
}

#[doc(alias = "SDL_GPUSupportsShaderFormats")]
pub fn are_formats_supported(fmts: ShaderFormats) -> bool {
    let fmts = SDL_GPUShaderFormat::new(fmts.bits());
    unsafe { SDL_GPUSupportsShaderFormats(fmts, std::ptr::null()) }
}

#[doc(alias = "SDL_GetNumGPUDrivers")]
pub fn num_drivers() -> i32 {
    unsafe { SDL_GetNumGPUDrivers() }
}

#[doc(alias = "SDL_GetGPUDriver")]
pub fn driver(i: i32) -> Option<&'static str> {
    let ptr = unsafe { SDL_GetGPUDriver(i) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { c_ptr_to_str(ptr) })
    }
}

#[doc(alias = "SDL_CalculateGPUTextureFormatSize")]
pub fn calculate_texture_format_size(
    format: TextureFormat,
    width: u32,
    height: u32,
    depth_or_layer_count: u32,
) -> u32 {
    unsafe {
        SDL_CalculateGPUTextureFormatSize(
            SDL_GPUTextureFormat::new(format as _),
            width,
            height,
            depth_or_layer_count,
        )
    }
}

#[doc(alias = "SDL_GPUTextureFormatTexelBlockSize")]
pub fn texture_format_texel_block_size(format: TextureFormat) -> u32 {
    unsafe { SDL_GPUTextureFormatTexelBlockSize(SDL_GPUTextureFormat::new(format as _)) }
}

#[doc(alias = "SDL_GetGPUTextureFormatFromPixelFormat")]
pub fn texture_format_from_pixel_format(pixel_format: SDL_PixelFormat) -> TextureFormat {
    SDL_GetGPUTextureFormatFromPixelFormat(pixel_format).into()
}

#[doc(alias = "SDL_GetPixelFormatFromGPUTextureFormat")]
pub fn pixel_format_from_texture_format(format: TextureFormat) -> SDL_PixelFormat {
    SDL_GetPixelFormatFromGPUTextureFormat(SDL_GPUTextureFormat::new(format as _))
}

#[doc(alias = "SDL_GPUSupportsProperties")]
pub fn supports_properties(props: SDL_PropertiesID) -> bool {
    unsafe { SDL_GPUSupportsProperties(props) }
}
