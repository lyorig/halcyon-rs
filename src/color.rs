use sdl3_sys::pixels::SDL_Color;

/// "Sub-struct" of `Rgba`, because some SDL functions only use
/// the RGB components and don't require the alpha.
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
#[derive(Clone, Copy)]
pub struct Rgba<T> {
    pub rgb: Rgb<T>,
    pub a: T,
}

pub type RgbaU8 = Rgba<u8>;
pub type RgbaF32 = Rgba<f32>;

pub trait OpacityBounds {
    const MIN_OPACITY: Self;
    const MAX_OPACITY: Self;
}

impl OpacityBounds for u8 {
    const MIN_OPACITY: Self = 0;
    const MAX_OPACITY: Self = Self::MAX;
}

impl OpacityBounds for f32 {
    const MIN_OPACITY: Self = 0.0;
    const MAX_OPACITY: Self = 1.0;
}

impl<T: OpacityBounds> Rgba<T> {
    pub fn new(rgb: Rgb<T>, a: T) -> Self {
        Self { rgb, a }
    }

    pub fn rgba(r: T, g: T, b: T, a: T) -> Self {
        Self::new(Rgb::new(r, g, b), a)
    }

    pub fn rgb(r: T, g: T, b: T) -> Self {
        Self::rgba(r, g, b, T::MAX_OPACITY)
    }
}

impl<T: OpacityBounds> From<Rgb<T>> for Rgba<T> {
    fn from(value: Rgb<T>) -> Self {
        Self {
            rgb: value,
            a: T::MAX_OPACITY,
        }
    }
}

impl From<RgbaU8> for SDL_Color {
    fn from(value: RgbaU8) -> Self {
        unsafe { std::mem::transmute_copy(&value) }
    }
}
