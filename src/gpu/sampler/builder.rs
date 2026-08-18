use std::ffi::{CStr, c_char};

use sdl3_sys::gpu::*;

use crate::{
    Result,
    gpu::{Device, EnableAnisotropy, EnableCompare},
    properties::Properties,
    resource::Ref,
};

use super::{AddressMode, CompareOp, Filter, MipmapMode, Sampler, SamplerCreateInfo};

const CREATE_PROPERTIES: [*const c_char; 1] = [SDL_PROP_GPU_SAMPLER_CREATE_NAME_STRING];

/// Builder for [`SamplerCreateInfo`] properties.
#[derive(Clone, Copy)]
pub struct SamplerCreateInfoBuilder<'p> {
    props: Ref<'p, Properties>,
}

impl<'p> SamplerCreateInfoBuilder<'p> {
    pub(super) fn new(props: Ref<'p, Properties>) -> Self {
        Self { props }
    }

    /// A name for the sampler, used for debugging.
    #[doc(alias = "SDL_PROP_GPU_SAMPLER_CREATE_NAME_STRING")]
    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let key = unsafe { CStr::from_ptr(SDL_PROP_GPU_SAMPLER_CREATE_NAME_STRING) };
        _ = self.props.set_string(key, Some(value));
        self
    }

    /// Clear all GPU sampler creation properties from a property group.
    pub fn clear_from(props: Ref<Properties>) {
        for key in CREATE_PROPERTIES {
            _ = props.clear(unsafe { CStr::from_ptr(key) });
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn build(
        &self,
        min_filter: Filter,
        mag_filter: Filter,
        mipmap_mode: MipmapMode,
        address_mode: (AddressMode, AddressMode, AddressMode),
        mip_lod_bias: f32,
        max_anisotropy: f32,
        compare_op: CompareOp,
        min_lod: f32,
        max_lod: f32,
        enable_anisotropy: EnableAnisotropy,
        enable_compare: EnableCompare,
    ) -> SamplerCreateInfo<'p> {
        SamplerCreateInfo::new_with_props(
            min_filter,
            mag_filter,
            mipmap_mode,
            address_mode,
            mip_lod_bias,
            max_anisotropy,
            compare_op,
            min_lod,
            max_lod,
            enable_anisotropy,
            enable_compare,
            self.props,
        )
    }

    /// Creates a [`Sampler`] using [`SamplerCreateInfo`], then removes all
    /// sampler creation properties from the attached property group.
    #[allow(clippy::too_many_arguments)]
    pub fn build_cleanup(
        &self,
        device: Ref<Device>,
        create_info: &SamplerCreateInfo<'p>,
    ) -> Result<Sampler> {
        let result = Sampler::new(device, create_info);
        Self::clear_from(self.props);
        result
    }
}
