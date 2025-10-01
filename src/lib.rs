#![allow(dead_code)]

mod properties;

pub mod clipboard;
pub mod color;
pub mod context;
pub mod defs;
pub mod error;
pub mod event;
pub mod keyboard;
pub mod rect;
pub mod renderer;
pub mod subsystem;
pub mod surface;
pub mod texture;
pub mod ttf;
pub mod util;
pub mod window;

#[cfg(test)]
mod tests {
    use crate::{context::Context, error::get_error, subsystem::Video};

    #[test]
    fn context() {
        let ctx = unsafe { Context::new() };
        let _vid = Video::new(&ctx).expect("Yuup");
    }

    #[test]
    fn error() {
        assert_eq!(get_error(), c"");
    }
}
