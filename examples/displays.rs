use halcyon::{context::Context, defs::SdlResult, display, subsystem::Video};

fn outer() -> SdlResult {
    let context = unsafe { Context::new() };
    let _video = Video::new(&context);

    for (i, disp) in display::all()?.iter().copied().enumerate() {
        println!(
            "Display #{}: \"{}\", bounds {} (usable {})",
            i,
            display::name(disp),
            display::bounds(disp),
            display::bounds_usable(disp),
        )
    }

    let p = display::primary()?;
    println!(
        "Primary display has ID {} and name \"{}\"",
        p,
        display::name(p)
    );

    println!("All primary desktop display modes:");
    for (x, y, hz) in display::display_modes(p.into())?
        .iter()
        .map(|dm| (dm.w, dm.h, dm.refresh_rate))
    {
        println!("{x}x{y}, {hz} Hz");
    }

    Ok(())
}

fn main() {
    if let Err(e) = outer() {
        println!("Something went wrong: {}", e.to_string_lossy());
    }
}
