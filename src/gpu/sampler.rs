//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_CreateGPUSampler
//! - [x] SDL_ReleaseGPUSampler

use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{
    Result,
    gpu::{EnableAnisotropy, EnableCompare},
    resource::Ref,
    resource_new_no_drop,
};

use super::device::Device;

#[repr(i32)]
#[doc(alias = "SDL_GPUFilter")]
pub enum Filter {
    Nearest = SDL_GPUFilter::NEAREST.0,
    Linear = SDL_GPUFilter::LINEAR.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUSamplerMipmapMode")]
pub enum MipmapMode {
    Nearest = SDL_GPUSamplerMipmapMode::NEAREST.0,
    Linear = SDL_GPUSamplerMipmapMode::LINEAR.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUSamplerAddressMode")]
pub enum AddressMode {
    Repeat = SDL_GPUSamplerAddressMode::REPEAT.0,
    MirroredRepeat = SDL_GPUSamplerAddressMode::MIRRORED_REPEAT.0,
    ClampToEdge = SDL_GPUSamplerAddressMode::CLAMP_TO_EDGE.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUCompareOp")]
pub enum CompareOp {
    Invalid = SDL_GPUCompareOp::INVALID.0,
    Never = SDL_GPUCompareOp::NEVER.0,
    Less = SDL_GPUCompareOp::LESS.0,
    Equal = SDL_GPUCompareOp::EQUAL.0,
    LessOrEqual = SDL_GPUCompareOp::LESS_OR_EQUAL.0,
    Greater = SDL_GPUCompareOp::GREATER.0,
    NotEqual = SDL_GPUCompareOp::NOT_EQUAL.0,
    GreaterOrEqual = SDL_GPUCompareOp::GREATER_OR_EQUAL.0,
    Always = SDL_GPUCompareOp::ALWAYS.0,
}

#[doc(alias = "SDL_GPUSamplerCreateInfo")]
#[derive(Clone, Copy)]
pub struct SamplerCreateInfo(SDL_GPUSamplerCreateInfo);
impl SamplerCreateInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        min_filter: Filter,
        mag_filter: Filter,
        mipmap_mode: MipmapMode,
        (u, v, w): (AddressMode, AddressMode, AddressMode),
        mip_lod_bias: f32,
        max_anisotropy: f32,
        compare_op: CompareOp,
        min_lod: f32,
        max_lod: f32,
        ea: EnableAnisotropy,
        ec: EnableCompare,
    ) -> Self {
        let inner = SDL_GPUSamplerCreateInfo {
            min_filter: SDL_GPUFilter::new(min_filter as _),
            mag_filter: SDL_GPUFilter::new(mag_filter as _),
            mipmap_mode: SDL_GPUSamplerMipmapMode::new(mipmap_mode as _),
            address_mode_u: SDL_GPUSamplerAddressMode::new(u as _),
            address_mode_v: SDL_GPUSamplerAddressMode::new(v as _),
            address_mode_w: SDL_GPUSamplerAddressMode::new(w as _),
            mip_lod_bias,
            max_anisotropy,
            compare_op: SDL_GPUCompareOp::new(compare_op as _),
            min_lod,
            max_lod,
            enable_anisotropy: ea.into(),
            enable_compare: ec.into(),
            props: SDL_PropertiesID::new(0),
            ..Default::default()
        };
        Self(inner)
    }
}

resource_new_no_drop!(SDL_GPUSampler, Sampler);
impl Sampler {
    #[doc(alias = "SDL_CreateGPUSampler")]
    pub fn new(device: Ref<Device>, create_info: &SamplerCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUSampler(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUSampler")]
    pub fn drop(self, device: Ref<Device>) {
        unsafe { SDL_ReleaseGPUSampler(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}
