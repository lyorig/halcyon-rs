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
pub mod keyboard;
pub mod rect;
pub mod renderer;
pub mod sdl_string;
pub mod subsystem;
pub mod surface;
pub mod texture;
pub mod ttf;
pub mod util;
pub mod window;

#[cfg(test)]
mod tests {
    use sdl3_sys::stdinc::SDL_strcmp;

    use crate::{context::Context, error, subsystem::Video};

    #[test]
    fn context() {
        let ctx = unsafe { Context::new() };
        let _vid = Video::new(&ctx).expect("Should be able to initialize video subsystem");
    }

    #[test]
    fn error() {
        assert_eq!(
            unsafe { SDL_strcmp(error::get().as_ptr(), c"".as_ptr()) },
            0
        );
    }
}
