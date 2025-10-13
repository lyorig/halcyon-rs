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
    color::{RgbF32, RgbU8},
    defs::SdlResult,
    rect::{PointF32, PointI32},
    renderer::RendererRef,
    resource,
    surface::SurfaceRef,
    traits::{BlendMode, ColorMod},
};

resource!(Texture);

impl TextureRef {
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
    pub fn renderer(&self) -> Option<RendererRef> {
        RendererRef::from_ptr(unsafe { SDL_GetRendererFromTexture(self.handle.as_ptr()) })
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

impl BlendMode for TextureRef {
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

impl ColorMod for TextureRef {
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

impl Texture {
    #[doc(alias = "SDL_CreateTexture")]
    pub fn new(
        rnd: impl Into<RendererRef>,
        fmt: SDL_PixelFormat,
        access: SDL_TextureAccess,
        size: PointI32,
    ) -> SdlResult<Texture> {
        Self::from_ptr(unsafe {
            SDL_CreateTexture(rnd.into().handle.as_ptr(), fmt, access, size.x, size.y)
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
