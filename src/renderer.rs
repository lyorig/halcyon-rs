use std::{
    ffi::{CStr, c_void},
    ptr::NonNull,
};

use sdl3_sys::{rect::SDL_FRect, render::*};

use crate::{
    defs::SdlResult,
    error,
    properties::Properties,
    surface::Surface,
    texture::Texture,
    util::{self, to_result},
    window::Window,
};

pub struct Builder {
    inner: Properties,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            inner: Properties::new(),
        }
    }

    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let _ = self
            .inner
            .set_string(SDL_PROP_RENDERER_CREATE_NAME_STRING, value);

        self
    }

    pub fn window(&mut self, value: &mut Window) -> &mut Self {
        let _ = self.inner.set_pointer(
            SDL_PROP_RENDERER_CREATE_WINDOW_POINTER,
            value.handle.as_ptr() as *mut c_void,
        );

        self
    }

    pub fn surface(&mut self, value: &mut Surface) -> &mut Self {
        let _ = self.inner.set_pointer(
            SDL_PROP_RENDERER_CREATE_SURFACE_POINTER,
            value.handle.as_ptr() as *mut c_void,
        );

        self
    }

    // TODO: Colorspace

    pub fn vsync(&mut self, value: i64) -> &mut Self {
        let _ = self
            .inner
            .set_number(SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER, value);

        self
    }

    pub fn build(&self) -> SdlResult<Renderer> {
        Renderer::from_ptr(unsafe { SDL_CreateRendererWithProperties(self.inner.id()) })
    }
}

pub struct Renderer {
    pub(crate) handle: NonNull<SDL_Renderer>,
}

impl Renderer {
    fn from_ptr(ptr: *mut SDL_Renderer) -> SdlResult<Self> {
        match NonNull::new(ptr) {
            None => Err(error::get()),
            Some(h) => Ok(Self { handle: h }),
        }
    }

    pub fn new(wnd: &Window) -> SdlResult<Self> {
        Self::from_ptr(unsafe { SDL_CreateRenderer(wnd.handle.as_ptr(), std::ptr::null()) })
    }

    pub fn clear(&self) -> SdlResult {
        to_result(unsafe { SDL_RenderClear(self.handle.as_ptr()) })
    }

    pub fn present(&self) -> SdlResult {
        to_result(unsafe { SDL_RenderPresent(self.handle.as_ptr()) })
    }

    pub fn draw(
        &self,
        tex: &Texture,
        src: Option<&SDL_FRect>,
        dst: Option<&SDL_FRect>,
    ) -> SdlResult {
        to_result(unsafe {
            SDL_RenderTexture(
                self.handle.as_ptr(),
                tex.handle.as_ptr(),
                util::opt2ptr(src),
                util::opt2ptr(dst),
            )
        })
    }
}
