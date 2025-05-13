use std::ffi::CString;

use sdl3_sys::{
    rect::SDL_FRect,
    render::{
        SDL_CreateRenderer, SDL_RenderClear, SDL_RenderPresent, SDL_RenderTexture, SDL_Renderer,
    },
};

use crate::{error, texture::Texture, util, window::Window};

pub struct Renderer {
    pub(crate) internal: *mut SDL_Renderer,
}

impl Renderer {
    pub fn new(wnd: &Window) -> Result<Self, CString> {
        let internal = unsafe { SDL_CreateRenderer(wnd.internal, std::ptr::null()) };

        if internal.is_null() {
            Err(error::get())
        } else {
            Ok(Self { internal })
        }
    }

    pub fn clear(&self) -> Result<(), CString> {
        util::btur(unsafe { SDL_RenderClear(self.internal) })
    }

    pub fn present(&self) -> Result<(), CString> {
        util::btur(unsafe { SDL_RenderPresent(self.internal) })
    }

    pub fn draw(
        &self,
        tex: &Texture,
        src: Option<&SDL_FRect>,
        dst: Option<&SDL_FRect>,
    ) -> Result<(), CString> {
        crate::util::btur(unsafe {
            SDL_RenderTexture(
                self.internal,
                tex.internal,
                util::opt2ptr(src),
                util::opt2ptr(dst),
            )
        })
    }
}
