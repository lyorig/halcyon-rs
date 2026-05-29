use std::{
    ffi::{CStr, c_char},
    ptr::NonNull,
};

use sdl3_sys::error::{SDL_GetError, SDL_SetError};

use crate::util::c_ptr_to_str;

/// Returns a pointer to the contents of [`SDL_GetError()`].
///
/// On the C side, the returned pointer points to a static
/// [`c_char`] array. However, its contents and length may change with
/// every [`SDL_SetError()`] call, so it is recommended to only store
/// the pointer, and create [`CStr`] and other string types as needed,
/// since they cache the length, assuming it won't change.
///
/// TL;DR: The string will always reside at the same address, but its
/// length may change arbitrarily. Be wary with structs that cache it.
#[doc(alias = "SDL_GetError")]
pub fn get() -> NonNull<c_char> {
    NonNull::new(SDL_GetError().cast_mut())
        .expect("SDL_GetError() should never return a null pointer")
}

/// Returns [`crate::error::get()`] as a `&'static str`.
///
/// This is purely a convenience function whose purpose is to
/// simplify printing the error string, or otherwise using it
/// as a one-off value, and discarding it right after the call.
///
/// # Safety
/// The TL;DR is: only use the returned value before anything
/// else on the thread has the chance to call [`SDL_SetError()`].
///
/// For more info, consult the [SDL docs](https://wiki.libsdl.org/SDL3/SDL_GetError).
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
pub fn set(err: &CStr) {
    unsafe {
        SDL_SetError(err.as_ptr());
    }
}
