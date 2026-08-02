//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryLog)):
//! - [x] SDL_GetLogPriority
//! - [x] SDL_Log
//! - [x] SDL_LogCritical
//! - [x] SDL_LogDebug
//! - [x] SDL_LogError
//! - [x] SDL_LogInfo
//! - [ ] SDL_LogMessage
//! - [x] SDL_LogTrace
//! - [x] SDL_LogVerbose
//! - [x] SDL_LogWarn
//! - [x] SDL_ResetLogPriorities
//! - [x] SDL_SetLogPriorities
//! - [x] SDL_SetLogPriority
//! - [ ] SDL_SetLogPriorityPrefix
//!
//! Not planned for implementation:
//! - SDL_GetDefaultLogOutputFunction
//! - SDL_GetLogOutputFunction
//! - SDL_LogMessageV
//! - SDL_SetLogOutputFunction

use std::{ffi::CString, fmt::Arguments};

use sdl3_sys::log::*;

#[repr(i32)]
#[derive(Clone, Copy)]
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

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_LogCategory")]
pub enum Category {
    Application = SDL_LogCategory::APPLICATION.0,
    Error = SDL_LogCategory::ERROR.0,
    Assert = SDL_LogCategory::ASSERT.0,
    System = SDL_LogCategory::SYSTEM.0,
    Audio = SDL_LogCategory::AUDIO.0,
    Video = SDL_LogCategory::VIDEO.0,
    Render = SDL_LogCategory::RENDER.0,
    Input = SDL_LogCategory::INPUT.0,
    Test = SDL_LogCategory::TEST.0,
    Gpu = SDL_LogCategory::GPU.0,
}

fn args2cstr(args: Arguments) -> CString {
    let s = args.to_string();
    unsafe { CString::from_vec_unchecked(s.into_bytes()) }
}

const FMT: *const i8 = c"%s".as_ptr();

macro_rules! log_for_priority {
    ($name:ident, $sdl:ident, $alias:literal) => {
        #[doc(alias = $alias)]
        pub fn $name(category: Category, args: Arguments) {
            let cs = args2cstr(args);
            unsafe { $sdl(category as _, FMT, cs.as_ptr()) };
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

#[doc(alias = "SDL_Log")]
pub fn log(args: Arguments) {
    let cs = args2cstr(args);
    unsafe { SDL_Log(FMT, cs.as_ptr()) };
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::log::log(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_trace {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log::trace($cat, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_verbose {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log::verbose($cat, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log::debug($cat, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_info {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log::info($cat, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log::warn($cat, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log::error($cat, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_critical {
    ($cat:expr, $($arg:tt)*) => {
        $crate::log::critical($cat, format_args!($($arg)*));
    };
}

#[doc(alias = "SDL_SetLogPriority")]
pub fn set_priority(category: Category, priority: Priority) {
    unsafe { SDL_SetLogPriority(category as _, priority.into()) }
}

#[doc(alias = "SDL_SetLogPriorities")]
pub fn set_priorities(priority: Priority) {
    unsafe { SDL_SetLogPriorities(priority.into()) }
}

#[doc(alias = "SDL_GetLogPriority")]
pub fn priority(category: Category) -> Priority {
    unsafe { SDL_GetLogPriority(category as _) }.into()
}

#[doc(alias = "SDL_ResetLogPriorities")]
pub fn reset_priorities() {
    unsafe { SDL_ResetLogPriorities() }
}
