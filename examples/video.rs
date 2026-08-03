#![windows_subsystem = "windows"]

use std::ffi::CStr;

use halcyon::{
    Context, Result,
    color::Rgba,
    event::{Event, EventIter},
    log::Category,
    properties::Property,
    rect::{Point, Rect},
    renderer::RendererBuilder,
    resource::Resource,
    subsystem::Video,
    window::{Window, WindowBuilder},
};

/// You can make the output of this function visible by setting
/// the env var `SDL_LOGGING=trace`, among others.
fn prop_enum(key: &CStr, value: Property) {
    halcyon::log_trace!("Property {} = {}", key.to_string_lossy(), value);
}

/// SAFETY: Only call this on the main thread!
unsafe fn run() -> Result {
    let ctx = Context::new();
    let _vid = Video::new(&ctx).expect("Video creation failed");

    let wnd = WindowBuilder::new()
        .position(Point::new(Window::POS_CENTERED, Window::POS_CENTERED))
        .title(c"Halcyon Example")
        .size(Point::new(640, 480))
        .build()?;

    wnd.sync()?;

    let rnd = RendererBuilder::new(wnd.as_ref()).vsync(1).build()?;
    rnd.clear()?;

    halcyon::log_trace!(
        "Platform = {}, renderer backend = {}",
        halcyon::platform(),
        rnd.name()
    );

    halcyon::log_trace!("Window properties:");
    wnd.properties().enumerate(prop_enum)?;

    halcyon::log_trace!("Renderer properties:");
    rnd.properties().enumerate(prop_enum)?;

    rnd.set_draw_color_f32(Rgba::rgb(1., 1., 1.));
    rnd.draw_line(Point::new(10., 10.), Point::new(128., 64.))?;
    rnd.fill_rect(Rect::xywh(10., 90., 256., 256.))?;

    rnd.set_draw_color_f32(Rgba::rgb(0., 1., 1.));
    rnd.fill_rects(&[
        Rect::xywh(100., 100., 10., 10.),
        Rect::xywh(110., 110., 20., 20.),
        Rect::xywh(130., 130., 20., 20.),
        Rect::xywh(150., 150., 30., 30.),
    ])?;

    rnd.present()?;

    'main: loop {
        rnd.clear()?;

        for event in EventIter::new() {
            if let Event::Quit = event {
                break 'main;
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = unsafe { run() } {
        halcyon::log_error!(Category::Error, "An error occurred: {}", e);
    }
}
