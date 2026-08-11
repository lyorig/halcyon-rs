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
    Result, boolenum,
    boxed::Box,
    error::Error,
    rect::{PointI32, RectI32},
    util::opt2res_map,
};

use sdl3_sys::video::*;
use std::{ffi::CStr, mem::MaybeUninit, num::NonZero, ptr::NonNull};

boolenum!(IncludeHighDensityModes);

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum DisplayOrientation {
    Landscape = SDL_DisplayOrientation::LANDSCAPE.0,
    LandscapeFlipped = SDL_DisplayOrientation::LANDSCAPE_FLIPPED.0,
    Portrait = SDL_DisplayOrientation::PORTRAIT.0,
    PortraitFlipped = SDL_DisplayOrientation::PORTRAIT_FLIPPED.0,
}

impl std::fmt::Display for DisplayOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

fn sdl2do(sdl: SDL_DisplayOrientation) -> Option<DisplayOrientation> {
    if sdl == SDL_DisplayOrientation::UNKNOWN {
        None
    } else {
        use std::mem::transmute;

        type Src = SDL_DisplayOrientation;
        type Dst = DisplayOrientation;
        Some(unsafe { transmute::<Src, Dst>(sdl) })
    }
}

/// A handle to a display owned by SDL.
///
/// This is essentially just a number, and since displays can be
/// added and removed at will, member functions mostly return a [`Result`].
///
/// Some SDL display-related functions provide owned data, while others return
/// a pointer to data managed by SDL itself. Due to the aforementioned display
/// invalidation stuff, this would be quite a mess to implement as a reference
/// in Rust (owing to lifetimes), so instead, the raw pointer is provided,
/// offloading the risk to you.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Display {
    id: NonZero<u32>,
}

impl Display {
    /// Accepts [`NonZero`], since zero is an invalid display ID.
    pub fn new(id: NonZero<u32>) -> Self {
        Self { id }
    }

    pub(crate) fn from_sdl(id: SDL_DisplayID) -> Result<Self> {
        opt2res_map(NonZero::new(id.0), |id| Self { id })
    }

    #[doc(alias = "SDL_GetDisplays")]
    pub fn all() -> Result<Box<[Self]>> {
        let mut count = MaybeUninit::uninit();
        let ptr = unsafe { SDL_GetDisplays(count.as_mut_ptr()) };

        // SAFETY: On success, SDL allocates `count` displays.
        unsafe { Box::from_raw_parts(ptr.cast(), count.assume_init() as _) }
    }

    #[doc(alias = "SDL_GetPrimaryDisplay")]
    pub fn primary() -> Result<Self> {
        Self::from_sdl(unsafe { SDL_GetPrimaryDisplay() })
    }

    #[doc(alias = "SDL_GetDisplayForPoint")]
    pub fn for_point(point: PointI32) -> Result<Self> {
        Self::from_sdl(unsafe { SDL_GetDisplayForPoint(point.as_sdl_ptr()) })
    }

    #[doc(alias = "SDL_GetDisplayForRect")]
    pub fn for_rect(rect: RectI32) -> Result<Self> {
        Self::from_sdl(unsafe { SDL_GetDisplayForRect(rect.as_sdl_ptr()) })
    }

    /// Returns the "raw" SDL handle type. Intended for interfacing with
    /// SDL display functions.
    pub fn id(&self) -> SDL_DisplayID {
        unsafe { std::mem::transmute(self.id) }
    }

    /// The returned string is guaranteed to be valid UTF-8.
    #[doc(alias = "SDL_GetDisplayName")]
    pub fn name(&self) -> Result<&CStr> {
        let ptr = unsafe { SDL_GetDisplayName(self.id()) };
        if ptr.is_null() {
            Err(Error::current())
        } else {
            Ok(unsafe { CStr::from_ptr(ptr) })
        }
    }

    #[doc(alias = "SDL_GetDisplayBounds")]
    pub fn bounds(&self) -> Result<RectI32> {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            if SDL_GetDisplayBounds(self.id(), ret.as_mut_ptr()) {
                Ok(std::mem::transmute_copy(ret.assume_init_ref()))
            } else {
                Err(Error::current())
            }
        }
    }

    #[doc(alias = "SDL_GetDisplayUsableBounds")]
    pub fn usable_bounds(&self) -> Result<RectI32> {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            if SDL_GetDisplayUsableBounds(self.id(), ret.as_mut_ptr()) {
                Ok(std::mem::transmute_copy(ret.assume_init_ref()))
            } else {
                Err(Error::current())
            }
        }
    }

    #[doc(alias = "SDL_GetCurrentDisplayMode")]
    pub fn current_mode(&self) -> Result<NonNull<SDL_DisplayMode>> {
        let ptr = unsafe { SDL_GetCurrentDisplayMode(self.id()) };
        if ptr.is_null() {
            Err(Error::current())
        } else {
            Ok(NonNull::new(ptr.cast_mut()).unwrap())
        }
    }

    #[doc(alias = "SDL_GetDesktopDisplayMode")]
    pub fn desktop_mode(&self) -> Result<NonNull<SDL_DisplayMode>> {
        let ptr = unsafe { SDL_GetDesktopDisplayMode(self.id()) };
        if ptr.is_null() {
            Err(Error::current())
        } else {
            Ok(NonNull::new(ptr.cast_mut()).unwrap())
        }
    }

    #[doc(alias = "SDL_GetFullscreenDisplayModes")]
    pub fn fullscreen_modes(&self) -> Result<Box<[NonNull<SDL_DisplayMode>]>> {
        let mut count = MaybeUninit::uninit();
        let ptr = unsafe { SDL_GetFullscreenDisplayModes(self.id(), count.as_mut_ptr()) };

        // SAFETY: On success, SDL allocates `count` display modes.
        unsafe { Box::from_raw_parts(ptr.cast(), count.assume_init() as _) }
    }

    #[doc(alias = "SDL_GetDisplayContentScale")]
    pub fn content_scale(&self) -> Result<f32> {
        let ret = unsafe { SDL_GetDisplayContentScale(self.id()) };
        if ret == 0. {
            Err(Error::current())
        } else {
            Ok(ret)
        }
    }

    #[doc(alias = "SDL_GetCurrentDisplayOrientation")]
    pub fn current_orientation(&self) -> Option<DisplayOrientation> {
        sdl2do(unsafe { SDL_GetCurrentDisplayOrientation(self.id()) })
    }

    #[doc(alias = "SDL_GetNaturalDisplayOrientation")]
    pub fn natural_orientation(&self) -> Option<DisplayOrientation> {
        sdl2do(unsafe { SDL_GetNaturalDisplayOrientation(self.id()) })
    }

    #[doc(alias = "SDL_GetClosestFullscreenDisplayMode")]
    pub fn closest_fullscreen_mode(
        &self,
        size: PointI32,
        refresh_rate: f32,
        ihdm: IncludeHighDensityModes,
    ) -> Result<SDL_DisplayMode> {
        let mut ret = MaybeUninit::uninit();

        unsafe {
            if SDL_GetClosestFullscreenDisplayMode(
                self.id(),
                size.x,
                size.y,
                refresh_rate,
                ihdm.into(),
                ret.as_mut_ptr(),
            ) {
                Ok(ret.assume_init())
            } else {
                Err(Error::current())
            }
        }
    }
}
