// SPDX-License-Identifier: MIT
// SigmaOS - Font Manager
// Inspired by GNOME Fonts and Font Manager
// Provides font installation and management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Font style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Regular,
    Italic,
    Bold,
    BoldItalic,
}

/// Font
#[derive(Debug, Clone)]
pub struct Font {
    pub name: String,
    pub family: String,
    pub style: FontStyle,
    pub path: String,
    pub is_installed: bool,
    pub is_system: bool,
}

/// Font family
#[derive(Debug, Clone)]
pub struct FontFamily {
    pub name: String,
    pub fonts: Vec<Font>,
}

/// Font manager
pub struct FontManager {
    pub families: BTreeMap<String, FontFamily>,
    pub installed_fonts: Vec<Font>,
    pub filter: String,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            families: BTreeMap::new(),
            installed_fonts: Vec::new(),
            filter: String::new(),
        }
    }

    /// Install font
    pub fn install_font(&mut self, path: String) -> Result<(), &'static str> {
        let name = path.split('/').last().unwrap_or("Unknown").to_string();
        let font = Font {
            name: name.clone(),
            family: "Unknown".to_string(),
            style: FontStyle::Regular,
            path: path.clone(),
            is_installed: true,
            is_system: false,
        };
        self.installed_fonts.push(font);
        Ok(())
    }

    /// Uninstall font
    pub fn uninstall_font(&mut self, name: String) -> Result<(), &'static str> {
        let index = self
            .installed_fonts
            .iter()
            .position(|f| f.name == name)
            .ok_or("Font not found")?;
        self.installed_fonts.remove(index);
        Ok(())
    }

    /// Get installed fonts
    pub fn get_installed_fonts(&self) -> Vec<&Font> {
        if self.filter.is_empty() {
            self.installed_fonts.iter().collect()
        } else {
            let filter_lower = self.filter.to_lowercase();
            self.installed_fonts
                .iter()
                .filter(|f| {
                    f.name.to_lowercase().contains(&filter_lower)
                        || f.family.to_lowercase().contains(&filter_lower)
                })
                .collect()
        }
    }

    /// Set filter
    pub fn set_filter(&mut self, filter: String) {
        self.filter = filter;
    }

    /// Font count
    pub fn font_count(&self) -> usize {
        self.installed_fonts.len()
    }
}

/// Global font manager
static mut FONT_MANAGER: Option<FontManager> = None;

/// Initialize font manager
pub fn init_font_manager() {
    unsafe {
        FONT_MANAGER = Some(FontManager::new());
    }
}

/// Get font manager
pub fn get_font_manager() -> &'static mut FontManager {
    unsafe { FONT_MANAGER.as_mut().expect("Font manager not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_manager_creation() {
        let fm = FontManager::new();
        assert_eq!(fm.font_count(), 0);
    }

    #[test]
    fn test_install_font() {
        let mut fm = FontManager::new();
        fm.install_font("/path/to/font.ttf".to_string()).unwrap();
        assert_eq!(fm.font_count(), 1);
    }

    #[test]
    fn test_uninstall_font() {
        let mut fm = FontManager::new();
        fm.install_font("/path/to/font.ttf".to_string()).unwrap();
        fm.uninstall_font("font.ttf".to_string()).unwrap();
        assert_eq!(fm.font_count(), 0);
    }

    #[test]
    fn test_filter() {
        let mut fm = FontManager::new();
        fm.install_font("/path/to/Roboto.ttf".to_string()).unwrap();
        fm.set_filter("roboto".to_string());
        let filtered = fm.get_installed_fonts();
        assert_eq!(filtered.len(), 1);
    }
}
