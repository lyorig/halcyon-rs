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

use crate::{defs::SdlResult, renderer::RendererRef, resource, surface::SurfaceRef};

resource!(Texture, TextureRef, SDL_Texture, SDL_DestroyTexture);

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
    pub fn rgb_mod(&self) -> (u8, u8, u8) {
        let mut ret = MaybeUninit::<(u8, u8, u8)>::uninit();
        let ptr = ret.as_mut_ptr();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureColorMod(
                self.handle.as_ptr(),
                &raw mut (*ptr).0,
                &raw mut (*ptr).1,
                &raw mut (*ptr).2,
            );

            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetTextureColorModFloat")]
    pub fn rgb_mod_float(&self) -> (f32, f32, f32) {
        let mut ret = MaybeUninit::<(f32, f32, f32)>::uninit();
        let ptr = ret.as_mut_ptr();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureColorModFloat(
                self.handle.as_ptr(),
                &raw mut (*ptr).0,
                &raw mut (*ptr).1,
                &raw mut (*ptr).2,
            );

            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetTextureAlphaMod")]
    pub fn alpha_mod(&self) -> u8 {
        let mut ret = MaybeUninit::<u8>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureAlphaMod(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetTextureAlphaModFloat")]
    pub fn alpha_mod_float(&self) -> f32 {
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
    pub fn color_mod(&mut self) -> (u8, u8, u8, u8) {
        let (r, g, b) = self.rgb_mod();
        let a = self.alpha_mod();

        (r, g, b, a)
    }

    /// Convenience function for `self.rgb_mod_float()` and `self.alpha_mod_float()`.
    /// SDL, for some reason, provides these two functions separately instead of
    /// providing a unified function using `SDL_Color`.
    pub fn color_mod_float(&mut self) -> (f32, f32, f32, f32) {
        let (r, g, b) = self.rgb_mod_float();
        let a = self.alpha_mod_float();

        (r, g, b, a)
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
    pub fn set_rgb_mod(&mut self, (r, g, b): (u8, u8, u8)) {
        unsafe {
            SDL_SetTextureColorMod(self.handle.as_ptr(), r, g, b);
        }
    }

    #[doc(alias = "SDL_SetTextureColorModFloat")]
    pub fn set_rgb_mod_float(&mut self, (r, g, b): (f32, f32, f32)) {
        unsafe {
            SDL_SetTextureColorModFloat(self.handle.as_ptr(), r, g, b);
        }
    }

    #[doc(alias = "SDL_SetTextureAlphaMod")]
    pub fn set_alpha_mod(&mut self, alpha: u8) {
        unsafe {
            SDL_SetTextureAlphaMod(self.handle.as_ptr(), alpha);
        }
    }

    #[doc(alias = "SDL_SetTextureAlphaModFloat")]
    pub fn set_alpha_mod_float(&mut self, alpha: f32) {
        unsafe {
            SDL_SetTextureAlphaModFloat(self.handle.as_ptr(), alpha);
        }
    }

    /// Convenience function for `self.set_rgb_mod()` and `self.set_alpha_mod()`.
    /// SDL, for some reason, provides these two functions separately instead of
    /// providing a unified function using `SDL_Color`.
    pub fn set_color_mod(&mut self, (r, g, b, a): (u8, u8, u8, u8)) {
        self.set_rgb_mod((r, g, b));
        self.set_alpha_mod(a);
    }

    /// Convenience function for `self.set_rgb_mod_float()` and `self.set_alpha_mod_float()`.
    /// SDL, for some reason, provides these two functions separately instead of
    /// providing a unified function using `SDL_Color`.
    pub fn set_color_mod_float(&mut self, (r, g, b, a): (f32, f32, f32, f32)) {
        self.set_rgb_mod_float((r, g, b));
        self.set_alpha_mod_float(a);
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
        (w, h): (i32, i32),
    ) -> SdlResult<Texture> {
        Self::from_ptr(unsafe { SDL_CreateTexture(rnd.handle.as_ptr(), fmt, access, w, h) })
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
