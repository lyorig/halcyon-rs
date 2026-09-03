//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryFilesystem)):
//! - [x] SDL_CopyFile
//! - [x] SDL_CreateDirectory
//! - [x] SDL_EnumerateDirectory
//! - [x] SDL_GetBasePath
//! - [x] SDL_GetCurrentDirectory
//! - [x] SDL_GetPathInfo
//! - [x] SDL_GetPrefPath
//! - [x] SDL_GetUserFolder
//! - [x] SDL_GlobDirectory
//! - [x] SDL_RemovePath
//! - [x] SDL_RenamePath

use std::{
    ffi::{CStr, c_char},
    mem::MaybeUninit,
    ptr::{NonNull, from_mut},
};

use bitflags::bitflags;
use sdl3_sys::filesystem::*;

use crate::{Result, boxed::Box, impl_enum_transmute, string::String, util::to_result};

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum Folder {
    Home = SDL_Folder::HOME.0,
    Desktop = SDL_Folder::DESKTOP.0,
    Documents = SDL_Folder::DOCUMENTS.0,
    Downloads = SDL_Folder::DOWNLOADS.0,
    Music = SDL_Folder::MUSIC.0,
    Pictures = SDL_Folder::PICTURES.0,
    PublicShare = SDL_Folder::PUBLICSHARE.0,
    SavedGames = SDL_Folder::SAVEDGAMES.0,
    Screenshots = SDL_Folder::SCREENSHOTS.0,
    Templates = SDL_Folder::TEMPLATES.0,
    Videos = SDL_Folder::VIDEOS.0,
}

impl_enum_transmute!(SDL_Folder, Folder);

#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PathType {
    #[default]
    None = SDL_PathType::NONE.0,
    File = SDL_PathType::FILE.0,
    Directory = SDL_PathType::DIRECTORY.0,
    Other = SDL_PathType::OTHER.0,
}

impl_enum_transmute!(SDL_PathType, PathType);

/// Mirror of [`SDL_PathInfo`]. Field `type` is renamed to [`Self::path_type`]
/// to avoid clashing with the Rust keyword.
#[doc(alias = "SDL_PathInfo")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PathInfo {
    pub path_type: PathType,
    pub size: u64,
    pub create_time: i64,
    pub modify_time: i64,
    pub access_time: i64,
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    #[doc(alias = "SDL_GlobFlags")]
    pub struct GlobFlags: u32 {
        const CASE_INSENSITIVE = SDL_GlobFlags::CASEINSENSITIVE.0;
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum EnumerationResult {
    Continue = SDL_EnumerationResult::CONTINUE.0,
    Success = SDL_EnumerationResult::SUCCESS.0,
    Failure = SDL_EnumerationResult::FAILURE.0,
}

impl_enum_transmute!(SDL_EnumerationResult, EnumerationResult);

#[doc(alias = "SDL_GetPrefPath")]
pub fn pref_path(org: &CStr, app: &CStr) -> Result<String> {
    unsafe {
        let ptr = SDL_GetPrefPath(org.as_ptr(), app.as_ptr());
        String::from_raw_nullck(ptr)
    }
}

#[doc(alias = "SDL_CreateDirectory")]
pub fn create_directory(path: &CStr) -> Result<()> {
    to_result(unsafe { SDL_CreateDirectory(path.as_ptr()) })
}

#[doc(alias = "SDL_RemovePath")]
pub fn remove_path(path: &CStr) -> Result<()> {
    to_result(unsafe { SDL_RemovePath(path.as_ptr()) })
}

#[doc(alias = "SDL_RenamePath")]
pub fn rename_path(old_path: &CStr, new_path: &CStr) -> Result<()> {
    to_result(unsafe { SDL_RenamePath(old_path.as_ptr(), new_path.as_ptr()) })
}

#[doc(alias = "SDL_CopyFile")]
pub fn copy_file(old_path: &CStr, new_path: &CStr) -> Result<()> {
    to_result(unsafe { SDL_CopyFile(old_path.as_ptr(), new_path.as_ptr()) })
}

#[doc(alias = "SDL_GetPathInfo")]
pub fn path_info(path: &CStr) -> Result<PathInfo> {
    let mut info = MaybeUninit::<PathInfo>::uninit();

    to_result(unsafe { SDL_GetPathInfo(path.as_ptr(), info.as_mut_ptr().cast()) })?;

    // SAFETY: SDL fully initializes the struct on success.
    // The layouts of both types match field for field.
    Ok(unsafe { info.assume_init() })
}

/// The callback receives the directory currently being enumerated
/// and the name of the current entry (no path prefix).
/// Both references are only valid for the duration of the callback.
/// Return [`EnumerationResult::Continue`] to keep enumerating,
/// [`EnumerationResult::Success`] to stop early (the overall call still succeeds),
/// or [`EnumerationResult::Failure`] to stop and fail.
#[doc(alias = "SDL_EnumerateDirectory")]
pub fn enumerate_directory<F>(path: &CStr, mut callback: F) -> Result<()>
where
    F: FnMut(&CStr, &CStr) -> EnumerationResult,
{
    unsafe extern "C" fn trampoline<F>(
        userdata: *mut std::ffi::c_void,
        dirname: *const c_char,
        fname: *const c_char,
    ) -> SDL_EnumerationResult
    where
        F: FnMut(&CStr, &CStr) -> EnumerationResult,
    {
        // SAFETY: SDL passes valid, nul-terminated strings,
        // valid for the duration of the call.
        unsafe {
            let callback = (userdata as *mut F).as_mut_unchecked();
            callback(CStr::from_ptr(dirname), CStr::from_ptr(fname)).into()
        }
    }

    to_result(unsafe {
        SDL_EnumerateDirectory(
            path.as_ptr(),
            Some(trampoline::<F>),
            from_mut(&mut callback).cast(),
        )
    })
}

/// Entries are returned as an SDL-allocated array of pointers.
/// It is a single allocation: individual strings are not to be freed,
/// only the array itself (handled by [`Box`]'s destructor).
#[doc(alias = "SDL_GlobDirectory")]
pub fn glob_directory(
    path: &CStr,
    pattern: Option<&CStr>,
    flags: GlobFlags,
) -> Result<Box<[NonNull<c_char>]>> {
    let pattern = pattern.map_or(std::ptr::null(), CStr::as_ptr);

    // Initialize to zero, so no uninitialized memory is read in case of failure.
    let mut count: i32 = 0;
    let ptr = unsafe {
        SDL_GlobDirectory(
            path.as_ptr(),
            pattern,
            SDL_GlobFlags::new(flags.bits()),
            &mut count,
        )
    };

    // SAFETY: On success, SDL allocates a single array of `count` string pointers.
    unsafe { Box::from_raw_parts_nullck(ptr.cast(), count as usize) }
}

#[doc(alias = "SDL_GetCurrentDirectory")]
pub fn current_directory() -> Result<String> {
    unsafe {
        let ptr = SDL_GetCurrentDirectory();
        String::from_raw_nullck(ptr)
    }
}
