//! SDL_ttf wrapper.
//!
//! Implementation checklist:
//! - [ ] TTF_AddFallbackFont
//! - [ ] TTF_ClearFallbackFonts
//! - [x] TTF_CloseFont
//! - [x] TTF_CopyFont
//! - [ ] TTF_FontHasGlyph
//! - [x] TTF_FontIsFixedWidth
//! - [x] TTF_FontIsScalable
//! - [ ] TTF_GetFontAscent
//! - [ ] TTF_GetFontCharSpacing
//! - [ ] TTF_GetFontDescent
//! - [ ] TTF_GetFontDirection
//! - [ ] TTF_GetFontDPI
//! - [x] TTF_GetFontFamilyName
//! - [ ] TTF_GetFontGeneration
//! - [ ] TTF_GetFontHeight
//! - [ ] TTF_GetFontHinting
//! - [ ] TTF_GetFontKerning
//! - [ ] TTF_GetFontLineSkip
//! - [ ] TTF_GetFontOutline
//! - [ ] TTF_GetFontProperties
//! - [ ] TTF_GetFontScript
//! - [ ] TTF_GetFontSDF
//! - [ ] TTF_GetFontSize
//! - [ ] TTF_GetFontStyle
//! - [ ] TTF_GetFontStyleName
//! - [ ] TTF_GetFontWeight
//! - [ ] TTF_GetFontWrapAlignment
//! - [ ] TTF_GetFreeTypeVersion
//! - [ ] TTF_GetGlyphImage
//! - [ ] TTF_GetGlyphImageForIndex
//! - [ ] TTF_GetGlyphKerning
//! - [ ] TTF_GetGlyphMetrics
//! - [ ] TTF_GetGlyphScript
//! - [ ] TTF_GetHarfBuzzVersion
//! - [ ] TTF_GetNumFontFaces
//! - [ ] TTF_GetStringSize
//! - [ ] TTF_GetStringSizeWrapped
//! - [x] TTF_Init
//! - [ ] TTF_MeasureString
//! - [x] TTF_OpenFont
//! - [ ] TTF_OpenFontIO
//! - [ ] TTF_OpenFontWithProperties
//! - [x] TTF_Quit
//! - [ ] TTF_RemoveFallbackFont
//! - [x] TTF_RenderGlyph_Blended
//! - [x] TTF_RenderGlyph_LCD
//! - [x] TTF_RenderGlyph_Shaded
//! - [x] TTF_RenderGlyph_Solid
//! - [x] TTF_RenderText_Blended
//! - [x] TTF_RenderText_Blended_Wrapped
//! - [x] TTF_RenderText_LCD
//! - [x] TTF_RenderText_LCD_Wrapped
//! - [x] TTF_RenderText_Shaded
//! - [x] TTF_RenderText_Shaded_Wrapped
//! - [x] TTF_RenderText_Solid
//! - [x] TTF_RenderText_Solid_Wrapped
//! - [ ] TTF_SetFontCharSpacing
//! - [ ] TTF_SetFontDirection
//! - [ ] TTF_SetFontHinting
//! - [ ] TTF_SetFontKerning
//! - [ ] TTF_SetFontLanguage
//! - [ ] TTF_SetFontLineSkip
//! - [ ] TTF_SetFontOutline
//! - [ ] TTF_SetFontScript
//! - [ ] TTF_SetFontSDF
//! - [ ] TTF_SetFontSize
//! - [ ] TTF_SetFontSizeDPI
//! - [ ] TTF_SetFontStyle
//! - [ ] TTF_SetFontWrapAlignment
//! - [ ] TTF_StringToTag
//! - [ ] TTF_TagToString
//! - [x] TTF_Version
//! - [x] TTF_WasInit

mod_reexport!(engine);
mod_reexport!(font);
mod_reexport!(rt_str);
mod_reexport!(text);

use std::ffi::CStr;

use sdl3_ttf_sys::ttf::*;

use crate::{Result, error::Error, mod_reexport};

#[doc(alias = "TTF_WasInit")]
pub fn num_init() -> i32 {
    unsafe { TTF_WasInit() }
}

#[doc(alias = "TTF_WasInit")]
pub fn is_init() -> bool {
    num_init() != 0
}

#[doc(alias = "TTF_Version")]
pub fn version() -> i32 {
    TTF_Version()
}

/// Ensures SDL_ttf (de)initialization.
pub struct Context;

impl Context {
    #[doc(alias = "TTF_Init")]
    pub fn new() -> Result<Self> {
        if unsafe { TTF_Init() } {
            Ok(Self {})
        } else {
            Err(Error::current())
        }
    }

    #[doc(alias = "TTF_OpenFont")]
    pub fn open(&self, file: &CStr, point_size: f32) -> Result<Font<'_>> {
        unsafe { Font::new_unchecked(file, point_size) }
    }
}

impl Drop for Context {
    #[doc(alias = "TTF_Quit")]
    fn drop(&mut self) {
        unsafe { TTF_Quit() };
    }
}
