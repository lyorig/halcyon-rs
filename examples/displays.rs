use halcyon::{
    context::Context, defs::SdlResult, display::DisplayHandle, subsystem::Video, util::c_ptr_to_str,
};

fn outer() -> SdlResult {
    let context = unsafe { Context::new() };
    let _video = Video::new(&context);

    for (i, disp) in DisplayHandle::all()?.iter().copied().enumerate() {
        println!(
            "Display #{}: \"{}\", bounds {} (usable {}), content scale = {:.2}",
            i,
            unsafe { c_ptr_to_str(disp.name()?) },
            disp.bounds()?,
            disp.bounds_usable()?,
            disp.content_scale()?
        )
    }

    let p = DisplayHandle::primary()?;
    println!(
        "Primary display has ID {} and name \"{}\"",
        p.id(),
        unsafe { c_ptr_to_str(p.name()?) }
    );

    println!("All primary desktop display modes:");
    for (x, y, hz) in p.modes()?.iter().map(|dm| {
        let dm = unsafe { dm.read() };
        (dm.w, dm.h, dm.refresh_rate)
    }) {
        println!("{x}x{y}, {hz} Hz");
    }

    Ok(())
}

fn main() {
    if let Err(e) = outer() {
        println!("Something went wrong: {}", unsafe { c_ptr_to_str(e) });
    }
}
