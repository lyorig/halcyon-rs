use halcyon::{
    context::Context,
    defs::SdlResult,
    event::{Event, EventIter},
    renderer::RendererBuilder,
    subsystem::Video,
    surface::Surface,
    texture::Texture,
    window::{Window, WindowBuilder},
};

use sdl3_sys::pixels::SDL_PixelFormat;

fn filled_surface(c: (u8, u8, u8, u8)) -> SdlResult<Surface> {
    let surf = Surface::from_size_and_format((128, 128), SDL_PixelFormat::RGB24)?;
    surf.fill(None, c)?;
    Ok(surf)
}

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
    let tex = Texture::from_surface(&rnd, &filled_surface((0x00, 0xFF, 0xFF, 0x00))?)?;

    'main: loop {
        let _ = rnd.clear();

        for event in EventIter::new() {
            match event {
                Event::Quit => break 'main,
                _ => (),
            }
        }

        let _ = rnd.draw(&tex, None, None);
        let _ = rnd.present();
    }

    Ok(())
}

fn main() {
    if let Err(e) = unsafe { run() } {
        println!("Nope, err: {}", e.to_string_lossy());
    } else {
        println!("All fine!")
    }
}
