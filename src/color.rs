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
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl<T: OpacityBounds> Rgb<T> {
    pub const BLACK: Self = Self::new(T::MIN_OPACITY, T::MIN_OPACITY, T::MIN_OPACITY);
    pub const RED: Self = Self::new(T::MAX_OPACITY, T::MIN_OPACITY, T::MIN_OPACITY);
    pub const GREEN: Self = Self::new(T::MIN_OPACITY, T::MAX_OPACITY, T::MIN_OPACITY);
    pub const BLUE: Self = Self::new(T::MIN_OPACITY, T::MIN_OPACITY, T::MAX_OPACITY);
    pub const CYAN: Self = Self::new(T::MIN_OPACITY, T::MAX_OPACITY, T::MAX_OPACITY);
    pub const WHITE: Self = Self::new(T::MAX_OPACITY, T::MAX_OPACITY, T::MAX_OPACITY);
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
    pub const fn new(rgb: Rgb<T>, a: T) -> Self {
        Self { rgb, a }
    }

    pub const fn opaque(rgb: Rgb<T>) -> Self {
        Self::new(rgb, T::MAX_OPACITY)
    }

    pub const fn rgb(r: T, g: T, b: T) -> Self {
        Self::rgba(r, g, b, T::MAX_OPACITY)
    }

    pub const fn rgba(r: T, g: T, b: T, a: T) -> Self {
        Self::new(Rgb::new(r, g, b), a)
    }

    pub const BLACK: Self = Self::opaque(Rgb::BLACK);
    pub const RED: Self = Self::opaque(Rgb::RED);
    pub const GREEN: Self = Self::opaque(Rgb::GREEN);
    pub const BLUE: Self = Self::opaque(Rgb::BLUE);
    pub const CYAN: Self = Self::opaque(Rgb::CYAN);
    pub const WHITE: Self = Self::opaque(Rgb::WHITE);

    pub const TRANSPARENT: Self = Self::new(Rgb::BLACK, T::MIN_OPACITY);
}

impl RgbaU8 {
    /// Create an `RgbaU8` from a hex (0xRRGGBB) representation.
    /// This forwards the extracted red, green, and blue components
    /// to `RgbaU8::rgb()`.
    pub const fn rgb_hex(val: u32) -> Self {
        Self::rgb((val >> 16) as u8, (val >> 8) as u8, val as u8)
    }

    /// Create an `RgbaU8` from a hex (0xRRGGBBAA) representation.
    /// This forwards the extracted red, green, blue, and alpha components
    /// to `RgbaU8::rgba()`.
    pub const fn rgba_hex(val: u32) -> Self {
        Self::rgba(
            (val >> 24) as u8,
            (val >> 16) as u8,
            (val >> 8) as u8,
            val as u8,
        )
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
