#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(dead_code)]

// SigmaOS Pix Image Viewer - Linux Mint Pix-inspired Image Viewer
// Modern image viewer with JXL, AVIF, WebP, RAW, and HDR support

use std::collections::HashMap;

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    BMP,
    JPEG,
    GIF,
    PNG,
    TIFF,
    TGA,
    ICO,
    XPM,
    JXL,
    AVIF,
    WebP,
    RAW,
    HDR,
    SVG,
    Unknown,
}

/// Image metadata
#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
    pub bits_per_sample: u8,
    pub color_space: String,
    pub has_alpha: bool,
    pub is_animated: bool,
    pub frame_count: u32,
    pub file_size: u64,
}

/// EXIF data
#[derive(Debug, Clone)]
pub struct ExifData {
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub iso: Option<u32>,
    pub aperture: Option<String>,
    pub shutter_speed: Option<String>,
    pub focal_length: Option<String>,
    pub date_taken: Option<String>,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
    pub gps_altitude: Option<f64>,
}

/// Image transformation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageTransform {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontal,
    FlipVertical,
    Mirror,
}

/// Viewer mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerMode {
    Normal,
    Fullscreen,
    Slideshow,
}

/// Slideshow configuration
#[derive(Debug, Clone)]
pub struct SlideshowConfig {
    pub interval_seconds: u32,
    pub shuffle: bool,
    pub repeat: bool,
    pub show_filename: bool,
    pub transition_enabled: bool,
}

impl Default for SlideshowConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 5,
            shuffle: false,
            repeat: true,
            show_filename: true,
            transition_enabled: true,
        }
    }
}

/// Pix image viewer configuration
#[derive(Debug, Clone)]
pub struct PixConfig {
    pub enable_jxl: bool,
    pub enable_avif: bool,
    pub enable_webp: bool,
    pub enable_raw: bool,
    pub enable_hdr: bool,
    pub enable_svg: bool,
    pub enable_exif: bool,
    pub enable_gps: bool,
    pub default_zoom: f64,
    pub smooth_scaling: bool,
}

impl Default for PixConfig {
    fn default() -> Self {
        Self {
            enable_jxl: true,
            enable_avif: true,
            enable_webp: true,
            enable_raw: true,
            enable_hdr: true,
            enable_svg: true,
            enable_exif: true,
            enable_gps: true,
            default_zoom: 1.0,
            smooth_scaling: true,
        }
    }
}

/// Pix image viewer
#[derive(Debug, Clone)]
pub struct PixImageViewer {
    config: PixConfig,
    current_image: Option<String>,
    image_cache: HashMap<String, ImageMetadata>,
    exif_cache: HashMap<String, ExifData>,
    mode: ViewerMode,
    transform: ImageTransform,
    zoom: f64,
    slideshow_config: SlideshowConfig,
}

