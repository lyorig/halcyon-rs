use std::fmt::Display;

/// Wrapper around `SDL_(F)Point`, can be transmuted.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub type PointI32 = Point<i32>;
pub type PointF32 = Point<f32>;

impl<T: Default> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

/// Wrapper around `SDL_(F)Rect`, can be transmuted.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Rect<T> {
    pub pos: Point<T>,
    pub size: Point<T>,
}

pub type RectI32 = Rect<i32>;
pub type RectF32 = Rect<f32>;

impl<T: Default> Rect<T> {
    /// Create a `Rect` with all fields specified.
    pub const fn new(x: T, y: T, w: T, h: T) -> Self {
        Self {
            pos: Point::new(x, y),
            size: Point::new(w, h),
        }
    }

    /// Convenience function, identical to `Rect::new(T::default(), T::default(), w, h)`.
    pub fn sized(w: T, h: T) -> Self {
        Self::new(T::default(), T::default(), w, h)
    }
}

impl<T: Display> Display for Rect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}x{})", self.pos, self.size.x, self.size.y)
    }
}
