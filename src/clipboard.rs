use std::ffi::CStr;

use sdl3_sys::clipboard::*;

use crate::{defs::SdlResult, sdl_string::SdlString, util::to_result};

pub fn has_text() -> bool {
    unsafe { SDL_HasClipboardText() }
}

pub fn text() -> SdlString {
    // SAFETY: `SDL_GetClipboardText()` never returns a null pointer.
    unsafe { SdlString::from_ptr(SDL_GetClipboardText()) }
}

pub fn set_text(text: &CStr) -> SdlResult {
    to_result(unsafe { SDL_SetClipboardText(text.as_ptr()) })
}
