use std::ffi::{CStr, c_char};

use sdl3_sys::gpu::*;

use crate::{Result, gpu::Device, properties::Properties, resource::Ref};

use super::{Shader, ShaderCreateInfo};

const CREATE_PROPERTIES: [*const c_char; 1] = [SDL_PROP_GPU_SHADER_CREATE_NAME_STRING];

/// Builder for [`ShaderCreateInfo`] properties.
#[derive(Clone, Copy)]
pub struct ShaderBuilder<'p> {
    props: Ref<'p, Properties>,
}

impl<'p> ShaderBuilder<'p> {
    pub(super) fn new(props: Ref<'p, Properties>) -> Self {
        Self { props }
    }

    /// A name for the shader, used for debugging.
    #[doc(alias = "SDL_PROP_GPU_SHADER_CREATE_NAME_STRING")]
    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let key = unsafe { CStr::from_ptr(SDL_PROP_GPU_SHADER_CREATE_NAME_STRING) };
        _ = self.props.set_string(key, Some(value));
        self
    }

    /// Clear all GPU shader creation properties from a property group.
    pub fn clear_from(props: Ref<Properties>) {
        for key in CREATE_PROPERTIES {
            _ = props.clear(unsafe { CStr::from_ptr(key) });
        }
    }

    pub fn build<'bc, 'ep>(
        &self,
        device: Ref<Device>,
        mut create_info: ShaderCreateInfo<'bc, 'ep>,
    ) -> Result<Shader> {
        create_info.0.props = self.props.id();
        Shader::new(device, &create_info)
    }

    /// Creates a [`Shader`] using [`ShaderCreateInfo`],
    /// then removes all shader creation properties from the attached property group.
    pub fn build_cleanup<'bc, 'ep>(
        &self,
        device: Ref<Device>,
        create_info: ShaderCreateInfo<'bc, 'ep>,
    ) -> Result<Shader> {
        let res = self.build(device, create_info);
        Self::clear_from(self.props);
        res
    }
}
