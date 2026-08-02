//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryLog)):
//! - [ ] SDL_GetDefaultLogOutputFunction
//! - [ ] SDL_GetLogOutputFunction
//! - [x] SDL_GetLogPriority
//! - [x] SDL_Log
//! - [x] SDL_LogCritical
//! - [x] SDL_LogDebug
//! - [x] SDL_LogError
//! - [x] SDL_LogInfo
//! - [x] SDL_LogMessage
//! - [ ] SDL_LogMessageV
//! - [x] SDL_LogTrace
//! - [x] SDL_LogVerbose
//! - [x] SDL_LogWarn
//! - [x] SDL_ResetLogPriorities
//! - [ ] SDL_SetLogOutputFunction
//! - [x] SDL_SetLogPriorities
//! - [x] SDL_SetLogPriority
//! - [x] SDL_SetLogPriorityPrefix

use std::{
    ffi::{CStr, CString},
    fmt::Arguments,
};

use sdl3_sys::log::*;

use crate::{Result, util::to_result};

#[repr(i32)]
#[doc(alias = "SDL_LogPriority")]
pub enum Priority {
    Trace = SDL_LogPriority::TRACE.0,
    Verbose = SDL_LogPriority::VERBOSE.0,
    Debug = SDL_LogPriority::DEBUG.0,
    Info = SDL_LogPriority::INFO.0,
    Warn = SDL_LogPriority::WARN.0,
    Error = SDL_LogPriority::ERROR.0,
    Critical = SDL_LogPriority::CRITICAL.0,
}

impl From<SDL_LogPriority> for Priority {
    fn from(value: SDL_LogPriority) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Priority> for SDL_LogPriority {
    fn from(value: Priority) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

/// A log category. Unlike most SDL enums, `SDL_LogCategory` is an open set:
/// values at and above [`Self::CUSTOM`] are available for application-defined
/// categories, so this is a newtype rather than a closed enum.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[doc(alias = "SDL_LogCategory")]
pub struct Category(pub i32);

impl Category {
    pub const APPLICATION: Self = Self(SDL_LogCategory::APPLICATION.0);
    pub const ERROR: Self = Self(SDL_LogCategory::ERROR.0);
    pub const ASSERT: Self = Self(SDL_LogCategory::ASSERT.0);
    pub const SYSTEM: Self = Self(SDL_LogCategory::SYSTEM.0);
    pub const AUDIO: Self = Self(SDL_LogCategory::AUDIO.0);
    pub const VIDEO: Self = Self(SDL_LogCategory::VIDEO.0);
    pub const RENDER: Self = Self(SDL_LogCategory::RENDER.0);
    pub const INPUT: Self = Self(SDL_LogCategory::INPUT.0);
    pub const TEST: Self = Self(SDL_LogCategory::TEST.0);
    pub const GPU: Self = Self(SDL_LogCategory::GPU.0);
}

// SDL's log functions are variadic; Rust cannot pass variadic arguments, so
// messages are formatted on the Rust side and passed as the sole argument.
// Note that SDL still interprets the message as a printf-style format string,
// so literal `%` characters must be escaped as `%%`.
#[doc(alias = "SDL_Log")]
pub fn log(args: Arguments) {
    let s = args.to_string();
    let cs = unsafe { CString::from_vec_unchecked(s.into_bytes()) };
    unsafe { SDL_Log(cs.as_ptr()) };
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::log::log(format_args!($($arg)*));
    };
}

macro_rules! log_for_priority {
    ($name:ident, $sdl:ident, $alias:literal) => {
        #[doc(alias = $alias)]
        pub fn $name(category: Category, msg: &CStr) {
            unsafe { $sdl(category.0, msg.as_ptr()) }
        }
    };
}

log_for_priority!(trace, SDL_LogTrace, "SDL_LogTrace");
log_for_priority!(verbose, SDL_LogVerbose, "SDL_LogVerbose");
log_for_priority!(debug, SDL_LogDebug, "SDL_LogDebug");
log_for_priority!(info, SDL_LogInfo, "SDL_LogInfo");
log_for_priority!(warn, SDL_LogWarn, "SDL_LogWarn");
log_for_priority!(error, SDL_LogError, "SDL_LogError");
log_for_priority!(critical, SDL_LogCritical, "SDL_LogCritical");

#[doc(alias = "SDL_LogMessage")]
pub fn log_message(category: Category, priority: Priority, msg: &CStr) {
    unsafe { SDL_LogMessage(category.0, priority.into(), msg.as_ptr()) }
}

#[doc(alias = "SDL_SetLogPriority")]
pub fn set_priority(category: Category, priority: Priority) {
    unsafe { SDL_SetLogPriority(category.0, priority.into()) }
}

#[doc(alias = "SDL_SetLogPriorities")]
pub fn set_priorities(priority: Priority) {
    unsafe { SDL_SetLogPriorities(priority.into()) }
}

#[doc(alias = "SDL_GetLogPriority")]
pub fn priority(category: Category) -> Priority {
    unsafe { SDL_GetLogPriority(category.0) }.into()
}

#[doc(alias = "SDL_ResetLogPriorities")]
pub fn reset_priorities() {
    unsafe { SDL_ResetLogPriorities() }
}

#[doc(alias = "SDL_SetLogPriorityPrefix")]
pub fn set_priority_prefix(priority: Priority, prefix: &CStr) -> Result {
    to_result(unsafe { SDL_SetLogPriorityPrefix(priority.into(), prefix.as_ptr()) })
}
