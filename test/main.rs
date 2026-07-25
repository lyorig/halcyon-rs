use sdl3_sys::{events::*, pixels::SDL_PixelFormat, render::SDL_TextureAccess};

use halcyon::{
    context::Context,
    event::{Event, EventIter},
    rect::Point,
    renderer::RendererBuilder,
    subsystem::Events,
    texture::Texture,
    traits::Resource,
    window::WindowBuilder,
};

/// Basic initialization stuff.
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

/// [`SDL_Event`] -> [`Event`] conversion.
fn event_sdl_to_hal() {
    // Manually set the timestamp for testing purposes.
    let ticks = halcyon::ticks_ns();

    let hal = Event::from(&SDL_Event {
        clipboard: SDL_ClipboardEvent {
            r#type: SDL_EVENT_CLIPBOARD_UPDATE,
            timestamp: ticks,
            ..Default::default()
        },
    });

    let Event::ClipboardUpdate(cu) = hal else {
        panic!("Expected clipboard update");
    };

    assert_eq!(cu.timestamp, ticks);
}

/// [`Event`] -> [`SDL_Event`] conversion.
fn event_hal_to_sdl() {
    let mut sdl = SDL_Event::from(&Event::Quit);

    // Manually set the timestamp for testing purposes.
    let ticks = halcyon::ticks_ns();
    sdl.quit.timestamp = ticks;

    assert!(unsafe { sdl.quit.r#type } == SDL_EVENT_QUIT);
    assert!(unsafe { sdl.r#type } == SDL_EVENT_QUIT);
    assert_eq!(unsafe { sdl.quit }.timestamp, ticks);
    assert_eq!(unsafe { sdl.common }.timestamp, ticks);
}

/// [`Event::set_timestamp`].
fn event_timestamp() {
    let mut evt = Event::Quit;
    let ticks = halcyon::ticks_ns();
    evt.set_timestamp(ticks);

    let sdl = SDL_Event::from(&evt);
    assert_eq!(unsafe { sdl.common }.timestamp, ticks);
}

/// [`Event::push()`] testing.
fn event_push() {
    // Should fail, since events aren't initialized.
    Event::Quit.push().unwrap_err();

    // Initialize events.
    let ctx = Context::new();
    let _evts = Events::new(&ctx);

    // Should work now.
    Event::Quit.push().unwrap();

    let evt = EventIter::new().next().unwrap();
    let Event::Quit = evt else {
        panic!("Expected quit event");
    };
}

fn main() {
    init();

    event_sdl_to_hal();
    event_hal_to_sdl();
    event_timestamp();
    event_push();
}
