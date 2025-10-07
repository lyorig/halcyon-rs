use std::{ffi::CStr, ptr::NonNull};

use sdl3_sys::clipboard::*;

use crate::{defs::SdlResult, sdl_string::SdlString, util::to_result};

#[doc(alias = "SDL_HasClipboardText")]
pub fn has_text() -> bool {
    unsafe { SDL_HasClipboardText() }
}

#[doc(alias = "SDL_GetClipboardText")]
pub fn text() -> SdlString {
    // SAFETY: `SDL_GetClipboardText()` always returns a valid string.
    unsafe { SdlString::from_ptr(NonNull::new_unchecked(SDL_GetClipboardText())) }
}

#[doc(alias = "SDL_SetClipboardText")]
pub fn set_text(text: &CStr) -> SdlResult {
    to_result(unsafe { SDL_SetClipboardText(text.as_ptr()) })
}
