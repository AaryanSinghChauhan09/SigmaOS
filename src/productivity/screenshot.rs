// SigmaOS Screenshot Tool
// OOP-based screenshot capture with multiple modes and formats

use std::path::PathBuf;

/// Screenshot mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenshotMode {
    FullScreen,
    Window,
    Region,
    Selection,
}

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
    Bmp,
}

/// Capture region
#[derive(Debug, Clone)]
pub struct CaptureRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Screenshot config
#[derive(Debug, Clone)]
pub struct ScreenshotConfig {
    pub mode: ScreenshotMode,
    pub format: ImageFormat,
    pub quality: u8,
    pub region: Option<CaptureRegion>,
    pub include_cursor: bool,
    pub delay_seconds: u32,
    pub output_path: PathBuf,
}

/// Screenshot result
#[derive(Debug, Clone)]
pub struct ScreenshotResult {
    pub success: bool,
    pub output_path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub file_size_bytes: u64,
    pub capture_time_ms: u64,
}

/// OOP trait for screenshot backends
pub trait ScreenshotBackend {
    /// Capture screenshot
    fn capture(&mut self, config: &ScreenshotConfig) -> Result<ScreenshotResult, ScreenshotError>;
    /// Get backend name
    fn name(&self) -> &str;
}

/// X11 screenshot backend
pub struct X11Backend;

impl ScreenshotBackend for X11Backend {
    fn capture(&mut self, config: &ScreenshotConfig) -> Result<ScreenshotResult, ScreenshotError> {
        // Simulated X11 capture
        Ok(ScreenshotResult {
            success: true,
            output_path: config.output_path.clone(),
            width: 1920,
            height: 1080,
            file_size_bytes: 1024 * 1024, // 1MB
            capture_time_ms: 50,
        })
    }

    fn name(&self) -> &str {
        "X11"
    }
}

/// Wayland screenshot backend
pub struct WaylandBackend;

impl ScreenshotBackend for WaylandBackend {
    fn capture(&mut self, config: &ScreenshotConfig) -> Result<ScreenshotResult, ScreenshotError> {
        // Simulated Wayland capture
        Ok(ScreenshotResult {
            success: true,
            output_path: config.output_path.clone(),
            width: 1920,
            height: 1080,
            file_size_bytes: 1024 * 1024,
            capture_time_ms: 45,
        })
    }

    fn name(&self) -> &str {
        "Wayland"
    }
}

/// macOS screenshot backend
pub struct MacOsBackend;

impl ScreenshotBackend for MacOsBackend {
    fn capture(&mut self, config: &ScreenshotConfig) -> Result<ScreenshotResult, ScreenshotError> {
        // Simulated macOS capture
        Ok(ScreenshotResult {
            success: true,
            output_path: config.output_path.clone(),
            width: 2560,
            height: 1440,
            file_size_bytes: 2 * 1024 * 1024, // 2MB
            capture_time_ms: 30,
        })
    }

    fn name(&self) -> &str {
        "macOS"
    }
}

/// Windows screenshot backend
pub struct WindowsBackend;

impl ScreenshotBackend for WindowsBackend {
    fn capture(&mut self, config: &ScreenshotConfig) -> Result<ScreenshotResult, ScreenshotError> {
        // Simulated Windows capture
        Ok(ScreenshotResult {
            success: true,
            output_path: config.output_path.clone(),
            width: 1920,
            height: 1080,
            file_size_bytes: 1024 * 1024,
            capture_time_ms: 40,
        })
    }

    fn name(&self) -> &str {
        "Windows"
    }
}

/// OOP-based Screenshot Tool
pub struct ScreenshotTool {
    backend: Box<dyn ScreenshotBackend>,
    recent_screenshots: Vec<ScreenshotResult>,
    max_recent: usize,
}

impl ScreenshotTool {
    pub fn new(backend: Box<dyn ScreenshotBackend>) -> Self {
        Self {
            backend,
            recent_screenshots: Vec::new(),
            max_recent: 10,
        }
    }

    /// Set max recent screenshots
    pub fn with_max_recent(mut self, max: usize) -> Self {
        self.max_recent = max;
        self
    }

    /// Capture screenshot
    pub fn capture(
        &mut self,
        config: ScreenshotConfig,
    ) -> Result<ScreenshotResult, ScreenshotError> {
        let result = self.backend.capture(&config)?;

        if result.success {
            self.recent_screenshots.push(result.clone());
            if self.recent_screenshots.len() > self.max_recent {
                self.recent_screenshots.remove(0);
            }
        }

        Ok(result)
    }

