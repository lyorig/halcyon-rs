#![allow(dead_code)]

mod properties;

mod util;

pub mod color;
pub mod context;
pub mod coord;
pub mod error;

pub mod renderer;
pub mod subsystem;
pub mod surface;
pub mod texture;
pub mod window;

#[cfg(test)]
mod tests {
    use crate::subsystem::Video;

    #[test]
    fn context() {
        use crate::context::Context;

        let ctx = unsafe { Context::new() };
        let _vid = Video::new(&ctx).expect("Yuup");
    }

    #[test]
    fn error() {
        use crate::error;
        use std::ops::Deref;

        assert_eq!(error::get().deref(), c"");
    }
}
