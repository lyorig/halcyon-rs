use halcyon::{
    color::Color,
    context::Context,
    defs::SdlResult,
    renderer::{self},
    subsystem::Video,
    surface::Surface,
    texture::Texture,
    window::{self, Window},
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
    let vid = Video::new(&ctx).expect("Video creation failed");

    let mut wnd = window::Builder::new(&vid)
        .position((Window::POS_CENTERED, Window::POS_CENTERED))
        .title(c"Halcyon Example")
        .size((640, 480))
        .build()
        .expect("Window creation failed");

    let _ = wnd.sync();

    let rnd = renderer::Builder::new()
        .vsync(1)
        .window(&mut wnd)
        .build()
        .expect("Renderer creation failed");

    let tex = Texture::from_surface(&rnd, &filled_surface(Color::CYAN)?)
        .expect("Texture creation failed");

    for _ in 0..240 {
        let _ = rnd.clear();
        rnd.draw(&tex, None, None).expect("Drawing failed");
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
