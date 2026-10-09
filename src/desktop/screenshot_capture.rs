// Screenshot Capture
// Omarchy-inspired screenshot capture with annotation, OCR, and region selection

use std::path::PathBuf;

/// Screenshot mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenshotMode {
    Region,     // Freeform region selection
    Window,     // Capture specific window
    Fullscreen, // Capture entire screen
    Monitor,    // Capture specific monitor
    Scroll,     // Scrolling region capture
}

/// Screenshot format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenshotFormat {
    Png,
    Jpeg,
    WebP,
}

/// Annotation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationType {
    Arrow,
    Line,
    Rectangle,
    Ellipse,
    Text,
    Highlight,
    Marker,
    Redaction,
}

/// Annotation
#[derive(Debug, Clone)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: String,
    pub text: Option<String>,
    pub font_size: u32,
}

/// Screenshot
#[derive(Debug, Clone)]
pub struct Screenshot {
    pub id: String,
    pub timestamp: String,
    pub file_path: PathBuf,
    pub format: ScreenshotFormat,
    pub width: u32,
    pub height: u32,
    pub mode: ScreenshotMode,
    pub annotations: Vec<Annotation>,
    pub ocr_text: Option<String>,
    pub clipboard_copied: bool,
}

/// Screenshot configuration
#[derive(Debug, Clone)]
pub struct ScreenshotConfig {
    pub format: ScreenshotFormat,
    pub output_dir: PathBuf,
    pub auto_save: bool,
    pub auto_copy: bool,
    pub show_preview: bool,
    pub preview_duration: u32,
    pub include_cursor: bool,
    pub delay_seconds: u32,
}

impl Default for ScreenshotConfig {
    fn default() -> Self {
        ScreenshotConfig {
            format: ScreenshotFormat::Png,
            output_dir: PathBuf::from("/home/user/Pictures/Screenshots"),
            auto_save: true,
            auto_copy: true,
            show_preview: true,
            preview_duration: 10,
            include_cursor: false,
            delay_seconds: 0,
        }
    }
}

/// Screenshot capture
#[derive(Debug, Clone)]
pub struct ScreenshotCapture {
    pub config: ScreenshotConfig,
    pub screenshots: Vec<Screenshot>,
    pub current_screenshot: Option<Screenshot>,
}

impl Default for ScreenshotCapture {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenshotCapture {
    pub fn new() -> Self {
        ScreenshotCapture {
            config: ScreenshotConfig::default(),
            screenshots: Vec::new(),
            current_screenshot: None,
        }
    }

    /// Set configuration
    pub fn set_config(&mut self, config: ScreenshotConfig) {
        self.config = config;
    }

    /// Take screenshot
    pub fn take_screenshot(&mut self, mode: ScreenshotMode) -> Result<Screenshot, String> {
        let timestamp = "2026-01-01_12-00-00".to_string();
        let filename = format!("screenshot-{}.png", timestamp);
        let file_path = self.config.output_dir.join(&filename);

        let screenshot = Screenshot {
            id: timestamp.clone(),
            timestamp,
            file_path: file_path.clone(),
            format: self.config.format,
            width: 1920,
            height: 1080,
            mode,
            annotations: Vec::new(),
            ocr_text: None,
            clipboard_copied: self.config.auto_copy,
        };

        self.current_screenshot = Some(screenshot.clone());
        self.screenshots.push(screenshot.clone());

        Ok(screenshot)
    }

    /// Take screenshot with delay
    pub fn take_screenshot_with_delay(
        &mut self,
        mode: ScreenshotMode,
        delay: u32,
    ) -> Result<Screenshot, String> {
        let original_delay = self.config.delay_seconds;
        self.config.delay_seconds = delay;
        let result = self.take_screenshot(mode);
        self.config.delay_seconds = original_delay;
        result
    }

    // Keep the history entry in sync with edits to the current screenshot.
    fn sync_current(&mut self) {
        if let (Some(current), Some(saved)) =
            (&self.current_screenshot, self.screenshots.last_mut())
        {
            *saved = current.clone();
        }
    }

    /// Add annotation
    pub fn add_annotation(&mut self, annotation: Annotation) -> Result<(), String> {
        if let Some(screenshot) = &mut self.current_screenshot {
            screenshot.annotations.push(annotation);
            self.sync_current();
            Ok(())
        } else {
            Err("No screenshot active".to_string())
        }
    }

