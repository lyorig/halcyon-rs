//! SDL's 2D texture API wrapper.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryRender)):
//! - [x] SDL_CreateTexture
//! - [x] SDL_CreateTextureFromSurface
//! - [x] SDL_CreateTextureWithProperties
//! - [x] SDL_DestroyTexture
//! - [ ] SDL_GetDefaultTextureScaleMode
//! - [x] SDL_GetTextureAlphaMod
//! - [x] SDL_GetTextureAlphaModFloat
//! - [x] SDL_GetTextureBlendMode
//! - [x] SDL_GetTextureColorMod
//! - [x] SDL_GetTextureColorModFloat
//! - [x] SDL_GetTextureProperties
//! - [x] SDL_GetTextureScaleMode
//! - [x] SDL_GetTextureSize
//! - [ ] SDL_LockTexture
//! - [ ] SDL_LockTextureToSurface
//! - [ ] SDL_SetDefaultTextureScaleMode
//! - [x] SDL_SetTextureAlphaMod
//! - [x] SDL_SetTextureAlphaModFloat
//! - [x] SDL_SetTextureBlendMode
//! - [x] SDL_SetTextureColorMod
//! - [x] SDL_SetTextureColorModFloat
//! - [x] SDL_SetTextureScaleMode
//! - [ ] SDL_UnlockTexture
//! - [ ] SDL_UpdateNVTexture
//! - [ ] SDL_UpdateTexture
//! - [ ] SDL_UpdateYUVTexture
//! - [x] SDL_GetRendererFromTexture

use std::mem::MaybeUninit;

use sdl3_sys::{
    blendmode::SDL_BlendMode,
    pixels::{SDL_Colorspace, SDL_PixelFormat},
    render::*,
    surface::SDL_ScaleMode,
};

use crate::{
    Result,
    color::{RgbF32, RgbU8},
    mod_reexport,
    properties::{Properties, PropertiesHandle},
    rect::{PointF32, PointI32},
    renderer::{Renderer, RendererHandle},
    resource::Ref,
    resource_new,
    surface::Surface,
    traits::{BlendMode, ColorModF32, ColorModU8},
};

