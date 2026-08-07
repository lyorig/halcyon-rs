use halcyon::{Context, Result, display::Display, subsystem::Video};

fn run() -> Result {
    let context = Context::new();
    let _video: Video = context.init()?;

    for (i, disp) in Display::all()?.iter().copied().enumerate() {
        halcyon::log!(
            "Display #{}: \"{}\", bounds {} (usable {}), content scale = {:.2}",
            i,
            disp.name()?.to_string_lossy(),
            disp.bounds()?,
            disp.usable_bounds()?,
            disp.content_scale()?
        )
    }

    let p = Display::primary()?;

    halcyon::log!("All primary desktop display modes:");
    for (x, y, hz) in p.fullscreen_modes()?.iter().map(|dm| {
        let dm = unsafe { dm.read() };
        (dm.w, dm.h, dm.refresh_rate)
    }) {
        halcyon::log!("{x}x{y}, {hz} Hz");
    }

    halcyon::log!(
        "Primary display has ID {}, and name \"{}\"",
        p.id().0,
        p.name()?.to_string_lossy(),
    );

    if let Some(o) = p.current_orientation() {
        halcyon::log!("Current orientation is available and is \"{o}\"");
    }

    if let Some(o) = p.natural_orientation() {
        halcyon::log!("Natural orientation is available and is \"{o}\"");
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        halcyon::log!("Something went wrong: {e}");
    }
}
