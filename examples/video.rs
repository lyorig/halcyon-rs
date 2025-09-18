use halcyon::{
    color::Color,
    context::Context,
    defs::SdlResult,
    renderer::RendererBuilder,
    subsystem::Video,
    surface::Surface,
    texture::Texture,
    window::{Window, WindowBuilder},
};

use sdl3_sys::pixels::SDL_PixelFormat;

fn filled_surface(c: Color) -> SdlResult<Surface> {
    let surf = Surface::from_size_and_format((128, 128), SDL_PixelFormat::RGB24)?;
    surf.fill(None, c)?;
    Ok(surf)
}

/// SAFETY: Only call this on the main thread!
unsafe fn run() -> SdlResult {
    let ctx = unsafe { Context::new() };
    let _vid = Video::new(&ctx).expect("Video creation failed");

    let wnd = WindowBuilder::new()
        .position((Window::POS_CENTERED, Window::POS_CENTERED))
        .title(c"Halcyon Example")
        .size((640, 480))
        .build()?;

    wnd.sync()?;

    let rnd = RendererBuilder::new(&wnd).vsync(1).build()?;
    let tex = Texture::from_surface(&rnd, &filled_surface(Color::CYAN)?)?;

    for _ in 0..240 {
        let _ = rnd.clear();
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
