//! SigmaOS Screenshot — capture screen regions, outputs, or windows.
//! Uses grim (Wayland) with slurp for region selection.

#![allow(dead_code)]

/// What to capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureTarget {
    FullScreen,
    ActiveWindow,
    Region,
}

/// A screenshot request.
#[derive(Debug, Clone)]
pub struct Screenshot {
    pub target: CaptureTarget,
    pub path: String,
    pub include_cursor: bool,
}

impl Screenshot {
    pub fn new(target: CaptureTarget, path: impl Into<String>) -> Self {
        Self { target, path: path.into(), include_cursor: false }
    }

    pub fn with_cursor(mut self) -> Self {
        self.include_cursor = true;
        self
    }

    /// Returns the shell command that would capture the screenshot.
    /// (Real impl executes this via std::process::Command)
    pub fn capture_command(&self) -> String {
        let grim = match self.target {
            CaptureTarget::FullScreen   => format!("grim {}", self.path),
            CaptureTarget::ActiveWindow => format!("grim -g \"$(hyprctl activeworkspace -j | jq -r .monitor)\" {}", self.path),
            CaptureTarget::Region       => format!("grim -g \"$(slurp)\" {}", self.path),
        };
        if self.include_cursor {
            format!("{} --include-cursor", grim)
        } else {
            grim
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_screen_command() {
        let s = Screenshot::new(CaptureTarget::FullScreen, "/tmp/ss.png");
        assert!(s.capture_command().starts_with("grim"));
        assert!(s.capture_command().contains("/tmp/ss.png"));
    }

    #[test]
    fn region_uses_slurp() {
        let s = Screenshot::new(CaptureTarget::Region, "/tmp/region.png");
        assert!(s.capture_command().contains("slurp"));
    }

    #[test]
    fn with_cursor_flag() {
        let s = Screenshot::new(CaptureTarget::FullScreen, "/tmp/c.png").with_cursor();
        assert!(s.include_cursor);
        assert!(s.capture_command().contains("--include-cursor"));
    }
}
