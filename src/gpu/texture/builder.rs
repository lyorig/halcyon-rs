use std::ffi::{CStr, c_char};

use sdl3_sys::gpu::*;

use crate::{Result, color::RgbaF32, gpu::*, properties::Properties, resource::Ref};

const CREATE_PROPERTIES: [*const c_char; 7] = [
    SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_R_FLOAT,
    SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_G_FLOAT,
    SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_B_FLOAT,
    SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_A_FLOAT,
    SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_DEPTH_FLOAT,
    SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_STENCIL_NUMBER,
    SDL_PROP_GPU_TEXTURE_CREATE_NAME_STRING,
];

/// Extends [`TextureCreateInfo`] with optional properties from [`SDL_GPUTextureCreateInfo`].
#[derive(Clone, Copy)]
pub struct TextureBuilder<'p> {
    props: Ref<'p, Properties>,
}

impl<'p> TextureBuilder<'p> {
    pub(super) fn new(props: Ref<'p, Properties>) -> Self {
        Self { props }
    }

    /// The clear color's red component for D3D12 render targets.
    #[doc(alias = "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_R_FLOAT")]
    pub fn d3d12_clear_r(&mut self, value: f32) -> &mut Self {
        self.set_float(SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_R_FLOAT, value)
    }

    /// The clear color's green component for D3D12 render targets.
    #[doc(alias = "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_G_FLOAT")]
    pub fn d3d12_clear_g(&mut self, value: f32) -> &mut Self {
        self.set_float(SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_G_FLOAT, value)
    }

    /// The clear color's blue component for D3D12 render targets.
    #[doc(alias = "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_B_FLOAT")]
    pub fn d3d12_clear_b(&mut self, value: f32) -> &mut Self {
        self.set_float(SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_B_FLOAT, value)
    }

    /// The clear color's alpha component for D3D12 render targets.
    #[doc(alias = "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_A_FLOAT")]
    pub fn d3d12_clear_a(&mut self, value: f32) -> &mut Self {
        self.set_float(SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_A_FLOAT, value)
    }

    /// Shorthand setter for the following properties:
    /// - [`TextureBuilder::d3d12_clear_r`]
    /// - [`TextureBuilder::d3d12_clear_g`]
    /// - [`TextureBuilder::d3d12_clear_b`]
    /// - [`TextureBuilder::d3d12_clear_a`]
    pub fn d3d12_clear_rgba(&mut self, value: RgbaF32) -> &mut Self {
        self.d3d12_clear_r(value.rgb.r);
        self.d3d12_clear_g(value.rgb.g);
        self.d3d12_clear_b(value.rgb.b);
        self.d3d12_clear_a(value.a)
    }

    /// The clear value for D3D12 depth targets.
    #[doc(alias = "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_DEPTH_FLOAT")]
    pub fn d3d12_clear_depth(&mut self, value: f32) -> &mut Self {
        self.set_float(SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_DEPTH_FLOAT, value)
    }

    /// The clear value for D3D12 stencil targets.
    #[doc(alias = "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_STENCIL_NUMBER")]
    pub fn d3d12_clear_stencil(&mut self, value: i64) -> &mut Self {
        self.set_number(
            SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_STENCIL_NUMBER,
            value,
        )
    }

    /// A name for the texture, used for debugging.
    #[doc(alias = "SDL_PROP_GPU_TEXTURE_CREATE_NAME_STRING")]
    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_GPU_TEXTURE_CREATE_NAME_STRING) };
        _ = self.props.set_string(cstr, Some(value));
        self
    }

    /// Clear all GPU texture creation properties from a property group.
    pub fn clear_from(props: Ref<Properties>) {
        for key in CREATE_PROPERTIES {
            let cstr = unsafe { CStr::from_ptr(key) };
            _ = props.clear(cstr);
        }
    }

    pub fn build(
        &self,
        device: Ref<Device>,
        mut create_info: TextureCreateInfo,
    ) -> Result<Texture> {
        create_info.0.props = self.props.id();
        Texture::new(device, &create_info)
    }

    pub fn build_cleanup(
        &self,
        device: Ref<Device>,
        create_info: TextureCreateInfo,
    ) -> Result<Texture> {
        let tex = self.build(device, create_info);
        Self::clear_from(self.props);
        tex
    }

    fn set_number(&mut self, key: *const c_char, value: i64) -> &mut Self {
        _ = self.props.set_number(unsafe { CStr::from_ptr(key) }, value);
        self
    }

    fn set_float(&mut self, key: *const c_char, value: f32) -> &mut Self {
        _ = self.props.set_float(unsafe { CStr::from_ptr(key) }, value);
        self
    }
}
