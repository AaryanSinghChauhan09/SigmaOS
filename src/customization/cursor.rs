use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Standard Freedesktop / XCursor shape variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CursorShape {
    DefaultPointer,
    TextIBeam,
    HandPointer,
    Crosshair,
    WaitSpinner,
    ResizeNs,
    ResizeEw,
    ResizeNwse,
    ResizeNesw,
    NotAllowed,
    Help,
    Custom,
}

impl CursorShape {
    pub fn to_xcursor_name(&self) -> &'static str {
        match self {
            CursorShape::DefaultPointer => "default",
            CursorShape::TextIBeam => "text",
            CursorShape::HandPointer => "pointer",
            CursorShape::Crosshair => "crosshair",
            CursorShape::WaitSpinner => "wait",
            CursorShape::ResizeNs => "ns-resize",
            CursorShape::ResizeEw => "ew-resize",
            CursorShape::ResizeNwse => "nwse-resize",
            CursorShape::ResizeNesw => "nesw-resize",
            CursorShape::NotAllowed => "not-allowed",
            CursorShape::Help => "help",
            CursorShape::Custom => "custom",
        }
    }
}

/// Single frame of an animated or static cursor image
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorImageFrame {
    pub width: u32,
    pub height: u32,
    pub hotspot_x: u32,
    pub hotspot_y: u32,
    pub delay_ms: u32,
    pub rgba_pixels: Vec<u8>,
}

/// A named Cursor Theme (e.g. Adwaita, Breeze, Bibata Modern, Yaru)
#[derive(Debug, Clone)]
pub struct CursorTheme {
    pub name: String,
    pub author: String,
    pub base_size: u32,
    pub cursors: BTreeMap<CursorShape, Vec<CursorImageFrame>>,
}

impl CursorTheme {
    pub fn new(name: &str, author: &str, base_size: u32) -> Self {
        Self {
            name: name.to_string(),
            author: author.to_string(),
            base_size,
            cursors: BTreeMap::new(),
        }
    }

    pub fn register_cursor_shape(&mut self, shape: CursorShape, frames: Vec<CursorImageFrame>) {
        self.cursors.insert(shape, frames);
    }
}

/// Freedesktop & XCursor Parity Cursor Theme Manager
pub struct CursorThemeEngine {
    pub active_theme_name: String,
    pub active_size: u32,
    pub themes: BTreeMap<String, CursorTheme>,
    pub current_shape: CursorShape,
    pub current_frame_index: usize,
}

impl CursorThemeEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            active_theme_name: "Adwaita".to_string(),
            active_size: 24,
            themes: BTreeMap::new(),
            current_shape: CursorShape::DefaultPointer,
            current_frame_index: 0,
        };

        // Bootstrap default Adwaita-inspired cursor theme
        let mut adwaita = CursorTheme::new("Adwaita", "GNOME Project", 24);
        adwaita.register_cursor_shape(
            CursorShape::DefaultPointer,
            vec![CursorImageFrame {
                width: 24,
                height: 24,
                hotspot_x: 0,
                hotspot_y: 0,
                delay_ms: 0,
                rgba_pixels: vec![255; 24 * 24 * 4],
            }],
        );

        engine.themes.insert("Adwaita".to_string(), adwaita);
        engine
    }

    pub fn set_active_theme(&mut self, theme_name: &str) -> Result<(), &'static str> {
        if self.themes.contains_key(theme_name) {
            self.active_theme_name = theme_name.to_string();
            self.current_frame_index = 0;
            Ok(())
        } else {
            Err("Cursor theme not found")
        }
    }

    pub fn set_cursor_size(&mut self, size: u32) -> Result<(), &'static str> {
        if [16, 24, 32, 48, 64].contains(&size) {
            self.active_size = size;
            Ok(())
        } else {
            Err("Unsupported cursor size")
        }
    }

    pub fn set_cursor_shape(&mut self, shape: CursorShape) {
        self.current_shape = shape;
        self.current_frame_index = 0;
    }

    pub fn advance_animation_frame(&mut self) -> Option<&CursorImageFrame> {
        let theme = self.themes.get(&self.active_theme_name)?;
        let frames = theme.cursors.get(&self.current_shape)?;

        if frames.is_empty() {
            return None;
        }

        self.current_frame_index = (self.current_frame_index + 1) % frames.len();
        Some(&frames[self.current_frame_index])
    }
}

impl Default for CursorThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_theme_engine() {
        let mut engine = CursorThemeEngine::new();
        assert_eq!(engine.active_theme_name, "Adwaita");
        assert_eq!(engine.active_size, 24);

        // Test XCursor name translations
        assert_eq!(CursorShape::DefaultPointer.to_xcursor_name(), "default");
        assert_eq!(CursorShape::TextIBeam.to_xcursor_name(), "text");
        assert_eq!(CursorShape::WaitSpinner.to_xcursor_name(), "wait");

        // Test custom theme creation & multi-frame animations
        let mut breeze = CursorTheme::new("Breeze", "KDE Community", 24);
        breeze.register_cursor_shape(
            CursorShape::WaitSpinner,
            vec![
                CursorImageFrame {
                    width: 24,
                    height: 24,
                    hotspot_x: 12,
                    hotspot_y: 12,
                    delay_ms: 50,
                    rgba_pixels: vec![10; 24 * 24 * 4],
                },
                CursorImageFrame {
                    width: 24,
                    height: 24,
                    hotspot_x: 12,
                    hotspot_y: 12,
                    delay_ms: 50,
                    rgba_pixels: vec![20; 24 * 24 * 4],
                },
            ],
        );

        engine.themes.insert("Breeze".to_string(), breeze);
        assert!(engine.set_active_theme("Breeze").is_ok());
        assert!(engine.set_active_theme("NonExistentTheme").is_err());

        engine.set_cursor_shape(CursorShape::WaitSpinner);
        let f1 = engine.advance_animation_frame().unwrap();
        assert_eq!(f1.rgba_pixels[0], 20);

        let f2 = engine.advance_animation_frame().unwrap();
        assert_eq!(f2.rgba_pixels[0], 10);

        assert!(engine.set_cursor_size(48).is_ok());
        assert_eq!(engine.active_size, 48);
        assert!(engine.set_cursor_size(128).is_err());
    }
}
