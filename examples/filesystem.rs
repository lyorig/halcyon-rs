//! Attempts to enumerate a directory specified by the user.

use std::{
    ffi::{CStr, CString},
    process::ExitCode,
};

use halcyon::{Result, fs};

fn run(path: &CStr) -> Result<()> {
    fs::enumerate_directory(path, |dir, file| {
        halcyon::log!("{} -> {}", dir.to_string_lossy(), file.to_string_lossy());
        fs::EnumerationResult::Continue
    })
}

fn main() -> ExitCode {
    let Some(arg) = std::env::args().nth(1) else {
        halcyon::log!("No path specified.");
        return ExitCode::FAILURE;
    };

    // SAFETY: Arguments get cut off at the first nul byte.
    let path = unsafe { CString::from_vec_unchecked(arg.into_bytes()) };

    if let Err(e) = run(&path) {
        halcyon::log!("An error occurred: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
