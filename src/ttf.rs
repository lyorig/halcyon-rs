//! SDL_ttf wrapper.
//!
//! Implementation checklist:
//! - [ ] TTF_AddFallbackFont
//! - [ ] TTF_AppendTextString
//! - [ ] TTF_ClearFallbackFonts
//! - [x] TTF_CloseFont
//! - [x] TTF_CopyFont
//! - [ ] TTF_CreateGPUTextEngine
//! - [ ] TTF_CreateGPUTextEngineWithProperties
//! - [ ] TTF_CreateRendererTextEngine
//! - [ ] TTF_CreateRendererTextEngineWithProperties
//! - [ ] TTF_CreateSurfaceTextEngine
//! - [x] TTF_CreateText
//! - [ ] TTF_DeleteTextString
//! - [ ] TTF_DestroyGPUTextEngine
//! - [ ] TTF_DestroyRendererTextEngine
//! - [ ] TTF_DestroySurfaceTextEngine
//! - [x] TTF_DestroyText
//! - [ ] TTF_DrawRendererText
//! - [ ] TTF_DrawSurfaceText
//! - [ ] TTF_FontHasGlyph
//! - [x] TTF_FontIsFixedWidth
//! - [ ] TTF_FontIsScalable
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
//! - [ ] TTF_GetGPUTextDrawData
//! - [ ] TTF_GetGPUTextEngineWinding
//! - [ ] TTF_GetHarfBuzzVersion
//! - [ ] TTF_GetNextTextSubString
//! - [ ] TTF_GetNumFontFaces
//! - [ ] TTF_GetPreviousTextSubString
//! - [ ] TTF_GetStringSize
//! - [ ] TTF_GetStringSizeWrapped
//! - [x] TTF_GetTextColor
//! - [ ] TTF_GetTextColorFloat
//! - [ ] TTF_GetTextDirection
//! - [ ] TTF_GetTextEngine
//! - [ ] TTF_GetTextFont
//! - [ ] TTF_GetTextPosition
//! - [ ] TTF_GetTextProperties
//! - [ ] TTF_GetTextScript
//! - [x] TTF_GetTextSize
//! - [ ] TTF_GetTextSubString
//! - [ ] TTF_GetTextSubStringForLine
//! - [ ] TTF_GetTextSubStringForPoint
//! - [ ] TTF_GetTextSubStringsForRange
//! - [ ] TTF_GetTextWrapWidth
//! - [x] TTF_Init
//! - [ ] TTF_InsertTextString
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
//! - [ ] TTF_SetGPUTextEngineWinding
//! - [x] TTF_SetTextColor
//! - [ ] TTF_SetTextColorFloat
//! - [ ] TTF_SetTextDirection
//! - [ ] TTF_SetTextEngine
//! - [ ] TTF_SetTextFont
//! - [ ] TTF_SetTextPosition
//! - [ ] TTF_SetTextScript
//! - [ ] TTF_SetTextString
//! - [ ] TTF_SetTextWrapWhitespaceVisible
//! - [ ] TTF_SetTextWrapWidth
//! - [ ] TTF_StringToTag
//! - [ ] TTF_TagToString
//! - [ ] TTF_TextWrapWhitespaceVisible
//! - [x] TTF_UpdateText
//! - [x] TTF_Version
//! - [x] TTF_WasInit

use std::{ffi::CStr, mem::MaybeUninit};

use sdl3_ttf_sys::ttf::*;

use crate::{
    color::RgbaU8,
    defs::SdlResult,
    error::get,
    rect::{PointF32, PointI32},
    resource,
    surface::{Surface, SurfaceRef},
    util::{c_ptr_to_str, to_result},
};

/// Ensures SDL_ttf (de)initialization.
pub struct TtfContext;

impl TtfContext {
    #[doc(alias = "TTF_Init")]
    pub fn new() -> SdlResult<Self> {
        if unsafe { TTF_Init() } {
            Ok(Self {})
        } else {
            Err(get())
        }
    }

