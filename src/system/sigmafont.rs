//! SigmaFont - Font Management System
//!
//! Provides comprehensive font management inspired by Font Craft.
//! Supports font loading, glyph management, and text rendering.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Font weight
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
;

/// Font style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
;

/// Glyph
#[derive(Debug, Clone)]
pub struct Glyph {
    pub codepoint: u32,
    pub width: u32,
    pub height: u32,
    pub bearing_x: i32,
    pub bearing_y: i32,
    pub advance: u32,
;

impl Glyph {
    pub fn new(codepoint: u32, width: u32, height: u32) -> Self {
        Self {
            codepoint,
            width,
            height,
            bearing_x: 0,
            bearing_y: 0,
            advance: width,
        }
    }

    pub fn set_bearing(&mut self, bearing_x: i32, bearing_y: i32) {
        self.bearing_x = bearing_x;
        self.bearing_y = bearing_y;
    }

    pub fn set_advance(&mut self, advance: u32) {
        self.advance = advance;
    }
;

/// Font
#[derive(Debug, Clone)]
pub struct Font {
    pub id: String,
    pub name: String,
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub size: u32, // in points
    pub glyphs: BTreeMap<u32, Glyph>,
;

impl Font {
    pub fn new(
        id: String,
        name: String,
        family: String,
        weight: FontWeight,
        style: FontStyle,
        size: u32,
    ) -> Self {
        Self {
            id,
            name,
            family,
            weight,
            style,
            size,
            glyphs: BTreeMap::new(),
        }
    }

    pub fn add_glyph(&mut self, glyph: Glyph) {
        self.glyphs.insert(glyph.codepoint, glyph);
    }

    pub fn remove_glyph(&mut self, codepoint: u32) -> bool {
        self.glyphs.remove(&codepoint).is_some()
    }

    pub fn get_glyph(&self, codepoint: u32) -> Option<&Glyph> {
        self.glyphs.get(&codepoint)
    }

    pub fn set_weight(&mut self, weight: FontWeight) {
        self.weight = weight;
    }

