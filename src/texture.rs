//! SDL's 2D texture API wrapper.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryRender)):
//! - [x] SDL_CreateTexture
//! - [x] SDL_CreateTextureFromSurface
//! - [ ] SDL_CreateTextureWithProperties
//! - [x] SDL_DestroyTexture
//! - [ ] SDL_GetDefaultTextureScaleMode
//! - [x] SDL_GetTextureAlphaMod
//! - [ ] SDL_GetTextureAlphaModFloat
//! - [ ] SDL_GetTextureBlendMode
//! - [x] SDL_GetTextureColorMod
//! - [ ] SDL_GetTextureColorModFloat
//! - [ ] SDL_GetTextureProperties
//! - [ ] SDL_GetTextureScaleMode
//! - [x] SDL_GetTextureSize
//! - [ ] SDL_LockTexture
//! - [ ] SDL_LockTextureToSurface
//! - [ ] SDL_SetDefaultTextureScaleMode
//! - [ ] SDL_SetTextureAlphaMod
//! - [ ] SDL_SetTextureAlphaModFloat
//! - [ ] SDL_SetTextureBlendMode
//! - [ ] SDL_SetTextureColorMod
//! - [ ] SDL_SetTextureColorModFloat
//! - [ ] SDL_SetTextureScaleMode
//! - [ ] SDL_UnlockTexture
//! - [ ] SDL_UpdateNVTexture
//! - [ ] SDL_UpdateTexture
//! - [ ] SDL_UpdateYUVTexture

use std::mem::MaybeUninit;

use sdl3_sys::{pixels::SDL_PixelFormat, render::*};

use crate::{color::Color, defs::SdlResult, renderer::RendererRef, resource, surface::SurfaceRef};

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

    pub fn color_mod(&self) -> Color {
        let mut ret = MaybeUninit::<Color>::uninit();
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

    pub fn alpha_mod(&self) -> u8 {
        let mut ret = MaybeUninit::<u8>::uninit();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetTextureAlphaMod(self.handle.as_ptr(), ret.as_mut_ptr());

            ret.assume_init()
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
