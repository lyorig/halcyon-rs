fn main() {
    #[cfg(target_os = "macos")]
    println!(r"cargo:rustc-link-search=/opt/homebrew/lib");
}
