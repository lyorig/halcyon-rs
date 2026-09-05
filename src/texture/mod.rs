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
    pixels::{SDL_Colorspace, SDL_PixelFormat},
    render::*,
    surface::SDL_ScaleMode,
};

use crate::{
    Result,
    color::{RgbF32, RgbU8},
    impl_enum_transmute, mod_reexport,
    pixels::{BlendMode, ScaleMode},
    properties::{Properties, PropertiesHandle},
    rect::{PointF32, PointI32},
    renderer::{Renderer, RendererHandle},
    resource::Ref,
    resource_new,
    surface::Surface,
    traits,
};

mod_reexport!(builder);
mod_reexport!(properties);

/// Pixel format.
///
/// # Remarks
///
/// SDL's pixel formats have the following naming convention:
///
/// - Names with a list of components and a single bit count, such as `RGB24`
///   and `ABGR32`, define a platform-independent encoding into bytes in the
///   order specified. For example, in `RGB24` data, each pixel is encoded in
///   3 bytes (red, green, blue) in that order, and in `ABGR32` data, each
///   pixel is encoded in 4 bytes (alpha, blue, green, red) in that order.
///   Use these names if the property of a format that is important to you is
///   the order of the bytes in memory or on disk.
/// - Names with a bit count per component, such as `ARGB8888` and
///   `XRGB1555`, are "packed" into an appropriately-sized integer in the
///   platform's native endianness. For example, `ARGB8888` is a sequence of
///   32-bit integers; in each integer, the most significant bits are alpha,
///   and the least significant bits are blue. On a little-endian CPU such as
///   x86, the least significant bits of each integer are arranged first in
///   memory, but on a big-endian CPU such as s390x, the most significant
///   bits are arranged first. Use these names if the property of a format
///   that is important to you is the meaning of each bit position within a
///   native-endianness integer.
/// - In indexed formats such as `INDEX4LSB`, each pixel is represented by
///   encoding an index into the palette into the indicated number of bits,
///   with multiple pixels packed into each byte if appropriate. In LSB
///   formats, the first (leftmost) pixel is stored in the least-significant
///   bits of the byte; in MSB formats, it's stored in the most-significant
///   bits. `INDEX8` does not need LSB/MSB variants, because each pixel
///   exactly fills one byte.
///
/// The 32-bit byte-array encodings such as [`PixelFormat::RGBA32`] are
/// aliases for the appropriate 8888 encoding for the current platform. For
/// example, `RGBA32` is an alias for `ABGR8888` on little-endian CPUs like
/// x86, or an alias for `RGBA8888` on big-endian CPUs.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "SDL_PixelFormat")]
pub enum PixelFormat {
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

impl PixelFormat {
    /// Alias for the appropriate RGBA 8888 encoding of color data for the
    /// current platform's endianness.
    pub const RGBA32: Self = Self::from_sdl(SDL_PixelFormat::RGBA8888);
    /// Alias for the appropriate ARGB 8888 encoding of color data for the
    /// current platform's endianness.
    pub const ARGB32: Self = Self::from_sdl(SDL_PixelFormat::ARGB8888);
    /// Alias for the appropriate BGRA 8888 encoding of color data for the
    /// current platform's endianness.
    pub const BGRA32: Self = Self::from_sdl(SDL_PixelFormat::BGRA8888);
    /// Alias for the appropriate ABGR 8888 encoding of color data for the
    /// current platform's endianness.
    pub const ABGR32: Self = Self::from_sdl(SDL_PixelFormat::ABGR8888);
    /// Alias for the appropriate RGBX 8888 encoding of color data for the
    /// current platform's endianness.
    pub const RGBX32: Self = Self::from_sdl(SDL_PixelFormat::RGBX8888);
    /// Alias for the appropriate XRGB 8888 encoding of color data for the
    /// current platform's endianness.
    pub const XRGB32: Self = Self::from_sdl(SDL_PixelFormat::XRGB8888);
    /// Alias for the appropriate BGRX 8888 encoding of color data for the
    /// current platform's endianness.
    pub const BGRX32: Self = Self::from_sdl(SDL_PixelFormat::BGRX8888);
    /// Alias for the appropriate XBGR 8888 encoding of color data for the
    /// current platform's endianness.
    pub const XBGR32: Self = Self::from_sdl(SDL_PixelFormat::XBGR8888);
}

/// Colorspace definitions.
///
/// # Remarks
///
/// Since similar colorspaces may vary in their details (matrix, transfer
/// function, etc.), this is not an exhaustive list, but rather a
/// representative sample of the kinds of colorspaces supported in SDL.
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "SDL_Colorspace")]
pub enum Colorspace {
    /// A gamma corrected colorspace, and the default colorspace for SDL
    /// rendering and 8-bit RGB surfaces.
    ///
    /// Equivalent to `DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709`.
    Srgb = SDL_Colorspace::SRGB.0,
    /// A linear colorspace and the default colorspace for floating point
    /// surfaces. On Windows this is the scRGB colorspace, and on Apple
    /// platforms this is `kCGColorSpaceExtendedLinearSRGB` for EDR content.
    ///
    /// Equivalent to `DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709`.
    SrgbLinear = SDL_Colorspace::SRGB_LINEAR.0,
    /// A non-linear HDR colorspace and the default colorspace for 10-bit
    /// surfaces.
    ///
    /// Equivalent to `DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020`.
    Hdr10 = SDL_Colorspace::HDR10.0,
    /// Equivalent to `DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601`.
    Jpeg = SDL_Colorspace::JPEG.0,
    /// Equivalent to `DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601`.
    Bt601Limited = SDL_Colorspace::BT601_LIMITED.0,
    /// Equivalent to `DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601`.
    Bt601Full = SDL_Colorspace::BT601_FULL.0,
    /// Equivalent to `DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709`.
    Bt709Limited = SDL_Colorspace::BT709_LIMITED.0,
    /// Equivalent to `DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709`.
    Bt709Full = SDL_Colorspace::BT709_FULL.0,
    /// Equivalent to `DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020`.
    Bt2020Limited = SDL_Colorspace::BT2020_LIMITED.0,
    /// Equivalent to `DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020`.
    Bt2020Full = SDL_Colorspace::BT2020_FULL.0,
}

