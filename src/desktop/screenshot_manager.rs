// SigmaOS Desktop Screenshot Manager
// Inspired by Linux Mint's screenshot tool and Omarchy's screenshot utilities

use std::collections::HashMap;
use std::path::PathBuf;

/// Screenshot mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopScreenshotMode {
    FullScreen,
    Window,
    Selection,
    Screen,
}

impl DesktopScreenshotMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopScreenshotMode::FullScreen => "Full Screen",
            DesktopScreenshotMode::Window => "Window",
            DesktopScreenshotMode::Selection => "Selection",
            DesktopScreenshotMode::Screen => "Screen",
        }
    }
}

/// Screenshot format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopScreenshotFormat {
    PNG,
    JPEG,
    BMP,
    WEBP,
}

impl DesktopScreenshotFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopScreenshotFormat::PNG => "PNG",
            DesktopScreenshotFormat::JPEG => "JPEG",
            DesktopScreenshotFormat::BMP => "BMP",
            DesktopScreenshotFormat::WEBP => "WebP",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            DesktopScreenshotFormat::PNG => "png",
            DesktopScreenshotFormat::JPEG => "jpg",
            DesktopScreenshotFormat::BMP => "bmp",
            DesktopScreenshotFormat::WEBP => "webp",
        }
    }
}

/// DesktopScreenshot
#[derive(Debug, Clone)]
pub struct DesktopScreenshot {
    pub id: String,
    pub filename: String,
    pub path: PathBuf,
    pub mode: DesktopScreenshotMode,
    pub format: DesktopScreenshotFormat,
    pub timestamp: u64,
    pub width: u32,
    pub height: u32,
}

impl DesktopScreenshot {
    pub fn new(
        id: String,
        filename: String,
        path: PathBuf,
        mode: DesktopScreenshotMode,
        format: DesktopScreenshotFormat,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        DesktopScreenshot {
            id,
            filename,
            path,
            mode,
            format,
            timestamp,
            width: 1920,
            height: 1080,
        }
    }

    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

/// DesktopScreenshotManager
pub struct DesktopScreenshotManager {
    screenshots: HashMap<String, DesktopScreenshot>,
    default_format: DesktopScreenshotFormat,
    default_mode: DesktopScreenshotMode,
    save_directory: PathBuf,
    next_screenshot_id: u32,
}

impl DesktopScreenshotManager {
    pub fn new() -> Self {
        let save_directory = PathBuf::from("/home/user/Pictures/Screenshots");

        let mut manager = DesktopScreenshotManager {
            screenshots: HashMap::new(),
            default_format: DesktopScreenshotFormat::PNG,
            default_mode: DesktopScreenshotMode::FullScreen,
            save_directory,
            next_screenshot_id: 1,
        };

        manager
    }

    pub fn set_default_format(&mut self, format: DesktopScreenshotFormat) {
        self.default_format = format;
    }

    pub fn set_default_mode(&mut self, mode: DesktopScreenshotMode) {
        self.default_mode = mode;
    }

    pub fn set_save_directory(&mut self, directory: PathBuf) {
        self.save_directory = directory;
    }

    pub fn capture(&mut self, mode: DesktopScreenshotMode) -> String {
        let id = format!("screenshot_{}", self.next_screenshot_id);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let filename = format!("Screenshot_{}.{}", timestamp, self.default_format.extension());
        let path = self.save_directory.join(&filename);

        let screenshot = DesktopScreenshot::new(
            id.clone(),
            filename,
            path,
            mode,
            self.default_format,
        );

        self.screenshots.insert(id.clone(), screenshot);
        self.next_screenshot_id += 1;
        id
    }

    pub fn capture_full_screen(&mut self) -> String {
        self.capture(DesktopScreenshotMode::FullScreen)
    }

    pub fn capture_window(&mut self) -> String {
        self.capture(DesktopScreenshotMode::Window)
    }

    pub fn capture_selection(&mut self) -> String {
        self.capture(DesktopScreenshotMode::Selection)
    }

    pub fn remove_screenshot(&mut self, id: &str) -> bool {
        self.screenshots.remove(id).is_some()
    }

    pub fn get_screenshot(&self, id: &str) -> Option<&DesktopScreenshot> {
        self.screenshots.get(id)
    }

    pub fn get_screenshots(&self) -> Vec<&DesktopScreenshot> {
        self.screenshots.values().collect()
    }

