use std::ptr::NonNull;

use crate::{
    color::Color,
    coord,
    defs::SdlResult,
    error,
    util::{self, to_result},
};

use sdl3_sys::{pixels::SDL_PixelFormat, rect::SDL_Rect, surface::*};

pub struct Surface {
    pub(crate) handle: NonNull<SDL_Surface>,
}

impl Surface {
    fn from_ptr(handle: *mut SDL_Surface) -> SdlResult<Self> {
        match NonNull::new(handle) {
            Some(handle) => Ok(Self { handle }),
            None => Err(error::get()),
        }
    }

    pub fn from_size_and_format(
        size: (coord::Pixel, coord::Pixel),
        format: SDL_PixelFormat,
    ) -> SdlResult<Self> {
        Self::from_ptr(unsafe { SDL_CreateSurface(size.0, size.1, format) })
    }

    pub fn size(&self) -> (coord::Pixel, coord::Pixel) {
        let ligma = unsafe { self.handle.as_ref() };

        (ligma.w, ligma.h)
    }

    pub fn format(&self) -> SDL_PixelFormat {
        let ligma = unsafe { self.handle.as_ref() };

        ligma.format
    }

    pub fn fill(&self, area: Option<&SDL_Rect>, c: Color) -> SdlResult {
        to_result(unsafe {
            SDL_FillSurfaceRect(self.handle.as_ptr(), util::opt2ptr(area), self.map_color(c))
        })
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) -> SdlResult {
        to_result(unsafe { SDL_ClearSurface(self.handle.as_ptr(), r, g, b, a) })
    }

    pub fn scale(&self, size: (i32, i32), sm: SDL_ScaleMode) -> SdlResult<Self> {
        Self::from_ptr(unsafe { SDL_ScaleSurface(self.handle.as_ptr(), size.0, size.1, sm) })
    }

    fn map_color(&self, c: Color) -> u32 {
        unsafe { SDL_MapSurfaceRGBA(self.handle.as_ptr(), c.r, c.g, c.b, c.a) }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroySurface(self.handle.as_ptr());
        }
    }
}
