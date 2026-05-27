//! SDL display API.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryVideo)):
//! - [x] SDL_GetClosestFullscreenDisplayMode
//! - [x] SDL_GetCurrentDisplayMode
//! - [x] SDL_GetCurrentDisplayOrientation
//! - [x] SDL_GetDesktopDisplayMode
//! - [x] SDL_GetDisplayBounds
//! - [x] SDL_GetDisplayContentScale
//! - [x] SDL_GetDisplayForPoint
//! - [x] SDL_GetDisplayForRect
//! - [x] SDL_GetDisplayName
//! - [ ] SDL_GetDisplayProperties
//! - [x] SDL_GetDisplays
//! - [x] SDL_GetDisplayUsableBounds
//! - [x] SDL_GetFullscreenDisplayModes
//! - [x] SDL_GetNaturalDisplayOrientation
//! - [x] SDL_GetPrimaryDisplay

use crate::{
    defs::SdlResult,
    error::get,
    rect::{PointI32, RectI32},
    sdl_box::SdlBoxArr,
    util::c_ptr_to_str,
};

use sdl3_sys::video::*;
use std::{ffi::c_char, mem::MaybeUninit, num::NonZero, ptr::NonNull};

/// A handle to a display owned by SDL.
///
/// This is essentially just a number, and since displays can be
/// added and removed at will, member functions mostly return an `SdlResult`.
///
/// Some SDL display-related functions provide owned data, while others return
/// a pointer to data managed by SDL itself. Due to the aforementioned display
/// invalidation stuff, this would be quite a mess to implement as a reference
/// in Rust (owing to lifetimes), so instead, the raw pointer is provided,
/// offloading the risk to you.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct DisplayHandle {
    /// Can't use `SDL_DisplayID` because it's a newtype and doesn't
    /// implement [`std::num::ZeroablePrimitive`].
    pub(crate) id: NonZero<u32>,
}

impl DisplayHandle {
    fn new(id: SDL_DisplayID) -> SdlResult<Self> {
        match NonZero::new(id.0) {
            Some(id) => Ok(Self { id }),
            None => Err(get()),
        }
    }

    #[doc(alias = "SDL_GetDisplays")]
    pub fn all() -> SdlResult<SdlBoxArr<Self>> {
        let mut count = MaybeUninit::uninit();
        unsafe { SdlBoxArr::from_ptr(SDL_GetDisplays(count.as_mut_ptr()).cast(), count) }
    }

    #[doc(alias = "SDL_GetPrimaryDisplay")]
    pub fn primary() -> SdlResult<Self> {
        Self::new(unsafe { SDL_GetPrimaryDisplay() })
    }

    pub fn for_point(point: PointI32) -> SdlResult<Self> {
        Self::new(unsafe { SDL_GetDisplayForPoint((&raw const point).cast()) })
    }

    pub fn for_rect(rect: RectI32) -> SdlResult<Self> {
        Self::new(unsafe { SDL_GetDisplayForRect((&raw const rect).cast()) })
    }

    pub fn id(&self) -> SDL_DisplayID {
        unsafe { std::mem::transmute(self.id) }
    }

    /// The returned string is guaranteed to be valid UTF-8.
    #[doc(alias = "SDL_GetDisplayName")]
    pub fn name(&self) -> SdlResult<NonNull<c_char>> {
        NonNull::new(unsafe { SDL_GetDisplayName(self.id()).cast_mut() }).ok_or_else(get)
    }

    /// Convenience function to return `DisplayHandle::name()` as an owned `String`.
    #[doc(alias = "SDL_GetDisplayName")]
    pub fn name_owned(&self) -> SdlResult<String> {
        let ptr = unsafe { SDL_GetDisplayName(self.id()) };
        if ptr.is_null() {
            Err(get())
        } else {
            Ok(unsafe { c_ptr_to_str(ptr).to_owned() })
        }
    }

    #[doc(alias = "SDL_GetDisplayBounds")]
    pub fn bounds(&self) -> SdlResult<RectI32> {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            if SDL_GetDisplayBounds(self.id(), ret.as_mut_ptr()) {
                Ok(std::mem::transmute_copy(ret.assume_init_ref()))
            } else {
                Err(get())
            }
        }
    }

    #[doc(alias = "SDL_GetDisplayUsableBounds")]
    pub fn bounds_usable(&self) -> SdlResult<RectI32> {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            if SDL_GetDisplayUsableBounds(self.id(), ret.as_mut_ptr()) {
                Ok(std::mem::transmute_copy(ret.assume_init_ref()))
            } else {
                Err(get())
            }
        }
    }

    #[doc(alias = "SDL_GetCurrentDisplayMode")]
    pub fn mode_current(&self) -> SdlResult<NonNull<SDL_DisplayMode>> {
        let ptr = unsafe { SDL_GetCurrentDisplayMode(self.id()) };
        if ptr.is_null() {
            Err(get())
        } else {
            Ok(NonNull::new(ptr.cast_mut()).unwrap())
        }
    }

    #[doc(alias = "SDL_GetDesktopDisplayMode")]
    pub fn mode_desktop(&self) -> SdlResult<NonNull<SDL_DisplayMode>> {
        let ptr = unsafe { SDL_GetDesktopDisplayMode(self.id()) };
        if ptr.is_null() {
            Err(get())
        } else {
            Ok(NonNull::new(ptr.cast_mut()).unwrap())
        }
    }

    #[doc(alias = "SDL_GetFullscreenDisplayModes")]
    pub fn modes(&self) -> SdlResult<SdlBoxArr<NonNull<SDL_DisplayMode>>> {
        let mut count = MaybeUninit::uninit();

        unsafe {
            SdlBoxArr::from_ptr(
                SDL_GetFullscreenDisplayModes(self.id(), count.as_mut_ptr()).cast(),
                count,
            )
        }
    }

    #[doc(alias = "SDL_GetDisplayContentScale")]
    pub fn content_scale(&self) -> SdlResult<f32> {
        let ret = unsafe { SDL_GetDisplayContentScale(self.id()) };
        if ret == 0. { Err(get()) } else { Ok(ret) }
    }

    #[doc(alias = "SDL_GetCurrentDisplayOrientation")]
    pub fn orientation_current(&self) -> SDL_DisplayOrientation {
        unsafe { SDL_GetCurrentDisplayOrientation(self.id()) }
    }

    #[doc(alias = "SDL_GetNaturalDisplayOrientation")]
    pub fn orientation_natural(&self) -> SDL_DisplayOrientation {
        unsafe { SDL_GetNaturalDisplayOrientation(self.id()) }
    }

    #[doc(alias = "SDL_GetClosestFullscreenDisplayMode")]
    pub fn closest_fullscreen_mode(
        &self,
        size: PointI32,
        refresh_rate: f32,
        include_high_density_modes: bool,
    ) -> SdlResult<SDL_DisplayMode> {
        let mut ret = MaybeUninit::uninit();

        unsafe {
            if SDL_GetClosestFullscreenDisplayMode(
                self.id(),
                size.x,
                size.y,
                refresh_rate,
                include_high_density_modes,
                ret.as_mut_ptr(),
            ) {
                Ok(ret.assume_init())
            } else {
                Err(get())
            }
        }
    }
}
