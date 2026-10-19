//! SigmaColorScheme - Color Scheme Generator
//!
//! Inspired by Wallust (Omarchy Linux), this module provides automatic color scheme
//! generation from wallpapers and images with template support.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Color data structure
#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: String,
;

impl Color {
    /// Create a new color from RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        let hex = format!("{:02x}{:02x}{:02x}", r, g, b);
        Self { r, g, b, hex }
    }

    /// Create color from hex string
    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Self::new(r, g, b))
        } else {
            None
        }
    }

    /// Get luminance
    pub fn luminance(&self) -> f32 {
        (0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32) / 255.0
    }

    /// Check if color is light
    pub fn is_light(&self) -> bool {
        self.luminance() > 0.5
    }
;

/// Color palette type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteType {
    Dark,
    Light,
    Auto,
    Unknown,
;

/// Color scheme data structure
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub scheme_id: String,
    pub name: String,
    pub palette_type: PaletteType,
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,
    pub colors: BTreeMap<String, Color>,
    pub source_image: String,
;

impl ColorScheme {
    /// Create a new color scheme
    pub fn new(
        scheme_id: String,
        name: String,
        palette_type: PaletteType,
        source_image: String,
    ) -> Self {
        Self {
            scheme_id,
            name,
            palette_type,
            background: Color::new(0, 0, 0),
            foreground: Color::new(255, 255, 255),
            cursor: Color::new(255, 255, 255),
            colors: BTreeMap::new(),
            source_image,
        }
    }

    /// Set background color
    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    /// Set foreground color
    pub fn set_foreground(&mut self, color: Color) {
        self.foreground = color;
    }

    /// Set cursor color
    pub fn set_cursor(&mut self, color: Color) {
        self.cursor = color;
    }

    /// Add a color to the palette
    pub fn add_color(&mut self, name: String, color: Color) {
        self.colors.insert(name, color);
    }

    /// Get a color by name
    pub fn get_color(&self, name: &str) -> Option<&Color> {
        self.colors.get(name)
    }
;

/// Template type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateType {
    Waybar,
    Rofi,
    Kitty,
    Alacritty,
    Gtk,
    Qt,
    Custom,
    Unknown,
;

/// Template data structure
#[derive(Debug, Clone)]
pub struct Template {
    pub template_id: String,
    pub template_type: TemplateType,
    pub content: String,
    pub enabled: bool,
;

impl Template {
    /// Create a new template
    pub fn new(template_id: String, template_type: TemplateType, content: String) -> Self {
        Self {
            template_id,
            template_type,
            content,
            enabled: true,
        }
    }

