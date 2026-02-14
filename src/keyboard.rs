//! SDL_CategoryKeyboard:
//!
//! - [ ] SDL_ClearComposition
//! - [ ] SDL_GetKeyboardFocus
//! - [ ] SDL_GetKeyboardNameForID
//! - [ ] SDL_GetKeyboards
//! - [ ] SDL_GetKeyboardState
//! - [ ] SDL_GetKeyFromName
//! - [ ] SDL_GetKeyFromScancode
//! - [x] SDL_GetKeyName
//! - [x] SDL_GetModState
//! - [ ] SDL_GetScancodeFromKey
//! - [ ] SDL_GetScancodeFromName
//! - [x] SDL_GetScancodeName
//! - [ ] SDL_GetTextInputArea
//! - [ ] SDL_HasKeyboard
//! - [ ] SDL_HasScreenKeyboardSupport
//! - [ ] SDL_ResetKeyboard
//! - [ ] SDL_ScreenKeyboardShown
//! - [ ] SDL_SetModState
//! - [ ] SDL_SetScancodeName
//! - [ ] SDL_SetTextInputArea
//! - [x] SDL_StartTextInput
//! - [ ] SDL_StartTextInputWithProperties
//! - [x] SDL_StopTextInput
//! - [x] SDL_TextInputActive

use std::ffi::CStr;

use sdl3_sys::{
    keyboard::*,
    keycode::{SDL_Keycode, SDL_Keymod},
    scancode::{SDL_SCANCODE_COUNT, SDL_Scancode},
};

use crate::{defs::SdlResult, util::to_result, window::WindowRef};

const NUM_SCANCODES: usize = SDL_SCANCODE_COUNT.0 as usize;

#[doc(alias = "SDL_GetScancodeName")]
pub fn scancode_name(scancode: SDL_Scancode) -> &'static str {
    unsafe {
        std::str::from_utf8_unchecked(CStr::from_ptr(SDL_GetScancodeName(scancode)).to_bytes())
    }
}

#[doc(alias = "SDL_GetKeyName")]
pub fn key_name(key: SDL_Keycode) -> &'static str {
    unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(SDL_GetKeyName(key)).to_bytes()) }
}

#[doc(alias = "SDL_GetKeyboardState")]
pub fn keyboard_state() -> &'static [bool; NUM_SCANCODES] {
    unsafe {
        (SDL_GetKeyboardState(std::ptr::null_mut()) as *const [bool; NUM_SCANCODES])
            .as_ref()
            .expect("SDL_GetKeyboardState returned a null pointer")
    }
}

#[doc(alias = "SDL_GetModState")]
pub fn mod_state() -> SDL_Keymod {
    unsafe { SDL_GetModState() }
}

#[doc(alias = "SDL_StartTextInput")]
pub fn text_input_start(wnd: WindowRef) -> SdlResult {
    to_result(unsafe { SDL_StartTextInput(wnd.handle.as_ptr()) })
}

#[doc(alias = "SDL_StopTextInput")]
pub fn text_input_stop(wnd: WindowRef) -> SdlResult {
    to_result(unsafe { SDL_StopTextInput(wnd.handle.as_ptr()) })
}

#[doc(alias = "SDL_TextInputActive")]
pub fn text_input_active(wnd: WindowRef) -> bool {
    unsafe { SDL_TextInputActive(wnd.handle.as_ptr()) }
}
