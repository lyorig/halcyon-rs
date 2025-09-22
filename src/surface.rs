use crate::{
    color::Color,
    defs::SdlResult,
    resource,
    util::{self, to_result},
};

use sdl3_sys::{pixels::SDL_PixelFormat, rect::SDL_Rect, surface::*};

resource!(Surface, SurfaceRef, SDL_Surface, SDL_DestroySurface);

impl SurfaceRef {
    pub fn size(&self) -> (i32, i32) {
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

    #[doc(alias = "SDL_DuplicateSurface")]
    fn try_clone(&self) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe { SDL_DuplicateSurface(self.handle.as_ptr()) })
    }
}

impl Surface {
    #[doc(alias = "SDL_CreateSurface")]
    pub fn from_size_and_format(size: (i32, i32), format: SDL_PixelFormat) -> SdlResult<Self> {
        Self::from_ptr(unsafe { SDL_CreateSurface(size.0, size.1, format) })
    }
}
