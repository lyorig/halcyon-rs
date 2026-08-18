use std::ffi::{CStr, c_char, c_int, c_void};

use sdl3_sys::{
    pixels::{SDL_Colorspace, SDL_PixelFormat},
    render::*,
};

use crate::{
    properties::{Properties, PropertiesHandle},
    resource::Ref,
    texture::{Colorspace, PixelFormat, TextureAccess},
};

/// Read-only properties of a texture, as documented by
/// [`SDL_GetTextureProperties`](https://wiki.libsdl.org/SDL3/SDL_GetTextureProperties).
///
/// Generic properties are returned bare since the docs guarantee their
/// existence; backend properties are returned as `Option` since they only
/// exist on their respective backends.
///
/// Covers the D3D11, D3D12, OpenGL, Vulkan and GPU backends. Not covered:
/// the Metal and OpenGLES2 backends, the plane-specific texture pointers,
/// the OpenGL texture target, and `SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT` /
/// `TEX_H_FLOAT`.
#[derive(Clone, Copy)]
pub struct TextureProperties<'a> {
    inner: Ref<'a, Properties>,
}

impl<'a> TextureProperties<'a> {
    pub(super) fn new(inner: Ref<'a, Properties>) -> Self {
        Self { inner }
    }

    fn opt_number(&self, key: *const c_char) -> Option<i64> {
        let cstr = unsafe { CStr::from_ptr(key) };
        self.inner.has(cstr).then(|| self.inner.number(cstr, 0))
    }

    fn opt_ptr(&self, key: *const c_char) -> Option<*mut c_void> {
        let cstr = unsafe { CStr::from_ptr(key) };
        let p = self.inner.pointer(cstr, std::ptr::null_mut());

        (!p.is_null()).then_some(p)
    }

    pub fn colorspace(&self) -> Colorspace {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_TEXTURE_COLORSPACE_NUMBER) };
        SDL_Colorspace(self.inner.number(cstr, 0) as u32).into()
    }

    pub fn format(&self) -> PixelFormat {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_TEXTURE_FORMAT_NUMBER) };
        SDL_PixelFormat(self.inner.number(cstr, 0) as c_int).into()
    }

    pub fn access(&self) -> TextureAccess {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_TEXTURE_ACCESS_NUMBER) };
        SDL_TextureAccess(self.inner.number(cstr, 0) as c_int).into()
    }

    pub fn width(&self) -> i64 {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_TEXTURE_WIDTH_NUMBER) };
        self.inner.number(cstr, 0)
    }

    pub fn height(&self) -> i64 {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_TEXTURE_HEIGHT_NUMBER) };
        self.inner.number(cstr, 0)
    }

    pub fn sdr_white_point(&self) -> f32 {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT) };
        self.inner.float(cstr, 0.)
    }

    pub fn hdr_headroom(&self) -> f32 {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT) };
        self.inner.float(cstr, 0.)
    }

    pub fn d3d11_texture(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER)
    }

    pub fn d3d12_texture(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER)
    }

    pub fn opengl_texture(&self) -> Option<i64> {
        self.opt_number(SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER)
    }

    pub fn vulkan_texture(&self) -> Option<i64> {
        self.opt_number(SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER)
    }

    pub fn gpu_texture(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_TEXTURE_GPU_TEXTURE_POINTER)
    }
}

impl std::ops::Deref for TextureProperties<'_> {
    type Target = PropertiesHandle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
