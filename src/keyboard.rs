use sdl3_sys::{
    keyboard::{SDL_GetKeyName, SDL_GetScancodeName},
    keycode::SDL_Keycode,
    scancode::SDL_Scancode,
};

use crate::util::c_to_str;

#[doc(alias = "SDL_GetScancodeName")]
pub fn scancode_name(scancode: SDL_Scancode) -> &'static str {
    unsafe { c_to_str(SDL_GetScancodeName(scancode)) }
}

#[doc(alias = "SDL_GetKeyName")]
pub fn key_name(key: SDL_Keycode) -> &'static str {
    unsafe { c_to_str(SDL_GetKeyName(key)) }
}
