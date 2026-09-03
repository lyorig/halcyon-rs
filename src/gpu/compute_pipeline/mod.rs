//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_CreateGPUComputePipeline
//! - [x] SDL_ReleaseGPUComputePipeline

use std::{ffi::CStr, marker::PhantomData};

use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{Result, mod_reexport, properties::Properties, resource::Ref, resource_new_no_drop};

use super::{ShaderFormat, device::Device};

mod_reexport!(builder);

#[doc(alias = "SDL_GPUComputePipelineCreateInfo")]
#[derive(Clone, Copy)]
pub struct ComputePipelineCreateInfo<'bc, 'ep>(
    SDL_GPUComputePipelineCreateInfo,
    PhantomData<&'bc [u8]>,
    PhantomData<&'ep CStr>,
);

impl<'bc, 'ep> ComputePipelineCreateInfo<'bc, 'ep> {
    pub const fn new(
        code: &'bc [u8],
        entrypoint: &'ep CStr,
        fmt: ShaderFormat,
        sampler_count: u32,
        uniform_buffer_count: u32,
        (
            readonly_storage_texture_count,
            readonly_storage_buffer_count,
            readwrite_storage_texture_count,
            readwrite_storage_buffer_count,
        ): (u32, u32, u32, u32),
        threadcount_xyz: (u32, u32, u32),
    ) -> Self {
        let inner = SDL_GPUComputePipelineCreateInfo {
            code_size: code.len(),
            code: code.as_ptr(),
            entrypoint: entrypoint.as_ptr(),
            format: SDL_GPUShaderFormat::new(fmt as _),
            num_samplers: sampler_count,
            num_readonly_storage_textures: readonly_storage_texture_count,
            num_readonly_storage_buffers: readonly_storage_buffer_count,
            num_readwrite_storage_textures: readwrite_storage_texture_count,
            num_readwrite_storage_buffers: readwrite_storage_buffer_count,
            num_uniform_buffers: uniform_buffer_count,
            threadcount_x: threadcount_xyz.0,
            threadcount_y: threadcount_xyz.1,
            threadcount_z: threadcount_xyz.2,
            props: SDL_PropertiesID::new(0),
        };

        Self(inner, PhantomData, PhantomData)
    }
}

resource_new_no_drop!(SDL_GPUComputePipeline, ComputePipeline);
impl ComputePipeline {
    /// Bind a builder to a property group.
    pub fn builder(props: Ref<'_, Properties>) -> ComputePipelineBuilder<'_> {
        ComputePipelineBuilder::new(props)
    }

    #[doc(alias = "SDL_CreateGPUComputePipeline")]
    pub fn new(device: Ref<Device>, create_info: &ComputePipelineCreateInfo) -> Result<Self> {
        let handle = unsafe {
            SDL_CreateGPUComputePipeline(device.handle.as_ptr(), &raw const create_info.0)
        };

        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUComputePipeline")]
    pub fn drop(self, device: Ref<Device>) {
        unsafe { SDL_ReleaseGPUComputePipeline(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}
