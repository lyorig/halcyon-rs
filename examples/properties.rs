use std::f32::consts::PI;

use halcyon::properties::Properties;

fn run() -> halcyon::Result<()> {
    let props = Properties::new()?;
    props.set_number(c"Integertastic", 20)?;
    props.set_string(c"Placeholder", Some(c"Lorem ipsum"))?;
    props.set_bool(c"Not a lie", true)?;
    props.set_pointer(c"Null and void", std::ptr::null_mut())?;
    props.set_float(c"All the digits", PI)?;

    props.enumerate(|k, v| println!("\"{k}\" = {v}"))?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Oops, error: \"{e}\"");
    }
}
