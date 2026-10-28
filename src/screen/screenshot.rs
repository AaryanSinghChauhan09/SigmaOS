//! SigmaOS Screenshot Tool — Inspired by omacom/omasnap, GNOME Screenshot, Flameshot.
//! Principles: CQS, Least Astonishment, Component Isolation, KISS.

#[derive(Debug, Clone, PartialEq)]
pub enum ScreenshotMode {
    /// Capture the entire desktop
    FullScreen,
    /// Capture only the active window
    ActiveWindow,
    /// Interactive rectangle selection
    SelectRegion,
    /// Timed capture (delay in seconds)
    Timed(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Webp,
    Avif,
}

impl ImageFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Png  => "png",
            Self::Jpeg => "jpg",
            Self::Webp => "webp",
            Self::Avif => "avif",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScreenshotConfig {
    pub mode: ScreenshotMode,
    pub format: ImageFormat,
    pub include_cursor: bool,
    pub include_border: bool,
    pub output_dir: String,
    pub copy_to_clipboard: bool,
    pub annotate_timestamp: bool,
}

impl Default for ScreenshotConfig {
    fn default() -> Self {
        Self {
            mode: ScreenshotMode::FullScreen,
            format: ImageFormat::Png,
            include_cursor: false,
            include_border: true,
            output_dir: "/home/user/Pictures/Screenshots".into(),
            copy_to_clipboard: true,
            annotate_timestamp: false,
        }
    }
}

pub struct ScreenshotTool {
    config: ScreenshotConfig,
    shot_count: u32,
}

impl ScreenshotTool {
    pub fn new(config: ScreenshotConfig) -> Self {
        Self { config, shot_count: 0 }
    }

    /// Generate an auto-filename for this screenshot (query — no side effects).
    pub fn generate_filename(&self) -> String {
        format!(
            "screenshot-{:04}.{}",
            self.shot_count + 1,
            self.config.format.extension()
        )
    }

    /// Capture a screenshot (command).
    pub fn capture(&mut self) -> Result<String, &'static str> {
        self.shot_count += 1;
        let filename = self.generate_filename();
        let full_path = format!("{}/{}", self.config.output_dir, filename);
        Ok(full_path)
    }

    pub fn shot_count(&self) -> u32 { self.shot_count }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_filename_png() {
        let t = ScreenshotTool::new(ScreenshotConfig::default());
        let name = t.generate_filename();
        assert!(name.ends_with(".png"));
    }

    #[test]
    fn test_capture_increments_count() {
        let mut t = ScreenshotTool::new(ScreenshotConfig::default());
        t.capture().unwrap();
        t.capture().unwrap();
        assert_eq!(t.shot_count(), 2);
    }

    #[test]
    fn test_format_extensions() {
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::Jpeg.extension(), "jpg");
        assert_eq!(ImageFormat::Webp.extension(), "webp");
        assert_eq!(ImageFormat::Avif.extension(), "avif");
    }
}
