use crate::error::Error;

/// Convenience type alias for [`Result<T, ()>`], where `T`
/// is the success type. If it's [`Err`], call [`Error::current()`] for more information.
pub type SdlResult<T = ()> = Result<T, Error>;