    pub fn set_style(&mut self, style: FontStyle) {
        self.style = style;
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    pub fn glyph_count(&self) -> usize {
        self.glyphs.len()
    }
;

/// Text run
#[derive(Debug, Clone)]
pub struct TextRun {
    pub id: String,
    pub text: String,
    pub font_id: String,
    pub font_size: u32,
    pub color: String,
;

impl TextRun {
    pub fn new(id: String, text: String, font_id: String, font_size: u32) -> Self {
        Self {
            id,
            text,
            font_id,
            font_size,
            color: "#000000".to_string(),
        }
    }

    pub fn set_color(&mut self, color: String) {
        self.color = color;
    }
;

/// SigmaFont - Font system manager
#[derive(Debug)]
pub struct SigmaFont {
    fonts: BTreeMap<String, Font>,
    text_runs: BTreeMap<String, TextRun>,
    default_font: Option<String>,
;

impl SigmaFont {
    pub fn new() -> Self {
        Self {
            fonts: BTreeMap::new(),
            text_runs: BTreeMap::new(),
            default_font: None,
        }
    }

    pub fn add_font(&mut self, font: Font) {
        self.fonts.insert(font.id.clone(), font);
    }

    pub fn remove_font(&mut self, id: &str) -> bool {
        if self.default_font.as_ref() == Some(&id.to_string()) {
            self.default_font = None;
        }
        self.fonts.remove(id).is_some()
    }

    pub fn get_font(&self, id: &str) -> Option<&Font> {
        self.fonts.get(id)
    }

    pub fn get_font_mut(&mut self, id: &str) -> Option<&mut Font> {
        self.fonts.get_mut(id)
    }

    pub fn set_default_font(&mut self, id: &str) -> bool {
        if self.fonts.contains_key(id) {
            self.default_font = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_default_font(&self) -> Option<&Font> {
        self.default_font.as_ref().and_then(|id| self.fonts.get(id))
    }

    pub fn add_text_run(&mut self, run: TextRun) {
        self.text_runs.insert(run.id.clone(), run);
    }

    pub fn remove_text_run(&mut self, id: &str) -> bool {
        self.text_runs.remove(id).is_some()
    }

    pub fn get_text_run(&self, id: &str) -> Option<&TextRun> {
        self.text_runs.get(id)
    }

    pub fn get_text_run_mut(&mut self, id: &str) -> Option<&mut TextRun> {
        self.text_runs.get_mut(id)
    }

    pub fn list_fonts(&self) -> Vec<&Font> {
        self.fonts.values().collect()
    }

    pub fn font_count(&self) -> usize {
        self.fonts.len()
    }

    pub fn text_run_count(&self) -> usize {
        self.text_runs.len()
    }
;

impl Default for SigmaFont {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaFont instance
static mut SIGMAFONT_INSTANCE: Option<SigmaFont> = None;

/// Initialize the global SigmaFont instance
pub fn init_sigmafont() {
    unsafe {
        SIGMAFONT_INSTANCE = Some(SigmaFont::new());
    }
;

/// Get the global SigmaFont instance
pub fn get_sigmafont() -> Option<&'static SigmaFont> {
    unsafe { SIGMAFONT_INSTANCE.as_ref() }
;

/// Get the global SigmaFont instance (mutable)
pub fn get_sigmafont_mut() -> Option<&'static mut SigmaFont> {
    unsafe { SIGMAFONT_INSTANCE.as_mut() }
;

/// Convenience function to add a font
pub fn add_font(font: Font) -> bool {
    if let Some(font_sys) = get_sigmafont_mut() {
        font_sys.add_font(font);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_creation() {
        let glyph = Glyph::new(65, 10, 20); // 'A'
        assert_eq!(glyph.codepoint, 65);
        assert_eq!(glyph.width, 10);
        assert_eq!(glyph.height, 20);
    }

    #[test]
    fn test_glyph_modification() {
        let mut glyph = Glyph::new(65, 10, 20);
        glyph.set_bearing(2, 3);
        glyph.set_advance(12);
        assert_eq!(glyph.bearing_x, 2);
        assert_eq!(glyph.advance, 12);
    }

    #[test]
    fn test_font_creation() {
        let font = Font::new(
            "font1".to_string(),
            "Arial".to_string(),
            "Arial".to_string(),
            FontWeight::Regular,
            FontStyle::Normal,
            12,
        );
        assert_eq!(font.id, "font1");
        assert_eq!(font.family, "Arial");
        assert_eq!(font.weight, FontWeight::Regular);
    }

    #[test]
    fn test_font_glyph_management() {
        let mut font = Font::new(
            "font1".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            FontWeight::Regular,
            FontStyle::Normal,
            12,
        );
        let glyph = Glyph::new(65, 10, 20);
        font.add_glyph(glyph);
        assert_eq!(font.glyph_count(), 1);
        assert!(font.remove_glyph(65));
        assert_eq!(font.glyph_count(), 0);
    }

    #[test]
    fn test_text_run_creation() {
        let run = TextRun::new(
            "run1".to_string(),
            "Hello".to_string(),
            "font1".to_string(),
            12,
        );
        assert_eq!(run.id, "run1");
        assert_eq!(run.text, "Hello");
        assert_eq!(run.font_size, 12);
    }

    #[test]
    fn test_sigmafont_creation() {
        let font_sys = SigmaFont::new();
        assert_eq!(font_sys.font_count(), 0);
        assert_eq!(font_sys.text_run_count(), 0);
    }

    #[test]
    fn test_sigmafont_management() {
        let mut font_sys = SigmaFont::new();
        let font = Font::new(
            "font1".to_string(),
            "Arial".to_string(),
            "Arial".to_string(),
            FontWeight::Regular,
            FontStyle::Normal,
            12,
        );
        font_sys.add_font(font);
        assert_eq!(font_sys.font_count(), 1);
    }
;
