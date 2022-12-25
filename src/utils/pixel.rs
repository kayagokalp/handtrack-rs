#[derive(Debug, Clone)]
/// Represents a single pixel in RGB format.
pub struct Pixel {
    /// Red value.
    pub r: u8,
    /// Green value.
    pub g: u8,
    /// Blue value.
    pub b: u8,
    /// X position of the `Pixel`.
    pub x: u32,
    /// Y position of the `Pixel`.
    pub y: u32,
}

impl Pixel {
    /// Creates a new `Pixel` from given R, G, and B values.
    pub fn new(r: u8, g: u8, b: u8, x: u32, y: u32) -> Self {
        Self { r, g, b, x, y }
    }
}
