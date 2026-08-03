use std::mem::ManuallyDrop;

use rustest::{main, test};
use sdl3_sys::{pixels::SDL_PixelFormat, render::SDL_TextureAccess};

use halcyon::{
    Context, rect::Point, renderer::RendererBuilder, resource::Resource, subsystem::Video,
    texture::Texture, window::WindowBuilder,
};

mod clipboard;
mod color;
mod error;
mod event;
mod log;
mod properties;
mod ttf;

/// Basic initialization stuff.
#[test]
fn main_init() {
    let _ctx = Context::new();

    const WINDOW_SIZE: Point<i32> = Point::new(128, 128);

    let wnd = WindowBuilder::new()
        .hidden(true)
        .size(WINDOW_SIZE)
        .build()
        .unwrap();

    assert_eq!(wnd.size(), WINDOW_SIZE);

    let rnd = RendererBuilder::new(wnd.as_ref()).build().unwrap();
    let tex = Texture::new(
        rnd.as_ref(),
        SDL_PixelFormat::RGB24,
        SDL_TextureAccess::STATIC,
        Point::new(16, 16),
    )
    .unwrap();

    assert_eq!(tex.size(), Point::new(16.0, 16.0));
}

#[test]
fn main_subsystems() {
    let ctx = Context::new();

    {
        let _vid = Video::new(&ctx).unwrap();
        assert!(Video::is_init());
    }

    assert!(!Video::is_init());
}

#[test]
fn main_manually_drop() {
    {
        let ctx = Context::new();

        {
            let _vid = ManuallyDrop::new(Video::new(&ctx).unwrap());
            assert!(Video::is_init());
        }

        // Still initialized, since `ManuallyDrop` skips the destructor.
        assert!(Video::is_init());
    }

    // Context should've cleaned everything up.
    assert!(!Video::is_init());
}

#[main]
fn main() {}
