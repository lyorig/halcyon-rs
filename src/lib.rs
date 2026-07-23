#![allow(dead_code)]

use std::ffi::CStr;

use sdl3_sys::{
    filesystem::{SDL_GetBasePath, SDL_GetPrefPath},
    init::SDL_IsMainThread,
    platform::SDL_GetPlatform,
    timer::{SDL_GetTicks, SDL_GetTicksNS},
};

use crate::{defs::SdlResult, sdl_string::SdlString, util::c_ptr_to_str};

mod properties;
mod sdl_box;

pub mod clipboard;
pub mod color;
pub mod context;
pub mod defs;
pub mod display;
pub mod error;
pub mod event;
pub mod gpu;
pub mod keyboard;
pub mod msgbox;
pub mod rect;
pub mod renderer;
pub mod sdl_string;
pub mod subsystem;
pub mod surface;
pub mod texture;
pub mod traits;
pub mod ttf;
pub mod util;
pub mod window;

#[doc(alias = "SDL_GetPlatform")]
pub fn platform() -> &'static str {
    // SAFETY: All SDL3 platform strings are UTF-8,
    // and they are stored statically.
    unsafe { c_ptr_to_str(SDL_GetPlatform()) }
}

#[doc(alias = "SDL_GetBasePath")]
pub fn base_path() -> &'static str {
    // SAFETY: The string returned by `SDL_GetBasePath()`
    // is guaranteed to be valid UTF-8.
    unsafe { c_ptr_to_str(SDL_GetBasePath()) }
}

#[doc(alias = "SDL_GetPrefPath")]
pub fn pref_path(org: &CStr, app: &CStr) -> SdlResult<SdlString> {
    let ptr = unsafe { SDL_GetPrefPath(org.as_ptr(), app.as_ptr()) };
    SdlString::from_ptr(ptr)
}

#[doc(alias = "SDL_IsMainThread")]
pub fn is_main_thread() -> bool {
    unsafe { SDL_IsMainThread() }
}

#[doc(alias = "SDL_GetTicks")]
pub fn ticks() -> u64 {
    unsafe { SDL_GetTicks() }
}

#[doc(alias = "SDL_GetTicksNS")]
pub fn ticks_ns() -> u64 {
    unsafe { SDL_GetTicksNS() }
}
