use sdl3_sys::init::SDL_Quit;

/// A zero-sized type that only exists to call [`SDL_Quit()`].
/// As such, think of it as a guard that creates a scope for
/// the initialization of subsystems, ensuring they're properly
/// quit once it goes out of scope.
pub struct Context;

impl Context {
    /// # Safety
    /// Only call this on the main thread.
    pub unsafe fn new() -> Self {
        Self {}
    }
}

impl Drop for Context {
    #[doc(alias = "SDL_Quit")]
    fn drop(&mut self) {
        unsafe { SDL_Quit() };
    }
}
