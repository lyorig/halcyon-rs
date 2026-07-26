//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategorySurface)):
//! - [ ] SDL_AddSurfaceAlternateImage
//! - [x] SDL_BlitSurface
//! - [x] SDL_BlitSurface9Grid
//! - [x] SDL_BlitSurfaceScaled
//! - [x] SDL_BlitSurfaceTiled
//! - [x] SDL_BlitSurfaceTiledWithScale
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
//! - [x] SDL_GetSurfaceAlphaMod
//! - [x] SDL_GetSurfaceBlendMode
//! - [ ] SDL_GetSurfaceClipRect
//! - [ ] SDL_GetSurfaceColorKey
//! - [x] SDL_GetSurfaceColorMod
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
//! - [x] SDL_SetSurfaceAlphaMod
//! - [x] SDL_SetSurfaceBlendMode
//! - [ ] SDL_SetSurfaceClipRect
//! - [ ] SDL_SetSurfaceColorKey
//! - [x] SDL_SetSurfaceColorMod
//! - [ ] SDL_SetSurfaceColorspace
//! - [ ] SDL_SetSurfacePalette
//! - [ ] SDL_SetSurfaceRLE
//! - [x] SDL_StretchSurface
//! - [ ] SDL_SurfaceHasAlternateImages
//! - [ ] SDL_SurfaceHasColorKey
//! - [ ] SDL_SurfaceHasRLE
//! - [ ] SDL_UnlockSurface
//! - [ ] SDL_WriteSurfacePixel
//! - [ ] SDL_WriteSurfacePixelFloat

use std::mem::MaybeUninit;

use crate::{
    Result,
    color::{RgbU8, RgbaF32, RgbaU8},
    rect::{PointI32, RectI32},
    resource,
    traits::{BlendMode, ColorModU8, Ref},
    util::{opt2ptr, to_result},
};

use sdl3_sys::{blendmode::SDL_BlendMode, pixels::SDL_PixelFormat, surface::*};

resource!(Surface);

impl SurfaceHandle {
    pub fn size(&self) -> PointI32 {
        let ligma = unsafe { self.handle.as_ref() };
        PointI32::new(ligma.w, ligma.h)
    }

    pub fn format(&self) -> SDL_PixelFormat {
        let ligma = unsafe { self.handle.as_ref() };
        ligma.format
    }

