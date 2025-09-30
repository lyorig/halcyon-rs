use halcyon::{
    context::Context,
    defs::SdlResult,
    event::{Event, EventIter},
    rect::{Point, Rect},
    renderer::RendererBuilder,
    subsystem::Video,
    window::{Window, WindowBuilder},
};

/// SAFETY: Only call this on the main thread!
unsafe fn run() -> SdlResult {
    let ctx = unsafe { Context::new() };
    let vid = Video::new(&ctx).expect("Video creation failed");

    let wnd = WindowBuilder::new()
        .position((Window::POS_CENTERED, Window::POS_CENTERED))
        .title(c"Halcyon Example")
        .size((640, 480))
        .build(&vid)?;

    wnd.sync()?;

    let rnd = RendererBuilder::new(&wnd).vsync(1).build()?;
    rnd.clear()?;

    println!(
        "Platform = {}, renderer backend = {}",
        Context::platform(),
        rnd.name()
    );

    rnd.set_draw_color((0xFF, 0xFF, 0xFF, 0xFF));
    rnd.draw_line(Point::new(10., 10.), Point::new(128., 64.))?;
    rnd.fill_rect(Rect::new(10., 90., 256., 256.))?;

    rnd.set_draw_color((0x00, 0xFF, 0xFF, 0xFF));
    rnd.fill_rects(&[
        Rect::new(100., 100., 10., 10.),
        Rect::new(110., 110., 20., 20.),
        Rect::new(130., 130., 20., 20.),
        Rect::new(150., 150., 30., 30.),
    ])?;

    rnd.present()?;

    'main: loop {
        rnd.clear()?;

        for event in EventIter::new() {
            match event {
                Event::Quit => break 'main,
                _ => (),
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = unsafe { run() } {
        println!("An error occurred: {}", e.to_string_lossy());
    }
}
