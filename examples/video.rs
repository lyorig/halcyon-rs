use halcyon::{
    color::Rgba,
    context::Context,
    defs::SdlResult,
    event::{Event, EventIter},
    rect::{Point, Rect},
    renderer::RendererBuilder,
    subsystem::Video,
    traits::Resource,
    window::{Window, WindowBuilder},
};

/// SAFETY: Only call this on the main thread!
unsafe fn run() -> SdlResult {
    let ctx = unsafe { Context::new() };
    let vid = Video::new(&ctx).expect("Video creation failed");

    let wnd = WindowBuilder::new()
        .position(Point::new(Window::POS_CENTERED, Window::POS_CENTERED))
        .title(c"Halcyon Example")
        .size(Point::new(640, 480))
        .build(&vid)?;

    wnd.sync()?;

    let rnd = RendererBuilder::new(wnd.as_ref()).vsync(1).build()?;
    rnd.clear()?;

    println!(
        "Platform = {}, renderer backend = {}",
        Context::platform(),
        rnd.name()
    );

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
        println!("An error occurred: {}", e);
    }
}
