use std::ffi::CStr;

use sdl3_sys::clipboard::*;

use crate::{defs::SdlResult, sdl_string::SdlString, util::to_result};

#[doc(alias = "SDL_HasClipboardText")]
pub fn has_text() -> bool {
    unsafe { SDL_HasClipboardText() }
}

#[doc(alias = "SDL_GetClipboardText")]
pub fn text() -> SdlString {
    let ptr = unsafe { SDL_GetClipboardText() };

    // SAFETY: `SDL_GetClipboardText()` always returns a valid string.
    unsafe { SdlString::from_ptr(ptr).unwrap_unchecked() }
}

#[doc(alias = "SDL_SetClipboardText")]
pub fn set_text(text: &CStr) -> SdlResult {
    to_result(unsafe { SDL_SetClipboardText(text.as_ptr()) })
}
