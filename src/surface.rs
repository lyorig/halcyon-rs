use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{
    color::Color,
    coord,
    defs::SdlResult,
    error,
    util::{self, to_result},
};

use sdl3_sys::{pixels::SDL_PixelFormat, rect::SDL_Rect, surface::*};

#[derive(Clone, Copy)]
pub struct SurfaceRef {
    pub(crate) handle: NonNull<SDL_Surface>,
}

impl SurfaceRef {
    pub fn size(&self) -> (coord::Pixel, coord::Pixel) {
        let ligma = unsafe { self.handle.as_ref() };

        (ligma.w, ligma.h)
    }

    pub fn format(&self) -> SDL_PixelFormat {
        let ligma = unsafe { self.handle.as_ref() };

        ligma.format
    }

    #[doc(alias = "SDL_FillSurfaceRect")]
    pub fn fill(&self, area: Option<&SDL_Rect>, c: Color) -> SdlResult {
        to_result(unsafe {
            SDL_FillSurfaceRect(self.handle.as_ptr(), util::opt2ptr(area), self.map_color(c))
        })
    }

    #[doc(alias = "SDL_ClearSurface")]
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) -> SdlResult {
        to_result(unsafe { SDL_ClearSurface(self.handle.as_ptr(), r, g, b, a) })
    }

    #[doc(alias = "SDL_ScaleSurface")]
    pub fn scale(&self, size: (i32, i32), sm: SDL_ScaleMode) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe { SDL_ScaleSurface(self.handle.as_ptr(), size.0, size.1, sm) })
    }

    #[doc(alias = "SDL_MapSurfaceRGBA")]
    fn map_color(&self, c: Color) -> u32 {
        unsafe { SDL_MapSurfaceRGBA(self.handle.as_ptr(), c.r, c.g, c.b, c.a) }
    }
}

pub struct Surface {
    pub(crate) inner: SurfaceRef,
}

impl Surface {
    fn from_ptr(handle: *mut SDL_Surface) -> SdlResult<Self> {
        match NonNull::new(handle) {
            Some(handle) => Ok(Self {
                inner: SurfaceRef { handle },
            }),
            None => Err(error::get()),
        }
    }

    #[doc(alias = "SDL_CreateSurface")]
    pub fn from_size_and_format(
        size: (coord::Pixel, coord::Pixel),
        format: SDL_PixelFormat,
    ) -> SdlResult<Self> {
        Self::from_ptr(unsafe { SDL_CreateSurface(size.0, size.1, format) })
    }
}

impl Deref for Surface {
    type Target = SurfaceRef;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Surface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<&Surface> for SurfaceRef {
    fn from(value: &Surface) -> Self {
        value.inner
    }
}

impl Clone for Surface {
    #[doc(alias = "SDL_DuplicateSurface")]
    fn clone(&self) -> Self {
        Self::from_ptr(unsafe { SDL_DuplicateSurface(self.inner.handle.as_ptr()) })
            .expect("Failed to duplicate surface")
    }
}

impl Drop for Surface {
    #[doc(alias = "SDL_DestroySurface")]
    fn drop(&mut self) {
        unsafe {
            SDL_DestroySurface(self.inner.handle.as_ptr());
        }
    }
}
