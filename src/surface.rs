use std::ffi::CString;

use crate::{color::Color, coord, error, util};

use sdl3_sys::{
    pixels::SDL_PixelFormat,
    rect::SDL_Rect,
    surface::{
        SDL_ClearSurface, SDL_CreateSurface, SDL_DestroySurface, SDL_FillSurfaceRect,
        SDL_MapSurfaceRGBA, SDL_ScaleMode, SDL_ScaleSurface, SDL_Surface,
    },
};

pub struct Surface {
    pub(crate) internal: *mut SDL_Surface,
}

impl Surface {
    pub fn from_size_and_format(
        size: (coord::Pixel, coord::Pixel),
        format: SDL_PixelFormat,
    ) -> Result<Self, CString> {
        Self::ctor(unsafe { SDL_CreateSurface(size.0, size.1, format) })
    }

    fn ctor(internal: *mut SDL_Surface) -> Result<Self, CString> {
        if internal.is_null() {
            Err(error::get())
        } else {
            Ok(Self { internal })
        }
    }

    pub fn size(&self) -> (coord::Pixel, coord::Pixel) {
        debug_assert!(!self.internal.is_null()); // Shouldn't happen.

        let ligma = unsafe { self.internal.as_ref() }.unwrap();

        (ligma.w, ligma.h)
    }

    pub fn format(&self) -> SDL_PixelFormat {
        debug_assert!(!self.internal.is_null()); // Shouldn't happen.

        let ligma = unsafe { self.internal.as_ref() }.unwrap();

        ligma.format
    }

    pub fn fill(&self, area: Option<&SDL_Rect>, c: Color) -> Result<(), CString> {
        crate::util::btur(unsafe {
            SDL_FillSurfaceRect(self.internal, util::opt2ptr(area), self.map_color(c))
        })
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) -> Result<(), CString> {
        crate::util::btur(unsafe { SDL_ClearSurface(self.internal, r, g, b, a) })
    }

    pub fn scale(&self, size: (i32, i32), sm: SDL_ScaleMode) -> Result<Self, CString> {
        Self::ctor(unsafe { SDL_ScaleSurface(self.internal, size.0, size.1, sm) })
    }

    fn map_color(&self, c: Color) -> u32 {
        unsafe { SDL_MapSurfaceRGBA(self.internal, c.r, c.g, c.b, c.a) }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroySurface(self.internal);
        }
    }
}
