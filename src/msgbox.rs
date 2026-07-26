use std::ffi::CStr;

use sdl3_sys::messagebox::*;

use crate::{Result, util::to_result};

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Severity {
    Info = SDL_MESSAGEBOX_INFORMATION.0,
    Warning = SDL_MESSAGEBOX_WARNING.0,
    Error = SDL_MESSAGEBOX_ERROR.0,
}

impl Severity {
    fn as_flags(&self) -> SDL_MessageBoxFlags {
        SDL_MessageBoxFlags(*self as _)
    }
}

pub fn show(sev: Severity, title: &CStr, message: &CStr) -> Result {
    to_result(unsafe {
        SDL_ShowSimpleMessageBox(
            sev.as_flags(),
            title.as_ptr(),
            message.as_ptr(),
            std::ptr::null_mut(),
        )
    })
}
