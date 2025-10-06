use std::{ffi::c_char, ptr::NonNull};

/// Convenience type alias for `Result<T, NonNull<c_char>>`, where `T`
/// is the success type, and `NonNull<c_char>` gets populated with `halcyon::error::get()`
/// in case of failure.
pub type SdlResult<T = ()> = Result<T, NonNull<c_char>>;
