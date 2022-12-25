use anyhow::Result;
use image::GenericImageView;
use std::path::Path;
use tensorflow::Tensor;

use crate::utils::pixel::Pixel;

#[derive(Debug, Clone)]
/// TODO: Implement resizing, recoloring etc.
/// Represents a single frame, an image.
pub struct Image {
    /// Width of the image.
    pub width: u64,
    /// Height of the image.
    pub height: u64,
    /// Actual pixel color values of the image.
    pub pixels: Vec<Pixel>,
}

impl Image {
    /// Creates a new `Image` from given `width`, `height` and `pixels`.
    pub fn new(width: u64, height: u64, pixels: Vec<Pixel>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    /// Read a image file from disk and create a `Image`.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let image_file = image::open(path)?;
        let height = image_file.height() as u64;
        let width = image_file.width() as u64;
        let pixels = image_file
            .pixels()
            .map(|(x, y, pixel)| Pixel::new(pixel[0], pixel[1], pixel[2], x, y))
            .collect();
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    /// Creates a new `Tensor` input from image.
    pub fn tensor(&self) -> Result<Tensor<u8>> {
        let pixel_values: Vec<u8> = self
            .pixels
            .iter()
            .flat_map(|pixel| [pixel.r, pixel.g, pixel.b])
            .collect();
        let input = Tensor::new(&[1, self.height, self.width, 3]).with_values(&pixel_values)?;
        Ok(input)
    }
}
