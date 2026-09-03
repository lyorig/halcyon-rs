//! Attempts to enumerate a directory specified by the user.

use std::ffi::{CStr, CString};

use halcyon::{Result, fs};

fn into_cstring(s: String) -> CString {
    unsafe { CString::from_vec_unchecked(s.into_bytes()) }
}

fn run(path: &CStr) -> Result<()> {
    fs::enumerate_directory(path, |dir, file| {
        halcyon::log!("{} -> {}", dir.to_string_lossy(), file.to_string_lossy());
        fs::EnumerationResult::Continue
    })?;

    Ok(())
}

fn main() {
    let Some(path) = std::env::args().nth(1).map(into_cstring) else {
        halcyon::log!("No path specified.");
        return;
    };

    if let Err(e) = run(&path) {
        halcyon::log!("An error occurred: {e}");
    }
}
