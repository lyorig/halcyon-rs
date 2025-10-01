//! SDL's 2D texture API wrapper.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryRender)):
//! - [x] SDL_CreateTexture
//! - [x] SDL_CreateTextureFromSurface
//! - [ ] SDL_CreateTextureWithProperties
//! - [x] SDL_DestroyTexture
//! - [ ] SDL_GetDefaultTextureScaleMode
//! - [x] SDL_GetTextureAlphaMod
//! - [x] SDL_GetTextureAlphaModFloat
//! - [x] SDL_GetTextureBlendMode
//! - [x] SDL_GetTextureColorMod
//! - [x] SDL_GetTextureColorModFloat
//! - [ ] SDL_GetTextureProperties
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
    blendmode::SDL_BlendMode, pixels::SDL_PixelFormat, render::*, surface::SDL_ScaleMode,
};

use crate::{
    color::{RgbF32, RgbU8, Rgba, RgbaF32, RgbaU8},
    defs::SdlResult,
    rect::PointI32,
    renderer::RendererRef,
    resource,
    surface::SurfaceRef,
};

resource!(Texture);

impl TextureRef {
    #[doc(alias = "SDL_GetTextureSize")]
    pub fn size(&self) -> (f32, f32) {
        let mut ret = MaybeUninit::<(f32, f32)>::uninit();
        let ptr = ret.as_mut_ptr();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureSize(self.handle.as_ptr(), &raw mut (*ptr).0, &raw mut (*ptr).1);
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetTextureColorMod")]
    pub fn rgb_mod_u8(&self) -> RgbU8 {
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

    #[doc(alias = "SDL_GetTextureColorModFloat")]
    pub fn rgb_mod_f32(&self) -> RgbF32 {
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

    #[doc(alias = "SDL_GetTextureAlphaMod")]
    pub fn alpha_mod_u8(&self) -> u8 {
        let mut ret = MaybeUninit::<u8>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureAlphaMod(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetTextureAlphaModFloat")]
    pub fn alpha_mod_f32(&self) -> f32 {
        let mut ret = MaybeUninit::<f32>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureAlphaModFloat(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    /// Convenience function for `self.rgb_mod()` and `self.alpha_mod()`.
    /// SDL, for some reason, provides these two functions separately instead of
    /// providing a unified function using `SDL_Color`.
    pub fn color_mod(&mut self) -> RgbaU8 {
        Rgba::new(self.rgb_mod_u8(), self.alpha_mod_u8())
    }

    /// Convenience function for `self.rgb_mod_float()` and `self.alpha_mod_float()`.
    /// SDL, for some reason, provides these two functions separately instead of
    /// providing a unified function using `SDL_Color`.
    pub fn color_mod_float(&mut self) -> RgbaF32 {
        Rgba::new(self.rgb_mod_f32(), self.alpha_mod_f32())
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
    pub fn renderer(&self) -> Option<RendererRef> {
        RendererRef::from_ptr(unsafe { SDL_GetRendererFromTexture(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_SetTextureColorMod")]
    pub fn set_rgb_mod_u8(&mut self, rgb: RgbU8) {
        unsafe {
            SDL_SetTextureColorMod(self.handle.as_ptr(), rgb.r, rgb.g, rgb.b);
        }
    }

    #[doc(alias = "SDL_SetTextureColorModFloat")]
    pub fn set_rgb_mod_f32(&mut self, rgb: RgbF32) {
        unsafe {
            SDL_SetTextureColorModFloat(self.handle.as_ptr(), rgb.r, rgb.g, rgb.b);
        }
    }

    #[doc(alias = "SDL_SetTextureAlphaMod")]
    pub fn set_alpha_mod_u8(&mut self, alpha: u8) {
        unsafe {
            SDL_SetTextureAlphaMod(self.handle.as_ptr(), alpha);
        }
    }

    #[doc(alias = "SDL_SetTextureAlphaModFloat")]
    pub fn set_alpha_mod_f32(&mut self, alpha: f32) {
        unsafe {
            SDL_SetTextureAlphaModFloat(self.handle.as_ptr(), alpha);
        }
    }

    /// Convenience function for `self.set_rgb_mod()` and `self.set_alpha_mod()`.
    /// SDL, for some reason, provides these two functions separately instead of
    /// providing a unified function using `SDL_Color`.
    pub fn set_color_mod_u8(&mut self, color: RgbaU8) {
        self.set_rgb_mod_u8(color.rgb);
        self.set_alpha_mod_u8(color.a);
    }

    /// Convenience function for `self.set_rgb_mod_float()` and `self.set_alpha_mod_float()`.
    /// SDL, for some reason, provides these two functions separately instead of
    /// providing a unified function using `SDL_Color`.
    pub fn set_color_mod_f32(&mut self, rgba: RgbaF32) {
        self.set_rgb_mod_f32(rgba.rgb);
        self.set_alpha_mod_f32(rgba.a);
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

impl Texture {
    #[doc(alias = "SDL_CreateTexture")]
    pub fn new(
        rnd: RendererRef,
        fmt: SDL_PixelFormat,
        access: SDL_TextureAccess,
        size: PointI32,
    ) -> SdlResult<Texture> {
        Self::from_ptr(unsafe {
            SDL_CreateTexture(rnd.handle.as_ptr(), fmt, access, size.x, size.y)
        })
    }

    #[doc(alias = "SDL_CreateTextureFromSurface")]
    pub fn from_surface(
        rnd: impl Into<RendererRef>,
        surf: impl Into<SurfaceRef>,
    ) -> SdlResult<Texture> {
        Self::from_ptr(unsafe {
            SDL_CreateTextureFromSurface(rnd.into().handle.as_ptr(), surf.into().handle.as_ptr())
        })
    }
}
