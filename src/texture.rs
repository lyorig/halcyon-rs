use std::{ffi::CString, mem::MaybeUninit};

use sdl3_sys::{
    pixels::SDL_PixelFormat,
    render::{
        SDL_CreateTexture, SDL_CreateTextureFromSurface, SDL_GetTextureSize, SDL_Texture,
        SDL_TextureAccess,
    },
};

use crate::{error, renderer::Renderer, surface::Surface};

pub struct Texture {
    pub(crate) internal: *mut SDL_Texture,
}

impl Texture {
    pub fn new(
        rnd: &Renderer,
        fmt: SDL_PixelFormat,
        access: SDL_TextureAccess,
        size: (i32, i32),
    ) -> Result<Self, CString> {
        Self::ctor(unsafe { SDL_CreateTexture(rnd.internal, fmt, access, size.0, size.1) })
    }

    pub fn from_surface(rnd: &Renderer, surf: &Surface) -> Result<Self, CString> {
        Self::ctor(unsafe { SDL_CreateTextureFromSurface(rnd.internal, surf.internal) })
    }

    fn ctor(internal: *mut SDL_Texture) -> Result<Self, CString> {
        if internal.is_null() {
            Err(error::get())
        } else {
            Ok(Self { internal })
        }
    }

    pub fn size(&self) -> (f32, f32) {
        debug_assert!(!self.internal.is_null());

        let mut ret = (MaybeUninit::uninit(), MaybeUninit::uninit());

        unsafe {
            assert!(SDL_GetTextureSize(
                self.internal,
                ret.0.as_mut_ptr(),
                ret.1.as_mut_ptr()
            ));

            (ret.0.assume_init(), ret.1.assume_init())
        }
    }
}
