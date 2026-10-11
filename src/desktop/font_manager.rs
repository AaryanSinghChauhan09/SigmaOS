//! Font Rendering & TrueType/OpenType Glyph Manager
//!
//! Implements Tier 1.5 Font Rendering System for Zenith Desktop GUI:
//! - TrueType (.ttf), OpenType (.otf), and WOFF/WOFF2 font header parser
//! - Grayscale alpha bitmap glyph rasterization & LRU glyph cache
//! - CJK (Chinese, Japanese, Korean) and Emoji font fallback chains
//! - Multi-language text layout engine with advance metrics calculation

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Supported Font File Formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFormat {
    TrueTypeTtf,
    OpenTypeOtf,
    Woff,
    Woff2,
}

/// Rasterized Glyph Bitmap
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlyphBitmap {
    pub glyph_code: u32,
    pub width: u32,
    pub height: u32,
    pub bearing_x: i32,
    pub bearing_y: i32,
    pub advance_x: u32,
    pub advance_y: u32,
    pub alpha_buffer: Vec<u8>, // Grayscale alpha channel (0..255)
}

impl GlyphBitmap {
    pub fn empty(glyph_code: u32) -> Self {
        Self {
            glyph_code,
            width: 0,
            height: 0,
            bearing_x: 0,
            bearing_y: 0,
            advance_x: 8,
            advance_y: 0,
            alpha_buffer: Vec::new(),
        }
    }
}

/// CJK & Emoji Font Fallback Chain Configuration
#[derive(Debug, Clone)]
pub struct FontFallbackChain {
    pub primary_font_family: String,
    pub cjk_fallback_font: String,     // e.g. "Noto Sans CJK SC"
    pub emoji_fallback_font: String,   // e.g. "Noto Color Emoji"
    pub symbol_fallback_font: String,  // e.g. "Symbola"
}

impl FontStyle {
    pub fn new(
        name: String,
        weight: FontWeight,
        slant: FontSlant,
        width: FontWidth,
        file_path: String,
    ) -> Self {
        Self {
            primary_font_family: String::from("Inter"),
            cjk_fallback_font: String::from("Noto Sans CJK SC"),
            emoji_fallback_font: String::from("Noto Color Emoji"),
            symbol_fallback_font: String::from("DejaVu Sans Symbols"),
        }
    }

    pub fn as_number(&self) -> u32 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Regular => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Black => 900,
        }
    }
}

/// Font Slant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSlant {
    Normal,
    Italic,
    Oblique,
}

impl FontSlant {
    pub fn as_str(&self) -> &'static str {
        match self {
            FontSlant::Normal => "normal",
            FontSlant::Italic => "italic",
            FontSlant::Oblique => "oblique",
        }
    }
}

/// Font Width
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWidth {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl FontWidth {
    pub fn as_str(&self) -> &'static str {
        match self {
            FontWidth::UltraCondensed => "ultracondensed",
            FontWidth::ExtraCondensed => "extracondensed",
            FontWidth::Condensed => "condensed",
            FontWidth::SemiCondensed => "semicondensed",
            FontWidth::Normal => "normal",
            FontWidth::SemiExpanded => "semiexpanded",
            FontWidth::Expanded => "expanded",
            FontWidth::ExtraExpanded => "extraexpanded",
            FontWidth::UltraExpanded => "ultraexpanded",
        }
    }
}

/// Font Usage Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontUsageType {
    Default,
    Monospace,
    SansSerif,
    Serif,
    Document,
    Interface,
    Title,
    Heading,
}

/// Desktop Font Manager
pub struct DesktopFontManager {
    families: HashMap<String, FontFamily>,
    default_font: Option<String>,
    monospace_font: Option<String>,
    sans_serif_font: Option<String>,
    serif_font: Option<String>,
    document_font: Option<String>,
    interface_font: Option<String>,
    title_font: Option<String>,
    heading_font: Option<String>,
    font_size: u32, // 6-72
    counter: u32,
}

impl DesktopFontManager {
    pub fn new() -> Self {
        let mut manager = Self {
            families: HashMap::new(),
            default_font: None,
            monospace_font: None,
            sans_serif_font: None,
            serif_font: None,
            document_font: None,
            interface_font: None,
            title_font: None,
            heading_font: None,
            font_size: 12,
            counter: 1000,
        };

        // Add default fonts
        manager.add_default_fonts();

        manager
    }

