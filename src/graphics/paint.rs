// SigmaOS Sovereign AI-Native Photo Editing Suite (SigmaPaint)
// Designed for high-performance raster image canvas and layer filtering

/// Image processing error states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhotoError {
    Success = 0,
    InvalidDimensions = 1,
    LayerOutOfBounds = 2,
    NotSupported = 3,
    ProcessingFailed = 4,
}

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        ColorRgba { r, g, b, a }
    }
}

/// Layer blend modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
}

/// Base OOP interface representing any image processing filter
pub trait ImageFilter {
    fn apply_filter(
        &self,
        width: u32,
        height: u32,
        pixels: &mut [ColorRgba],
    ) -> Result<(), PhotoError>;
}

/// Base OOP interface representing a composite layer inside a Canvas
pub trait CanvasLayer {
    fn name(&self) -> &str;
    fn opacity(&self) -> f32; // 0.0 to 1.0
    fn blend_mode(&self) -> BlendMode;
    fn get_pixels(&self) -> &[ColorRgba];
    fn get_pixels_mut(&mut self) -> &mut [ColorRgba];
}

// ==========================================
// 1. Concrete Canvas Layer Implementation
// ==========================================

pub struct RasterLayer {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub pixels: Vec<ColorRgba>,
}

impl RasterLayer {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut pixels = Vec::new();
        for _ in 0..size {
            pixels.push(ColorRgba::new(0, 0, 0, 0));
        }
        RasterLayer {
            name,
            width,
            height,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            pixels,
        }
    }
}

impl CanvasLayer for RasterLayer {
    fn name(&self) -> &str {
        &self.name
    }
    fn opacity(&self) -> f32 {
        self.opacity
    }
    fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }
    fn get_pixels(&self) -> &[ColorRgba] {
        &self.pixels
    }
    fn get_pixels_mut(&mut self) -> &mut [ColorRgba] {
        &mut self.pixels
    }
}

// ==========================================
// 2. Concrete Convolution Gaussian Blur Filter
// ==========================================

pub struct GaussianBlurFilter {
    pub radius: u32,
}

impl GaussianBlurFilter {
    pub fn new(radius: u32) -> Self {
        GaussianBlurFilter { radius }
    }
}

impl ImageFilter for GaussianBlurFilter {
    fn apply_filter(
        &self,
        width: u32,
        height: u32,
        pixels: &mut [ColorRgba],
    ) -> Result<(), PhotoError> {
        if width == 0 || height == 0 || pixels.len() != (width * height) as usize {
            return Err(PhotoError::InvalidDimensions);
        }

        // Simple box-blur representing convolution filter for valid no_std environments
        let mut temp_pixels = Vec::new();
        for &p in pixels.iter() {
            temp_pixels.push(p);
        }

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let idx = (y * width + x) as usize;

                // Average 3x3 surrounding pixels
                let mut sum_r: u32 = 0;
                let mut sum_g: u32 = 0;
                let mut sum_b: u32 = 0;
                let mut sum_a: u32 = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let offset_idx =
                            (((y as i32 + dy) * width as i32) + (x as i32 + dx)) as usize;
                        let p = temp_pixels[offset_idx];
                        sum_r += p.r as u32;
                        sum_g += p.g as u32;
                        sum_b += p.b as u32;
                        sum_a += p.a as u32;
                    }
                }

                pixels[idx] = ColorRgba::new(
                    (sum_r / 9) as u8,
                    (sum_g / 9) as u8,
                    (sum_b / 9) as u8,
                    (sum_a / 9) as u8,
                );
            }
        }

        Ok(())
    }
}

// ==========================================
// 3. Complete Color Space Conversion Filter
// ==========================================

pub struct GrayscaleConversionFilter;

impl ImageFilter for GrayscaleConversionFilter {
    fn apply_filter(
        &self,
        _width: u32,
        _height: u32,
        pixels: &mut [ColorRgba],
    ) -> Result<(), PhotoError> {
        for pixel in pixels.iter_mut() {
            // Standard NTSC Grayscale coefficients
            let gray =
                (0.299 * pixel.r as f32 + 0.587 * pixel.g as f32 + 0.114 * pixel.b as f32) as u8;
            pixel.r = gray;
            pixel.g = gray;
            pixel.b = gray;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raster_layer_creation() {
        let layer = RasterLayer::new("Background".to_string(), 10, 10);
        assert_eq!(layer.name(), "Background");
        assert_eq!(layer.get_pixels().len(), 100);
    }

    #[test]
    fn test_grayscale_filter() {
        let mut layer = RasterLayer::new("Layer 1".to_string(), 2, 2);
        layer.get_pixels_mut()[0] = ColorRgba::new(100, 150, 200, 255);
        let filter = GrayscaleConversionFilter;
        filter.apply_filter(2, 2, layer.get_pixels_mut()).unwrap();
        let p = layer.get_pixels()[0];
        assert_eq!(p.r, p.g);
        assert_eq!(p.g, p.b);
    }
}
||||||| 43be3a7e8
// SigmaOS Sovereign AI-Native Photo Editing Suite (SigmaPaint)
// Designed for high-performance raster image canvas and layer filtering

use std::collections::HashMap;

/// Image processing error states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhotoError {
    Success = 0,
    InvalidDimensions = 1,
    LayerOutOfBounds = 2,
    NotSupported = 3,
    ProcessingFailed = 4,
}

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        ColorRgba { r, g, b, a }
    }
}