mod_reexport!(builder);
mod_reexport!(properties);

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "SDL_PixelFormat")]
pub enum PixelFormat {
    Unknown = SDL_PixelFormat::UNKNOWN.0,
    Index1Lsb = SDL_PixelFormat::INDEX1LSB.0,
    Index1Msb = SDL_PixelFormat::INDEX1MSB.0,
    Index2Lsb = SDL_PixelFormat::INDEX2LSB.0,
    Index2Msb = SDL_PixelFormat::INDEX2MSB.0,
    Index4Lsb = SDL_PixelFormat::INDEX4LSB.0,
    Index4Msb = SDL_PixelFormat::INDEX4MSB.0,
    Index8 = SDL_PixelFormat::INDEX8.0,
    Rgb332 = SDL_PixelFormat::RGB332.0,
    Xrgb4444 = SDL_PixelFormat::XRGB4444.0,
    Xbgr4444 = SDL_PixelFormat::XBGR4444.0,
    Xrgb1555 = SDL_PixelFormat::XRGB1555.0,
    Xbgr1555 = SDL_PixelFormat::XBGR1555.0,
    Argb4444 = SDL_PixelFormat::ARGB4444.0,
    Rgba4444 = SDL_PixelFormat::RGBA4444.0,
    Abgr4444 = SDL_PixelFormat::ABGR4444.0,
    Bgra4444 = SDL_PixelFormat::BGRA4444.0,
    Argb1555 = SDL_PixelFormat::ARGB1555.0,
    Rgba5551 = SDL_PixelFormat::RGBA5551.0,
    Abgr1555 = SDL_PixelFormat::ABGR1555.0,
    Bgra5551 = SDL_PixelFormat::BGRA5551.0,
    Rgb565 = SDL_PixelFormat::RGB565.0,
    Bgr565 = SDL_PixelFormat::BGR565.0,
    Rgb24 = SDL_PixelFormat::RGB24.0,
    Bgr24 = SDL_PixelFormat::BGR24.0,
    Xrgb8888 = SDL_PixelFormat::XRGB8888.0,
    Rgbx8888 = SDL_PixelFormat::RGBX8888.0,
    Xbgr8888 = SDL_PixelFormat::XBGR8888.0,
    Bgrx8888 = SDL_PixelFormat::BGRX8888.0,
    Argb8888 = SDL_PixelFormat::ARGB8888.0,
    Rgba8888 = SDL_PixelFormat::RGBA8888.0,
    Abgr8888 = SDL_PixelFormat::ABGR8888.0,
    Bgra8888 = SDL_PixelFormat::BGRA8888.0,
    Xrgb2101010 = SDL_PixelFormat::XRGB2101010.0,
    Xbgr2101010 = SDL_PixelFormat::XBGR2101010.0,
    Argb2101010 = SDL_PixelFormat::ARGB2101010.0,
    Abgr2101010 = SDL_PixelFormat::ABGR2101010.0,
    Rgb48 = SDL_PixelFormat::RGB48.0,
    Bgr48 = SDL_PixelFormat::BGR48.0,
    Rgba64 = SDL_PixelFormat::RGBA64.0,
    Argb64 = SDL_PixelFormat::ARGB64.0,
    Bgra64 = SDL_PixelFormat::BGRA64.0,
    Abgr64 = SDL_PixelFormat::ABGR64.0,
    Rgb48Float = SDL_PixelFormat::RGB48_FLOAT.0,
    Bgr48Float = SDL_PixelFormat::BGR48_FLOAT.0,
    Rgba64Float = SDL_PixelFormat::RGBA64_FLOAT.0,
    Argb64Float = SDL_PixelFormat::ARGB64_FLOAT.0,
    Bgra64Float = SDL_PixelFormat::BGRA64_FLOAT.0,
    Abgr64Float = SDL_PixelFormat::ABGR64_FLOAT.0,
    Rgb96Float = SDL_PixelFormat::RGB96_FLOAT.0,
    Bgr96Float = SDL_PixelFormat::BGR96_FLOAT.0,
    Rgba128Float = SDL_PixelFormat::RGBA128_FLOAT.0,
    Argb128Float = SDL_PixelFormat::ARGB128_FLOAT.0,
    Bgra128Float = SDL_PixelFormat::BGRA128_FLOAT.0,
    Abgr128Float = SDL_PixelFormat::ABGR128_FLOAT.0,
    Yv12 = SDL_PixelFormat::YV12.0,
    Iyuv = SDL_PixelFormat::IYUV.0,
    Yuy2 = SDL_PixelFormat::YUY2.0,
    Uyvy = SDL_PixelFormat::UYVY.0,
    Yvyu = SDL_PixelFormat::YVYU.0,
    Nv12 = SDL_PixelFormat::NV12.0,
    Nv21 = SDL_PixelFormat::NV21.0,
    P010 = SDL_PixelFormat::P010.0,
    ExternalOes = SDL_PixelFormat::EXTERNAL_OES.0,
    Mjpg = SDL_PixelFormat::MJPG.0,
}

impl From<PixelFormat> for SDL_PixelFormat {
    fn from(value: PixelFormat) -> Self {
        Self::new(value as _)
    }
}

impl From<SDL_PixelFormat> for PixelFormat {
    fn from(value: SDL_PixelFormat) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "SDL_Colorspace")]
pub enum Colorspace {
    Unknown = SDL_Colorspace::UNKNOWN.0,
    Srgb = SDL_Colorspace::SRGB.0,
    SrgbLinear = SDL_Colorspace::SRGB_LINEAR.0,
    Hdr10 = SDL_Colorspace::HDR10.0,
    Jpeg = SDL_Colorspace::JPEG.0,
    Bt601Limited = SDL_Colorspace::BT601_LIMITED.0,
    Bt601Full = SDL_Colorspace::BT601_FULL.0,
    Bt709Limited = SDL_Colorspace::BT709_LIMITED.0,
    Bt709Full = SDL_Colorspace::BT709_FULL.0,
    Bt2020Limited = SDL_Colorspace::BT2020_LIMITED.0,
    Bt2020Full = SDL_Colorspace::BT2020_FULL.0,
}