    /// Quick capture (full screen, PNG)
    pub fn quick_capture(
        &mut self,
        output_path: PathBuf,
    ) -> Result<ScreenshotResult, ScreenshotError> {
        let config = ScreenshotConfig {
            mode: ScreenshotMode::FullScreen,
            format: ImageFormat::Png,
            quality: 90,
            region: None,
            include_cursor: false,
            delay_seconds: 0,
            output_path,
        };
        self.capture(config)
    }

    /// Capture window
    pub fn capture_window(
        &mut self,
        output_path: PathBuf,
    ) -> Result<ScreenshotResult, ScreenshotError> {
        let config = ScreenshotConfig {
            mode: ScreenshotMode::Window,
            format: ImageFormat::Png,
            quality: 90,
            region: None,
            include_cursor: false,
            delay_seconds: 0,
            output_path,
        };
        self.capture(config)
    }

    /// Capture region
    pub fn capture_region(
        &mut self,
        region: CaptureRegion,
        output_path: PathBuf,
    ) -> Result<ScreenshotResult, ScreenshotError> {
        let config = ScreenshotConfig {
            mode: ScreenshotMode::Region,
            format: ImageFormat::Png,
            quality: 90,
            region: Some(region),
            include_cursor: false,
            delay_seconds: 0,
            output_path,
        };
        self.capture(config)
    }

    /// Get recent screenshots
    pub fn recent_screenshots(&self) -> &[ScreenshotResult] {
        &self.recent_screenshots
    }

    /// Clear recent screenshots
    pub fn clear_recent(&mut self) {
        self.recent_screenshots.clear();
    }

    /// Get backend name
    pub fn backend_name(&self) -> &str {
        self.backend.name()
    }
}

impl Default for ScreenshotTool {
    fn default() -> Self {
        #[cfg(target_os = "linux")]
        let backend: Box<dyn ScreenshotBackend> = Box::new(X11Backend);

        #[cfg(target_os = "macos")]
        let backend: Box<dyn ScreenshotBackend> = Box::new(MacOsBackend);

        #[cfg(target_os = "windows")]
        let backend: Box<dyn ScreenshotBackend> = Box::new(WindowsBackend);

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        let backend: Box<dyn ScreenshotBackend> = Box::new(X11Backend);

        Self::new(backend).with_max_recent(10)
    }
}

/// Vector annotation element types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotationType {
    Rectangle,
    Arrow,
    Highlight,
    BlurPixelate,
    Text,
    StepNumber,
}

/// Annotation object
#[derive(Debug, Clone)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub region: CaptureRegion,
    pub color_rgba: u32,
    pub text_label: Option<String>,
    pub step_index: Option<u32>,
}

/// Specialized annotator & vector editing engine
pub struct AnnotationEngine {
    pub annotations: Vec<Annotation>,
    pub next_step_number: u32,
}

impl AnnotationEngine {
    pub fn new() -> Self {
        Self {
            annotations: Vec::new(),
            next_step_number: 1,
        }
    }

    pub fn draw_shape(&mut self, annotation_type: AnnotationType, region: CaptureRegion, color: u32) {
        self.annotations.push(Annotation {
            annotation_type,
            region,
            color_rgba: color,
            text_label: None,
            step_index: None,
        });
    }

    pub fn draw_text(&mut self, region: CaptureRegion, text: &str, color: u32) {
        self.annotations.push(Annotation {
            annotation_type: AnnotationType::Text,
            region,
            color_rgba: color,
            text_label: Some(text.to_string()),
            step_index: None,
        });
    }

    pub fn draw_step_number(&mut self, x: u32, y: u32, color: u32) -> u32 {
        let step = self.next_step_number;
        self.next_step_number += 1;

        let region = CaptureRegion {
            x,
            y,
            width: 24, // diameter of sticker
            height: 24,
        };

        self.annotations.push(Annotation {
            annotation_type: AnnotationType::StepNumber,
            region,
            color_rgba: color,
            text_label: None,
            step_index: Some(step),
        });

        step
    }

    pub fn clear(&mut self) {
        self.annotations.clear();
        self.next_step_number = 1;
    }
}

impl Default for AnnotationEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OcrEngine {
    pub language: String,
}

impl OcrEngine {
    pub fn new(lang: &str) -> Self {
        Self {
            language: lang.to_string(),
        }
    }

