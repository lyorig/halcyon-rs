#[cfg(target_endian = "big")]
compile_error!("The color crate doesn't support big-endian architectures.");

/// A representation of a 24-bit RGBA color.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

macro_rules! hex {
    ($name:ident, $hex_rgb:literal) => {
        pub const $name: Color = Color::from_hex($hex_rgb);
    };
}

impl Color {
    hex!(RED, 0xFF0000);
    hex!(GREEN, 0x00FF00);
    hex!(BLUE, 0x0000FF);

    hex!(CYAN, 0x00FFFF);

    hex!(BLACK, 0x000000);
    hex!(WHITE, 0xFFFFFF);

    /// Construct a new color from red, green, and blue components.
    /// The alpha is set to opaque (`u8::MAX`).
    ///
    /// ```rust
    /// let c = Color::from_rgb(0xAA, 0xBB, 0xCC);
    /// assert_eq!(c.as_bytes(), [0xAA, 0xBB, 0xCC, 0xFF]);
    /// ```
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, u8::MAX)
    }

    /// Construct a new color from red, green, blue, and alpha components.
    ///
    /// ```rust
    /// let c = Color::from_rgba(0xAA, 0xBB, 0xCC, 0xDD);
    /// assert_eq!(c.as_bytes(), [0xAA, 0xBB, 0xCC, 0xDD]);
    /// ```
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Construct a new color from a packed color in the form of 0xRRGGBB.
    /// The alpha is set to opaque (`u8::MAX`).
    ///
    /// ```rust
    /// let c = Color::from_hex(0xAABBCC);
    /// assert_eq!(c.as_bytes(), [0xAA, 0xBB, 0xCC, 0xFF]);
    /// ```
    pub const fn from_hex(rgb: u32) -> Self {
        Self::from_hex_with_alpha((rgb << 8) | u8::MAX as u32)
    }

    /// Construct a new color from a packed color in the form of 0xRRGGBBAA.
    ///
    /// ```rust
    /// let c = Color::from_hex_with_alpha(0xAABBCCDD);
    /// assert_eq!(c.as_bytes(), [0xAA, 0xBB, 0xCC, 0xDD]);
    /// ```
    pub const fn from_hex_with_alpha(rgba: u32) -> Self {
        unsafe { std::mem::transmute_copy(&rgba.swap_bytes()) }
    }

    /// Construct a new color from a packed color in the form of 0xRRGGBB and a separate alpha
    /// component.
    ///
    /// ```rust
    /// let c = Color::from_hex_and_alpha(0xAABBCC, 0xDD);
    /// assert_eq!(c.as_bytes(), [0xAA, 0xBB, 0xCC, 0xDD]);
    /// ```
    pub const fn from_hex_and_alpha(rgb: u32, a: u8) -> Self {
        Self::from_hex_with_alpha(rgb << 8 | a as u32)
    }

    pub const fn as_hex_rgb(&self) -> u32 {
        self.as_hex_rgba() >> 8
    }

    pub const fn as_hex_rgba(&self) -> u32 {
        unsafe { std::mem::transmute_copy::<Self, u32>(self) }.swap_bytes()
    }

    pub const fn as_bytes(&self) -> [u8; 4] {
        unsafe { std::mem::transmute_copy(&self.as_hex_rgba()) }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self::from_rgba(value.0, value.1, value.2, value.3)
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    /// This doesn't perform any operation on the opacity;
    /// rather, the `a` field of `self` is used.
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_rgba(
            self.r.saturating_add(rhs.r),
            self.g.saturating_add(rhs.g),
            self.b.saturating_add(rhs.b),
            self.a,
        )
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    /// This doesn't perform any operation on the opacity;
    /// rather, the `a` field of `self` is used.
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_rgba(
            self.r.saturating_sub(rhs.r),
            self.g.saturating_sub(rhs.g),
            self.b.saturating_sub(rhs.b),
            self.a,
        )
    }
}

impl std::ops::Neg for Color {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_rgb(u8::MAX - self.r, u8::MAX - self.g, u8::MAX - self.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::*;

    #[test]
    fn add() {
        assert_eq!(Color::RED + Color::CYAN, Color::WHITE);
        assert_eq!(Color::WHITE + Color::RED, Color::WHITE);
    }

    #[test]
    fn sub() {
        assert_eq!(Color::WHITE - Color::RED, Color::CYAN);
    }

    #[test]
    fn neg() {
        assert_eq!(-Color::BLACK, Color::WHITE);
        assert_eq!(-Color::RED, Color::CYAN);
    }

    #[test]
    fn conv() {
        assert_eq!(Color::WHITE.as_hex_rgb(), 0xFFFFFF);
        assert_eq!(Color::CYAN.as_hex_rgba(), 0x00FFFF_FF);
    }
}