    #[doc(alias = "TTF_WasInit")]
    pub fn is_initialized() -> bool {
        unsafe { TTF_WasInit() != 0 }
    }

    #[doc(alias = "TTF_Version")]
    pub fn version() -> i32 {
        unsafe { TTF_Version() }
    }
}

impl Drop for TtfContext {
    #[doc(alias = "TTF_Quit")]
    fn drop(&mut self) {
        unsafe { TTF_Quit() };
    }
}

resource!(Font, TTF, Close);

impl<'a> FontRef {
    #[doc(alias = "TTF_CopyFont")]
    pub fn try_clone(&self) -> SdlResult<Font> {
        Font::from_ptr(unsafe { TTF_CopyFont(self.handle.as_ptr()) })
    }

    #[doc(alias = "TTF_RenderGlyph_Blended")]
    pub fn render_glyph_blended(&self, ch: char, color: RgbaU8) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderGlyph_Blended(self.handle.as_ptr(), ch.into(), color.into())
        })
    }

    #[doc(alias = "TTF_RenderGlyph_LCD")]
    pub fn render_glyph_lcd(&self, ch: char, fg: RgbaU8, bg: RgbaU8) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderGlyph_LCD(self.handle.as_ptr(), ch.into(), fg.into(), bg.into())
        })
    }

    #[doc(alias = "TTF_RenderGlyph_Shaded")]
    pub fn render_glyph_shaded(&self, ch: char, fg: RgbaU8, bg: RgbaU8) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderGlyph_Shaded(self.handle.as_ptr(), ch.into(), fg.into(), bg.into())
        })
    }

    #[doc(alias = "TTF_RenderGlyph_Solid")]
    pub fn render_glyph_solid(&self, ch: char, color: RgbaU8) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderGlyph_Solid(self.handle.as_ptr(), ch.into(), color.into())
        })
    }

    #[doc(alias = "TTF_RenderText_Blended")]
    pub fn render_text_blended(&self, text: &str, color: RgbaU8) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Blended(
                self.handle.as_ptr(),
                text.as_ptr().cast(),
                text.len(),
                color.into(),
            )
        })
    }

    #[doc(alias = "TTF_RenderText_LCD")]
    pub fn render_text_lcd(&self, text: &str, fg: RgbaU8, bg: RgbaU8) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_LCD(
                self.handle.as_ptr(),
                text.as_ptr().cast(),
                text.len(),
                fg.into(),
                bg.into(),
            )
        })
    }

    #[doc(alias = "TTF_RenderText_Shaded")]
    pub fn render_text_shaded(&self, text: &str, fg: RgbaU8, bg: RgbaU8) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Shaded(
                self.handle.as_ptr(),
                text.as_ptr().cast(),
                text.len(),
                fg.into(),
                bg.into(),
            )
        })
    }

    #[doc(alias = "TTF_RenderText_Solid")]
    pub fn render_text_solid(&self, text: &str, color: RgbaU8) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Solid(
                self.handle.as_ptr(),
                text.as_ptr().cast(),
                text.len(),
                color.into(),
            )
        })
    }

    #[doc(alias = "TTF_RenderText_Blended_Wrapped")]
    pub fn render_text_blended_wrapped(
        &self,
        text: &CStr,
        color: RgbaU8,
        wrap_length: i32,
    ) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Blended_Wrapped(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.count_bytes(),
                color.into(),
                wrap_length,
            )
        })
    }

    #[doc(alias = "TTF_RenderText_LCD_Wrapped")]
    pub fn render_text_lcd_wrapped(
        &self,
        text: &CStr,
        fg: RgbaU8,
        bg: RgbaU8,
        wrap_length: i32,
    ) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_LCD_Wrapped(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.count_bytes(),
                fg.into(),
                bg.into(),
                wrap_length,
            )
        })
    }

    #[doc(alias = "TTF_RenderText_Shaded_Wrapped")]
    pub fn render_text_shaded_wrapped(
        &self,
        text: &CStr,
        fg: RgbaU8,
        bg: RgbaU8,
        wrap_length: i32,
    ) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Shaded_Wrapped(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.count_bytes(),
                fg.into(),
                bg.into(),
                wrap_length,
            )
        })
    }

    #[doc(alias = "TTF_RenderText_Solid_Wrapped")]
    pub fn render_text_solid_wrapped(
        &self,
        text: &CStr,
        color: RgbaU8,
        wrap_length: i32,
    ) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Solid_Wrapped(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.count_bytes(),
                color.into(),
                wrap_length,
            )
        })
    }

    #[doc(alias = "TTF_GetFontFamilyName")]
    pub fn family(&self) -> &'a str {
        unsafe { c_ptr_to_str(TTF_GetFontFamilyName(self.handle.as_ptr())) }
    }

    #[doc(alias = "TTF_FontIsFixedWidth")]
    pub fn is_mono(&self) -> bool {
        unsafe { TTF_FontIsFixedWidth(self.handle.as_ptr()) }
    }
}

