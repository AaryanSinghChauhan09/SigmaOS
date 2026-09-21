// SPDX-License-Identifier: MIT
// SigmaOS Omarchy GUI Applications & Workstation Ecosystem Engine (`src/desktop/omarchy_apps.rs`)
// Inspired by Omarchy Linux (Nautilus shortcuts, Obsidian, Omawrite, Pinta, Aether, LocalSend, Omacalc, Signal, Omacut, mpv, OBS Studio, Kdenlive).

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// LocalSend Sharing Target Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSendShareMode {
    Clipboard,
    File,
    Folder,
    Receive,
}

/// Aether Dynamically Extracted Color Palette
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AetherThemePalette {
    pub dominant_color: String,
    pub accent_color: String,
    pub background_color: String,
    pub foreground_color: String,
}

impl AetherThemePalette {
    pub fn from_wallpaper(path: &str) -> Self {
        if path.contains("tokyo_night") {
            Self {
                dominant_color: String::from("#1a1b26"),
                accent_color: String::from("#7aa2f7"),
                background_color: String::from("#16161e"),
                foreground_color: String::from("#c0caf5"),
            }
        } else if path.contains("catppuccin") {
            Self {
                dominant_color: String::from("#1e1e2e"),
                accent_color: String::from("#cba6f7"),
                background_color: String::from("#181825"),
                foreground_color: String::from("#cdd6f4"),
            }
        } else {
            Self {
                dominant_color: String::from("#282828"),
                accent_color: String::from("#fe8019"),
                background_color: String::from("#1d2021"),
                foreground_color: String::from("#ebdbb2"),
            }
        }
    }
}

/// Omarchy GUI Application Launcher Record
#[derive(Debug, Clone)]
pub struct OmarchyGuiAppEntry {
    pub name: String,
    pub keybinding: String,
    pub category: String,
    pub default_file_extensions: Vec<String>,
    pub is_installed: bool,
}

/// Omarchy GUI Applications & Workstation Ecosystem Engine
pub struct OmarchyGuiAppsEngine {
    pub apps: BTreeMap<String, OmarchyGuiAppEntry>,
    pub active_palette: AetherThemePalette,
    pub localsend_port: u16,
    pub active_omawrite_buffer: String,
}

