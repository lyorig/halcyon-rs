use sdl3_sys::pixels::*;

/// "Sub-struct" of [`Rgba`], because some SDL functions only use
/// the RGB components and don't require the alpha.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
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

    pub const fn to_rgba(&self) -> Rgba<T> {
        Rgba {
            rgb: *self,
            a: T::MAX_OPACITY,
        }
    }

    pub const fn with_alpha(&self, a: T) -> Rgba<T> {
        Rgba { rgb: *self, a }
    }
}

impl From<RgbU8> for RgbF32 {
    fn from(value: RgbU8) -> Self {
        Self::new(
            value.r as f32 / 255.0,
            value.g as f32 / 255.0,
            value.b as f32 / 255.0,
        )
    }
}

impl From<RgbF32> for RgbU8 {
    fn from(value: RgbF32) -> Self {
        Self::new(
            (value.r * 255.0) as _,
            (value.g * 255.0) as _,
            (value.b * 255.0) as _,
        )
    }
}

impl From<RgbaU8> for RgbaF32 {
    fn from(value: RgbaU8) -> Self {
        Self::new(
            value.rgb.r as f32 / 255.0,
            value.rgb.g as f32 / 255.0,
            value.rgb.b as f32 / 255.0,
            value.a as f32 / 255.0,
        )
    }
}

impl From<RgbaF32> for RgbaU8 {
    fn from(value: RgbaF32) -> Self {
        Self::new(
            (value.rgb.r * 255.0) as _,
            (value.rgb.g * 255.0) as _,
            (value.rgb.b * 255.0) as _,
            (value.a * 255.0) as _,
        )
    }
}

/// Wrapper around [`SDL_Color`]. Can be transmuted.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba<T> {
    pub rgb: Rgb<T>,
    pub a: T,
}

pub type RgbaU8 = Rgba<u8>;
pub type RgbaF32 = Rgba<f32>;

pub trait OpacityBounds: Copy {
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
    pub const fn rgb(r: T, g: T, b: T) -> Self {
        Self::new(r, g, b, T::MAX_OPACITY)
    }

    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self {
            rgb: Rgb::new(r, g, b),
            a,
        }
    }

    pub const BLACK: Self = Rgb::BLACK.to_rgba();
    pub const RED: Self = Rgb::RED.to_rgba();
    pub const GREEN: Self = Rgb::GREEN.to_rgba();
    pub const BLUE: Self = Rgb::BLUE.to_rgba();
    pub const CYAN: Self = Rgb::CYAN.to_rgba();
    pub const WHITE: Self = Rgb::WHITE.to_rgba();

    pub const TRANSPARENT: Self = Rgb::BLACK.with_alpha(T::MIN_OPACITY);
}

impl RgbaU8 {
    /// Create an [`RgbaU8`] from a hex (0xRRGGBB) representation.
    /// This forwards the extracted red, green, and blue components
    /// to [`RgbaU8::rgb()`].
    pub const fn rgb_hex(val: u32) -> Self {
        Self::rgb((val >> 16) as u8, (val >> 8) as u8, val as u8)
    }

    /// Create an [`RgbaU8`] from a hex (0xRRGGBBAA) representation.
    /// This forwards the extracted red, green, blue, and alpha components
    /// to [`RgbaU8::new()`].
    pub const fn rgba_hex(val: u32) -> Self {
        Self::new(
            (val >> 24) as u8,
            (val >> 16) as u8,
            (val >> 8) as u8,
            val as u8,
        )
    }
}

impl<T: OpacityBounds> From<Rgb<T>> for Rgba<T> {
    fn from(value: Rgb<T>) -> Self {
        value.to_rgba()
    }
}

impl From<RgbaU8> for SDL_Color {
    fn from(value: RgbaU8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<SDL_Color> for RgbaU8 {
    fn from(value: SDL_Color) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<RgbaF32> for SDL_FColor {
    fn from(value: RgbaF32) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<SDL_FColor> for RgbaF32 {
    fn from(value: SDL_FColor) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