impl Font {
    #[doc(alias = "TTF_OpenFont")]
    pub fn new(file: &CStr, point_size: f32) -> SdlResult<Self> {
        Self::from_ptr(unsafe { TTF_OpenFont(file.as_ptr(), point_size) })
    }
}

resource!(Text, TTF);

impl TextRef {
    #[doc(alias = "TTF_GetTextSize")]
    pub fn size(&self) -> PointI32 {
        let mut ret = MaybeUninit::<PointI32>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            TTF_GetTextSize(self.handle.as_ptr(), &raw mut (*ptr).x, &raw mut (*ptr).y);
            ret.assume_init()
        }
    }

    #[doc(alias = "TTF_GetTextColor")]
    pub fn color(&self) -> RgbaU8 {
        let mut col = MaybeUninit::<RgbaU8>::uninit();
        let ptr = col.as_mut_ptr();

        unsafe {
            TTF_GetTextColor(
                self.handle.as_ptr(),
                &raw mut (*ptr).rgb.r,
                &raw mut (*ptr).rgb.g,
                &raw mut (*ptr).rgb.b,
                &raw mut (*ptr).a,
            );

            col.assume_init()
        }
    }

    #[doc(alias = "TTF_SetTextColor")]
    pub fn set_color(&self, color: RgbaU8) -> SdlResult {
        to_result(unsafe {
            TTF_SetTextColor(
                self.handle.as_ptr(),
                color.rgb.r,
                color.rgb.g,
                color.rgb.b,
                color.a,
            )
        })
    }

    #[doc(alias = "TTF_UpdateText")]
    pub fn update(&self) -> SdlResult {
        to_result(unsafe { TTF_UpdateText(self.handle.as_ptr()) })
    }

    pub fn draw_to_surface(&self, surf: impl Into<SurfaceRef>, pos: PointI32) -> SdlResult {
        to_result(unsafe {
            TTF_DrawSurfaceText(
                self.handle.as_ptr(),
                pos.x,
                pos.y,
                surf.into().handle.as_ptr(),
            )
        })
    }

    pub fn draw_to_renderer(&self, pos: PointF32) -> SdlResult {
        to_result(unsafe { TTF_DrawRendererText(self.handle.as_ptr(), pos.x, pos.y) })
    }
}

impl Text {
    #[doc(alias = "TTF_CreateText")]
    pub fn new(font: impl Into<FontRef>, text: &str) -> SdlResult<Self> {
        Self::from_ptr(unsafe {
            TTF_CreateText(
                std::ptr::null_mut(),
                font.into().handle.as_ptr(),
                text.as_ptr().cast(),
                text.len(),
            )
        })
    }
}
