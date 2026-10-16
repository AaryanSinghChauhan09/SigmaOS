// SPDX-License-Identifier: MIT
// SigmaOS - Color Picker
// Inspired by Gpick and KColorChooser
// Provides color selection and palette management

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Color format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorFormat {
    Hex,
    RGB,
    RGBA,
    HSL,
    HSV,
    CMYK,
}

/// Color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Convert to hex
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    /// Convert to RGB string
    pub fn to_rgb_string(&self) -> String {
        format!("rgb({}, {}, {})", self.red, self.green, self.blue)
    }

    /// Convert to RGBA string
    pub fn to_rgba_string(&self) -> String {
        format!(
            "rgba({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        )
    }

    /// Convert to HSL
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let r = self.red as f32 / 255.0;
        let g = self.green as f32 / 255.0;
        let b = self.blue as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        let l = (max + min) / 2.0;

        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        (h, s, l)
    }

    /// Convert to HSV
    pub fn to_hsv(&self) -> (f32, f32, f32) {
        let r = self.red as f32 / 255.0;
        let g = self.green as f32 / 255.0;
        let b = self.blue as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        (h, s, v)
    }

    /// Convert to CMYK
    pub fn to_cmyk(&self) -> (f32, f32, f32, f32) {
        let r = self.red as f32 / 255.0;
        let g = self.green as f32 / 255.0;
        let b = self.blue as f32 / 255.0;

        let k = 1.0 - r.max(g).max(b);
        let c = (1.0 - r - k) / (1.0 - k);
        let m = (1.0 - g - k) / (1.0 - k);
        let y = (1.0 - b - k) / (1.0 - k);

        (c, m, y, k)
    }

    /// From hex
    pub fn from_hex(hex: &str) -> Result<Color, &'static str> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Invalid hex format");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex")?;

        Ok(Color::new(r, g, b, 255))
    }
}

/// Palette
#[derive(Debug, Clone)]
pub struct Palette {
    pub name: String,
    pub colors: Vec<Color>,
}

/// Color picker
pub struct ColorPicker {
    pub current_color: Color,
    pub palettes: Vec<Palette>,
    pub recent_colors: Vec<Color>,
    pub color_format: ColorFormat,
    pub max_recent: usize,
}

impl ColorPicker {
    pub fn new() -> Self {
        Self {
            current_color: Color::new(255, 255, 255, 255),
            palettes: Vec::new(),
            recent_colors: Vec::new(),
            color_format: ColorFormat::Hex,
            max_recent: 16,
        }
    }

    /// Set color
    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
        self.add_to_recent(color);
    }

    /// Set color from RGB
    pub fn set_color_rgb(&mut self, red: u8, green: u8, blue: u8) {
        self.set_color(Color::new(red, green, blue, 255));
    }

    /// Set color from hex
    pub fn set_color_hex(&mut self, hex: &str) -> Result<(), &'static str> {
        let color = Color::from_hex(hex)?;
        self.set_color(color);
        Ok(())
    }

    /// Get current color
    pub fn get_current_color(&self) -> Color {
        self.current_color
    }

    /// Get color in specified format
    pub fn get_color_string(&self, format: ColorFormat) -> String {
        match format {
            ColorFormat::Hex => self.current_color.to_hex(),
            ColorFormat::RGB => self.current_color.to_rgb_string(),
            ColorFormat::RGBA => self.current_color.to_rgba_string(),
            ColorFormat::HSL => {
                let (h, s, l) = self.current_color.to_hsl();
                format!("hsl({}, {:.0}%, {:.0}%)", h as i32, s * 100.0, l * 100.0)
            }
            ColorFormat::HSV => {
                let (h, s, v) = self.current_color.to_hsv();
                format!("hsv({}, {:.0}%, {:.0}%)", h as i32, s * 100.0, v * 100.0)
            }
            ColorFormat::CMYK => {
                let (c, m, y, k) = self.current_color.to_cmyk();
                format!(
                    "cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)",
                    c * 100.0,
                    m * 100.0,
                    y * 100.0,
                    k * 100.0
                )
            }
        }
    }

    /// Add to recent
    fn add_to_recent(&mut self, color: Color) {
        // Remove if already exists
        self.recent_colors
            .retain(|c| c.red != color.red || c.green != color.green || c.blue != color.blue);

        // Add to front
        self.recent_colors.insert(0, color);

        // Limit size
        while self.recent_colors.len() > self.max_recent {
            self.recent_colors.pop();
        }
    }

    /// Get recent colors
    pub fn get_recent_colors(&self) -> &[Color] {
        &self.recent_colors
    }

    /// Create palette
    pub fn create_palette(&mut self, name: String) {
        let palette = Palette {
            name,
            colors: Vec::new(),
        };
        self.palettes.push(palette);
    }

    /// Add color to palette
    pub fn add_to_palette(
        &mut self,
        palette_index: usize,
        color: Color,
    ) -> Result<(), &'static str> {
        let palette = self
            .palettes
            .get_mut(palette_index)
            .ok_or("Palette not found")?;
        palette.colors.push(color);
        Ok(())
    }

    /// Get palettes
    pub fn get_palettes(&self) -> &[Palette] {
        &self.palettes
    }

    /// Set color format
    pub fn set_color_format(&mut self, format: ColorFormat) {
        self.color_format = format;
    }

    /// Copy color to clipboard
    pub fn copy_color(&self) -> String {
        self.get_color_string(self.color_format)
    }
}

/// Global color picker
static mut COLOR_PICKER: Option<ColorPicker> = None;

/// Initialize color picker
pub fn init_color_picker() {
    unsafe {
        COLOR_PICKER = Some(ColorPicker::new());
    }
}

/// Get color picker
pub fn get_color_picker() -> &'static mut ColorPicker {
    unsafe { COLOR_PICKER.as_mut().expect("Color picker not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_picker_creation() {
        let picker = ColorPicker::new();
        assert_eq!(picker.get_current_color().red, 255);
    }

    #[test]
    fn test_set_color() {
        let mut picker = ColorPicker::new();
        picker.set_color_rgb(128, 64, 32);
        assert_eq!(picker.get_current_color().red, 128);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::new(255, 128, 64, 255);
        assert_eq!(color.to_hex(), "FF8040");
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("FF8040").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_recent_colors() {
        let mut picker = ColorPicker::new();
        picker.set_color_rgb(255, 0, 0);
        picker.set_color_rgb(0, 255, 0);
        assert_eq!(picker.get_recent_colors().len(), 2);
    }

    #[test]
    fn test_create_palette() {
        let mut picker = ColorPicker::new();
        picker.create_palette("My Palette".to_string());
        assert_eq!(picker.get_palettes().len(), 1);
    }
}
