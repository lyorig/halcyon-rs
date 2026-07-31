//! Minimal SDL_gpu wrapper, plus some convenience functions.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [ ] SDL_AcquireGPUSwapchainTexture
//! - [ ] SDL_BindGPUComputeSamplers
//! - [ ] SDL_BindGPUComputeStorageBuffers
//! - [ ] SDL_BindGPUComputeStorageTextures
//! - [ ] SDL_BindGPUFragmentSamplers
//! - [ ] SDL_BindGPUFragmentStorageBuffers
//! - [ ] SDL_BindGPUFragmentStorageTextures
//! - [ ] SDL_BindGPUIndexBuffer
//! - [ ] SDL_BindGPUVertexBuffers
//! - [ ] SDL_BindGPUVertexSamplers
//! - [ ] SDL_BindGPUVertexStorageBuffers
//! - [ ] SDL_BindGPUVertexStorageTextures
//! - [ ] SDL_CalculateGPUTextureFormatSize
//! - [ ] SDL_CopyGPUBufferToBuffer
//! - [ ] SDL_CopyGPUTextureToTexture
//! - [ ] SDL_CreateGPUDeviceWithProperties
//! - [ ] SDL_CreateGPUSampler
//! - [ ] SDL_DispatchGPUComputeIndirect
//! - [ ] SDL_DrawGPUIndexedPrimitives
//! - [ ] SDL_DrawGPUIndexedPrimitivesIndirect
//! - [ ] SDL_GDKResumeGPU
//! - [ ] SDL_GDKSuspendGPU
//! - [ ] SDL_GenerateMipmapsForGPUTexture
//! - [ ] SDL_GetGPUDeviceProperties
//! - [x] SDL_GetGPUDriver
//! - [ ] SDL_GetGPUSwapchainTextureFormat
//! - [ ] SDL_GetGPUTextureFormatFromPixelFormat
//! - [x] SDL_GetNumGPUDrivers
//! - [ ] SDL_GetPixelFormatFromGPUTextureFormat
//! - [ ] SDL_GPUSupportsProperties
//! - [x] SDL_GPUSupportsShaderFormats
//! - [ ] SDL_GPUTextureFormatTexelBlockSize
//! - [ ] SDL_GPUTextureSupportsFormat
//! - [ ] SDL_GPUTextureSupportsSampleCount
//! - [ ] SDL_InsertGPUDebugLabel
//! - [ ] SDL_PopGPUDebugGroup
//! - [ ] SDL_PushGPUComputeUniformData
//! - [ ] SDL_PushGPUDebugGroup
//! - [ ] SDL_PushGPUFragmentUniformData
//! - [ ] SDL_PushGPUVertexUniformData
//! - [ ] SDL_ReleaseGPUSampler
//! - [ ] SDL_SetGPUBlendConstants
//! - [ ] SDL_SetGPUStencilReference
//! - [ ] SDL_SetGPUSwapchainParameters
//! - [ ] SDL_SetGPUViewport

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
use sdl3_sys::gpu::*;

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
