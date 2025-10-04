use halcyon::{context::Context, defs::SdlResult, display::DisplayHandle, subsystem::Video};

fn outer() -> SdlResult {
    let context = unsafe { Context::new() };
    let _video = Video::new(&context);

    for (i, disp) in DisplayHandle::all()?.iter().copied().enumerate() {
        println!(
            "Display #{}: \"{}\", bounds {} (usable {})",
            i,
            disp.name(),
            disp.bounds(),
            disp.bounds_usable(),
        )
    }

    let p = DisplayHandle::primary()?;
    println!(
        "Primary display has ID {} and name \"{}\"",
        p.id(),
        p.name()
    );

    println!("All primary desktop display modes:");
    for (x, y, hz) in p.display_modes()?.iter().map(|dm| {
        let dm = unsafe { dm.read() };
        (dm.w, dm.h, dm.refresh_rate)
    }) {
        println!("{x}x{y}, {hz} Hz");
    }

    Ok(())
}

fn main() {
    if let Err(e) = outer() {
        println!("Something went wrong: {}", e.to_string_lossy());
    }
}
