#![allow(dead_code)]

mod properties;
mod sdl_box;

pub mod clipboard;
pub mod color;
pub mod context;
pub mod defs;
pub mod display;
pub mod error;
pub mod event;
pub mod gpu;
pub mod keyboard;
pub mod mixer;
pub mod msgbox;
pub mod rect;
pub mod renderer;
pub mod resource_loader;
pub mod sdl_string;
pub mod subsystem;
pub mod surface;
pub mod texture;
pub mod traits;
pub mod ttf;
pub mod util;
pub mod window;

#[cfg(test)]
mod tests {
    use crate::{context::Context, subsystem::Video};

    #[test]
    fn context() {
        // FIXME: This fails due to the test harness executing on a separate thread.
        let ctx = unsafe { Context::new() };
        let _vid = Video::new(&ctx).expect("Should be able to initialize video subsystem");
    }
}
