use std::path::Path;

use sdl3_sys::filesystem::SDL_GetBasePath;
use sdl3_sys::init::SDL_Quit;
use sdl3_sys::platform::SDL_GetPlatform;

use crate::util::c_ptr_to_str;

/// A zero-sized type that only exists to call `SDL_Quit()`.
/// As such, think of it as a guard that creates a scope for
/// the initialization of subsystems, ensuring they're properly
/// quit once it goes out of scope.
pub struct Context;

impl Context {
    /// SAFETY: Only call this on the main thread.
    pub unsafe fn new() -> Self {
        Self {}
    }

    #[doc(alias = "SDL_GetPlatform")]
    pub fn platform() -> &'static str {
        // SAFETY: All SDL3 platform strings are UTF-8.
        unsafe { c_ptr_to_str(SDL_GetPlatform()) }
    }

    #[doc(alias = "SDL_GetBasePath")]
    pub fn base_path() -> &'static Path {
        // SAFETY: SDL caches this call internally.
        Path::new(unsafe { c_ptr_to_str(SDL_GetBasePath()) })
    }
}

impl Drop for Context {
    #[doc(alias = "SDL_Quit")]
    fn drop(&mut self) {
        unsafe { SDL_Quit() };
    }
}
