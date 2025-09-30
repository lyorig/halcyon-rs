//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategorySurface)):
//! - [ ] SDL_AddSurfaceAlternateImage
//! - [ ] SDL_BlitSurface
//! - [ ] SDL_BlitSurface9Grid
//! - [ ] SDL_BlitSurfaceScaled
//! - [ ] SDL_BlitSurfaceTiled
//! - [ ] SDL_BlitSurfaceTiledWithScale
//! - [ ] SDL_BlitSurfaceUnchecked
//! - [ ] SDL_BlitSurfaceUncheckedScaled
//! - [x] SDL_ClearSurface
//! - [ ] SDL_ConvertPixels
//! - [ ] SDL_ConvertPixelsAndColorspace
//! - [ ] SDL_ConvertSurface
//! - [ ] SDL_ConvertSurfaceAndColorspace
//! - [x] SDL_CreateSurface
//! - [ ] SDL_CreateSurfaceFrom
//! - [ ] SDL_CreateSurfacePalette
//! - [x] SDL_DestroySurface
//! - [x] SDL_DuplicateSurface
//! - [x] SDL_FillSurfaceRect
//! - [x] SDL_FillSurfaceRects
//! - [x] SDL_FlipSurface
//! - [ ] SDL_GetSurfaceAlphaMod
//! - [ ] SDL_GetSurfaceBlendMode
//! - [ ] SDL_GetSurfaceClipRect
//! - [ ] SDL_GetSurfaceColorKey
//! - [ ] SDL_GetSurfaceColorMod
//! - [ ] SDL_GetSurfaceColorspace
//! - [ ] SDL_GetSurfaceImages
//! - [ ] SDL_GetSurfacePalette
//! - [ ] SDL_GetSurfaceProperties
//! - [ ] SDL_LoadBMP
//! - [ ] SDL_LoadBMP_IO
//! - [ ] SDL_LockSurface
//! - [x] SDL_MapSurfaceRGB
//! - [x] SDL_MapSurfaceRGBA
//! - [ ] SDL_PremultiplyAlpha
//! - [ ] SDL_PremultiplySurfaceAlpha
//! - [ ] SDL_ReadSurfacePixel
//! - [ ] SDL_ReadSurfacePixelFloat
//! - [ ] SDL_RemoveSurfaceAlternateImages
//! - [ ] SDL_SaveBMP
//! - [ ] SDL_SaveBMP_IO
//! - [x] SDL_ScaleSurface
//! - [ ] SDL_SetSurfaceAlphaMod
//! - [ ] SDL_SetSurfaceBlendMode
//! - [ ] SDL_SetSurfaceClipRect
//! - [ ] SDL_SetSurfaceColorKey
//! - [ ] SDL_SetSurfaceColorMod
//! - [ ] SDL_SetSurfaceColorspace
//! - [ ] SDL_SetSurfacePalette
//! - [ ] SDL_SetSurfaceRLE
//! - [ ] SDL_StretchSurface
//! - [ ] SDL_SurfaceHasAlternateImages
//! - [ ] SDL_SurfaceHasColorKey
//! - [ ] SDL_SurfaceHasRLE
//! - [ ] SDL_UnlockSurface
//! - [ ] SDL_WriteSurfacePixel
//! - [ ] SDL_WriteSurfacePixelFloat

use crate::{
    color::{RgbU8, RgbaF32, RgbaU8},
    defs::SdlResult,
    rect::{PointI32, RectI32},
    resource,
    util::to_result,
};

use sdl3_sys::{pixels::SDL_PixelFormat, surface::*};

resource!(Surface);

impl SurfaceRef {
    pub fn size(&self) -> PointI32 {
        let ligma = unsafe { self.handle.as_ref() };

        PointI32::new(ligma.w, ligma.h)
    }

    pub fn format(&self) -> SDL_PixelFormat {
        let ligma = unsafe { self.handle.as_ref() };

        ligma.format
    }

    #[doc(alias = "SDL_FillSurfaceRect")]
    pub fn fill(&self, c: RgbaU8) -> SdlResult {
        to_result(unsafe {
            SDL_FillSurfaceRect(self.handle.as_ptr(), std::ptr::null(), self.map_rgba(c))
        })
    }

    #[doc(alias = "SDL_FillSurfaceRect")]
    pub fn fill_rect(&self, pos: RectI32, c: RgbaU8) -> SdlResult {
        to_result(unsafe {
            SDL_FillSurfaceRect(
                self.handle.as_ptr(),
                (&raw const pos).cast(),
                self.map_rgba(c),
            )
        })
    }

    #[doc(alias = "SDL_FillSurfaceRects")]
    pub fn fill_rects(&self, pos: &[RectI32], c: RgbaU8) -> SdlResult {
        to_result(unsafe {
            SDL_FillSurfaceRects(
                self.handle.as_ptr(),
                (&raw const pos).cast(),
                pos.len() as i32,
                self.map_rgba(c),
            )
        })
    }

    #[doc(alias = "SDL_ClearSurface")]
    pub fn clear(&self, c: RgbaF32) -> SdlResult {
        to_result(unsafe { SDL_ClearSurface(self.handle.as_ptr(), c.rgb.r, c.rgb.g, c.rgb.b, c.a) })
    }

    #[doc(alias = "SDL_FlipSurface")]
    pub fn flip(&self, fm: SDL_FlipMode) -> SdlResult {
        to_result(unsafe { SDL_FlipSurface(self.handle.as_ptr(), fm) })
    }

    #[doc(alias = "SDL_ScaleSurface")]
    pub fn scale(&self, size: PointI32, sm: SDL_ScaleMode) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe { SDL_ScaleSurface(self.handle.as_ptr(), size.x, size.y, sm) })
    }

    #[doc(alias = "SDL_MapSurfaceRGB")]
    fn map_rgb(&self, rgb: RgbU8) -> u32 {
        unsafe { SDL_MapSurfaceRGB(self.handle.as_ptr(), rgb.r, rgb.g, rgb.b) }
    }

    #[doc(alias = "SDL_MapSurfaceRGBA")]
    fn map_rgba(&self, rgba: RgbaU8) -> u32 {
        unsafe {
            SDL_MapSurfaceRGBA(
                self.handle.as_ptr(),
                rgba.rgb.r,
                rgba.rgb.g,
                rgba.rgb.b,
                rgba.a,
            )
        }
    }

    #[doc(alias = "SDL_DuplicateSurface")]
    fn try_clone(&self) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe { SDL_DuplicateSurface(self.handle.as_ptr()) })
    }
}

impl Surface {
    #[doc(alias = "SDL_CreateSurface")]
    pub fn from_size_and_format(size: PointI32, format: SDL_PixelFormat) -> SdlResult<Self> {
        Self::from_ptr(unsafe { SDL_CreateSurface(size.x, size.y, format) })
    }
}
