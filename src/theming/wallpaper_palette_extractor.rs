#![no_std]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn luminance(&self) -> f32 {
        let rs = self.r as f32 / 255.0;
        let gs = self.g as f32 / 255.0;
        let bs = self.b as f32 / 255.0;

        let r_c = if rs <= 0.03928 {
            rs / 12.92
        } else {
            ((rs + 0.055) / 1.055).powf(2.4)
        };
        let g_c = if gs <= 0.03928 {
            gs / 12.92
        } else {
            ((gs + 0.055) / 1.055).powf(2.4)
        };
        let b_c = if bs <= 0.03928 {
            bs / 12.92
        } else {
            ((bs + 0.055) / 1.055).powf(2.4)
        };

        0.2126 * r_c + 0.7152 * g_c + 0.0722 * b_c
    }

    pub fn contrast_ratio(&self, other: &Color) -> f32 {
        let l1 = self.luminance();
        let l2 = other.luminance();

        if l1 > l2 {
            (l1 + 0.05) / (l2 + 0.05)
        } else {
            (l2 + 0.05) / (l1 + 0.05)
        }
    }
}

pub struct Palette {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub border: Color,
}

pub struct WallpaperPaletteExtractor;

impl WallpaperPaletteExtractor {
    pub fn extract(pixels: &[[u8; 3]]) -> Palette {
        // Simplified mock extraction: return a dark theme palette
        Palette {
            background: Color::new(10, 10, 15),
            foreground: Color::new(240, 240, 245),
            accent: Color::new(100, 150, 255),
            primary: Color::new(80, 120, 200),
            secondary: Color::new(60, 90, 150),
            surface: Color::new(30, 30, 40),
            border: Color::new(50, 50, 60),
        }
    }

    pub fn check_wcag_compliance(bg: &Color, fg: &Color) -> bool {
        bg.contrast_ratio(fg) >= 4.5
    }

    pub fn export_css(palette: &Palette) -> String {
        format!(
            ":root {{\n  --background: {};\n  --foreground: {};\n  --accent: {};\n  --primary: {};\n  --secondary: {};\n  --surface: {};\n  --border: {};\n}}",
            palette.background.to_hex(),
            palette.foreground.to_hex(),
            palette.accent.to_hex(),
            palette.primary.to_hex(),
            palette.secondary.to_hex(),
            palette.surface.to_hex(),
            palette.border.to_hex()
        )
    }

    pub fn export_hyprland(palette: &Palette) -> String {
        format!(
            "$background = rgba({:02x}{:02x}{:02x}ff)\n$foreground = rgba({:02x}{:02x}{:02x}ff)\n$accent = rgba({:02x}{:02x}{:02x}ff)",
            palette.background.r, palette.background.g, palette.background.b,
            palette.foreground.r, palette.foreground.g, palette.foreground.b,
            palette.accent.r, palette.accent.g, palette.accent.b
        )
    }

    pub fn export_ghostty(palette: &Palette) -> String {
        format!(
            "background = {}\nforeground = {}\ncursor-color = {}",
            palette.background.to_hex().replace("#", ""),
            palette.foreground.to_hex().replace("#", ""),
            palette.accent.to_hex().replace("#", "")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_hex() {
        let c = Color::new(255, 0, 128);
        assert_eq!(c.to_hex(), "#ff0080");
    }

    #[test]
    fn test_contrast_ratio() {
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        assert!(black.contrast_ratio(&white) > 20.0);
    }

    #[test]
    fn test_wcag_compliance() {
        let dark_bg = Color::new(10, 10, 10);
        let light_text = Color::new(250, 250, 250);
        assert!(WallpaperPaletteExtractor::check_wcag_compliance(
            &dark_bg,
            &light_text
        ));
    }

    #[test]
    fn test_css_export() {
        let palette = Palette {
            background: Color::new(0, 0, 0),
            foreground: Color::new(255, 255, 255),
            accent: Color::new(255, 0, 0),
            primary: Color::new(0, 255, 0),
            secondary: Color::new(0, 0, 255),
            surface: Color::new(50, 50, 50),
            border: Color::new(100, 100, 100),
        };
        let css = WallpaperPaletteExtractor::export_css(&palette);
        assert!(css.contains("--background: #000000;"));
        assert!(css.contains("--accent: #ff0000;"));
    }

    #[test]
    fn test_hyprland_export() {
        let palette = Palette {
            background: Color::new(0, 0, 0),
            foreground: Color::new(255, 255, 255),
            accent: Color::new(10, 20, 30),
            primary: Color::new(0, 0, 0),
            secondary: Color::new(0, 0, 0),
            surface: Color::new(0, 0, 0),
            border: Color::new(0, 0, 0),
        };
        let hl = WallpaperPaletteExtractor::export_hyprland(&palette);
        assert!(hl.contains("$background = rgba(000000ff)"));
        assert!(hl.contains("$accent = rgba(0a141eff)"));
    }
}