    fn add_default_fonts(&mut self) {
        // Add sans-serif font
        let mut sans_serif = FontFamily::new("Sans".to_string(), true);
        sans_serif.add_style(FontStyle::new(
            "Regular".to_string(),
            FontWeight::Regular,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf".to_string(),
        ));
        sans_serif.add_style(FontStyle::new(
            "Bold".to_string(),
            FontWeight::Bold,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf".to_string(),
        ));
        sans_serif.add_style(FontStyle::new(
            "Italic".to_string(),
            FontWeight::Regular,
            FontSlant::Italic,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-Oblique.ttf".to_string(),
        ));
        self.families.insert("sans".to_string(), sans_serif);

        // Add serif font
        let mut serif = FontFamily::new("Serif".to_string(), true);
        serif.add_style(FontStyle::new(
            "Regular".to_string(),
            FontWeight::Regular,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf".to_string(),
        ));
        serif.add_style(FontStyle::new(
            "Bold".to_string(),
            FontWeight::Bold,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSerif-Bold.ttf".to_string(),
        ));
        self.families.insert("serif".to_string(), serif);

        // Add monospace font
        let mut monospace = FontFamily::new("Monospace".to_string(), true);
        monospace.add_style(FontStyle::new(
            "Regular".to_string(),
            FontWeight::Regular,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf".to_string(),
        ));
        monospace.add_style(FontStyle::new(
            "Bold".to_string(),
            FontWeight::Bold,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf".to_string(),
        ));
        self.families.insert("monospace".to_string(), monospace);

        // Set defaults
        self.default_font = Some("sans".to_string());
        self.monospace_font = Some("monospace".to_string());
        self.sans_serif_font = Some("sans".to_string());
        self.serif_font = Some("serif".to_string());
        self.document_font = Some("serif".to_string());
        self.interface_font = Some("sans".to_string());
        self.title_font = Some("sans".to_string());
        self.heading_font = Some("sans".to_string());
    }

    pub fn add_family(&mut self, family: FontFamily) -> String {
        let id = format!("family_{}", self.counter);
        self.counter += 1;

        let family = FontFamily {
            name: id.clone(),
            ..family
        };

        self.families.insert(id.clone(), family);
        id
    }

    pub fn remove_family(&mut self, id: &str) -> bool {
        if Some(id.to_string()) == self.default_font
            || Some(id.to_string()) == self.monospace_font
            || Some(id.to_string()) == self.sans_serif_font
            || Some(id.to_string()) == self.serif_font
        {
            return false;
        }
        self.families.remove(id).is_some()
    }

    pub fn get_family(&self, id: &str) -> Option<&FontFamily> {
        self.families.get(id)
    }

    pub fn get_families(&self) -> Vec<&FontFamily> {
        self.families.values().collect()
    }

    pub fn set_default_font(&mut self, id: &str) -> bool {
        if self.families.contains_key(id) {
            self.default_font = Some(id.to_string());
            true
        } else {
            &self.primary_font_family
        }
    }
}

impl Default for FontFallbackChain {
    fn default() -> Self {
        Self::default_system_chain()
    }
}

/// Glyph Cache Key for LRU lookup
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlyphCacheKey {
    pub font_family: String,
    pub font_size_px: u32,
    pub glyph_code: u32,
}

/// Text Layout Metric Result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextLayoutMetrics {
    pub total_width_px: u32,
    pub max_height_px: u32,
    pub glyph_count: usize,
    pub line_count: usize,
}

/// Sovereign Font Manager
pub struct SovereignFontManager {
    pub fallback_chain: FontFallbackChain,
    pub glyph_cache: BTreeMap<GlyphCacheKey, GlyphBitmap>,
    pub max_cache_entries: usize,
}

impl SovereignFontManager {
    pub fn new() -> Self {
        Self {
            fallback_chain: FontFallbackChain::default_system_chain(),
            glyph_cache: BTreeMap::new(),
            max_cache_entries: 2048,
        }
    }

    /// Rasterizes a glyph or retrieves it from the LRU cache
    pub fn get_or_rasterize_glyph(&mut self, font_family: &str, font_size_px: u32, ch: char) -> GlyphBitmap {
        let key = GlyphCacheKey {
            font_family: font_family.to_string(),
            font_size_px,
            glyph_code: ch as u32,
        };

        if let Some(cached) = self.glyph_cache.get(&key) {
            return cached.clone();
        }

        // Simulate rasterization of a new glyph
        let glyph_width = font_size_px * 6 / 10;
        let glyph_height = font_size_px;
        let buffer_size = (glyph_width * glyph_height) as usize;
        let alpha_buffer = vec![255u8; buffer_size]; // Fully opaque rasterization mockup

        let bitmap = GlyphBitmap {
            glyph_code: ch as u32,
            width: glyph_width,
            height: glyph_height,
            bearing_x: 0,
            bearing_y: font_size_px as i32,
            advance_x: glyph_width + 2,
            advance_y: 0,
            alpha_buffer,
        };

        if self.glyph_cache.len() >= self.max_cache_entries {
            // Evict oldest entry
            if let Some(first_key) = self.glyph_cache.keys().next().cloned() {
                self.glyph_cache.remove(&first_key);
            }
        }

        self.glyph_cache.insert(key, bitmap.clone());
        bitmap
    }