impl From<Colorspace> for SDL_Colorspace {
    fn from(value: Colorspace) -> Self {
        Self::new(value as _)
    }
}

impl From<SDL_Colorspace> for Colorspace {
    fn from(value: SDL_Colorspace) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "SDL_TextureAccess")]
pub enum TextureAccess {
    Static = SDL_TextureAccess::STATIC.0,
    Streaming = SDL_TextureAccess::STREAMING.0,
    Target = SDL_TextureAccess::TARGET.0,
}

impl From<TextureAccess> for SDL_TextureAccess {
    fn from(value: TextureAccess) -> Self {
        Self::new(value as _)
    }
}

impl From<SDL_TextureAccess> for TextureAccess {
    fn from(value: SDL_TextureAccess) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

resource_new!(SDL_Texture, Texture, SDL_DestroyTexture);

impl TextureHandle {
    #[doc(alias = "SDL_GetTextureSize")]
    pub fn size(&self) -> PointF32 {
        let mut ret = MaybeUninit::<PointF32>::uninit();
        let ptr = ret.as_mut_ptr();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureSize(self.handle.as_ptr(), &raw mut (*ptr).x, &raw mut (*ptr).y);
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetTextureBlendMode")]
    pub fn blend_mode(&self) -> SDL_BlendMode {
        let mut ret = MaybeUninit::<SDL_BlendMode>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureBlendMode(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetTextureScaleMode")]
    pub fn scale_mode(&self) -> SDL_ScaleMode {
        let mut ret = MaybeUninit::<SDL_ScaleMode>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureScaleMode(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetRendererFromTexture")]
    pub fn renderer(&self) -> Option<RendererHandle> {
        RendererHandle::from_ptr(unsafe { SDL_GetRendererFromTexture(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_SetTextureBlendMode")]
    pub fn set_blend_mode(&mut self, bm: SDL_BlendMode) {
        unsafe {
            SDL_SetTextureBlendMode(self.handle.as_ptr(), bm);
        }
    }

    #[doc(alias = "SDL_SetTextureScaleMode")]
    pub fn set_scale_mode(&mut self, sm: SDL_ScaleMode) {
        unsafe {
            SDL_SetTextureScaleMode(self.handle.as_ptr(), sm);
        }
    }
}

impl BlendMode for TextureHandle {
    fn blend_mode(&self) -> SDL_BlendMode {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            SDL_GetTextureBlendMode(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    fn set_blend_mode(&self, bm: SDL_BlendMode) {
        unsafe {
            SDL_SetTextureBlendMode(self.handle.as_ptr(), bm);
        }
    }
}

impl ColorModU8 for TextureHandle {
    #[doc(alias = "SDL_GetTextureColorMod")]
    fn rgb_mod_u8(&self) -> RgbU8 {
        let mut ret = MaybeUninit::<RgbU8>::uninit();
        let ptr = ret.as_mut_ptr();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureColorMod(
                self.handle.as_ptr(),
                &raw mut (*ptr).r,
                &raw mut (*ptr).g,
                &raw mut (*ptr).b,
            );

            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetTextureAlphaMod")]
    fn alpha_mod_u8(&self) -> u8 {
        let mut ret = MaybeUninit::<u8>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureAlphaMod(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_SetTextureColorMod")]
    fn set_rgb_mod_u8(&self, rm: RgbU8) {
        unsafe {
            SDL_SetTextureColorMod(self.handle.as_ptr(), rm.r, rm.g, rm.b);
        }
    }

    #[doc(alias = "SDL_SetTextureAlphaMod")]
    fn set_alpha_mod_u8(&self, am: u8) {
        unsafe {
            SDL_SetTextureAlphaMod(self.handle.as_ptr(), am);
        }
    }
}

impl ColorModF32 for TextureHandle {
    #[doc(alias = "SDL_GetTextureColorModFloat")]
    fn rgb_mod_f32(&self) -> RgbF32 {
        let mut ret = MaybeUninit::<RgbF32>::uninit();
        let ptr = ret.as_mut_ptr();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureColorModFloat(
                self.handle.as_ptr(),
                &raw mut (*ptr).r,
                &raw mut (*ptr).g,
                &raw mut (*ptr).b,
            );

            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetTextureAlphaModFloat")]
    fn alpha_mod_f32(&self) -> f32 {
        let mut ret = MaybeUninit::<f32>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureAlphaModFloat(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_SetTextureColorModFloat")]
    fn set_rgb_mod_f32(&self, rm: RgbF32) {
        unsafe {
            SDL_SetTextureColorModFloat(self.handle.as_ptr(), rm.r, rm.g, rm.b);
        }
    }

    #[doc(alias = "SDL_SetTextureAlphaModFloat")]
    fn set_alpha_mod_f32(&self, am: f32) {
        unsafe {
            SDL_SetTextureAlphaModFloat(self.handle.as_ptr(), am);
        }
    }
}

impl TextureHandle {
    /// Read-only properties of this texture, as documented by
    /// [`SDL_GetTextureProperties`](https://wiki.libsdl.org/SDL3/SDL_GetTextureProperties).
    ///
    /// Covers the generic properties plus the D3D11, D3D12, OpenGL, Vulkan
    /// and GPU backends. Not covered: the Metal and OpenGLES2 backends, the
    /// plane-specific texture pointers, the OpenGL texture target, and
    /// `SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT`/`TEX_H_FLOAT`.
    #[doc(alias = "SDL_GetTextureProperties")]
    pub fn properties(&self) -> TextureProperties<'_> {
        let id = unsafe { SDL_GetTextureProperties(self.handle.as_ptr()) };
        let handle = PropertiesHandle::from_id(id).expect("A valid texture should have properties");

        let r = unsafe { Ref::from_handle(handle) };
        TextureProperties::new(r)
    }
}

impl Texture {
    /// Bind the builder to a renderer and an existing property group.
    ///
    /// Unlike the window, renderer and GPU device builders, the renderer is
    /// a required parameter here, since `SDL_CreateTextureWithProperties`
    /// takes it directly.
    ///
    /// A single [`Properties`] can be shared between the window, renderer,
    /// GPU device and texture builders, since their creation properties
    /// (`SDL_PROP_WINDOW_CREATE_*`, `SDL_PROP_RENDERER_CREATE_*`,
    /// `SDL_PROP_GPU_DEVICE_CREATE_*`, `SDL_PROP_TEXTURE_CREATE_*`) never
    /// collide with each other.
    pub fn builder<'a>(
        renderer: Ref<'a, Renderer>,
        props: Ref<'a, Properties>,
    ) -> TextureBuilder<'a> {
        TextureBuilder::new(renderer, props)
    }

    #[doc(alias = "SDL_CreateTexture")]
    pub fn new(
        rnd: Ref<Renderer>,
        fmt: PixelFormat,
        access: TextureAccess,
        size: PointI32,
    ) -> Result<Texture> {
        Self::from_ptr(unsafe {
            SDL_CreateTexture(
                rnd.handle.as_ptr(),
                fmt.into(),
                access.into(),
                size.x,
                size.y,
            )
        })
    }

    #[doc(alias = "SDL_CreateTextureFromSurface")]
    pub fn from_surface(rnd: Ref<Renderer>, surf: Ref<Surface>) -> Result<Texture> {
        Self::from_ptr(unsafe {
            SDL_CreateTextureFromSurface(rnd.handle.as_ptr(), surf.handle.as_ptr())
        })
    }
}