    /// Remove annotation
    pub fn remove_annotation(&mut self, index: usize) -> Result<(), String> {
        if let Some(screenshot) = &mut self.current_screenshot {
            if index < screenshot.annotations.len() {
                screenshot.annotations.remove(index);
                self.sync_current();
                Ok(())
            } else {
                Err("Annotation index out of bounds".to_string())
            }
        } else {
            Err("No screenshot active".to_string())
        }
    }

    /// Clear annotations
    pub fn clear_annotations(&mut self) -> Result<(), String> {
        if let Some(screenshot) = &mut self.current_screenshot {
            screenshot.annotations.clear();
            self.sync_current();
            Ok(())
        } else {
            Err("No screenshot active".to_string())
        }
    }

    /// Perform OCR
    pub fn perform_ocr(&mut self) -> Result<String, String> {
        if let Some(screenshot) = &mut self.current_screenshot {
            let text = "OCR extracted text".to_string();
            // In real implementation, this would use Tesseract or similar
            Ok(text.clone())
        } else {
            Err("No screenshot active".to_string())
        }
    }

    /// Crop screenshot
    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> Result<(), String> {
        if let Some(screenshot) = &mut self.current_screenshot {
            screenshot.width = width;
            screenshot.height = height;
            self.sync_current();
            Ok(())
        } else {
            Err("No screenshot active".to_string())
        }
    }

    /// Copy to clipboard
    pub fn copy_to_clipboard(&mut self) -> Result<(), String> {
        if let Some(screenshot) = &mut self.current_screenshot {
            screenshot.clipboard_copied = true;
            self.sync_current();
            Ok(())
        } else {
            Err("No screenshot active".to_string())
        }
    }

    /// Save screenshot
    pub fn save(&mut self) -> Result<PathBuf, String> {
        if let Some(screenshot) = &self.current_screenshot {
            Ok(screenshot.file_path.clone())
        } else {
            Err("No screenshot active".to_string())
        }
    }

    /// Get current screenshot
    pub fn get_current_screenshot(&self) -> Option<&Screenshot> {
        self.current_screenshot.as_ref()
    }

    /// Get all screenshots
    pub fn get_screenshots(&self) -> &[Screenshot] {
        &self.screenshots
    }

    /// Delete screenshot
    pub fn delete_screenshot(&mut self, id: &str) -> Result<(), String> {
        self.screenshots.retain(|s| s.id != id);
        if self
            .current_screenshot
            .as_ref()
            .map(|s| s.id == id)
            .unwrap_or(false)
        {
            self.current_screenshot = None;
        }
        Ok(())
    }

    /// Set format
    pub fn set_format(&mut self, format: ScreenshotFormat) {
        self.config.format = format;
    }

    /// Set output directory
    pub fn set_output_dir(&mut self, dir: PathBuf) {
        self.config.output_dir = dir;
    }

    /// Set auto save
    pub fn set_auto_save(&mut self, auto_save: bool) {
        self.config.auto_save = auto_save;
    }

    /// Set auto copy
    pub fn set_auto_copy(&mut self, auto_copy: bool) {
        self.config.auto_copy = auto_copy;
    }

    /// Set show preview
    pub fn set_show_preview(&mut self, show_preview: bool) {
        self.config.show_preview = show_preview;
    }

    /// Set include cursor
    pub fn set_include_cursor(&mut self, include_cursor: bool) {
        self.config.include_cursor = include_cursor;
    }

    /// Get configuration
    pub fn get_config(&self) -> &ScreenshotConfig {
        &self.config
    }