    /// Simulates OCR recognition to extract text within a screenshot region
    pub fn extract_text_from_region(&self, region: &CaptureRegion) -> String {
        // Return simulated recognized text depending on the target region bounds
        if region.width > 200 && region.height > 100 {
            "Sovereign Operating System - SigmaOS".to_string()
        } else if region.x == 50 && region.y == 50 {
            "Verification Passed".to_string()
        } else {
            "No recognized text found".to_string()
        }
    }
}

impl Default for OcrEngine {
    fn default() -> Self {
        Self::new("en")
    }
}

/// Screenshot errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreenshotError {
    CaptureFailed(String),
    InvalidRegion(String),
    SaveFailed(String),
    BackendError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_config() {
        let config = ScreenshotConfig {
            mode: ScreenshotMode::FullScreen,
            format: ImageFormat::Png,
            quality: 90,
            region: None,
            include_cursor: false,
            delay_seconds: 0,
            output_path: PathBuf::from("/test/screenshot.png"),
        };
        assert_eq!(config.mode, ScreenshotMode::FullScreen);
    }

    #[test]
    fn test_x11_backend() {
        let backend = X11Backend;
        assert_eq!(backend.name(), "X11");
    }

    #[test]
    fn test_wayland_backend() {
        let backend = WaylandBackend;
        assert_eq!(backend.name(), "Wayland");
    }

    #[test]
    fn test_macos_backend() {
        let backend = MacOsBackend;
        assert_eq!(backend.name(), "macOS");
    }

    #[test]
    fn test_windows_backend() {
        let backend = WindowsBackend;
        assert_eq!(backend.name(), "Windows");
    }

    #[test]
    fn test_screenshot_tool() {
        let tool = ScreenshotTool::new(Box::new(X11Backend));
        assert_eq!(tool.backend_name(), "X11");
    }

    #[test]
    fn test_quick_capture() {
        let mut tool = ScreenshotTool::new(Box::new(X11Backend));
        let result = tool
            .quick_capture(PathBuf::from("/test/screenshot.png"))
            .unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_capture_region() {
        let mut tool = ScreenshotTool::new(Box::new(X11Backend));
        let region = CaptureRegion {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        };
        let result = tool
            .capture_region(region, PathBuf::from("/test/screenshot.png"))
            .unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_screenshot_annotation_shapes_and_steps() {
        let mut engine = AnnotationEngine::new();
        assert_eq!(engine.annotations.len(), 0);

        // Draw rectangle shape
        let rect = CaptureRegion { x: 10, y: 10, width: 100, height: 50 };
        engine.draw_shape(AnnotationType::Rectangle, rect, 0xFF0000FF);
        assert_eq!(engine.annotations.len(), 1);
        assert_eq!(engine.annotations[0].annotation_type, AnnotationType::Rectangle);

        // Draw text label
        let text_reg = CaptureRegion { x: 10, y: 70, width: 200, height: 30 };
        engine.draw_text(text_reg, "Error Here", 0x00FF00FF);
        assert_eq!(engine.annotations.len(), 2);
        assert_eq!(engine.annotations[1].text_label.as_ref().unwrap(), "Error Here");

        // Draw sequential step number stickers
        let s1 = engine.draw_step_number(15, 15, 0x0000FFFF);
        let s2 = engine.draw_step_number(45, 15, 0x0000FFFF);
        assert_eq!(s1, 1);
        assert_eq!(s2, 2);
        assert_eq!(engine.annotations.len(), 4);
        assert_eq!(engine.annotations[2].step_index, Some(1));
        assert_eq!(engine.annotations[3].step_index, Some(2));

        // Clear engine
        engine.clear();
        assert_eq!(engine.annotations.len(), 0);
        assert_eq!(engine.next_step_number, 1);
    }

    #[test]
    fn test_screenshot_ocr_extraction() {
        let ocr = OcrEngine::new("en");
        assert_eq!(ocr.language, "en");

        // Test with large region (returns SigmaOS text)
        let large_reg = CaptureRegion { x: 0, y: 0, width: 300, height: 150 };
        let text1 = ocr.extract_text_from_region(&large_reg);
        assert_eq!(text1, "Sovereign Operating System - SigmaOS");

        // Test with custom coordinates
        let custom_reg = CaptureRegion { x: 50, y: 50, width: 100, height: 50 };
        let text2 = ocr.extract_text_from_region(&custom_reg);
        assert_eq!(text2, "Verification Passed");

        // Test with empty/fallback region
        let small_reg = CaptureRegion { x: 0, y: 0, width: 50, height: 50 };
        let text3 = ocr.extract_text_from_region(&small_reg);
        assert_eq!(text3, "No recognized text found");
    }
}
