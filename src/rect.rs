#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Point<T: Default> {
    pub x: T,
    pub y: T,
}

pub type PointI = Point<i32>;
pub type PointF = Point<f32>;

impl<T: Default> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Rect<T: Default> {
    pub pos: Point<T>,
    pub size: Point<T>,
}

pub type RectI = Rect<i32>;
pub type RectF = Rect<f32>;

impl<T: Default> Rect<T> {
    /// Create a `Rect` with all fields specified.
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Self::from_points(Point::new(x, y), Point::new(w, h))
    }

    /// Create a `Rect` from a pair of points (the internal representation).
    pub fn from_points(pos: Point<T>, size: Point<T>) -> Self {
        Self { pos, size }
    }

    /// Convenience function, identical to `RectI::new(T::default(), T::default(), w, h)`.
    pub fn sized(w: T, h: T) -> Self {
        Self::new(T::default(), T::default(), w, h)
    }
}
