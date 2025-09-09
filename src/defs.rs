use std::ffi::CStr;

pub type SdlResult<T = ()> = Result<T, &'static CStr>;
