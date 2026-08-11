//! API checklist ([source](https://wiki.libsdl.org/SDL3/CategoryRender)):
//! - [x] SDL_ClearClipboardData
//! - [x] SDL_GetClipboardData
//! - [x] SDL_GetClipboardMimeTypes
//! - [x] SDL_GetClipboardText
//! - [ ] SDL_GetPrimarySelectionText
//! - [x] SDL_HasClipboardData
//! - [x] SDL_HasClipboardText
//! - [ ] SDL_HasPrimarySelectionText
//! - [ ] SDL_SetClipboardData
//! - [x] SDL_SetClipboardText
//! - [ ] SDL_SetPrimarySelectionText

use std::{ffi::CStr, mem::MaybeUninit};

use sdl3_sys::clipboard::*;

use crate::{Result, boxed::Box, sdl_string::SdlString, util::to_result};

#[doc(alias = "SDL_ClearClipboardData")]
pub fn clear_data() -> Result {
    to_result(unsafe { SDL_ClearClipboardData() })
}

#[doc(alias = "SDL_GetClipboardData")]
pub fn data(mime_type: &CStr) -> Result<Box<[u8]>> {
    let mut len = MaybeUninit::<usize>::uninit();
    let ptr = unsafe { SDL_GetClipboardData(mime_type.as_ptr(), len.as_mut_ptr()) };
    // SAFETY: On success, SDL allocates `len` bytes.
    unsafe { Box::from_raw_parts(ptr.cast(), len.assume_init()) }
}

#[doc(alias = "SDL_GetClipboardMimeTypes")]
pub fn mime_types() -> Result<Box<[*mut i8]>> {
    let mut len = MaybeUninit::<usize>::uninit();
    let ptr = unsafe { SDL_GetClipboardMimeTypes(len.as_mut_ptr()) };
    // SAFETY: On success, SDL allocates `len` mime type strings.
    unsafe { Box::from_raw_parts(ptr, len.assume_init()) }
}

#[doc(alias = "SDL_GetClipboardText")]
pub fn text() -> SdlString {
    let ptr = unsafe { SDL_GetClipboardText() };

    // SAFETY: `SDL_GetClipboardText()` always returns a valid string.
    unsafe { SdlString::from_ptr(ptr).unwrap_unchecked() }
}

#[doc(alias = "SDL_HasClipboardData")]
pub fn has_data(mime_type: &CStr) -> bool {
    unsafe { SDL_HasClipboardData(mime_type.as_ptr()) }
}

#[doc(alias = "SDL_HasClipboardText")]
pub fn has_text() -> bool {
    unsafe { SDL_HasClipboardText() }
}

#[doc(alias = "SDL_SetClipboardText")]
pub fn set_text(text: &CStr) -> Result {
    to_result(unsafe { SDL_SetClipboardText(text.as_ptr()) })
}
