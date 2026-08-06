#![windows_subsystem = "windows"]

use std::mem::ManuallyDrop;

use halcyon::{
    Context, Result,
    color::Rgba,
    event::{Event, EventIter},
    log::Category,
    properties::Properties,
    rect::{Point, Rect},
    renderer::{Renderer, RendererProperties},
    resource::Resource,
    subsystem::Video,
    window::Window,
};

/// You can make the output of this function visible by setting
/// the env var `SDL_LOGGING=trace`, among others.
fn print_properties(props: RendererProperties) {
    halcyon::log!("Renderer name: \"{}\"", props.name());
    halcyon::log!("HDR enabled: {}", props.hdr_enabled());
    halcyon::log!("HDR headroom: {}", props.hdr_headroom());
    halcyon::log!("Max texture size: {} px", props.max_texture_size());
    halcyon::log!("# of texture formats: {}", props.texture_formats().len());
}

fn run() -> Result {
    let ctx = Context::new();
    let _vid = ManuallyDrop::new(Video::new(&ctx)?);

    let props = Properties::global()?;

    let wnd = Window::builder(props)
        .position(Point::new(Window::POS_CENTERED, Window::POS_CENTERED))
        .title(c"Halcyon Example")
        .size(Point::new(640, 480))
        .build_cleanup()?;

    wnd.sync()?;

    let rnd = Renderer::builder(props)
        .window(wnd.as_ref())
        .vsync(1)
        .build_cleanup()?;

    rnd.clear()?;

    print_properties(rnd.properties());

    halcyon::log_trace!("Platform = {}", halcyon::platform());

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
    if let Err(e) = run() {
        halcyon::log_error!(Category::Error, "An error occurred: {}", e);
    }
}
