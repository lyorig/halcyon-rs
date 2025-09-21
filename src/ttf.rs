//! SDL_ttf wrapper.

use std::{ffi::CStr, mem::MaybeUninit};

use sdl3_ttf_sys::ttf::*;

use crate::{color::Color, defs::SdlResult, error::get_error, resource};

/// Ensures SDL_ttf (de)initialization.
pub struct TtfContext;

impl TtfContext {
    #[doc(alias = "TTF_Init")]
    fn new() -> SdlResult<Self> {
        if unsafe { TTF_Init() } {
            Ok(Self {})
        } else {
            Err(get_error())
        }
    }
}

impl Drop for TtfContext {
    #[doc(alias = "TTF_Quit")]
    fn drop(&mut self) {
        unsafe { TTF_Quit() };
    }
}

resource!(Font, FontRef, TTF_Font, TTF_CloseFont);

impl FontRef {
    #[doc(alias = "TTF_CopyFont")]
    fn try_clone(&self) -> SdlResult<Font> {
        Font::from_ptr(unsafe { TTF_CopyFont(self.handle.as_ptr()) })
    }
}

impl Font {
    #[doc(alias = "TTF_OpenFont")]
    pub fn new(_ctx: &TtfContext, file: &CStr, point_size: f32) -> SdlResult<Self> {
        Self::from_ptr(unsafe { TTF_OpenFont(file.as_ptr(), point_size) })
    }
}

resource!(Text, TextRef, TTF_Text, TTF_DestroyText);

impl TextRef {
    #[doc(alias = "TTF_GetTextSize")]
    pub fn size(&self) -> (i32, i32) {
        let mut size = (MaybeUninit::uninit(), MaybeUninit::uninit());
        unsafe {
            TTF_GetTextSize(
                self.handle.as_ptr(),
                size.0.as_mut_ptr(),
                size.1.as_mut_ptr(),
            );
            (size.0.assume_init(), size.1.assume_init())
        }
    }

    #[doc(alias = "TTF_GetTextColor")]
    pub fn color(&self) -> Color {
        let mut col = MaybeUninit::<Color>::uninit();
        let ptr = col.as_mut_ptr();

        unsafe {
            TTF_GetTextColor(
                self.handle.as_ptr(),
                &raw mut (*ptr).r,
                &raw mut (*ptr).g,
                &raw mut (*ptr).b,
                &raw mut (*ptr).a,
            );

            col.assume_init()
        }
    }
}
