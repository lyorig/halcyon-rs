use std::fmt::Display;

/// Wrapper around `SDL_(F)Point`, can be transmuted.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub type PointI32 = Point<i32>;
pub type PointF32 = Point<f32>;

impl From<PointI32> for PointF32 {
    fn from(value: PointI32) -> Self {
        Point::new(value.x as _, value.y as _)
    }
}

impl From<PointF32> for PointI32 {
    fn from(value: PointF32) -> Self {
        Point::new(value.x as _, value.y as _)
    }
}

impl<T: Copy> Point<T> {
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
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Rect<T> {
    pub pos: Point<T>,
    pub size: Point<T>,
}

pub type RectI32 = Rect<i32>;
pub type RectF32 = Rect<f32>;

impl From<RectI32> for RectF32 {
    fn from(value: RectI32) -> Self {
        Rect::new(value.pos.into(), value.size.into())
    }
}

impl From<RectF32> for RectI32 {
    fn from(value: RectF32) -> Self {
        Rect::new(value.pos.into(), value.size.into())
    }
}

impl<T: Default + Copy> Rect<T> {
    pub const fn new(pos: Point<T>, size: Point<T>) -> Self {
        Self { pos, size }
    }

    /// Create a `Rect` with all fields specified.
    pub const fn xywh(x: T, y: T, w: T, h: T) -> Self {
        Self::new(Point::new(x, y), Point::new(w, h))
    }

    /// Convenience function, identical to `Rect::xywh(x, y, T::default(), T::default())`.
    pub fn xy(x: T, y: T) -> Self {
        Self::xywh(x, y, T::default(), T::default())
    }

    /// Convenience function, identical to `Rect::xywh(T::default(), T::default(), w, h)`.
    pub fn wh(w: T, h: T) -> Self {
        Self::xywh(T::default(), T::default(), w, h)
    }
}

impl<T: Display> Display for Rect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}x{})", self.pos, self.size.x, self.size.y)
    }
}
