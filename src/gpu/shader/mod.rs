//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_CreateGPUShader
//! - [x] SDL_ReleaseGPUShader

use std::{ffi::CStr, marker::PhantomData};

use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{Result, mod_reexport, properties::Properties, resource::Ref, resource_new_no_drop};

use super::{ShaderFormat, device::Device};

mod_reexport!(builder);

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_GPUShaderStage")]
pub enum ShaderStage {
    Vertex = SDL_GPUShaderStage::VERTEX.0,
    Fragment = SDL_GPUShaderStage::FRAGMENT.0,
}

/// `'bc` and `'ep` tie the create-info to the lifetime of the shader bytecode and its
#[doc(alias = "SDL_GPUShaderCreateInfo")]
#[derive(Clone, Copy)]
pub struct ShaderCreateInfo<'bc, 'ep>(
    SDL_GPUShaderCreateInfo,
    PhantomData<&'bc [u8]>,
    PhantomData<&'ep CStr>,
);

impl<'bc, 'ep> ShaderCreateInfo<'bc, 'ep> {
    pub const fn new(
        code: &'bc [u8],
        entrypoint: &'ep CStr,
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

        Self(inner, PhantomData, PhantomData)
    }

    /// Creates a vertex-shader [`ShaderCreateInfo`] with no resource bindings.
    pub const fn vertex(code: &'bc [u8], entrypoint: &'ep CStr, fmt: ShaderFormat) -> Self {
        Self::new(code, entrypoint, fmt, ShaderStage::Vertex, 0, (0, 0, 0))
    }

    /// Creates a fragment-shader [`ShaderCreateInfo`] with no resource bindings.
    pub const fn fragment(code: &'bc [u8], entrypoint: &'ep CStr, fmt: ShaderFormat) -> Self {
        Self::new(code, entrypoint, fmt, ShaderStage::Fragment, 0, (0, 0, 0))
    }
}

resource_new_no_drop!(SDL_GPUShader, Shader);
impl Shader {
    /// Bind a builder to a property group.
    pub fn builder(props: Ref<'_, Properties>) -> ShaderBuilder<'_> {
        ShaderBuilder::new(props)
    }

    #[doc(alias = "SDL_CreateGPUShader")]
    pub fn new(device: Ref<Device>, create_info: &ShaderCreateInfo) -> Result<Self> {
        let handle =
            unsafe { SDL_CreateGPUShader(device.handle.as_ptr(), &raw const create_info.0) };

        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUShader")]
    pub fn drop(self, device: Ref<Device>) {
        unsafe {
            SDL_ReleaseGPUShader(device.handle.as_ptr(), self.handle.as_ptr());
        }
    }
}
