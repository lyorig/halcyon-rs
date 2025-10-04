//! SDL display API.
//!
//! Implementation checklist (source):
//! - [ ] SDL_GetClosestFullscreenDisplayMode
//! - [x] SDL_GetCurrentDisplayMode
//! - [ ] SDL_GetCurrentDisplayOrientation
//! - [x] SDL_GetDesktopDisplayMode
//! - [x] SDL_GetDisplayBounds
//! - [ ] SDL_GetDisplayContentScale
//! - [ ] SDL_GetDisplayForPoint
//! - [ ] SDL_GetDisplayForRect
//! - [x] SDL_GetDisplayName
//! - [ ] SDL_GetDisplayProperties
//! - [x] SDL_GetDisplays
//! - [x] SDL_GetDisplayUsableBounds
//! - [x] SDL_GetFullscreenDisplayModes
//! - [ ] SDL_GetNaturalDisplayOrientation
//! - [x] SDL_GetPrimaryDisplay

use crate::{
    defs::SdlResult,
    error::get_error,
    rect::{PointI32, RectI32},
    sdl_box::SdlBoxArr,
    util::c_to_str,
};
use sdl3_sys::video::*;
use std::{mem::MaybeUninit, num::NonZero, ptr::NonNull};

pub type DisplayId = NonZero<SDL_DisplayID>;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct DisplayHandle {
    pub(crate) id: NonZero<SDL_DisplayID>,
}

impl DisplayHandle {
    #[doc(alias = "SDL_GetDisplays")]
    pub fn all() -> SdlResult<SdlBoxArr<Self>> {
        let mut count = MaybeUninit::uninit();
        unsafe { SdlBoxArr::from_ptr(SDL_GetDisplays(count.as_mut_ptr()).cast(), count) }
    }

    #[doc(alias = "SDL_GetPrimaryDisplay")]
    pub fn primary() -> SdlResult<Self> {
        match NonZero::new(unsafe { SDL_GetPrimaryDisplay() }) {
            Some(id) => Ok(Self { id }),
            None => Err(get_error()),
        }
    }

    pub fn for_point(point: PointI32) -> SdlResult<Self> {
        match NonZero::new(unsafe { SDL_GetDisplayForPoint((&raw const point).cast()) }) {
            Some(id) => Ok(Self { id }),
            None => Err(get_error()),
        }
    }

    pub fn for_rect(rect: RectI32) -> SdlResult<Self> {
        match NonZero::new(unsafe { SDL_GetDisplayForRect((&raw const rect).cast()) }) {
            Some(id) => Ok(Self { id }),
            None => Err(get_error()),
        }
    }

    pub fn id(&self) -> NonZero<SDL_DisplayID> {
        self.id
    }

    #[doc(alias = "SDL_GetDisplayName")]
    pub fn name(&self) -> &'static str {
        unsafe { c_to_str(SDL_GetDisplayName(self.id.get())) }
    }

    #[doc(alias = "SDL_GetDisplayBounds")]
    pub fn bounds(&self) -> RectI32 {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            SDL_GetDisplayBounds(self.id.get(), ret.as_mut_ptr());
            std::mem::transmute_copy(ret.assume_init_ref())
        }
    }

    #[doc(alias = "SDL_GetDisplayUsableBounds")]
    pub fn bounds_usable(&self) -> RectI32 {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            SDL_GetDisplayUsableBounds(self.id.get(), ret.as_mut_ptr());
            std::mem::transmute_copy(ret.assume_init_ref())
        }
    }

    #[doc(alias = "SDL_GetCurrentDisplayMode")]
    pub fn display_mode_current(&self) -> SdlResult<NonNull<SDL_DisplayMode>> {
        let ptr = unsafe { SDL_GetCurrentDisplayMode(self.id.get()) };
        if ptr.is_null() {
            Err(get_error())
        } else {
            Ok(NonNull::new(ptr.cast_mut()).unwrap())
        }
    }

    #[doc(alias = "SDL_GetDesktopDisplayMode")]
    pub fn display_mode_desktop(&self) -> SdlResult<NonNull<SDL_DisplayMode>> {
        let ptr = unsafe { SDL_GetDesktopDisplayMode(self.id.get()) };
        if ptr.is_null() {
            Err(get_error())
        } else {
            Ok(NonNull::new(ptr.cast_mut()).unwrap())
        }
    }

    #[doc(alias = "SDL_GetFullscreenDisplayModes")]
    pub fn display_modes(&self) -> SdlResult<SdlBoxArr<NonNull<SDL_DisplayMode>>> {
        let mut count = MaybeUninit::uninit();

        unsafe {
            SdlBoxArr::from_ptr(
                SDL_GetFullscreenDisplayModes(self.id.get(), count.as_mut_ptr()).cast(),
                count,
            )
        }
    }
}
