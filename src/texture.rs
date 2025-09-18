use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use sdl3_sys::{
    pixels::SDL_PixelFormat,
    render::{
        SDL_CreateTexture, SDL_CreateTextureFromSurface, SDL_DestroyTexture, SDL_GetTextureSize,
        SDL_Texture, SDL_TextureAccess,
    },
};

use crate::{coord::Pixel, defs::SdlResult, error, renderer::RendererRef, surface::SurfaceRef};

#[derive(Clone, Copy)]
pub struct TextureRef {
    pub(crate) handle: NonNull<SDL_Texture>,
}

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

pub struct Texture {
    inner: TextureRef,
}

impl Texture {
    fn from_ptr(handle: *mut SDL_Texture) -> SdlResult<Texture> {
        match NonNull::new(handle) {
            Some(handle) => Ok(Texture {
                inner: TextureRef { handle },
            }),
            None => Err(error::get()),
        }
    }

    #[doc(alias = "SDL_CreateTexture")]
    pub fn new(
        rnd: impl Into<RendererRef>,
        fmt: SDL_PixelFormat,
        access: SDL_TextureAccess,
        size: (Pixel, Pixel),
    ) -> SdlResult<Texture> {
        Self::from_ptr(unsafe {
            SDL_CreateTexture(rnd.into().handle.as_ptr(), fmt, access, size.0, size.1)
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

impl Deref for Texture {
    type Target = TextureRef;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Texture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<&Texture> for TextureRef {
    fn from(value: &Texture) -> Self {
        value.inner
    }
}

impl Drop for Texture {
    #[doc(alias = "SDL_DestroyTexture")]
    fn drop(&mut self) {
        unsafe { SDL_DestroyTexture(self.inner.handle.as_ptr()) }
    }
}