impl Colorspace {
    /// The default colorspace for RGB surfaces if no colorspace is specified.
    pub const RGB_DEFAULT: Self = Self::from_sdl(SDL_Colorspace::RGB_DEFAULT);
    /// The default colorspace for YUV surfaces if no colorspace is specified.
    pub const YUV_DEFAULT: Self = Self::from_sdl(SDL_Colorspace::YUV_DEFAULT);
}

/// The access pattern allowed for a texture.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "SDL_TextureAccess")]
pub enum TextureAccess {
    /// Changes rarely, not lockable.
    Static = SDL_TextureAccess::STATIC.0,
    /// Changes frequently, lockable.
    Streaming = SDL_TextureAccess::STREAMING.0,
    /// Texture can be used as a render target.
    Target = SDL_TextureAccess::TARGET.0,
}

impl_enum_transmute!(SDL_PixelFormat, PixelFormat);
impl_enum_transmute!(SDL_Colorspace, Colorspace);
impl_enum_transmute!(SDL_TextureAccess, TextureAccess);

resource_new!(SDL_Texture, Texture, SDL_DestroyTexture);

impl TextureHandle {
    /// Get the size of a texture, as floating point values.
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

    /// Get the scale mode used for texture scale operations.
    #[doc(alias = "SDL_GetTextureScaleMode")]
    pub fn scale_mode(&self) -> ScaleMode {
        let mut ret = MaybeUninit::<SDL_ScaleMode>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureScaleMode(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init().into()
        }
    }

    /// Get the renderer that created a texture.
    ///
    /// Returns [`None`] on failure.
    #[doc(alias = "SDL_GetRendererFromTexture")]
    pub fn renderer(&self) -> Option<RendererHandle> {
        RendererHandle::from_ptr(unsafe { SDL_GetRendererFromTexture(self.handle.as_ptr()) })
    }

    /// Set the scale mode used for texture scale operations.
    ///
    /// # Remarks
    ///
    /// The default texture scale mode is `SDL_SCALEMODE_LINEAR`.
    ///
    /// If the scale mode is not supported, the closest supported mode is
    /// chosen.
    #[doc(alias = "SDL_SetTextureScaleMode")]
    pub fn set_scale_mode(&mut self, sm: ScaleMode) {
        unsafe {
            SDL_SetTextureScaleMode(self.handle.as_ptr(), sm.into());
        }
    }
}

impl traits::BlendMode for TextureHandle {
    /// Get the blend mode used for texture copy operations.
    #[doc(alias = "SDL_GetTextureBlendMode")]
    fn blend_mode(&self) -> BlendMode {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            SDL_GetTextureBlendMode(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init().into()
        }
    }

    /// Set the blend mode for a texture, used by
    /// [`RendererHandle::draw`] and its variants.
    ///
    /// # Remarks
    ///
    /// If the blend mode is not supported, the closest supported mode is
    /// chosen.
    #[doc(alias = "SDL_SetTextureBlendMode")]
    fn set_blend_mode(&self, bm: BlendMode) {
        unsafe {
            SDL_SetTextureBlendMode(self.handle.as_ptr(), bm.into());
        }
    }
}

impl traits::ColorModU8 for TextureHandle {
    /// Get the additional color value multiplied into render copy
    /// operations.
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

