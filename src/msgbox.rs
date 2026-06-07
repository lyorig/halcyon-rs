use std::ffi::CStr;

use sdl3_sys::messagebox::{SDL_MessageBoxFlags, SDL_ShowSimpleMessageBox};

use crate::{defs::SdlResult, util::to_result};

pub fn show(flags: SDL_MessageBoxFlags, title: &CStr, message: &CStr) -> SdlResult {
    to_result(unsafe {
        SDL_ShowSimpleMessageBox(
            flags,
            title.as_ptr(),
            message.as_ptr(),
            std::ptr::null_mut(),
        )
    })
}