    /// Get statistics
    pub fn get_statistics(&self) -> (usize, usize) {
        let total_annotations = self.screenshots.iter().map(|s| s.annotations.len()).sum();
        (self.screenshots.len(), total_annotations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_capture_creation() {
        let capture = ScreenshotCapture::new();
        assert_eq!(capture.screenshots.len(), 0);
        assert_eq!(capture.config.auto_save, true);
    }

    #[test]
    fn test_take_screenshot() {
        let mut capture = ScreenshotCapture::new();
        let screenshot = capture.take_screenshot(ScreenshotMode::Fullscreen).unwrap();
        assert_eq!(capture.screenshots.len(), 1);
        assert_eq!(screenshot.mode, ScreenshotMode::Fullscreen);
    }

    #[test]
    fn test_take_screenshot_with_delay() {
        let mut capture = ScreenshotCapture::new();
        let screenshot = capture
            .take_screenshot_with_delay(ScreenshotMode::Region, 5)
            .unwrap();
        assert_eq!(capture.screenshots.len(), 1);
    }

    #[test]
    fn test_add_annotation() {
        let mut capture = ScreenshotCapture::new();
        capture.take_screenshot(ScreenshotMode::Fullscreen).unwrap();
        let annotation = Annotation {
            annotation_type: AnnotationType::Arrow,
            x: 100,
            y: 100,
            width: 50,
            height: 50,
            color: "red".to_string(),
            text: None,
            font_size: 12,
        };
        capture.add_annotation(annotation).unwrap();
        let screenshot = capture.get_current_screenshot().unwrap();
        assert_eq!(screenshot.annotations.len(), 1);
        assert_eq!(capture.get_screenshots()[0].annotations.len(), 1);
    }

    #[test]
    fn test_remove_annotation() {
        let mut capture = ScreenshotCapture::new();
        capture.take_screenshot(ScreenshotMode::Fullscreen).unwrap();
        let annotation = Annotation {
            annotation_type: AnnotationType::Arrow,
            x: 100,
            y: 100,
            width: 50,
            height: 50,
            color: "red".to_string(),
            text: None,
            font_size: 12,
        };
        capture.add_annotation(annotation).unwrap();
        capture.remove_annotation(0).unwrap();
        let screenshot = capture.get_current_screenshot().unwrap();
        assert_eq!(screenshot.annotations.len(), 0);
    }

    #[test]
    fn test_clear_annotations() {
        let mut capture = ScreenshotCapture::new();
        capture.take_screenshot(ScreenshotMode::Fullscreen).unwrap();
        let annotation = Annotation {
            annotation_type: AnnotationType::Arrow,
            x: 100,
            y: 100,
            width: 50,
            height: 50,
            color: "red".to_string(),
            text: None,
            font_size: 12,
        };
        capture.add_annotation(annotation).unwrap();
        capture.clear_annotations().unwrap();
        let screenshot = capture.get_current_screenshot().unwrap();
        assert_eq!(screenshot.annotations.len(), 0);
    }

    #[test]
    fn test_crop() {
        let mut capture = ScreenshotCapture::new();
        capture.take_screenshot(ScreenshotMode::Fullscreen).unwrap();
        capture.crop(100, 100, 800, 600).unwrap();
        let screenshot = capture.get_current_screenshot().unwrap();
        assert_eq!(screenshot.width, 800);
        assert_eq!(screenshot.height, 600);
    }

    #[test]
    fn test_copy_to_clipboard() {
        let mut capture = ScreenshotCapture::new();
        capture.take_screenshot(ScreenshotMode::Fullscreen).unwrap();
        capture.copy_to_clipboard().unwrap();
        let screenshot = capture.get_current_screenshot().unwrap();
        assert!(screenshot.clipboard_copied);
    }

    #[test]
    fn test_delete_screenshot() {
        let mut capture = ScreenshotCapture::new();
        let screenshot = capture.take_screenshot(ScreenshotMode::Fullscreen).unwrap();
        let id = screenshot.id.clone();
        capture.delete_screenshot(&id).unwrap();
        assert_eq!(capture.screenshots.len(), 0);
    }

    #[test]
    fn test_configuration() {
        let mut capture = ScreenshotCapture::new();
        capture.set_format(ScreenshotFormat::Jpeg);
        assert_eq!(capture.config.format, ScreenshotFormat::Jpeg);
        capture.set_auto_save(false);
        assert_eq!(capture.config.auto_save, false);
    }

    #[test]
    fn test_statistics() {
        let mut capture = ScreenshotCapture::new();
        capture.take_screenshot(ScreenshotMode::Fullscreen).unwrap();
        let annotation = Annotation {
            annotation_type: AnnotationType::Arrow,
            x: 100,
            y: 100,
            width: 50,
            height: 50,
            color: "red".to_string(),
            text: None,
            font_size: 12,
        };
        capture.add_annotation(annotation).unwrap();
        let (count, annotations) = capture.get_statistics();
        assert_eq!(count, 1);
        assert_eq!(annotations, 1);
    }
}
