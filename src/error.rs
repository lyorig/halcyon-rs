use std::{ffi::c_char, ptr::NonNull};

use sdl3_sys::error::{SDL_GetError, SDL_SetError};

use crate::util::c_ptr_to_str;

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

/// Returns `error::get()` as a `&'static str`, which isn't really
/// sound, but hey, it's a convenience function, and the `unsafe`
/// moniker should communicate that its usage is at your own risk.
///
/// Storing this value isn't recommended, as SDL's error buffer contents
/// may change with any `SDL_SetError()` call at any point (although
/// they're stored in a `static char[]` on the C side, so the address
/// remains fixed).
#[doc(alias = "SDL_GetError")]
pub unsafe fn get_str() -> &'static str {
    unsafe { c_ptr_to_str(get().as_ptr()) }
}

/// Returns `error::get()` as an owned `String`.
#[doc(alias = "SDL_GetError")]
pub fn get_owned() -> String {
    unsafe { get_str() }.to_owned()
}

/// Not sure why you'd ever need this, but it's provided regardless
/// for completeness' sake.
#[doc(alias = "SDL_SetError")]
pub fn set(err: &str) {
    unsafe {
        SDL_SetError(err.as_ptr().cast());
    }
}