    pub fn get_screenshots_by_mode(&self, mode: DesktopScreenshotMode) -> Vec<&DesktopScreenshot> {
        self.screenshots
            .values()
            .filter(|s| s.mode == mode)
            .collect()
    }

    pub fn get_screenshots_by_format(&self, format: DesktopScreenshotFormat) -> Vec<&DesktopScreenshot> {
        self.screenshots
            .values()
            .filter(|s| s.format == format)
            .collect()
    }

    pub fn get_default_format(&self) -> DesktopScreenshotFormat {
        self.default_format
    }

    pub fn get_default_mode(&self) -> DesktopScreenshotMode {
        self.default_mode
    }

    pub fn get_save_directory(&self) -> &PathBuf {
        &self.save_directory
    }

    pub fn get_statistics(&self) -> DesktopScreenshotStatistics {
        DesktopScreenshotStatistics {
            total_screenshots: self.screenshots.len(),
            full_screen_count: self.get_screenshots_by_mode(DesktopScreenshotMode::FullScreen).len(),
            window_count: self.get_screenshots_by_mode(DesktopScreenshotMode::Window).len(),
            selection_count: self.get_screenshots_by_mode(DesktopScreenshotMode::Selection).len(),
        }
    }
}

impl Default for DesktopScreenshotManager {
    fn default() -> Self {
        Self::new()
    }
}

/// DesktopScreenshotStatistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopScreenshotStatistics {
    pub total_screenshots: usize,
    pub full_screen_count: usize,
    pub window_count: usize,
    pub selection_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_manager_initialization() {
        let manager = DesktopScreenshotManager::new();
        assert_eq!(manager.get_default_format(), DesktopScreenshotFormat::PNG);
        assert_eq!(manager.get_default_mode(), DesktopScreenshotMode::FullScreen);
    }

    #[test]
    fn test_capture() {
        let mut manager = DesktopScreenshotManager::new();
        let id = manager.capture(DesktopScreenshotMode::FullScreen);
        assert!(manager.get_screenshot(&id).is_some());
        assert_eq!(manager.get_screenshots().len(), 1);
    }

    #[test]
    fn test_capture_full_screen() {
        let mut manager = DesktopScreenshotManager::new();
        let id = manager.capture_full_screen();
        assert!(manager.get_screenshot(&id).is_some());
        assert_eq!(manager.get_screenshot(&id).unwrap().mode, DesktopScreenshotMode::FullScreen);
    }

    #[test]
    fn test_capture_window() {
        let mut manager = DesktopScreenshotManager::new();
        let id = manager.capture_window();
        assert!(manager.get_screenshot(&id).is_some());
        assert_eq!(manager.get_screenshot(&id).unwrap().mode, DesktopScreenshotMode::Window);
    }

    #[test]
    fn test_remove_screenshot() {
        let mut manager = DesktopScreenshotManager::new();
        let id = manager.capture(DesktopScreenshotMode::FullScreen);
        assert!(manager.remove_screenshot(&id));
        assert!(!manager.get_screenshot(&id).is_some());
        assert_eq!(manager.get_screenshots().len(), 0);
    }

    #[test]
    fn test_set_default_format() {
        let mut manager = DesktopScreenshotManager::new();
        manager.set_default_format(DesktopScreenshotFormat::JPEG);
        assert_eq!(manager.get_default_format(), DesktopScreenshotFormat::JPEG);
    }

    #[test]
    fn test_set_default_mode() {
        let mut manager = DesktopScreenshotManager::new();
        manager.set_default_mode(DesktopScreenshotMode::Window);
        assert_eq!(manager.get_default_mode(), DesktopScreenshotMode::Window);
    }

    #[test]
    fn test_get_screenshots_by_mode() {
        let mut manager = DesktopScreenshotManager::new();
        manager.capture(DesktopScreenshotMode::FullScreen);
        manager.capture(DesktopScreenshotMode::Window);
        manager.capture(DesktopScreenshotMode::Selection);

        let full_screen = manager.get_screenshots_by_mode(DesktopScreenshotMode::FullScreen);
        assert_eq!(full_screen.len(), 1);
    }

    #[test]
    fn test_statistics() {
        let mut manager = DesktopScreenshotManager::new();
        manager.capture(DesktopScreenshotMode::FullScreen);
        manager.capture(DesktopScreenshotMode::Window);
        let stats = manager.get_statistics();
        assert_eq!(stats.total_screenshots, 2);
    }
}
