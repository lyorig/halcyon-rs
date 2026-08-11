#![allow(dead_code)]

use std::ffi::CStr;

use sdl3_sys::{
    filesystem::{SDL_GetBasePath, SDL_GetPrefPath},
    init::{SDL_IsMainThread, SDL_Quit},
    platform::SDL_GetPlatform,
    timer::{SDL_GetTicks, SDL_GetTicksNS},
};

use crate::{sdl_string::SdlString, subsystem::Subsystem, util::c_ptr_to_str};

mod boxed;

pub mod clipboard;
pub mod color;
pub mod display;
pub mod error;
pub mod event;
pub mod gpu;
pub mod keyboard;
pub mod log;
pub mod msgbox;
pub mod properties;
pub mod rect;
pub mod renderer;
pub mod resource;
pub mod sdl_string;
pub mod subsystem;
pub mod surface;
pub mod texture;
pub mod traits;
pub mod ttf;
pub mod util;
pub mod window;

/// A zero-sized type that only exists to call [`SDL_Quit()`].
/// As such, think of it as a guard that creates a scope for
/// the initialization of subsystems, ensuring they're properly
/// quit once it goes out of scope.
pub struct Context;

impl Context {
    /// Like [`Self::new()`], without the safety checks.
    ///
    /// # Safety
    /// Only call this on the main thread.
    pub unsafe fn new_unchecked() -> Self {
        Self {}
    }

    /// Panics if this function is not called on the main thread.
    ///
    /// # Why doesn't this return a [`Result`] instead?
    /// TL;DR: It's less error-prone.
    /// Contexts are sometimes left unused, i.e.
    /// ```
    /// use halcyon::Context;
    ///
    /// let _ctx = Context::new();
    /// ```
    /// If [`Self::new()`] returned [`Err`], this snippet would silently skip
    /// the destructor and not quit SDL in case of an error. Not running on
    /// the main thread isn't really something that can happen by chance and you
    /// can recover from. If necessary, check yourself via [`crate::is_main_thread()`].
    ///
    /// In addition, [`Result`] is only intended to originate from SDL API calls.
    /// Since [`Context`] is a ZST providing an abstraction over SDL initialization,
    /// this would newly require a way to create a "custom" error.
    pub fn new() -> Self {
        assert!(crate::is_main_thread(), "Context not on main thread");
        Self {}
    }

    pub fn init<const N: u32>(&self) -> Result<Subsystem<'_, N>> {
        Subsystem::new(self)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Context {
    #[doc(alias = "SDL_Quit")]
    fn drop(&mut self) {
        unsafe { SDL_Quit() };
    }
}

/// Convenience alias for [`std::result::Result<T, Error>`.]
/// Used as the return type throughout this crate.
pub type Result<T = ()> = std::result::Result<T, error::Error>;

#[doc(alias = "SDL_GetPlatform")]
pub fn platform() -> &'static str {
    // SAFETY: All SDL3 platform strings are UTF-8,
    // and are stored statically.
    unsafe { c_ptr_to_str(SDL_GetPlatform()) }
}

#[doc(alias = "SDL_GetBasePath")]
pub fn base_path() -> &'static str {
    // SAFETY: The string returned by `SDL_GetBasePath()`
    // is guaranteed to be valid UTF-8.
    unsafe { c_ptr_to_str(SDL_GetBasePath()) }
}

#[doc(alias = "SDL_GetPrefPath")]
pub fn pref_path(org: &CStr, app: &CStr) -> Result<SdlString> {
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
