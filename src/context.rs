use sdl3_sys::init::SDL_Quit;

use crate::{defs::SdlResult, subsystem::Subsystem};

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
    /// # Why doesn't this return a [`SdlResult`] instead?
    /// TL;DR: It's less error-prone.
    /// Contexts are sometimes left unused, i.e.
    /// ```
    /// let _ctx = Context::new();
    /// ```
    /// If [`Self::new()`] returned [`Err`], this snippet would silently skip
    /// the destructor and not quit SDL in case of an error. Not running on
    /// the main thread isn't really something that can happen by chance and you
    /// can recover from. If necessary, check yourself via [`crate::is_main_thread()`].
    ///
    /// In addition, [`SdlResult`] is only intended to originate from SDL API calls.
    /// Since [`Context`] is a ZST providing an abstraction over SDL initialization,
    /// this would newly require a way to create a "custom" error.
    pub fn new() -> Self {
        assert!(crate::is_main_thread(), "Context not on main thread");
        Self {}
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
