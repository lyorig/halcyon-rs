use std::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use sdl3_ttf_sys::ttf::*;

use crate::{
    Result,
    color::RgbaU8,
    surface::Surface,
    ttf::{Context, RtStr},
    util::c_ptr_to_str,
};

crate::resource_new_tied!(TTF_Font, Font, TTF_CloseFont, Context);

impl Clone for Font<'_> {
    #[doc(alias = "TTF_CopyFont")]
    fn clone(&self) -> Self {
        let ptr = unsafe { TTF_CopyFont(self.handle.as_ptr()) };
        let handle = unsafe { NonNull::new_unchecked(ptr) };
        let inner = FontHandle { handle };

        Self {
            inner,
            marker: PhantomData,
        }
    }
}

impl FontHandle {
    #[doc(alias = "TTF_RenderGlyph_Blended")]
    pub fn render_glyph_blended(&self, ch: char, color: RgbaU8) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderGlyph_Blended(self.handle.as_ptr(), ch.into(), color.into())
        })
    }

    #[doc(alias = "TTF_RenderGlyph_LCD")]
    pub fn render_glyph_lcd(&self, ch: char, fg: RgbaU8, bg: RgbaU8) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderGlyph_LCD(self.handle.as_ptr(), ch.into(), fg.into(), bg.into())
        })
    }

    #[doc(alias = "TTF_RenderGlyph_Shaded")]
    pub fn render_glyph_shaded(&self, ch: char, fg: RgbaU8, bg: RgbaU8) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderGlyph_Shaded(self.handle.as_ptr(), ch.into(), fg.into(), bg.into())
        })
    }

    #[doc(alias = "TTF_RenderGlyph_Solid")]
    pub fn render_glyph_solid(&self, ch: char, color: RgbaU8) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderGlyph_Solid(self.handle.as_ptr(), ch.into(), color.into())
        })
    }

    #[doc(alias = "TTF_RenderText_Blended")]
    pub fn render_text_blended(&self, text: RtStr, color: RgbaU8) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Blended(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.len(),
                color.into(),
            )
        })
    }

    #[doc(alias = "TTF_RenderText_LCD")]
    pub fn render_text_lcd(&self, text: RtStr, fg: RgbaU8, bg: RgbaU8) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_LCD(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.len(),
                fg.into(),
                bg.into(),
            )
        })
    }

    #[doc(alias = "TTF_RenderText_Shaded")]
    pub fn render_text_shaded(&self, text: RtStr, fg: RgbaU8, bg: RgbaU8) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Shaded(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.len(),
                fg.into(),
                bg.into(),
            )
        })
    }

    #[doc(alias = "TTF_RenderText_Solid")]
    pub fn render_text_solid(&self, text: RtStr, color: RgbaU8) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Solid(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.len(),
                color.into(),
            )
        })
    }

    #[doc(alias = "TTF_RenderText_Blended_Wrapped")]
    pub fn render_text_blended_wrapped(
        &self,
        text: RtStr,
        color: RgbaU8,
        wrap_length: i32,
    ) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Blended_Wrapped(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.len(),
                color.into(),
                wrap_length,
            )
        })
    }

    #[doc(alias = "TTF_RenderText_LCD_Wrapped")]
    pub fn render_text_lcd_wrapped(
        &self,
        text: RtStr,
        fg: RgbaU8,
        bg: RgbaU8,
        wrap_length: i32,
    ) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_LCD_Wrapped(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.len(),
                fg.into(),
                bg.into(),
                wrap_length,
            )
        })
    }

    #[doc(alias = "TTF_RenderText_Shaded_Wrapped")]
    pub fn render_text_shaded_wrapped(
        &self,
        text: RtStr,
        fg: RgbaU8,
        bg: RgbaU8,
        wrap_length: i32,
    ) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Shaded_Wrapped(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.len(),
                fg.into(),
                bg.into(),
                wrap_length,
            )
        })
    }

    #[doc(alias = "TTF_RenderText_Solid_Wrapped")]
    pub fn render_text_solid_wrapped(
        &self,
        text: RtStr,
        color: RgbaU8,
        wrap_length: i32,
    ) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            TTF_RenderText_Solid_Wrapped(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.len(),
                color.into(),
                wrap_length,
            )
        })
    }

    #[doc(alias = "TTF_GetFontFamilyName")]
    pub fn family(&self) -> &str {
        unsafe { c_ptr_to_str(TTF_GetFontFamilyName(self.as_ptr())) }
    }

    #[doc(alias = "TTF_FontIsFixedWidth")]
    pub fn is_mono(&self) -> bool {
        unsafe { TTF_FontIsFixedWidth(self.as_ptr()) }
    }

    #[doc(alias = "TTF_FontIsScalable")]
    pub fn is_scalable(&self) -> bool {
        unsafe { TTF_FontIsScalable(self.as_ptr()) }
    }
}

impl<'ttf> Font<'ttf> {
    #[doc(alias = "TTF_OpenFont")]
    pub fn new(_ctx: &'ttf Context, file: &CStr, point_size: f32) -> Result<Self> {
        unsafe { Self::new_unchecked(file, point_size) }
    }

    /// # Safety
    /// Ensure a [`Context`] will exist for the entire lifetime of the returned font.
    /// That includes the point at which it's dropped. A segfault will probably
    /// happen otherwise.
    #[doc(alias = "TTF_OpenFont")]
    pub unsafe fn new_unchecked(file: &CStr, point_size: f32) -> Result<Self> {
        Self::from_ptr(unsafe { TTF_OpenFont(file.as_ptr(), point_size) })
    }
}
