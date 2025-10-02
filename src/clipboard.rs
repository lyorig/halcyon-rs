use sdl3_sys::clipboard::*;

use crate::sdl_cstring::SdlCString;

pub fn has_text() -> bool {
    unsafe { SDL_HasClipboardText() }
}

pub fn text() -> SdlCString {
    // SAFETY: `SDL_GetClipboardText` never returns a null pointer.
    unsafe { SdlCString::from_ptr(SDL_GetClipboardText()) }
}
