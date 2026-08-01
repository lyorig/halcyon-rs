//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_CreateGPUShader
//! - [x] SDL_ReleaseGPUShader

use std::ffi::CStr;

use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{Result, resource::Ref, resource_new_no_drop};

use super::{ShaderFormat, device::Device};

#[repr(i32)]
#[doc(alias = "SDL_GPUShaderStage")]
pub enum ShaderStage {
    Vertex = SDL_GPUShaderStage::VERTEX.0,
    Fragment = SDL_GPUShaderStage::FRAGMENT.0,
}

#[doc(alias = "SDL_GPUShaderCreateInfo")]
#[derive(Clone, Copy)]
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

resource_new_no_drop!(SDL_GPUShader, Shader);
impl Shader {
    #[doc(alias = "SDL_CreateGPUShader")]
    pub fn new(device: Ref<Device>, create_info: &ShaderCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUShader(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUShader")]
    pub fn drop(self, device: Ref<Device>) {
        unsafe {
            SDL_ReleaseGPUShader(device.handle.as_ptr(), self.handle.as_ptr());
        }
    }
}
