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

        Self(inner, PhantomData, PhantomData)
    }
}

resource_new_no_drop!(SDL_GPUComputePipeline, ComputePipeline);
impl ComputePipeline {
    /// Bind a builder to a property group.
    pub fn builder<'p>(props: Ref<'p, Properties>) -> ComputePipelineBuilder<'p> {
        ComputePipelineBuilder::new(props)
    }

    #[doc(alias = "SDL_CreateGPUComputePipeline")]
    pub fn new(device: Ref<Device>, create_info: &ComputePipelineCreateInfo) -> Result<Self> {
        let handle =
            unsafe { SDL_CreateGPUComputePipeline(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUComputePipeline")]
    pub fn drop(self, device: Ref<Device>) {
        unsafe { SDL_ReleaseGPUComputePipeline(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}
