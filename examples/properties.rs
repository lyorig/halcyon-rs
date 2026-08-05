use halcyon::properties::Properties;

fn run() -> halcyon::Result<()> {
    let props = Properties::new()?;
    props.set_number(c"MyAge", 21)?;
    props.set_string(c"ThisProjectName", c"halcyon-rs".as_ptr())?;
    props.enumerate(|k, v| {
        halcyon::log!("Property {} = {}", k.to_string_lossy(), v);
    })?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        halcyon::log!("Oops, error: \"{e}\"");
    }
}
