// SPDX-License-Identifier: MIT
// SigmaOS - Screenshot Tool
// Inspired by GNOME Screenshot and Spectacle
// Provides screenshot capture functionality

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Capture mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode {
    Screen,
    Window,
    Selection,
    Area,
}

/// Capture delay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureDelay {
    None,
    OneSecond,
    ThreeSeconds,
    FiveSeconds,
    Custom(u32),
}

/// Include cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncludeCursor {
    None,
    Default,
    White,
    Black,
}

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    PNG,
    JPEG,
    WEBP,
    BMP,
}

/// Screenshot options
#[derive(Debug, Clone)]
pub struct ScreenshotOptions {
    pub capture_mode: CaptureMode,
    pub delay: CaptureDelay,
    pub include_cursor: IncludeCursor,
    pub format: ImageFormat,
    pub quality: u32,
    pub save_to_clipboard: bool,
    pub save_to_file: bool,
    pub file_path: String,
}

/// Screenshot
#[derive(Debug, Clone)]
pub struct Screenshot {
    pub id: u32,
    pub timestamp: i64,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub file_path: String,
    pub thumbnail: Vec<u8>,
}

/// Screenshot tool
pub struct ScreenshotTool {
    pub screenshots: Vec<Screenshot>,
    pub default_options: ScreenshotOptions,
    pub last_screenshot: Option<Screenshot>,
}

impl ScreenshotTool {
    pub fn new() -> Self {
        Self {
            screenshots: Vec::new(),
            default_options: ScreenshotOptions {
                capture_mode: CaptureMode::Screen,
                delay: CaptureDelay::None,
                include_cursor: IncludeCursor::Default,
                format: ImageFormat::PNG,
                quality: 90,
                save_to_clipboard: false,
                save_to_file: true,
                file_path: String::new(),
            },
            last_screenshot: None,
        }
    }

    /// Capture screenshot
    pub fn capture(&mut self, options: ScreenshotOptions) -> Result<Screenshot, &'static str> {
        // Simplified - in real implementation would capture actual screen
        let screenshot = Screenshot {
            id: self.screenshots.len() as u32,
            timestamp: 0, // Simplified
            width: 1920,
            height: 1080,
            format: options.format,
            file_path: options.file_path.clone(),
            thumbnail: Vec::new(),
        };

        self.screenshots.push(screenshot.clone());
        self.last_screenshot = Some(screenshot.clone());
        Ok(screenshot)
    }

    /// Capture screen
    pub fn capture_screen(&mut self) -> Result<Screenshot, &'static str> {
        let options = ScreenshotOptions {
            capture_mode: CaptureMode::Screen,
            delay: CaptureDelay::None,
            include_cursor: IncludeCursor::Default,
            format: ImageFormat::PNG,
            quality: 90,
            save_to_clipboard: false,
            save_to_file: true,
            file_path: String::new(),
        };
        self.capture(options)
    }

    /// Capture window
    pub fn capture_window(&mut self) -> Result<Screenshot, &'static str> {
        let options = ScreenshotOptions {
            capture_mode: CaptureMode::Window,
            delay: CaptureDelay::None,
            include_cursor: IncludeCursor::Default,
            format: ImageFormat::PNG,
            quality: 90,
            save_to_clipboard: false,
            save_to_file: true,
            file_path: String::new(),
        };
        self.capture(options)
    }

    /// Capture selection
    pub fn capture_selection(&mut self) -> Result<Screenshot, &'static str> {
        let options = ScreenshotOptions {
            capture_mode: CaptureMode::Selection,
            delay: CaptureDelay::None,
            include_cursor: IncludeCursor::Default,
            format: ImageFormat::PNG,
            quality: 90,
            save_to_clipboard: false,
            save_to_file: true,
            file_path: String::new(),
        };
        self.capture(options)
    }

    /// Set default options
    pub fn set_default_options(&mut self, options: ScreenshotOptions) {
        self.default_options = options;
    }

    /// Get last screenshot
    pub fn get_last_screenshot(&self) -> Option<&Screenshot> {
        self.last_screenshot.as_ref()
    }

    /// Get all screenshots
    pub fn get_all_screenshots(&self) -> &[Screenshot] {
        &self.screenshots
    }

    /// Remove screenshot
    pub fn remove_screenshot(&mut self, id: u32) -> Result<(), &'static str> {
        if id as usize >= self.screenshots.len() {
            return Err("Screenshot not found");
        }
        self.screenshots.remove(id as usize);
        if self.last_screenshot.as_ref().map(|s| s.id) == Some(id) {
            self.last_screenshot = None;
        }
        Ok(())
    }

    /// Clear all screenshots
    pub fn clear_all(&mut self) {
        self.screenshots.clear();
        self.last_screenshot = None;
    }

    /// Screenshot count
    pub fn screenshot_count(&self) -> usize {
        self.screenshots.len()
    }
}

/// Global screenshot tool
static mut SCREENSHOT_TOOL: Option<ScreenshotTool> = None;

/// Initialize screenshot tool
pub fn init_screenshot_tool() {
    unsafe {
        SCREENSHOT_TOOL = Some(ScreenshotTool::new());
    }
}

/// Get screenshot tool
pub fn get_screenshot_tool() -> &'static mut ScreenshotTool {
    unsafe {
        SCREENSHOT_TOOL
            .as_mut()
            .expect("Screenshot tool not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_tool_creation() {
        let tool = ScreenshotTool::new();
        assert_eq!(tool.screenshot_count(), 0);
    }

    #[test]
    fn test_capture_screen() {
        let mut tool = ScreenshotTool::new();
        let result = tool.capture_screen();
        assert!(result.is_ok());
        assert_eq!(tool.screenshot_count(), 1);
    }

    #[test]
    fn test_capture_window() {
        let mut tool = ScreenshotTool::new();
        let result = tool.capture_window();
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_screenshot() {
        let mut tool = ScreenshotTool::new();
        tool.capture_screen().unwrap();
        tool.remove_screenshot(0).unwrap();
        assert_eq!(tool.screenshot_count(), 0);
    }

    #[test]
    fn test_clear_all() {
        let mut tool = ScreenshotTool::new();
        tool.capture_screen().unwrap();
        tool.capture_screen().unwrap();
        tool.clear_all();
        assert_eq!(tool.screenshot_count(), 0);
    }
}
