use std::{ffi::CStr, ptr::NonNull};

use sdl3_sys::clipboard::*;

use crate::{defs::SdlResult, sdl_string::SdlString, util::to_result};

pub fn has_text() -> bool {
    unsafe { SDL_HasClipboardText() }
}

pub fn text() -> SdlString {
    SdlString::from_ptr(
        NonNull::new(unsafe { SDL_GetClipboardText() })
            .expect("SDL_GetClipboardText() should never return a null pointer"),
    )
}

pub fn set_text(text: &CStr) -> SdlResult {
    to_result(unsafe { SDL_SetClipboardText(text.as_ptr()) })
}
