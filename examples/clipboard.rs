use halcyon::{Context, Result, subsystem::Video};

fn run() -> Result {
    let ctx = Context::new();
    let _vid = Video::new(&ctx)?;

    halcyon::log!(
        "Clipboard content before the program was run: \"{}\"",
        halcyon::clipboard::text()
    );

    halcyon::clipboard::set_text(c"And now I see, with eye serene")?;

    halcyon::log!("New clipboard content: \"{}\"", halcyon::clipboard::text());

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        halcyon::log!("Something went wrong: {e}");
    }
}