    /// Get the additional alpha value multiplied into render copy
    /// operations.
    #[doc(alias = "SDL_GetTextureAlphaMod")]
    fn alpha_mod_u8(&self) -> u8 {
        let mut ret = MaybeUninit::<u8>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureAlphaMod(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    /// Set an additional color value multiplied into render copy
    /// operations.
    ///
    /// # Remarks
    ///
    /// When this texture is rendered, during the copy operation each source
    /// color channel is modulated by the appropriate color value according
    /// to the following formula:
    ///
    /// `srcC = srcC * (color / 255)`
    ///
    /// Color modulation is not always supported by the renderer.
    #[doc(alias = "SDL_SetTextureColorMod")]
    fn set_rgb_mod_u8(&self, rm: RgbU8) {
        unsafe {
            SDL_SetTextureColorMod(self.handle.as_ptr(), rm.r, rm.g, rm.b);
        }
    }

    /// Set an additional alpha value multiplied into render copy
    /// operations.
    ///
    /// # Remarks
    ///
    /// When this texture is rendered, during the copy operation the source
    /// alpha value is modulated by this alpha value according to the
    /// following formula:
    ///
    /// `srcA = srcA * (alpha / 255)`
    ///
    /// Alpha modulation is not always supported by the renderer.
    #[doc(alias = "SDL_SetTextureAlphaMod")]
    fn set_alpha_mod_u8(&self, am: u8) {
        unsafe {
            SDL_SetTextureAlphaMod(self.handle.as_ptr(), am);
        }
    }
}

impl traits::ColorModF32 for TextureHandle {
    /// Get the additional color value multiplied into render copy
    /// operations.
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

    /// Get the additional alpha value multiplied into render copy
    /// operations.
    #[doc(alias = "SDL_GetTextureAlphaModFloat")]
    fn alpha_mod_f32(&self) -> f32 {
        let mut ret = MaybeUninit::<f32>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureAlphaModFloat(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    /// Set an additional color value multiplied into render copy
    /// operations.
    ///
    /// # Remarks
    ///
    /// When this texture is rendered, during the copy operation each source
    /// color channel is modulated by the appropriate color value according
    /// to the following formula:
    ///
    /// `srcC = srcC * color`
    ///
    /// Color modulation is not always supported by the renderer.
    #[doc(alias = "SDL_SetTextureColorModFloat")]
    fn set_rgb_mod_f32(&self, rm: RgbF32) {
        unsafe {
            SDL_SetTextureColorModFloat(self.handle.as_ptr(), rm.r, rm.g, rm.b);
        }
    }

    /// Set an additional alpha value multiplied into render copy
    /// operations.
    ///
    /// # Remarks
    ///
    /// When this texture is rendered, during the copy operation the source
    /// alpha value is modulated by this alpha value according to the
    /// following formula:
    ///
    /// `srcA = srcA * alpha`
    ///
    /// Alpha modulation is not always supported by the renderer.
    #[doc(alias = "SDL_SetTextureAlphaModFloat")]
    fn set_alpha_mod_f32(&self, am: f32) {
        unsafe {
            SDL_SetTextureAlphaModFloat(self.handle.as_ptr(), am);
        }
    }
}

impl TextureHandle {
    /// Get the properties associated with a texture.
    ///
    /// Read-only properties of this texture, as documented by
    /// [`SDL_GetTextureProperties`](https://wiki.libsdl.org/SDL3/SDL_GetTextureProperties).
    ///
    /// Covers the generic properties plus the D3D11, D3D12, OpenGL, Vulkan
    /// and GPU backends. Not covered: the Metal and OpenGLES2 backends, the
    /// plane-specific texture pointers, the OpenGL texture target, and
    /// `SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT`/`TEX_H_FLOAT`.
    #[doc(alias = "SDL_GetTextureProperties")]
    pub fn properties(&self) -> TextureProperties<'_> {
        unsafe {
            let id = SDL_GetTextureProperties(self.handle.as_ptr());
            let handle = PropertiesHandle::from_id(id).unwrap_unchecked();
            let r = Ref::from_handle(handle);

            TextureProperties::new(r)
        }
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

    /// Create a texture for a rendering context.
    ///
    /// `fmt` is the pixel format of the texture, `access` its allowed access
    /// pattern, and `size` its width and height in pixels.
    ///
    /// # Remarks
    ///
    /// The contents of a texture when first created are not defined.
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

    /// Create a texture from an existing surface.
    ///
    /// # Remarks
    ///
    /// The surface is not modified or freed by this function.
    ///
    /// The access hint for the created texture is
    /// [`TextureAccess::Static`].
    ///
    /// The pixel format of the created texture may be different from the
    /// pixel format of the surface, and can be queried using the
    /// `SDL_PROP_TEXTURE_FORMAT_NUMBER` property (see
    /// [`TextureHandle::properties`]).
    #[doc(alias = "SDL_CreateTextureFromSurface")]
    pub fn from_surface(rnd: Ref<Renderer>, surf: Ref<Surface>) -> Result<Texture> {
        Self::from_ptr(unsafe {
            SDL_CreateTextureFromSurface(rnd.handle.as_ptr(), surf.handle.as_ptr())
        })
    }
}
