use crate::error::Error;

/// Convenience type alias for [`Result<T, ()>`], where `T`
/// is the success type. If it's [`Err`], call [`crate::error::get()`] for more information.
pub type SdlResult<T = ()> = Result<T, Error>;
