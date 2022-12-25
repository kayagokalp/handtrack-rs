/// Represents a detected object. Contains the detected object's location.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectionBox {
    /// The location of the box.
    pub rect: Rectangle,
    /// The score for this box.
    pub score: f32,
}

/// Represents a rectangular area. Contains locations for each point of the rectangle.
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    /// Top left point.
    pub lt: Point,
    /// Bottom right point.
    pub rb: Point,
}

/// Represents a single point.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    /// Creates a new `Point`.
    pub(crate) fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl Rectangle {
    /// Creates a new `Rectangle`.
    pub(crate) fn new(lt: Point, rb: Point) -> Self {
        Self { lt, rb }
    }
}

impl DetectionBox {
    /// Creates a new `DetectionBox`.
    pub(crate) fn new(rect: Rectangle, score: f32) -> Self {
        Self { rect, score }
    }
}
