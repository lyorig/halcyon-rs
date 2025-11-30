use halcyon::ttf::{Font, TtfContext};

fn main() {
    let font: Font;

    {
        font = unsafe { Font::new_unchecked(c"", 16.0).unwrap() };
    }

    println!("{}", font.family());
}
