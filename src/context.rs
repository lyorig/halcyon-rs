use sdl3_sys::init::SDL_Quit;
use sdl3_sys::platform::SDL_GetPlatform;

/// This struct only exists to call `SDL_Quit()`.
/// However, its existence is required for everything else.
pub struct Context;

impl Context {
    // SAFETY: Only call this on the main thread.
    pub unsafe fn new() -> Self {
        Self {}
    }

    pub fn platform() -> &'static str {
        use std::ffi::CStr;

        // SAFETY: All SDL3 platform strings are UTF-8.
        unsafe {
            let c = CStr::from_ptr(SDL_GetPlatform());

            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                c.as_ptr() as *const u8,
                c.count_bytes(),
            ))
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { SDL_Quit() };
    }
}
