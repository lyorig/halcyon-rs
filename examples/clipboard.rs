use std::ffi::CStr;

use halcyon::{Context, Result, clipboard, subsystem::Video};

const DESIRED_MIME: &CStr = c"image/png";

fn run() -> Result {
    let ctx = Context::new();
    let _vid = Video::new(&ctx)?;

    if clipboard::has_data(DESIRED_MIME) {
        halcyon::log!("Clipboard has MIME data");
        halcyon::log!("-- begin MIME type enumeration --");
        for ptr in clipboard::mime_types()?.iter() {
            let cs = unsafe { CStr::from_ptr(*ptr) };
            halcyon::log!("{}", cs.to_string_lossy());
        }
        halcyon::log!("-- end MIME type enumeration --");

        let data = clipboard::data(DESIRED_MIME)?;
        halcyon::log!("Clipboard data is {} bytes", data.len());
    } else if clipboard::has_text() {
        halcyon::log!("Clipboard has text");
        halcyon::log!("Text: \"{}\"", clipboard::text());
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        halcyon::log!("Something went wrong: {e}");
    }
}
