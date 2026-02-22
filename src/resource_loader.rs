use std::{
    ffi::{CStr, CString},
    path::{Path, PathBuf},
};

use crate::context::Context;

/// An abstraction over `Context::base_path()` that provides
/// utility methods for constructing `Path`s relative to the
/// directory of the program that's running.
///
/// This struct is made up of a single `&'static Path`, so it's
/// safe to clone/copy.
#[derive(Clone, Copy)]
pub struct ResourceLoader {
    root: &'static Path,
}

impl ResourceLoader {
    pub fn new() -> Self {
        Self {
            root: Context::base_path(),
        }
    }

    pub fn resolve(&self, path: &str) -> Box<CStr> {
        let pb = PathBuf::from_iter([self.root, Path::new(path)]);
        CString::new(pb.as_os_str().as_encoded_bytes())
            .expect(
                "ResourceLoader::resolve() should never be given a Path with embedded NUL bytes",
            )
            .into_boxed_c_str()
    }
}

impl Default for ResourceLoader {
    fn default() -> Self {
        Self::new()
    }
}