    #[doc(alias = "SDL_GetSurfaceBlendMode")]
    pub fn blend_mode(&self) -> SDL_BlendMode {
        let mut ret = MaybeUninit::uninit();

        unsafe {
            SDL_GetSurfaceBlendMode(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_FillSurfaceRect")]
    pub fn fill(&self, c: RgbaU8) -> Result {
        to_result(unsafe {
            SDL_FillSurfaceRect(self.handle.as_ptr(), std::ptr::null(), self.map_rgba(c))
        })
    }

    #[doc(alias = "SDL_FillSurfaceRect")]
    pub fn fill_rect(&self, pos: RectI32, c: RgbaU8) -> Result {
        to_result(unsafe {
            SDL_FillSurfaceRect(
                self.handle.as_ptr(),
                (&raw const pos).cast(),
                self.map_rgba(c),
            )
        })
    }

    #[doc(alias = "SDL_FillSurfaceRects")]
    pub fn fill_rects(&self, pos: &[RectI32], c: RgbaU8) -> Result {
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
    pub fn clear(&self, c: RgbaF32) -> Result {
        to_result(unsafe { SDL_ClearSurface(self.handle.as_ptr(), c.rgb.r, c.rgb.g, c.rgb.b, c.a) })
    }

    #[doc(alias = "SDL_FlipSurface")]
    pub fn flip(&self, fm: SDL_FlipMode) -> Result {
        to_result(unsafe { SDL_FlipSurface(self.handle.as_ptr(), fm) })
    }

    #[doc(alias = "SDL_ScaleSurface")]
    pub fn scale(&self, size: PointI32, sm: SDL_ScaleMode) -> Result<Surface> {
        Surface::from_ptr(unsafe { SDL_ScaleSurface(self.handle.as_ptr(), size.x, size.y, sm) })
    }

    #[doc(alias = "SDL_MapSurfaceRGB")]
    pub fn map_rgb(&self, rgb: RgbU8) -> u32 {
        unsafe { SDL_MapSurfaceRGB(self.handle.as_ptr(), rgb.r, rgb.g, rgb.b) }
    }

    #[doc(alias = "SDL_MapSurfaceRGBA")]
    pub fn map_rgba(&self, rgba: RgbaU8) -> u32 {
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
    pub fn try_clone(&self) -> Result<Surface> {
        Surface::from_ptr(unsafe { SDL_DuplicateSurface(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_StretchSurface")]
    pub fn stretch(
        &self,
        target: Ref<Surface>,
        src: Option<&PointI32>,
        dst: Option<&PointI32>,
        scale_mode: SDL_ScaleMode,
    ) -> Result {
        to_result(unsafe {
            SDL_StretchSurface(
                self.handle.as_ptr(),
                opt2ptr(src).cast(),
                target.handle.as_ptr(),
                opt2ptr(dst).cast(),
                scale_mode,
            )
        })
    }

    #[doc(alias = "SDL_SetSurfaceBlendMode")]
    pub fn set_blend_mode(&self, bm: SDL_BlendMode) -> Result {
        to_result(unsafe { SDL_SetSurfaceBlendMode(self.handle.as_ptr(), bm) })
    }

    #[doc(alias = "SDL_BlitSurface")]
    pub fn blit(
        &self,
        target: Ref<Surface>,
        src: Option<&PointI32>,
        dst: Option<&PointI32>,
    ) -> Result {
        to_result(unsafe {
            SDL_BlitSurface(
                self.handle.as_ptr(),
                opt2ptr(src).cast(),
                target.handle.as_ptr(),
                opt2ptr(dst).cast(),
            )
        })
    }

    #[doc(alias = "SDL_BlitSurface9Grid")]
    pub fn blit_9grid(
        &self,
        target: Ref<Surface>,
        src: Option<&PointI32>,
        dst: Option<&PointI32>,
        (left_width, right_width, top_height, bottom_height): (i32, i32, i32, i32),
        scale: f32,
        scale_mode: SDL_ScaleMode,
    ) -> Result {
        to_result(unsafe {
            SDL_BlitSurface9Grid(
                self.handle.as_ptr(),
                opt2ptr(src).cast(),
                left_width,
                right_width,
                top_height,
                bottom_height,
                scale,
                scale_mode,
                target.handle.as_ptr(),
                opt2ptr(dst).cast(),
            )
        })
    }

    #[doc(alias = "SDL_BlitSurfaceScaled")]
    pub fn blit_scaled(
        &self,
        target: Ref<Surface>,
        src: Option<&PointI32>,
        dst: Option<&PointI32>,
        scale_mode: SDL_ScaleMode,
    ) -> Result {
        to_result(unsafe {
            SDL_BlitSurfaceScaled(
                self.handle.as_ptr(),
                opt2ptr(src).cast(),
                target.handle.as_ptr(),
                opt2ptr(dst).cast(),
                scale_mode,
            )
        })
    }

    #[doc(alias = "SDL_BlitSurfaceTiled")]
    pub fn blit_tiled(
        &self,
        target: Ref<Surface>,
        src: Option<&PointI32>,
        dst: Option<&PointI32>,
    ) -> Result {
        to_result(unsafe {
            SDL_BlitSurfaceTiled(
                self.handle.as_ptr(),
                opt2ptr(src).cast(),
                target.handle.as_ptr(),
                opt2ptr(dst).cast(),
            )
        })
    }

    #[doc(alias = "SDL_BlitSurfaceTiledWithScale")]
    pub fn blit_tiled_scaled(
        &self,
        target: Ref<Surface>,
        src: Option<&PointI32>,
        dst: Option<&PointI32>,
        scale: f32,
        scale_mode: SDL_ScaleMode,
    ) -> Result {
        to_result(unsafe {
            SDL_BlitSurfaceTiledWithScale(
                self.handle.as_ptr(),
                opt2ptr(src).cast(),
                scale,
                scale_mode,
                target.handle.as_ptr(),
                opt2ptr(dst).cast(),
            )
        })
    }
}

impl BlendMode for SurfaceHandle {
    fn blend_mode(&self) -> SDL_BlendMode {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            SDL_GetSurfaceBlendMode(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    fn set_blend_mode(&self, bm: SDL_BlendMode) {
        unsafe {
            SDL_SetSurfaceBlendMode(self.handle.as_ptr(), bm);
        }
    }
}

impl ColorModU8 for SurfaceHandle {
    #[doc(alias = "SDL_GetSurfaceColorMod")]
    fn rgb_mod_u8(&self) -> RgbU8 {
        let mut ret = MaybeUninit::<RgbU8>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_GetSurfaceColorMod(
                self.handle.as_ptr(),
                &raw mut (*ptr).r,
                &raw mut (*ptr).g,
                &raw mut (*ptr).b,
            );
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetSurfaceAlphaMod")]
    fn alpha_mod_u8(&self) -> u8 {
        let mut ret = MaybeUninit::uninit();

        unsafe {
            SDL_GetSurfaceAlphaMod(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_SetSurfaceColorMod")]
    fn set_rgb_mod_u8(&self, rm: RgbU8) {
        unsafe { SDL_SetSurfaceColorMod(self.handle.as_ptr(), rm.r, rm.g, rm.b) };
    }

    #[doc(alias = "SDL_SetSurfaceAlphaMod")]
    fn set_alpha_mod_u8(&self, am: u8) {
        unsafe { SDL_SetSurfaceAlphaMod(self.handle.as_ptr(), am) };
    }
}

impl Surface {
    #[doc(alias = "SDL_CreateSurface")]
    pub fn from_size_and_format(size: PointI32, format: SDL_PixelFormat) -> Result<Self> {
        Self::from_ptr(unsafe { SDL_CreateSurface(size.x, size.y, format) })
    }
}
