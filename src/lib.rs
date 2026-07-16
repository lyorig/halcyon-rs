#![allow(dead_code)]

use std::{ffi::CStr, ptr::NonNull};

use sdl3_sys::{
    filesystem::{SDL_GetBasePath, SDL_GetPrefPath},
    init::SDL_IsMainThread,
    platform::SDL_GetPlatform,
};

use crate::{defs::SdlResult, error::Error, sdl_string::SdlString, util::c_ptr_to_str};

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
    match NonNull::new(ptr) {
        Some(n) => Ok(unsafe { SdlString::from_ptr(n) }),
        None => Err(Error::current()),
    }
}

#[doc(alias = "SDL_IsMainThread")]
pub fn is_main_thread() -> bool {
    unsafe { SDL_IsMainThread() }
}

#[cfg(test)]
mod tests {
    use crate::{context::Context, subsystem::Video};

    #[test]
    fn context() {
        // FIXME: This fails due to the test harness executing on a separate thread.
        let ctx = Context::new();
        let _vid = Video::new(&ctx).expect("Should be able to initialize video subsystem");
    }
}
