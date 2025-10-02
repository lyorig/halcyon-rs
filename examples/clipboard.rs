use halcyon::{context::Context, subsystem::Video};

fn main() {
    let ctx = unsafe { Context::new() };
    let _vid = Video::new(&ctx).expect("Cannot initialize video subsystem");

    println!(
        "Clipboard content before the program was run: \"{}\"",
        halcyon::clipboard::text()
    );

    halcyon::clipboard::set_text(c"And now I see, with eye serene")
        .expect("Couldn't set clipboard text");

    println!("New clipboard content: \"{}\"", halcyon::clipboard::text());
}
