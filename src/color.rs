/// "Sub-struct" of `Rgba`, because some functions only use the RGB components
/// and don't require the alpha.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub type RgbU8 = Rgb<u8>;
pub type RgbF32 = Rgb<f32>;

impl<T> Rgb<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

/// Wrapper around `SDL_Color`. Can be transmuted.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Rgba<T> {
    pub rgb: Rgb<T>,
    pub a: T,
}

pub type RgbaU8 = Rgba<u8>;
pub type RgbaF32 = Rgba<f32>;

impl<T> Rgba<T> {
    pub fn new(rgb: Rgb<T>, a: T) -> Self {
        Self { rgb, a }
    }

    pub fn rgba(r: T, g: T, b: T, a: T) -> Self {
        Self::new(Rgb::new(r, g, b), a)
    }
}

impl Rgba<u8> {
    pub fn rgb(rgb: Rgb<u8>) -> Self {
        Self { rgb, a: u8::MAX }
    }
}

impl Rgba<f32> {
    pub fn rgb(rgb: Rgb<f32>) -> Self {
        Self { rgb, a: 1. }
    }
}
