// Wallpaper Palette Extractor inspired by Omarchy
// Extract dominant colors from images for theme generation

use std::fmt;

/// Color extracted from image
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Calculate brightness (0-255)
    pub fn brightness(&self) -> u8 {
        ((self.r as u32 * 299 + self.g as u32 * 587 + self.b as u32 * 114) / 1000) as u8
    }

    /// Calculate saturation (0-255)
    pub fn saturation(&self) -> u8 {
        let max = self.r.max(self.g).max(self.b) as f32;
        let min = self.r.min(self.g).min(self.b) as f32;
        
        if max == 0.0 {
            0
        } else {
            (((max - min) / max) * 255.0) as u8
        }
    }

    /// Check if color is dark
    pub fn is_dark(&self) -> bool {
        self.brightness() < 128
    }

    /// Calculate Euclidean distance to another color
    pub fn distance_to(&self, other: &Color) -> f32 {
        let dr = (self.r as f32 - other.r as f32).powi(2);
        let dg = (self.g as f32 - other.g as f32).powi(2);
        let db = (self.b as f32 - other.b as f32).powi(2);
        (dr + dg + db).sqrt()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Color category for theme mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorCategory {
    Dark,
    Middle,
    Light,
    Accent,
}

impl ColorCategory {
    /// Determine category from color brightness
    pub fn from_color(color: &Color) -> Self {
        let brightness = color.brightness();
        if brightness < 85 {
            ColorCategory::Dark
        } else if brightness < 170 {
            ColorCategory::Middle
        } else {
            ColorCategory::Light
        }
    }
}

/// Extracted color palette
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub colors: Vec<Color>,
    pub categories: Vec<ColorCategory>,
    pub dominant: Color,
    pub source_image: String,
}

impl ColorPalette {
    pub fn new(source_image: String) -> Self {
        Self {
            colors: Vec::new(),
            categories: Vec::new(),
            dominant: Color::new(0, 0, 0),
            source_image,
        }
    }

    /// Add color to palette
    pub fn add_color(&mut self, color: Color) {
        self.colors.push(color);
        self.categories.push(ColorCategory::from_color(&color));
    }

    /// Set dominant color
    pub fn set_dominant(&mut self, color: Color) {
        self.dominant = color;
    }

    /// Get colors by category
    pub fn get_colors_by_category(&self, category: ColorCategory) -> Vec<Color> {
        self.colors.iter()
            .zip(self.categories.iter())
            .filter(|(_, cat)| **cat == category)
            .map(|(color, _)| *color)
            .collect()
    }

    /// Get dark colors
    pub fn dark_colors(&self) -> Vec<Color> {
        self.get_colors_by_category(ColorCategory::Dark)
    }

    /// Get light colors
    pub fn light_colors(&self) -> Vec<Color> {
        self.get_colors_by_category(ColorCategory::Light)
    }

    /// Get middle colors
    pub fn middle_colors(&self) -> Vec<Color> {
        self.get_colors_by_category(ColorCategory::Middle)
    }

    /// Generate accent color using golden ratio
    pub fn generate_accent(&self) -> Color {
        // Simple accent generation: pick most saturated color
        self.colors.iter()
            .max_by(|a, b| a.saturation().cmp(&b.saturation()))
            .copied()
            .unwrap_or(Color::new(255, 100, 100))
    }

    /// Validate contrast for readability
    pub fn validate_contrast(&self, foreground: &Color, background: &Color) -> bool {
        let fg_brightness = foreground.brightness() as f32;
        let bg_brightness = background.brightness() as f32;
        (fg_brightness - bg_brightness).abs() > 50.0
    }
}

/// Extraction options
#[derive(Debug, Clone)]
pub struct ExtractionOptions {
    pub num_clusters: usize,
    pub generate_preview: bool,
    pub validate_contrast: bool,
}

impl ExtractionOptions {
    pub fn new() -> Self {
        Self {
            num_clusters: 8,
            generate_preview: false,
            validate_contrast: true,
        }
    }

    /// Set number of color clusters
    pub fn with_clusters(mut self, clusters: usize) -> Self {
        self.num_clusters = clusters;
        self
    }

    /// Enable preview generation
    pub fn with_preview(mut self, preview: bool) -> Self {
        self.generate_preview = preview;
        self
    }

    /// Enable contrast validation
    pub fn with_contrast_validation(mut self, validate: bool) -> Self {
        self.validate_contrast = validate;
        self
    }
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Palette extraction result
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    pub palette: ColorPalette,
    pub success: bool,
    pub error: Option<String>,
}

impl ExtractionResult {
    pub fn success(palette: ColorPalette) -> Self {
        Self {
            palette,
            success: true,
            error: None,
        }
    }

    pub fn failure(error: String) -> Self {
        Self {
            palette: ColorPalette::new("".to_string()),
            success: false,
            error: Some(error),
        }
    }
}

/// Wallpaper Palette Extractor manager
pub struct PaletteExtractor {
    palettes: Vec<ColorPalette>,
}

