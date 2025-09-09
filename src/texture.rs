use std::{mem::MaybeUninit, ptr::NonNull};

use sdl3_sys::{
    pixels::SDL_PixelFormat,
    render::{
        SDL_CreateTexture, SDL_CreateTextureFromSurface, SDL_GetTextureSize, SDL_Texture,
        SDL_TextureAccess,
    },
};

use crate::{defs::SdlResult, error, renderer::Renderer, surface::Surface};

pub struct Texture {
    pub(crate) handle: NonNull<SDL_Texture>,
}

impl Texture {
    fn from_ptr(handle: *mut SDL_Texture) -> SdlResult<Self> {
        match NonNull::new(handle) {
            Some(handle) => Ok(Self { handle }),
            None => Err(error::get()),
        }
    }

    pub fn new(
        rnd: &Renderer,
        fmt: SDL_PixelFormat,
        access: SDL_TextureAccess,
        size: (i32, i32),
    ) -> SdlResult<Self> {
        Self::from_ptr(unsafe {
            SDL_CreateTexture(rnd.handle.as_ptr(), fmt, access, size.0, size.1)
        })
    }

    pub fn from_surface(rnd: &Renderer, surf: &Surface) -> SdlResult<Self> {
        Self::from_ptr(unsafe {
            SDL_CreateTextureFromSurface(rnd.handle.as_ptr(), surf.handle.as_ptr())
        })
    }

    pub fn size(&self) -> (f32, f32) {
        let mut ret = (MaybeUninit::uninit(), MaybeUninit::uninit());

        unsafe {
            SDL_GetTextureSize(self.handle.as_ptr(), ret.0.as_mut_ptr(), ret.1.as_mut_ptr());

            (ret.0.assume_init(), ret.1.assume_init())
        }
    }
}
