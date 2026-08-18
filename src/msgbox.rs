use std::ffi::CStr;

use sdl3_sys::messagebox::*;

use crate::{Result, util::to_result};

#[repr(u32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_MessageBoxFlags")]
pub enum Severity {
    Error = SDL_MessageBoxFlags::ERROR.0,
    Warning = SDL_MessageBoxFlags::WARNING.0,
    Info = SDL_MessageBoxFlags::INFORMATION.0,
}

#[repr(u32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_MessageBoxFlags")]
pub enum ButtonLayout {
    LeftToRight = SDL_MessageBoxFlags::BUTTONS_LEFT_TO_RIGHT.0,
    RightToLeft = SDL_MessageBoxFlags::BUTTONS_RIGHT_TO_LEFT.0,
}

#[doc(alias = "SDL_ShowSimpleMessageBox")]
pub fn show(sev: Severity, bl: ButtonLayout, title: &CStr, message: &CStr) -> Result {
    let flags = sev as u32 | bl as u32;
    to_result(unsafe {
        SDL_ShowSimpleMessageBox(
            SDL_MessageBoxFlags::new(flags),
            title.as_ptr(),
            message.as_ptr(),
            std::ptr::null_mut(),
        )
    })
}
