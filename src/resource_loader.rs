use std::{
    ffi::{CStr, CString},
    path::{Path, PathBuf},
};

/// An abstraction over [`crate::base_path`] that provides
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
    pub fn from_base() -> Self {
        Self::from_path(Path::new(crate::base_path()))
    }

    pub fn from_path(root: &'static Path) -> Self {
        Self { root }
    }

    pub fn resolve(&self, path: &str) -> Box<CStr> {
        const NUL_ERROR: &str =
            "ResourceLoader::resolve() should never be given a Path with embedded NUL bytes";

        let pb = PathBuf::from_iter([self.root, Path::new(path)]);
        let enc = pb.as_os_str().as_encoded_bytes();

        CString::new(enc).expect(NUL_ERROR).into_boxed_c_str()
    }
}

impl Default for ResourceLoader {
    fn default() -> Self {
        Self::from_base()
    }
}
