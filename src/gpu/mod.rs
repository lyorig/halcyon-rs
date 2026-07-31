//! Minimal SDL_gpu wrapper, plus some convenience functions.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_AcquireGPUSwapchainTexture
//! - [x] SDL_BindGPUComputeSamplers
//! - [x] SDL_BindGPUComputeStorageBuffers
//! - [x] SDL_BindGPUComputeStorageTextures
//! - [x] SDL_BindGPUFragmentSamplers
//! - [x] SDL_BindGPUFragmentStorageBuffers
//! - [x] SDL_BindGPUFragmentStorageTextures
//! - [x] SDL_BindGPUIndexBuffer
//! - [x] SDL_BindGPUVertexBuffers
//! - [x] SDL_BindGPUVertexSamplers
//! - [x] SDL_BindGPUVertexStorageBuffers
//! - [x] SDL_BindGPUVertexStorageTextures
//! - [x] SDL_CalculateGPUTextureFormatSize
//! - [x] SDL_CopyGPUBufferToBuffer
//! - [x] SDL_CopyGPUTextureToTexture
//! - [x] SDL_CreateGPUDeviceWithProperties
//! - [ ] SDL_CreateGPUSampler
//! - [x] SDL_DispatchGPUComputeIndirect
//! - [x] SDL_DrawGPUIndexedPrimitives
//! - [x] SDL_DrawGPUIndexedPrimitivesIndirect
//! - [ ] SDL_GDKResumeGPU
//! - [ ] SDL_GDKSuspendGPU
//! - [x] SDL_GenerateMipmapsForGPUTexture
//! - [x] SDL_GetGPUDeviceProperties
//! - [x] SDL_GetGPUDriver
//! - [x] SDL_GetGPUSwapchainTextureFormat
//! - [x] SDL_GetGPUTextureFormatFromPixelFormat
//! - [x] SDL_GetNumGPUDrivers
//! - [x] SDL_GetPixelFormatFromGPUTextureFormat
//! - [x] SDL_GPUSupportsProperties
//! - [x] SDL_GPUSupportsShaderFormats
//! - [x] SDL_GPUTextureFormatTexelBlockSize
//! - [x] SDL_GPUTextureSupportsFormat
//! - [x] SDL_GPUTextureSupportsSampleCount
//! - [x] SDL_InsertGPUDebugLabel
//! - [x] SDL_PopGPUDebugGroup
//! - [x] SDL_PushGPUComputeUniformData
//! - [x] SDL_PushGPUDebugGroup
//! - [x] SDL_PushGPUFragmentUniformData
//! - [x] SDL_PushGPUVertexUniformData
//! - [ ] SDL_ReleaseGPUSampler
//! - [x] SDL_SetGPUBlendConstants
//! - [x] SDL_SetGPUStencilReference
//! - [x] SDL_SetGPUSwapchainParameters
//! - [x] SDL_SetGPUViewport

pub mod buffer;
pub mod command_buffer;
pub mod compute_pass;
pub mod compute_pipeline;
pub mod copy_pass;
pub mod device;
pub mod fence;
pub mod graphics_pipeline;
pub mod render_pass;
pub mod shader;
pub mod texture;
pub mod transfer_buffer;

pub use buffer::*;
pub use command_buffer::*;
pub use compute_pass::*;
pub use compute_pipeline::*;
pub use copy_pass::*;
pub use device::*;
pub use fence::*;
pub use graphics_pipeline::*;
pub use render_pass::*;
pub use shader::*;
pub use texture::*;
pub use transfer_buffer::*;

use bitmask_enum::bitmask;
use sdl3_sys::{gpu::*, pixels::SDL_PixelFormat, properties::SDL_PropertiesID};

use crate::util::c_ptr_to_str;

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
    format: SDL_GPUTextureFormat,
    width: u32,
    height: u32,
    depth_or_layer_count: u32,
) -> u32 {
    unsafe { SDL_CalculateGPUTextureFormatSize(format, width, height, depth_or_layer_count) }
}

#[doc(alias = "SDL_GPUTextureFormatTexelBlockSize")]
pub fn texture_format_texel_block_size(format: SDL_GPUTextureFormat) -> u32 {
    unsafe { SDL_GPUTextureFormatTexelBlockSize(format) }
}

#[doc(alias = "SDL_GetGPUTextureFormatFromPixelFormat")]
pub fn texture_format_from_pixel_format(pixel_format: SDL_PixelFormat) -> SDL_GPUTextureFormat {
    SDL_GetGPUTextureFormatFromPixelFormat(pixel_format)
}

#[doc(alias = "SDL_GetPixelFormatFromGPUTextureFormat")]
pub fn pixel_format_from_texture_format(format: SDL_GPUTextureFormat) -> SDL_PixelFormat {
    SDL_GetPixelFormatFromGPUTextureFormat(format)
}

#[doc(alias = "SDL_GPUSupportsProperties")]
pub fn supports_properties(props: SDL_PropertiesID) -> bool {
    unsafe { SDL_GPUSupportsProperties(props) }
}
