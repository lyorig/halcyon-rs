use std::{ffi::c_char, ptr::NonNull};

use sdl3_sys::error::SDL_GetError;

/// Returns a pointer to the contents of `SDL_GetError()`.
///
/// On the C side, the returned pointer points to a static
/// `char` array. However, its contents and length may change with
/// every `SDL_SetError()` call, so it is recommended to only store
/// the pointer, and create `CStr` and other string types as needed,
/// since they cache the length, assuming it won't change.
///
/// TL;DR: The string will always reside at the same address, but its
/// length may change arbitrarily. Be wary with structs that cache it.
#[doc(alias = "SDL_GetError")]
pub fn get() -> NonNull<c_char> {
    unsafe {
        NonNull::new(SDL_GetError().cast_mut())
            .expect("SDL_GetError() should never return a null pointer")
    }
}
