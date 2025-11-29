use halcyon::ttf::{Font, TtfContext};

fn main() {
    let ctx = TtfContext::new().unwrap();
    let font: Font;

    {
        font = Font::new(&ctx, c"", 16.0).unwrap();
    }

    println!("{}", font.family());
}
