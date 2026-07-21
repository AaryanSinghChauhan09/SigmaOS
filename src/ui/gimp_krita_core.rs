#![no_std]

extern crate alloc;
use alloc::vec::Vec;

/// Raster Image Editor Core (GIMP/Krita Parity)
/// Non-destructive layer blending and pixel manipulation engine.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
}

#[derive(Debug, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Layer {
    pub name: &'static str,
    pub pixels: Vec<Pixel>,
    pub width: usize,
    pub height: usize,
    pub blend_mode: BlendMode,
    pub opacity: f32, // 0.0 to 1.0
}

pub struct ImageComposition {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
}

impl ImageComposition {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            layers: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        if layer.width == self.width && layer.height == self.height {
            self.layers.push(layer);
        }
    }

    /// Basic blend mode simulation (flatten image)
    pub fn flatten(&self) -> Vec<Pixel> {
        let mut result = alloc::vec![Pixel { r: 0, g: 0, b: 0, a: 255 }; self.width * self.height];
        
        for layer in &self.layers {
            for (i, p) in layer.pixels.iter().enumerate() {
                // Simplified "Normal" blending: just overwrite if opaque.
                // A real implementation would apply alpha and blend modes.
                if p.a > 128 {
                    result[i] = p.clone();
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_flattening() {
        let mut comp = ImageComposition::new(2, 2);
        let layer1 = Layer {
            name: "Background",
            pixels: alloc::vec![Pixel { r: 255, g: 0, b: 0, a: 255 }; 4],
            width: 2, height: 2,
            blend_mode: BlendMode::Normal,
            opacity: 1.0,
        };
        let layer2 = Layer {
            name: "Foreground",
            pixels: alloc::vec![
                Pixel { r: 0, g: 255, b: 0, a: 255 }, Pixel { r: 0, g: 0, b: 0, a: 0 },
                Pixel { r: 0, g: 0, b: 0, a: 0 }, Pixel { r: 0, g: 255, b: 0, a: 255 }
            ],
            width: 2, height: 2,
            blend_mode: BlendMode::Normal,
            opacity: 1.0,
        };
        
        comp.add_layer(layer1);
        comp.add_layer(layer2);
        
        let flat = comp.flatten();
        assert_eq!(flat[0].g, 255); // Top layer pixel
        assert_eq!(flat[1].r, 255); // Bottom layer pixel (top was transparent)
    }
}