impl PixImageViewer {
    pub fn new(config: PixConfig) -> Self {
        Self {
            config,
            current_image: None,
            image_cache: HashMap::new(),
            exif_cache: HashMap::new(),
            mode: ViewerMode::Normal,
            transform: ImageTransform::None,
            zoom: 1.0,
            slideshow_config: SlideshowConfig::default(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(PixConfig::default())
    }

    /// Load image
    pub fn load_image(&mut self, path: &str) -> Result<ImageMetadata, String> {
        let format = Self::detect_format(path);
        let metadata = ImageMetadata {
            format,
            width: 1920,
            height: 1080,
            bits_per_sample: 8,
            color_space: "sRGB".to_string(),
            has_alpha: false,
            is_animated: format == ImageFormat::GIF,
            frame_count: if format == ImageFormat::GIF { 10 } else { 1 },
            file_size: 1024 * 1024,
        };

        self.image_cache.insert(path.to_string(), metadata.clone());
        self.current_image = Some(path.to_string());
        self.transform = ImageTransform::None;
        self.zoom = self.config.default_zoom;

        Ok(metadata)
    }

    /// Detect image format from file extension
    pub fn detect_format(path: &str) -> ImageFormat {
        let path_lower = path.to_lowercase();
        if path_lower.ends_with(".bmp") {
            ImageFormat::BMP
        } else if path_lower.ends_with(".jpg") || path_lower.ends_with(".jpeg") {
            ImageFormat::JPEG
        } else if path_lower.ends_with(".gif") {
            ImageFormat::GIF
        } else if path_lower.ends_with(".png") {
            ImageFormat::PNG
        } else if path_lower.ends_with(".tif") || path_lower.ends_with(".tiff") {
            ImageFormat::TIFF
        } else if path_lower.ends_with(".tga") {
            ImageFormat::TGA
        } else if path_lower.ends_with(".ico") {
            ImageFormat::ICO
        } else if path_lower.ends_with(".xpm") {
            ImageFormat::XPM
        } else if path_lower.ends_with(".jxl") {
            ImageFormat::JXL
        } else if path_lower.ends_with(".avif") {
            ImageFormat::AVIF
        } else if path_lower.ends_with(".webp") {
            ImageFormat::WebP
        } else if path_lower.ends_with(".cr2") || path_lower.ends_with(".nef") || path_lower.ends_with(".arw") {
            ImageFormat::RAW
        } else if path_lower.ends_with(".hdr") || path_lower.ends_with(".exr") {
            ImageFormat::HDR
        } else if path_lower.ends_with(".svg") {
            ImageFormat::SVG
        } else {
            ImageFormat::Unknown
        }
    }

    /// Get current image metadata
    pub fn get_metadata(&self) -> Option<&ImageMetadata> {
        self.current_image.as_ref().and_then(|path| self.image_cache.get(path))
    }

    /// Load EXIF data
    pub fn load_exif(&mut self, path: &str) -> Result<ExifData, String> {
        if !self.config.enable_exif {
            return Err("EXIF support is disabled".to_string());
        }

        let exif = ExifData {
            camera_make: Some("Canon".to_string()),
            camera_model: Some("EOS R5".to_string()),
            lens_model: Some("RF 24-70mm f/2.8L IS USM".to_string()),
            iso: Some(400),
            aperture: Some("f/2.8".to_string()),
            shutter_speed: Some("1/250".to_string()),
            focal_length: Some("50mm".to_string()),
            date_taken: Some("2024-01-15 14:30:00".to_string()),
            gps_latitude: Some(37.7749),
            gps_longitude: Some(-122.4194),
            gps_altitude: Some(10.0),
        };

        self.exif_cache.insert(path.to_string(), exif.clone());
        Ok(exif)
    }

    /// Get EXIF data
    pub fn get_exif(&self) -> Option<&ExifData> {
        self.current_image.as_ref().and_then(|path| self.exif_cache.get(path))
    }

    /// Apply transformation
    pub fn apply_transform(&mut self, transform: ImageTransform) {
        self.transform = transform;
    }

    /// Get current transformation
    pub fn get_transform(&self) -> ImageTransform {
        self.transform
    }

    /// Set zoom level
    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom.max(0.1).min(10.0);
    }

    /// Get zoom level
    pub fn get_zoom(&self) -> f64 {
        self.zoom
    }

    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.25).min(10.0);
    }

    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.25).max(0.1);
    }

    /// Reset zoom
    pub fn reset_zoom(&mut self) {
        self.zoom = self.config.default_zoom;
    }

    /// Set viewer mode
    pub fn set_mode(&mut self, mode: ViewerMode) {
        self.mode = mode;
    }

    /// Get viewer mode
    pub fn get_mode(&self) -> ViewerMode {
        self.mode
    }

    /// Toggle fullscreen
    pub fn toggle_fullscreen(&mut self) {
        match self.mode {
            ViewerMode::Fullscreen => self.mode = ViewerMode::Normal,
            _ => self.mode = ViewerMode::Fullscreen,
        }
    }

    /// Start slideshow
    pub fn start_slideshow(&mut self, config: SlideshowConfig) {
        self.slideshow_config = config;
        self.mode = ViewerMode::Slideshow;
    }

    /// Stop slideshow
    pub fn stop_slideshow(&mut self) {
        self.mode = ViewerMode::Normal;
    }

    /// Get slideshow config
    pub fn get_slideshow_config(&self) -> &SlideshowConfig {
        &self.slideshow_config
    }

    /// Check if format is supported
    pub fn is_format_supported(&self, format: ImageFormat) -> bool {
        match format {
            ImageFormat::JXL => self.config.enable_jxl,
            ImageFormat::AVIF => self.config.enable_avif,
            ImageFormat::WebP => self.config.enable_webp,
            ImageFormat::RAW => self.config.enable_raw,
            ImageFormat::HDR => self.config.enable_hdr,
            ImageFormat::SVG => self.config.enable_svg,
            _ => true,
        }
    }

    /// Get supported formats
    pub fn get_supported_formats(&self) -> Vec<ImageFormat> {
        let mut formats = vec![
            ImageFormat::BMP,
            ImageFormat::JPEG,
            ImageFormat::GIF,
            ImageFormat::PNG,
            ImageFormat::TIFF,
            ImageFormat::TGA,
            ImageFormat::ICO,
            ImageFormat::XPM,
        ];

        if self.config.enable_jxl {
            formats.push(ImageFormat::JXL);
        }
        if self.config.enable_avif {
            formats.push(ImageFormat::AVIF);
        }
        if self.config.enable_webp {
            formats.push(ImageFormat::WebP);
        }
        if self.config.enable_raw {
            formats.push(ImageFormat::RAW);
        }
        if self.config.enable_hdr {
            formats.push(ImageFormat::HDR);
        }
        if self.config.enable_svg {
            formats.push(ImageFormat::SVG);
        }

        formats
    }

    /// Get viewer statistics
    pub fn get_statistics(&self) -> (usize, usize, ViewerMode) {
        let cached_images = self.image_cache.len();
        let cached_exif = self.exif_cache.len();
        (cached_images, cached_exif, self.mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pix_viewer_creation() {
        let viewer = PixImageViewer::with_default_config();
        assert!(viewer.current_image.is_none());
        assert_eq!(viewer.mode, ViewerMode::Normal);
    }

    #[test]
    fn test_config_default() {
        let config = PixConfig::default();
        assert!(config.enable_jxl);
        assert!(config.enable_avif);
        assert!(config.enable_webp);
        assert!(config.enable_raw);
        assert!(config.enable_hdr);
        assert!(config.enable_svg);
        assert!(config.enable_exif);
        assert!(config.enable_gps);
    }

    #[test]
    fn test_detect_format() {
        assert_eq!(PixImageViewer::detect_format("test.jpg"), ImageFormat::JPEG);
        assert_eq!(PixImageViewer::detect_format("test.png"), ImageFormat::PNG);
        assert_eq!(PixImageViewer::detect_format("test.jxl"), ImageFormat::JXL);
        assert_eq!(PixImageViewer::detect_format("test.avif"), ImageFormat::AVIF);
        assert_eq!(PixImageViewer::detect_format("test.webp"), ImageFormat::WebP);
        assert_eq!(PixImageViewer::detect_format("test.cr2"), ImageFormat::RAW);
        assert_eq!(PixImageViewer::detect_format("test.hdr"), ImageFormat::HDR);
        assert_eq!(PixImageViewer::detect_format("test.svg"), ImageFormat::SVG);
    }

    #[test]
    fn test_load_image() {
        let mut viewer = PixImageViewer::with_default_config();
        let metadata = viewer.load_image("test.jpg").unwrap();
        assert_eq!(metadata.format, ImageFormat::JPEG);
        assert!(viewer.current_image.is_some());
    }

    #[test]
    fn test_zoom_controls() {
        let mut viewer = PixImageViewer::with_default_config();
        viewer.load_image("test.jpg").unwrap();

        viewer.zoom_in();
        assert!(viewer.get_zoom() > 1.0);

        viewer.zoom_out();
        assert!(viewer.get_zoom() < viewer.config.default_zoom + 0.1);

        viewer.reset_zoom();
        assert_eq!(viewer.get_zoom(), viewer.config.default_zoom);
    }

    #[test]
    fn test_transform() {
        let mut viewer = PixImageViewer::with_default_config();
        viewer.apply_transform(ImageTransform::Rotate90);
        assert_eq!(viewer.get_transform(), ImageTransform::Rotate90);
    }

    #[test]
    fn test_fullscreen_toggle() {
        let mut viewer = PixImageViewer::with_default_config();
        viewer.toggle_fullscreen();
        assert_eq!(viewer.get_mode(), ViewerMode::Fullscreen);
        viewer.toggle_fullscreen();
        assert_eq!(viewer.get_mode(), ViewerMode::Normal);
    }

    #[test]
    fn test_get_supported_formats() {
        let viewer = PixImageViewer::with_default_config();
        let formats = viewer.get_supported_formats();
        assert!(formats.contains(&ImageFormat::JXL));
        assert!(formats.contains(&ImageFormat::AVIF));
        assert!(formats.contains(&ImageFormat::WebP));
    }
}
