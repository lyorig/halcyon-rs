use std::{ffi::c_char, ptr::NonNull};

use sdl3_sys::error::SDL_GetError;

/// Returned an pointer to the contents of `SDL_GetError()`.
#[doc(alias = "SDL_GetError")]
pub fn get() -> NonNull<c_char> {
    unsafe {
        NonNull::new(SDL_GetError().cast_mut())
            .expect("SDL_GetError() should never return a null pointer")
    }
}