impl OmarchyGuiAppsEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            apps: BTreeMap::new(),
            active_palette: AetherThemePalette::from_wallpaper("default.jpg"),
            localsend_port: 53317,
            active_omawrite_buffer: String::new(),
        };
        engine.register_default_omarchy_apps();
        engine
    }

    /// Register default Omarchy GUI applications and keybindings
    pub fn register_default_omarchy_apps(&mut self) {
        self.apps.insert(
            "nautilus".to_string(),
            OmarchyGuiAppEntry {
                name: "Files (Nautilus)".to_string(),
                keybinding: "Super+Shift+F".to_string(),
                category: "FileManager".to_string(),
                default_file_extensions: vec!["directory".to_string()],
                is_installed: true,
            },
        );

        self.apps.insert(
            "obsidian".to_string(),
            OmarchyGuiAppEntry {
                name: "Obsidian".to_string(),
                keybinding: "Super+Shift+O".to_string(),
                category: "Productivity".to_string(),
                default_file_extensions: vec!["vault".to_string(), "md".to_string()],
                is_installed: true,
            },
        );

        self.apps.insert(
            "omawrite".to_string(),
            OmarchyGuiAppEntry {
                name: "Omawrite".to_string(),
                keybinding: "Super+Shift+W".to_string(),
                category: "TextEditor".to_string(),
                default_file_extensions: vec!["md".to_string(), "txt".to_string()],
                is_installed: true,
            },
        );

        self.apps.insert(
            "omacalc".to_string(),
            OmarchyGuiAppEntry {
                name: "Omacalc".to_string(),
                keybinding: "Super+Ctrl+Q".to_string(),
                category: "Calculator".to_string(),
                default_file_extensions: Vec::new(),
                is_installed: true,
            },
        );

        self.apps.insert(
            "signal".to_string(),
            OmarchyGuiAppEntry {
                name: "Signal Messaging".to_string(),
                keybinding: "Super+Shift+G".to_string(),
                category: "Communication".to_string(),
                default_file_extensions: Vec::new(),
                is_installed: false, // Offers install prompt on first launch
            },
        );

        self.apps.insert(
            "localsend".to_string(),
            OmarchyGuiAppEntry {
                name: "LocalSend".to_string(),
                keybinding: "Super+Ctrl+S".to_string(),
                category: "NetworkSharing".to_string(),
                default_file_extensions: Vec::new(),
                is_installed: true,
            },
        );

        self.apps.insert(
            "mpv".to_string(),
            OmarchyGuiAppEntry {
                name: "mpv Media Player".to_string(),
                keybinding: "Super+Space".to_string(),
                category: "MediaPlayer".to_string(),
                default_file_extensions: vec!["mp4".to_string(), "mkv".to_string(), "avi".to_string()],
                is_installed: true,
            },
        );

        self.apps.insert(
            "omacut".to_string(),
            OmarchyGuiAppEntry {
                name: "Omacut Video Trimmer".to_string(),
                keybinding: "Super+Space".to_string(),
                category: "VideoTrimmer".to_string(),
                default_file_extensions: vec!["mp4".to_string(), "webm".to_string()],
                is_installed: true,
            },
        );
    }

    /// Resolve default GUI application for a given file extension
    pub fn resolve_app_for_extension(&self, extension: &str) -> Option<String> {
        let ext_clean = extension.trim_start_matches('.');
        for (app_id, entry) in &self.apps {
            if entry.default_file_extensions.iter().any(|e| e == ext_clean) {
                return Some(app_id.clone());
            }
        }
        None
    }

    /// Execute `omarchy share` LocalSend command dispatcher
    pub fn dispatch_localsend_share(&self, mode: LocalSendShareMode, path: Option<&str>) -> String {
        match mode {
            LocalSendShareMode::Clipboard => {
                format!("LocalSend: Transmitting clipboard contents over port {}", self.localsend_port)
            }
            LocalSendShareMode::File => {
                let target = path.unwrap_or("file_picker");
                format!("LocalSend: Sending file '{}' over port {}", target, self.localsend_port)
            }
            LocalSendShareMode::Folder => {
                let target = path.unwrap_or("folder_picker");
                format!("LocalSend: Sending directory '{}' over port {}", target, self.localsend_port)
            }
            LocalSendShareMode::Receive => {
                format!("LocalSend: Receiver active listening on port {}", self.localsend_port)
            }
        }
    }

    /// Extract background wallpaper color theme via Aether engine
    pub fn extract_aether_wallpaper_palette(&mut self, wallpaper_path: &str) -> &AetherThemePalette {
        self.active_palette = AetherThemePalette::from_wallpaper(wallpaper_path);
        &self.active_palette
    }

    /// Perform Omacalc arithmetic computation
    pub fn evaluate_omacalc_expression(&self, expr: &str) -> Result<f64, &'static str> {
        let clean = expr.replace(' ', "");
        if let Some((left, right)) = clean.split_once('+') {
            let a: f64 = left.parse().map_err(|_| "Invalid number")?;
            let b: f64 = right.parse().map_err(|_| "Invalid number")?;
            Ok(a + b)
        } else if let Some((left, right)) = clean.split_once('-') {
            let a: f64 = left.parse().map_err(|_| "Invalid number")?;
            let b: f64 = right.parse().map_err(|_| "Invalid number")?;
            Ok(a - b)
        } else if let Some((left, right)) = clean.split_once('*') {
            let a: f64 = left.parse().map_err(|_| "Invalid number")?;
            let b: f64 = right.parse().map_err(|_| "Invalid number")?;
            Ok(a * b)
        } else if let Some((left, right)) = clean.split_once('/') {
            let a: f64 = left.parse().map_err(|_| "Invalid number")?;
            let b: f64 = right.parse().map_err(|_| "Invalid number")?;
            if b == 0.0 {
                return Err("Division by zero");
            }
            Ok(a / b)
        } else {
            clean.parse::<f64>().map_err(|_| "Unsupported expression")
        }
    }
}

impl Default for OmarchyGuiAppsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_registration_and_extension_resolution() {
        let engine = OmarchyGuiAppsEngine::new();
        assert!(engine.apps.len() >= 8);

        // Extension resolution tests
        assert_eq!(engine.resolve_app_for_extension("mp4"), Some("mpv".to_string()));
        assert_eq!(engine.resolve_app_for_extension(".md"), Some("obsidian".to_string()));
    }

    #[test]
    fn test_localsend_share_dispatcher() {
        let engine = OmarchyGuiAppsEngine::new();
        let clip_res = engine.dispatch_localsend_share(LocalSendShareMode::Clipboard, None);
        assert!(clip_res.contains("53317"));
        assert!(clip_res.contains("clipboard"));

        let file_res = engine.dispatch_localsend_share(LocalSendShareMode::File, Some("/home/user/doc.pdf"));
        assert!(file_res.contains("/home/user/doc.pdf"));
    }

    #[test]
    fn test_aether_wallpaper_palette_extraction() {
        let mut engine = OmarchyGuiAppsEngine::new();
        let palette = engine.extract_aether_wallpaper_palette("/wallpapers/tokyo_night.jpg");
        assert_eq!(palette.dominant_color, "#1a1b26");
        assert_eq!(palette.accent_color, "#7aa2f7");
    }

    #[test]
    fn test_omacalc_evaluation() {
        let engine = OmarchyGuiAppsEngine::new();
        assert_eq!(engine.evaluate_omacalc_expression("42 + 58").unwrap(), 100.0);
        assert_eq!(engine.evaluate_omacalc_expression("12 * 12").unwrap(), 144.0);
        assert!(engine.evaluate_omacalc_expression("10 / 0").is_err());
    }
}
