use crate::{defs::SdlResult, error::get_error, rect::RectI32, sdl_box::SdlBoxArr, util::c_to_str};
use sdl3_sys::video::*;
use std::{mem::MaybeUninit, num::NonZero};

pub type DisplayId = NonZero<SDL_DisplayID>;

pub fn all() -> SdlResult<SdlBoxArr<DisplayId>> {
    let mut count = MaybeUninit::uninit();
    unsafe { SdlBoxArr::from_ptr(SDL_GetDisplays(count.as_mut_ptr()).cast(), count) }
}

pub fn name(id: DisplayId) -> &'static str {
    unsafe { c_to_str(SDL_GetDisplayName(id.get())) }
}

pub fn bounds(id: DisplayId) -> RectI32 {
    let mut ret = MaybeUninit::uninit();
    unsafe {
        SDL_GetDisplayBounds(id.get(), ret.as_mut_ptr());
        std::mem::transmute_copy(ret.assume_init_ref())
    }
}

pub fn bounds_usable(id: DisplayId) -> RectI32 {
    let mut ret = MaybeUninit::uninit();
    unsafe {
        SDL_GetDisplayUsableBounds(id.get(), ret.as_mut_ptr());
        std::mem::transmute_copy(ret.assume_init_ref())
    }
}

pub fn primary() -> SdlResult<DisplayId> {
    NonZero::new(unsafe { SDL_GetPrimaryDisplay() }).ok_or_else(get_error)
}

pub fn display_mode_current(id: DisplayId) -> SdlResult<SDL_DisplayMode> {
    let ptr = unsafe { SDL_GetCurrentDisplayMode(id.get()) };
    if ptr.is_null() {
        Err(get_error())
    } else {
        Ok(unsafe { ptr.read() })
    }
}

pub fn display_mode_desktop(id: DisplayId) -> SdlResult<SDL_DisplayMode> {
    let ptr = unsafe { SDL_GetDesktopDisplayMode(id.get()) };
    if ptr.is_null() {
        Err(get_error())
    } else {
        Ok(unsafe { ptr.read() })
    }
}

pub fn display_modes(id: DisplayId) -> SdlResult<Box<[SDL_DisplayMode]>> {
    let mut count = MaybeUninit::uninit();
    let ptr = unsafe { SDL_GetFullscreenDisplayModes(id.get(), count.as_mut_ptr()) };

    unsafe {
        // NOTE: The usage of SdlBoxArr is to ensure proper deallocation via `SDL_free()`.
        SdlBoxArr::from_ptr(ptr, count).map(|_| {
            let slice = std::slice::from_raw_parts(ptr, count.assume_init() as _);
            slice.iter().map(|ptr| ptr.read()).collect()
        })
    }
}
