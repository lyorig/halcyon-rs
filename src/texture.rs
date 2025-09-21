use std::mem::MaybeUninit;

use sdl3_sys::{
    pixels::SDL_PixelFormat,
    render::{
        SDL_CreateTexture, SDL_CreateTextureFromSurface, SDL_DestroyTexture, SDL_GetTextureSize,
        SDL_Texture, SDL_TextureAccess,
    },
};

use crate::{coord::Pixel, defs::SdlResult, renderer::RendererRef, resource, surface::SurfaceRef};

resource!(Texture, TextureRef, SDL_Texture, SDL_DestroyTexture);

impl TextureRef {
    #[doc(alias = "SDL_GetTextureSize")]
    pub fn size(&self) -> (f32, f32) {
        let mut ret = (MaybeUninit::uninit(), MaybeUninit::uninit());

        unsafe {
            SDL_GetTextureSize(self.handle.as_ptr(), ret.0.as_mut_ptr(), ret.1.as_mut_ptr());

            (ret.0.assume_init(), ret.1.assume_init())
        }
    }
}

impl Texture {
    #[doc(alias = "SDL_CreateTexture")]
    pub fn new(
        rnd: RendererRef,
        fmt: SDL_PixelFormat,
        access: SDL_TextureAccess,
        size: (Pixel, Pixel),
    ) -> SdlResult<Texture> {
        Self::from_ptr(unsafe {
            SDL_CreateTexture(rnd.handle.as_ptr(), fmt, access, size.0, size.1)
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