/// Layer blend modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
}

/// Base OOP interface representing any image processing filter
pub trait ImageFilter {
    fn apply_filter(&self, width: u32, height: u32, pixels: &mut [ColorRgba]) -> Result<(), PhotoError>;
}

/// Base OOP interface representing a composite layer inside a Canvas
pub trait CanvasLayer {
    fn name(&self) -> &str;
    fn opacity(&self) -> f32; // 0.0 to 1.0
    fn blend_mode(&self) -> BlendMode;
    fn get_pixels(&self) -> &[ColorRgba];
    fn get_pixels_mut(&mut self) -> &mut [ColorRgba];
}

// ==========================================
// 1. Concrete Canvas Layer Implementation
// ==========================================

pub struct RasterLayer {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub pixels: Vec<ColorRgba>,
}

impl RasterLayer {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut pixels = Vec::new();
        for _ in 0..size {
            pixels.push(ColorRgba::new(0, 0, 0, 0));
        }
        RasterLayer {
            name,
            width,
            height,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            pixels,
        }
    }
}

impl CanvasLayer for RasterLayer {
    fn name(&self) -> &str {
        &self.name
    }
    fn opacity(&self) -> f32 {
        self.opacity
    }
    fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }
    fn get_pixels(&self) -> &[ColorRgba] {
        &self.pixels
    }
    fn get_pixels_mut(&mut self) -> &mut [ColorRgba] {
        &mut self.pixels
    }
}

// ==========================================
// 2. Concrete Convolution Gaussian Blur Filter
// ==========================================

pub struct GaussianBlurFilter {
    pub radius: u32,
}

impl GaussianBlurFilter {
    pub fn new(radius: u32) -> Self {
        GaussianBlurFilter { radius }
    }
}

impl ImageFilter for GaussianBlurFilter {
    fn apply_filter(&self, width: u32, height: u32, pixels: &mut [ColorRgba]) -> Result<(), PhotoError> {
        if width == 0 || height == 0 || pixels.len() != (width * height) as usize {
            return Err(PhotoError::InvalidDimensions);
        }

        // Simple box-blur representing convolution filter for valid no_std environments
        let mut temp_pixels = Vec::new();
        for &p in pixels.iter() {
            temp_pixels.push(p);
        }

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let idx = (y * width + x) as usize;

                // Average 3x3 surrounding pixels
                let mut sum_r: u32 = 0;
                let mut sum_g: u32 = 0;
                let mut sum_b: u32 = 0;
                let mut sum_a: u32 = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let offset_idx = (((y as i32 + dy) * width as i32) + (x as i32 + dx)) as usize;
                        let p = temp_pixels[offset_idx];
                        sum_r += p.r as u32;
                        sum_g += p.g as u32;
                        sum_b += p.b as u32;
                        sum_a += p.a as u32;
                    }
                }

                pixels[idx] = ColorRgba::new(
                    (sum_r / 9) as u8,
                    (sum_g / 9) as u8,
                    (sum_b / 9) as u8,
                    (sum_a / 9) as u8,
                );
            }
        }

        Ok(())
    }
}

// ==========================================
// 3. Complete Color Space Conversion Filter
// ==========================================

pub struct GrayscaleConversionFilter;

impl ImageFilter for GrayscaleConversionFilter {
    fn apply_filter(&self, _width: u32, _height: u32, pixels: &mut [ColorRgba]) -> Result<(), PhotoError> {
        for pixel in pixels.iter_mut() {
            // Standard NTSC Grayscale coefficients
            let gray = (0.299 * pixel.r as f32 + 0.587 * pixel.g as f32 + 0.114 * pixel.b as f32) as u8;
            pixel.r = gray;
            pixel.g = gray;
            pixel.b = gray;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raster_layer_creation() {
        let layer = RasterLayer::new("Background".to_string(), 10, 10);
        assert_eq!(layer.name(), "Background");
        assert_eq!(layer.get_pixels().len(), 100);
    }

    #[test]
    fn test_grayscale_filter() {
        let mut layer = RasterLayer::new("Layer 1".to_string(), 2, 2);
        layer.get_pixels_mut()[0] = ColorRgba::new(100, 150, 200, 255);
        let filter = GrayscaleConversionFilter;
        filter.apply_filter(2, 2, layer.get_pixels_mut()).unwrap();
        let p = layer.get_pixels()[0];
        assert_eq!(p.r, p.g);
        assert_eq!(p.g, p.b);
    }
}