impl PaletteExtractor {
    pub fn new() -> Self {
        Self {
            palettes: Vec::new(),
        }
    }

    /// Extract palette from image
    pub fn extract_palette(&mut self, image_path: &str, options: &ExtractionOptions) -> ExtractionResult {
        // In a real implementation, this would:
        // 1. Load the image
        // 2. Run K-means clustering
        // 3. Extract dominant colors
        // 4. Map to categories
        
        let mut palette = ColorPalette::new(image_path.to_string());
        
        // Mock extraction - add some sample colors
        let sample_colors = vec![
            Color::new(45, 45, 45),   // Dark
            Color::new(128, 128, 128), // Middle
            Color::new(220, 220, 220), // Light
            Color::new(255, 100, 100), // Accent (red)
            Color::new(100, 255, 100), // Accent (green)
            Color::new(100, 100, 255), // Accent (blue)
            Color::new(255, 255, 100), // Accent (yellow)
            Color::new(255, 100, 255), // Accent (magenta)
        ];

        for color in sample_colors.iter().take(options.num_clusters) {
            palette.add_color(*color);
        }

        palette.set_dominant(palette.colors[0]);
        
        self.palettes.push(palette.clone());
        ExtractionResult::success(palette)
    }

    /// Extract palette from URL
    pub fn extract_from_url(&mut self, url: &str, options: &ExtractionOptions) -> ExtractionResult {
        // In a real implementation, this would download the image first
        // For now, treat it as a local path
        self.extract_palette(url, options)
    }

    /// Get all palettes
    pub fn get_palettes(&self) -> Vec<ColorPalette> {
        self.palettes.clone()
    }

    /// Get palette by source image
    pub fn get_palette(&self, source: &str) -> Option<ColorPalette> {
        self.palettes.iter().find(|p| p.source_image == source).cloned()
    }

    /// Generate theme colors from palette
    pub fn generate_theme_colors(&self, palette: &ColorPalette) -> ThemeColors {
        let dark = palette.dark_colors();
        let light = palette.light_colors();
        let accent = palette.generate_accent();

        ThemeColors {
            background: dark.get(0).copied().unwrap_or(Color::new(30, 30, 30)),
            foreground: light.get(0).copied().unwrap_or(Color::new(240, 240, 240)),
            accent,
            middle: palette.middle_colors().get(0).copied(),
            dark1: dark.get(1).copied(),
            dark2: dark.get(2).copied(),
            light1: light.get(1).copied(),
            light2: light.get(2).copied(),
        }
    }

