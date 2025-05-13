use std::ffi::CString;

use halcyon::{
    color::Color,
    context::Context,
    renderer::Renderer,
    subsystem::Video,
    surface::Surface,
    texture::Texture,
    window::{Window, WindowFlags},
};

use sdl3_sys::pixels::SDL_PixelFormat;

fn filled_surface(c: Color) -> Result<Surface, CString> {
    let surf = Surface::from_size_and_format((128, 128), SDL_PixelFormat::RGB24)?;
    surf.fill(None, c)?;
    Ok(surf)
}

/// SAFETY: Only call this on the main thread!
unsafe fn run() -> Result<(), CString> {
    let ctx = unsafe { Context::new() };
    let vid = Video::new(&ctx)?;

    let wnd = Window::new(&vid, c"Halcyon", 640, 480, WindowFlags::none())?;
    wnd.sync()?;

    let rnd = Renderer::new(&wnd)?;
    let tex = Texture::from_surface(&rnd, &filled_surface(Color::CYAN)?)?;

    rnd.clear()?;
    rnd.draw(&tex, None, None)?;
    rnd.present()?;

    Ok(())
}

fn main() {
    if let Err(e) = unsafe { run() } {
        println!("Nope, err: {}", e.to_string_lossy());
    }
}