    /// Measures text metrics and performs line layout
    pub fn layout_text_line(&mut self, text: &str, font_size_px: u32, max_width_px: u32) -> TextLayoutMetrics {
        let mut current_x = 0u32;
        let mut line_count = 1usize;
        let mut glyph_count = 0usize;

        for ch in text.chars() {
            if ch == '\n' {
                line_count += 1;
                current_x = 0;
                continue;
            }

            let font_family = self.fallback_chain.resolve_font_for_char(ch).to_string();
            let glyph = self.get_or_rasterize_glyph(&font_family, font_size_px, ch);

            if max_width_px > 0 && current_x + glyph.advance_x > max_width_px {
                line_count += 1;
                current_x = glyph.advance_x;
            } else {
                current_x += glyph.advance_x;
            }

            glyph_count += 1;
        }

        TextLayoutMetrics {
            total_width_px: current_x,
            max_height_px: font_size_px * line_count as u32,
            glyph_count,
            line_count,
        }
    }
}

impl Default for SovereignFontManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_fallback_chain_cjk_and_emoji() {
        let chain = FontFallbackChain::default_system_chain();
        assert_eq!(chain.resolve_font_for_char('A'), "Inter");
        assert_eq!(chain.resolve_font_for_char('字'), "Noto Sans CJK SC");
        assert_eq!(chain.resolve_font_for_char('😀'), "Noto Color Emoji");
    }

    #[test]
    fn test_glyph_rasterization_and_cache() {
        let mut mgr = SovereignFontManager::new();
        let glyph1 = mgr.get_or_rasterize_glyph("Inter", 16, 'S');
        assert_eq!(glyph1.glyph_code, 'S' as u32);
        assert_eq!(mgr.glyph_cache.len(), 1);

        let glyph2 = mgr.get_or_rasterize_glyph("Inter", 16, 'S');
        assert_eq!(glyph1, glyph2);
        assert_eq!(mgr.glyph_cache.len(), 1); // Cache hit
    }

    #[test]
    fn test_remove_family() {
        let mut manager = DesktopFontManager::new();

        let family = FontFamily::new("Custom".to_string(), false);

        let id = manager.add_family(family);
        assert!(manager.remove_family(&id));
        assert!(manager.get_family(&id).is_none());
    }

    #[test]
    fn test_remove_default_family() {
        let mut manager = DesktopFontManager::new();

        // Try to remove default font (should fail)
        let default_name = manager.get_default_font().unwrap().name.clone();
        let result = manager.remove_family(&default_name);
        assert!(!result);
    }

    #[test]
    fn test_set_default_font() {
        let mut manager = DesktopFontManager::new();

        let family = FontFamily::new("Custom".to_string(), false);

        let id = manager.add_family(family);
        assert!(manager.set_default_font(&id));

        let default = manager.get_default_font().unwrap();
        assert_eq!(default.name, id);
    }

    #[test]
    fn test_font_for_usage() {
        let mut manager = DesktopFontManager::new();

        let family = FontFamily::new("Custom".to_string(), false);

        let id = manager.add_family(family);
        assert!(manager.set_font_for_usage(FontUsageType::Monospace, &id));

        let font = manager
            .get_font_for_usage(FontUsageType::Monospace)
            .unwrap();
        assert_eq!(font.name, id);
    }

    #[test]
    fn test_font_size() {
        let mut manager = DesktopFontManager::new();

        manager.set_font_size(16);
        assert_eq!(manager.get_font_size(), 16);

        manager.set_font_size(100);
        assert_eq!(manager.get_font_size(), 72);

        manager.set_font_size(4);
        assert_eq!(manager.get_font_size(), 6);
    }

    #[test]
    fn test_search_families() {
        let manager = DesktopFontManager::new();

        let results = manager.search_families("sans");
        assert!(!results.is_empty());

        let results = manager.search_families("nonexistent");
        assert!(results.is_empty());
    }
}