    /// Remove palette
    pub fn remove_palette(&mut self, source: &str) -> bool {
        if let Some(pos) = self.palettes.iter().position(|p| p.source_image == source) {
            self.palettes.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for PaletteExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Generated theme colors
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub middle: Option<Color>,
    pub dark1: Option<Color>,
    pub dark2: Option<Color>,
    pub light1: Option<Color>,
    pub light2: Option<Color>,
}

impl ThemeColors {
    /// Convert to JSON format
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"background": "{}", "foreground": "{}", "accent": "{}"}}"#,
            self.background.to_hex(),
            self.foreground.to_hex(),
            self.accent.to_hex()
        )
    }

    /// Validate theme contrast
    pub fn validate(&self) -> bool {
        let palette = ColorPalette::new("theme".to_string());
        palette.validate_contrast(&self.foreground, &self.background)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.to_hex(), "#FF8040");
    }

    #[test]
    fn test_color_brightness() {
        let dark = Color::new(50, 50, 50);
        let light = Color::new(200, 200, 200);
        assert!(dark.brightness() < light.brightness());
    }

    #[test]
    fn test_color_is_dark() {
        let dark = Color::new(50, 50, 50);
        let light = Color::new(200, 200, 200);
        assert!(dark.is_dark());
        assert!(!light.is_dark());
    }

    #[test]
    fn test_color_distance() {
        let color1 = Color::new(255, 0, 0);
        let color2 = Color::new(0, 255, 0);
        let distance = color1.distance_to(&color2);
        assert!(distance > 0.0);
    }

    #[test]
    fn test_color_category_from_color() {
        let dark = Color::new(50, 50, 50);
        let light = Color::new(200, 200, 200);
        assert_eq!(ColorCategory::from_color(&dark), ColorCategory::Dark);
        assert_eq!(ColorCategory::from_color(&light), ColorCategory::Light);
    }

    #[test]
    fn test_color_palette_creation() {
        let palette = ColorPalette::new("test.jpg".to_string());
        assert_eq!(palette.source_image, "test.jpg");
        assert_eq!(palette.colors.len(), 0);
    }

    #[test]
    fn test_color_palette_add_color() {
        let mut palette = ColorPalette::new("test.jpg".to_string());
        palette.add_color(Color::new(100, 100, 100));
        assert_eq!(palette.colors.len(), 1);
    }

    #[test]
    fn test_color_palette_set_dominant() {
        let mut palette = ColorPalette::new("test.jpg".to_string());
        palette.set_dominant(Color::new(255, 0, 0));
        assert_eq!(palette.dominant, Color::new(255, 0, 0));
    }

    #[test]
    fn test_color_palette_get_by_category() {
        let mut palette = ColorPalette::new("test.jpg".to_string());
        palette.add_color(Color::new(50, 50, 50));
        palette.add_color(Color::new(200, 200, 200));
        palette.add_color(Color::new(128, 128, 128));
        
        let dark = palette.dark_colors();
        let light = palette.light_colors();
        assert_eq!(dark.len(), 1);
        assert_eq!(light.len(), 1);
    }

    #[test]
    fn test_color_palette_generate_accent() {
        let mut palette = ColorPalette::new("test.jpg".to_string());
        palette.add_color(Color::new(255, 100, 100));
        palette.add_color(Color::new(100, 100, 100));
        let accent = palette.generate_accent();
        assert_eq!(accent, Color::new(255, 100, 100));
    }

    #[test]
    fn test_color_palette_validate_contrast() {
        let palette = ColorPalette::new("test.jpg".to_string());
        let dark = Color::new(30, 30, 30);
        let light = Color::new(240, 240, 240);
        assert!(palette.validate_contrast(&light, &dark));
    }

    #[test]
    fn test_extraction_options() {
        let options = ExtractionOptions::new()
            .with_clusters(12)
            .with_preview(true)
            .with_contrast_validation(false);
        assert_eq!(options.num_clusters, 12);
        assert!(options.generate_preview);
        assert!(!options.validate_contrast);
    }

    #[test]
    fn test_extraction_options_default() {
        let options = ExtractionOptions::default();
        assert_eq!(options.num_clusters, 8);
        assert!(!options.generate_preview);
        assert!(options.validate_contrast);
    }

    #[test]
    fn test_palette_extractor_creation() {
        let extractor = PaletteExtractor::new();
        assert_eq!(extractor.palettes.len(), 0);
    }

    #[test]
    fn test_extract_palette() {
        let mut extractor = PaletteExtractor::new();
        let options = ExtractionOptions::default();
        let result = extractor.extract_palette("test.jpg", &options);
        assert!(result.success);
        assert_eq!(extractor.palettes.len(), 1);
    }

    #[test]
    fn test_extract_from_url() {
        let mut extractor = PaletteExtractor::new();
        let options = ExtractionOptions::default();
        let result = extractor.extract_from_url("http://example.com/image.jpg", &options);
        assert!(result.success);
    }

    #[test]
    fn test_get_palette() {
        let mut extractor = PaletteExtractor::new();
        let options = ExtractionOptions::default();
        extractor.extract_palette("test.jpg", &options);
        let palette = extractor.get_palette("test.jpg");
        assert!(palette.is_some());
    }

    #[test]
    fn test_get_palette_not_found() {
        let extractor = PaletteExtractor::new();
        let palette = extractor.get_palette("nonexistent.jpg");
        assert!(palette.is_none());
    }

    #[test]
    fn test_generate_theme_colors() {
        let mut extractor = PaletteExtractor::new();
        let options = ExtractionOptions::default();
        extractor.extract_palette("test.jpg", &options);
        let palette = extractor.get_palette("test.jpg").unwrap();
        let theme = extractor.generate_theme_colors(&palette);
        assert_eq!(theme.background.to_hex(), "#2D2D2D");
    }

    #[test]
    fn test_theme_colors_to_json() {
        let theme = ThemeColors {
            background: Color::new(30, 30, 30),
            foreground: Color::new(240, 240, 240),
            accent: Color::new(255, 100, 100),
            middle: None,
            dark1: None,
            dark2: None,
            light1: None,
            light2: None,
        };
        let json = theme.to_json();
        assert!(json.contains("#1E1E1E"));
        assert!(json.contains("#F0F0F0"));
    }

    #[test]
    fn test_theme_colors_validate() {
        let theme = ThemeColors {
            background: Color::new(30, 30, 30),
            foreground: Color::new(240, 240, 240),
            accent: Color::new(255, 100, 100),
            middle: None,
            dark1: None,
            dark2: None,
            light1: None,
            light2: None,
        };
        assert!(theme.validate());
    }

    #[test]
    fn test_remove_palette() {
        let mut extractor = PaletteExtractor::new();
        let options = ExtractionOptions::default();
        extractor.extract_palette("test.jpg", &options);
        let removed = extractor.remove_palette("test.jpg");
        assert!(removed);
        assert_eq!(extractor.palettes.len(), 0);
    }

    #[test]
    fn test_remove_palette_not_found() {
        let mut extractor = PaletteExtractor::new();
        let removed = extractor.remove_palette("nonexistent.jpg");
        assert!(!removed);
    }

    #[test]
    fn test_extraction_result_success() {
        let palette = ColorPalette::new("test.jpg".to_string());
        let result = ExtractionResult::success(palette);
        assert!(result.success);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_extraction_result_failure() {
        let result = ExtractionResult::failure("Test error".to_string());
        assert!(!result.success);
        assert_eq!(result.error, Some("Test error".to_string()));
    }
}