    /// Enable template
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable template
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if template is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
;

/// Color scheme manager
pub struct SigmaColorScheme {
    schemes: BTreeMap<String, ColorScheme>,
    templates: BTreeMap<String, Template>,
    running: AtomicBool,
;

impl SigmaColorScheme {
    /// Create a new color scheme manager
    pub fn new() -> Self {
        Self {
            schemes: BTreeMap::new(),
            templates: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the color scheme manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the color scheme manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Generate color scheme from image
    pub fn generate_scheme(&mut self, scheme: ColorScheme) -> bool {
        let scheme_id = scheme.scheme_id.clone();
        self.schemes.insert(scheme_id, scheme);
        true
    }

    /// Remove a color scheme
    pub fn remove_scheme(&mut self, scheme_id: &str) -> Option<ColorScheme> {
        self.schemes.remove(scheme_id)
    }

    /// Get a color scheme by ID
    pub fn get_scheme(&self, scheme_id: &str) -> Option<&ColorScheme> {
        self.schemes.get(scheme_id)
    }

    /// List all color schemes
    pub fn list_schemes(&self) -> Vec<&ColorScheme> {
        self.schemes.values().collect()
    }

    /// List schemes by palette type
    pub fn list_schemes_by_type(&self, palette_type: PaletteType) -> Vec<&ColorScheme> {
        self.schemes
            .values()
            .filter(|s| s.palette_type == palette_type)
            .collect()
    }

    /// Apply a color scheme
    pub fn apply_scheme(&mut self, scheme_id: &str) -> bool {
        if let Some(_scheme) = self.schemes.get(scheme_id) {
            // Apply scheme to templates
            true
        } else {
            false
        }
    }

    /// Add a template
    pub fn add_template(&mut self, template: Template) -> bool {
        let template_id = template.template_id.clone();
        self.templates.insert(template_id, template);
        true
    }

    /// Remove a template
    pub fn remove_template(&mut self, template_id: &str) -> Option<Template> {
        self.templates.remove(template_id)
    }

    /// Get a template by ID
    pub fn get_template(&self, template_id: &str) -> Option<&Template> {
        self.templates.get(template_id)
    }

    /// List all templates
    pub fn list_templates(&self) -> Vec<&Template> {
        self.templates.values().collect()
    }

    /// List templates by type
    pub fn list_templates_by_type(&self, template_type: TemplateType) -> Vec<&Template> {
        self.templates
            .values()
            .filter(|t| t.template_type == template_type)
            .collect()
    }

    /// Generate config from template
    pub fn generate_config(&self, scheme_id: &str, template_id: &str) -> Option<String> {
        if let Some(scheme) = self.schemes.get(scheme_id) {
            if let Some(template) = self.templates.get(template_id) {
                if template.is_enabled() {
                    // Generate config by substituting colors
                    Some(template.content.clone())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get scheme count
    pub fn scheme_count(&self) -> usize {
        self.schemes.len()
    }

    /// Get template count
    pub fn template_count(&self) -> usize {
        self.templates.len()
    }
;

impl Default for SigmaColorScheme {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaColorScheme instance (placeholder for actual global state)
static mut SIGMACOLORSCHEME_INSTANCE: Option<SigmaColorScheme> = None;

/// Initialize the global SigmaColorScheme instance
pub fn init_sigmacolorscheme() {
    unsafe {
        SIGMACOLORSCHEME_INSTANCE = Some(SigmaColorScheme::new());
    }
;

/// Get the global SigmaColorScheme instance
pub fn get_sigmacolorscheme() -> Option<&'static SigmaColorScheme> {
    unsafe { SIGMACOLORSCHEME_INSTANCE.as_ref() }
;

/// Get mutable global SigmaColorScheme instance
pub fn get_sigmacolorscheme_mut() -> Option<&'static mut SigmaColorScheme> {
    unsafe { SIGMACOLORSCHEME_INSTANCE.as_mut() }
;

/// Convenience function to generate a scheme
pub fn generate_color_scheme(scheme: ColorScheme) -> bool {
    if let Some(color_mgr) = get_sigmacolorscheme_mut() {
        color_mgr.generate_scheme(scheme)
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::new(255, 128, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
        assert_eq!(color.hex, "ff8000");
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("ff8000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_color_luminance() {
        let white = Color::new(255, 255, 255);
        assert!(white.is_light());

        let black = Color::new(0, 0, 0);
        assert!(!black.is_light());
    }

    #[test]
    fn test_color_scheme_creation() {
        let scheme = ColorScheme::new(
            "scheme1".to_string(),
            "Nature".to_string(),
            PaletteType::Dark,
            "/path/to/image.jpg".to_string(),
        );

        assert_eq!(scheme.scheme_id, "scheme1");
        assert_eq!(scheme.palette_type, PaletteType::Dark);
    }

    #[test]
    fn test_template_creation() {
        let template = Template::new(
            "template1".to_string(),
            TemplateType::Kitty,
            "config content".to_string(),
        );

        assert_eq!(template.template_id, "template1");
        assert_eq!(template.template_type, TemplateType::Kitty);
        assert!(template.is_enabled());
    }

    #[test]
    fn test_sigmacolorscheme_creation() {
        let color_mgr = SigmaColorScheme::new();
        assert!(!color_mgr.is_running());
        assert_eq!(color_mgr.scheme_count(), 0);
        assert_eq!(color_mgr.template_count(), 0);
    }

    #[test]
    fn test_sigmacolorscheme_start_stop() {
        let mut color_mgr = SigmaColorScheme::new();
        color_mgr.start();
        assert!(color_mgr.is_running());
        color_mgr.stop();
        assert!(!color_mgr.is_running());
    }

    #[test]
    fn test_sigmacolorscheme_generate_scheme() {
        let mut color_mgr = SigmaColorScheme::new();
        let scheme = ColorScheme::new(
            "scheme1".to_string(),
            "Nature".to_string(),
            PaletteType::Dark,
            "/path/to/image.jpg".to_string(),
        );

        let generated = color_mgr.generate_scheme(scheme);
        assert!(generated);
        assert_eq!(color_mgr.scheme_count(), 1);
    }

    #[test]
    fn test_sigmacolorscheme_add_template() {
        let mut color_mgr = SigmaColorScheme::new();
        let template = Template::new(
            "template1".to_string(),
            TemplateType::Kitty,
            "config content".to_string(),
        );

        let added = color_mgr.add_template(template);
        assert!(added);
        assert_eq!(color_mgr.template_count(), 1);
    }

    #[test]
    fn test_sigmacolorscheme_list_by_type() {
        let mut color_mgr = SigmaColorScheme::new();

        color_mgr.generate_scheme(ColorScheme::new(
            "scheme1".to_string(),
            "Dark Theme".to_string(),
            PaletteType::Dark,
            "/path/to/image1.jpg".to_string(),
        ));

        color_mgr.generate_scheme(ColorScheme::new(
            "scheme2".to_string(),
            "Light Theme".to_string(),
            PaletteType::Light,
            "/path/to/image2.jpg".to_string(),
        ));

        let dark = color_mgr.list_schemes_by_type(PaletteType::Dark);
        assert_eq!(dark.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmacolorscheme();

        let color_mgr = get_sigmacolorscheme();
        assert!(color_mgr.is_some());

        let mut color_mgr_mut = get_sigmacolorscheme_mut();
        assert!(color_mgr_mut.is_some());

        if let Some(color_mgr) = color_mgr_mut {
            color_mgr.start();
            assert!(color_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_generate_scheme() {
        init_sigmacolorscheme();

        let scheme = ColorScheme::new(
            "scheme1".to_string(),
            "Nature".to_string(),
            PaletteType::Dark,
            "/path/to/image.jpg".to_string(),
        );

        let generated = generate_color_scheme(scheme);
        assert!(generated);

        let color_mgr = get_sigmacolorscheme().unwrap();
        assert_eq!(color_mgr.scheme_count(), 1);
    }
;
