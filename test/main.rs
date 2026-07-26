use rustest::{main, test};
use sdl3_sys::{pixels::SDL_PixelFormat, render::SDL_TextureAccess};

use halcyon::{
    Context, rect::Point, renderer::RendererBuilder, texture::Texture, traits::Resource,
    window::WindowBuilder,
};

mod color;
mod event;

/// Basic initialization stuff.
#[test]
fn init() {
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

#[main]
fn main() {
    color::test();
    event::test();

    init();
}
