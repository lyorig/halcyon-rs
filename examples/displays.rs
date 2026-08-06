use halcyon::{Context, Result, display::DisplayHandle, subsystem::Video};

fn run() -> Result {
    let context = Context::new();
    let _video: Video = context.init()?;

    for (i, disp) in DisplayHandle::all()?.iter().copied().enumerate() {
        halcyon::log!(
            "Display #{}: \"{}\", bounds {} (usable {}), content scale = {:.2}",
            i,
            disp.name()?.to_string_lossy(),
            disp.bounds()?,
            disp.bounds_usable()?,
            disp.content_scale()?
        )
    }

    let p = DisplayHandle::primary()?;
    halcyon::log!(
        "Primary display has ID {} and name \"{}\"",
        p.id().0,
        p.name()?.to_string_lossy()
    );

    halcyon::log!("All primary desktop display modes:");
    for (x, y, hz) in p.modes()?.iter().map(|dm| {
        let dm = unsafe { dm.read() };
        (dm.w, dm.h, dm.refresh_rate)
    }) {
        halcyon::log!("{x}x{y}, {hz} Hz");
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        halcyon::log!("Something went wrong: {e}");
    }
}
