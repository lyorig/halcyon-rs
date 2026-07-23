use sdl3_sys::{events::*, pixels::SDL_PixelFormat, render::SDL_TextureAccess};

use halcyon::{
    context::Context, event::Event, rect::Point, renderer::RendererBuilder, texture::Texture,
    traits::Resource, window::WindowBuilder,
};

fn main() {
    init();
    event_sdl_to_hal();
    event_hal_to_sdl();
}

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

fn event_sdl_to_hal() {
    let hal = Event::from(&SDL_Event {
        clipboard: SDL_ClipboardEvent {
            r#type: SDL_EVENT_CLIPBOARD_UPDATE,
            ..Default::default()
        },
    });

    let Event::ClipboardUpdate(_) = hal else {
        panic!("Expected clipboard update");
    };
}

fn event_hal_to_sdl() {
    let sdl = SDL_Event::from(&Event::Quit);
    assert!(unsafe { sdl.quit.r#type } == SDL_EVENT_QUIT);
}
