use sdl3_sys::init::SDL_Quit;

use crate::{defs::SdlResult, subsystem::Subsystem};

/// A zero-sized type that only exists to call [`SDL_Quit()`].
/// As such, think of it as a guard that creates a scope for
/// the initialization of subsystems, ensuring they're properly
/// quit once it goes out of scope.
pub struct Context;

impl Context {
    /// # Safety
    /// Only call this on the main thread.
    pub unsafe fn new_unchecked() -> Self {
        Self {}
    }

    /// Panics if this function is not called on the main thread.
    pub fn new() -> Self {
        assert!(
            crate::is_main_thread(),
            "Halcyon can only be initialized on the main thread"
        );

        unsafe { Self::new_unchecked() }
    }

    pub fn init<const N: u32>(&self) -> SdlResult<Subsystem<'_, N>> {
        Subsystem::new(self)
    }
}

impl Drop for Context {
    #[doc(alias = "SDL_Quit")]
    fn drop(&mut self) {
        unsafe { SDL_Quit() };
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
