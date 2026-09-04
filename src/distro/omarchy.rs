//! Omarchy (Modern Arch + Hyprland + Omakub Inspiration) Subsystem for SigmaOS
//!
//! Inspired by Omarchy 1.1.0:
//! - Declarative Hyprland Wayland Compositor Configuration & Dwindle Tiling
//! - Dynamic Theme Switcher with Tokyo-Night, Catppuccin, Gruvbox, Nord, Everforest, Kanagawa
//! - Web2App PWA Launcher & Sandbox Generation
//! - Interactive Keybinding Fuzzy-Finder (Wofi / Rofi Parity)
//! - GPU & NVIDIA Early-KMS Hardware Acceleration Configuration
//! - Fast Terminal & Development Environment Provisioner

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use alloc::vec;

/// Supported Omarchy Curated Themes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OmarchyTheme {
    TokyoNight,
    Catppuccin,
    Gruvbox,
    Nord,
    Everforest,
    Kanagawa,
}

impl OmarchyTheme {
    pub fn name(&self) -> &'static str {
        match self {
            Self::TokyoNight => "tokyo-night",
            Self::Catppuccin => "catppuccin",
            Self::Gruvbox => "gruvbox",
            Self::Nord => "nord",
            Self::Everforest => "everforest",
            Self::Kanagawa => "kanagawa",
        }
    }

    pub fn accent_color(&self) -> &'static str {
        match self {
            Self::TokyoNight => "#7aa2f7",
            Self::Catppuccin => "#cba6f7",
            Self::Gruvbox => "#fe8019",
            Self::Nord => "#88c0d0",
            Self::Everforest => "#a7c080",
            Self::Kanagawa => "#7e9cd8",
        }
    }

    pub fn bg_color(&self) -> &'static str {
        match self {
            Self::TokyoNight => "#1a1b26",
            Self::Catppuccin => "#1e1e2e",
            Self::Gruvbox => "#282828",
            Self::Nord => "#2e3440",
            Self::Everforest => "#2d353b",
            Self::Kanagawa => "#1f1f28",
        }
    }

    pub fn fg_color(&self) -> &'static str {
        match self {
            Self::TokyoNight => "#c0caf5",
            Self::Catppuccin => "#cdd6f4",
            Self::Gruvbox => "#ebdbb2",
            Self::Nord => "#d8dee9",
            Self::Everforest => "#d3c6aa",
            Self::Kanagawa => "#dcd7ba",
        }
    }
}

/// Keybinding Action
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeybindingDefinition {
    pub modifiers: Vec<String>,
    pub key: String,
    pub command: String,
    pub description: String,
}

/// Web2App Desktop Launcher Definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebAppSpec {
    pub name: String,
    pub url: String,
    pub icon_url: String,
    pub ozone_wayland: bool,
    pub custom_class: String,
}

/// GPU Acceleration Configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuDriverConfig {
    pub is_nvidia: bool,
    pub driver_package: String,
    pub kernel_headers: String,
    pub early_kms_enabled: bool,
    pub egl_wayland: bool,
    pub vaapi_enabled: bool,
}

/// The Core Omarchy Modern Desktop Engine
#[derive(Debug, Clone)]
pub struct OmarchyModernDesktopEngine {
    pub current_theme: OmarchyTheme,
    pub themes_catalog: Vec<OmarchyTheme>,
    pub keybindings: Vec<KeybindingDefinition>,
    pub webapps: BTreeMap<String, WebAppSpec>,
    pub gpu_config: Option<GpuDriverConfig>,
    pub dark_mode: bool,
}

impl OmarchyModernDesktopEngine {
    pub fn new() -> Self {
        let default_bindings = vec![
            KeybindingDefinition {
                modifiers: vec!["SUPER".to_string()],
                key: "Return".to_string(),
                command: "alacritty".to_string(),
                description: "Launch terminal".to_string(),
            },
            KeybindingDefinition {
                modifiers: vec!["SUPER".to_string()],
                key: "B".to_string(),
                command: "chromium --ozone-platform=wayland".to_string(),
                description: "Launch browser".to_string(),
            },
            KeybindingDefinition {
                modifiers: vec!["SUPER".to_string()],
                key: "Space".to_string(),
                command: "wofi --show drun".to_string(),
                description: "Application launcher".to_string(),
            },
            KeybindingDefinition {
                modifiers: vec!["SUPER".to_string(), "SHIFT".to_string(), "CTRL".to_string()],
                key: "Space".to_string(),
                command: "omarchy-theme-next".to_string(),
                description: "Cycle next desktop theme".to_string(),
            },
            KeybindingDefinition {
                modifiers: vec!["SUPER".to_string()],
                key: "K".to_string(),
                command: "omarchy-show-keybindings".to_string(),
                description: "Show interactive keybindings".to_string(),
            },
        ];

        let mut engine = Self {
            current_theme: OmarchyTheme::TokyoNight,
            themes_catalog: vec![
                OmarchyTheme::TokyoNight,
                OmarchyTheme::Catppuccin,
                OmarchyTheme::Gruvbox,
                OmarchyTheme::Nord,
                OmarchyTheme::Everforest,
                OmarchyTheme::Kanagawa,
            ],
            keybindings: default_bindings,
            webapps: BTreeMap::new(),
            gpu_config: None,
            dark_mode: true,
        };

        // Register default modern webapps inspired by Omarchy
        engine.register_webapp(
            "WhatsApp",
            "https://web.whatsapp.com/",
            "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/whatsapp.png",
        );
        engine.register_webapp(
            "ChatGPT",
            "https://chatgpt.com/",
            "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/chatgpt.png",
        );
        engine.register_webapp(
            "GitHub",
            "https://github.com/",
            "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/github-light.png",
        );
        engine.register_webapp(
            "YouTube",
            "https://youtube.com/",
            "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/youtube.png",
        );

        engine
    }

    /// Cycle to next curated theme
    pub fn cycle_next_theme(&mut self) -> OmarchyTheme {
        let current_idx = self
            .themes_catalog
            .iter()
            .position(|t| t == &self.current_theme)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % self.themes_catalog.len();
        self.current_theme = self.themes_catalog[next_idx].clone();
        self.current_theme.clone()
    }

    /// Generate declarative Wayland / Hyprland styling configuration
    pub fn generate_hyprland_theme_config(&self) -> String {
        format!(
            "# Omarchy Autogenerated Hyprland Look & Feel
\            \ = {}
\            \ = rgb({})
\            \ = rgb({})
\            
\            general {{
    \                gaps_in = 6
    \                gaps_out = 12
    \                border_size = 2
    \                col.active_border = rgba({}ee) rgba({}ee) 45deg
    \                col.inactive_border = rgba({}aa)
    \                layout = dwindle
\            }}
\            
\            decoration {{
    \                rounding = 10
    \                blur {{
        \                    enabled = true
        \                    size = 5
        \                    passes = 2
    \                }}
\            }}
",
            self.current_theme.name(),
            self.current_theme.accent_color().trim_start_matches('#'),
            self.current_theme.bg_color().trim_start_matches('#'),
            self.current_theme.accent_color().trim_start_matches('#'),
            self.current_theme.fg_color().trim_start_matches('#'),
            self.current_theme.bg_color().trim_start_matches('#'),
        )
    }

    /// Generate Alacritty terminal configuration for the active theme
    pub fn generate_alacritty_theme_config(&self) -> String {
        format!(
            "[colors.primary]
\            background = "{}"
\            foreground = "{}"
\            
\            [colors.cursor]
\            text = "{}"
\            cursor = "{}"
",
            self.current_theme.bg_color(),
            self.current_theme.fg_color(),
            self.current_theme.bg_color(),
            self.current_theme.accent_color()
        )
    }

    /// Register a Web2App PWA launcher
    pub fn register_webapp(&mut self, name: &str, url: &str, icon_url: &str) {
        let spec = WebAppSpec {
            name: name.to_string(),
            url: url.to_string(),
            icon_url: icon_url.to_string(),
            ozone_wayland: true,
            custom_class: name.to_string(),
        };
        self.webapps.insert(name.to_string(), spec);
    }

    /// Generate  entry content for Web2App launcher
    pub fn generate_desktop_entry(&self, app_name: &str) -> Option<String> {
        self.webapps.get(app_name).map(|app| {
            format!(
                "[Desktop Entry]
\                Version=1.0
\                Name={}
\                Comment=Omarchy Web2App for {}
\                Exec=chromium --new-window --ozone-platform=wayland --app="{}" --name="{}" --class="{}"
\                Terminal=false
\                Type=Application
\                Icon={}
\                StartupNotify=true
",
                app.name, app.name, app.url, app.custom_class, app.custom_class, app.icon_url
            )
        })
    }

    /// Format keybindings for interactive display/searching
    pub fn export_keybindings_guide(&self) -> Vec<String> {
        self.keybindings
            .iter()
            .map(|b| {
                let combo = if b.modifiers.is_empty() {
                    b.key.clone()
                } else {
                    format!("{} + {}", b.modifiers.join(" + "), b.key)
                };
                format!("{:<25} -> {:<30} # {}", combo, b.command, b.description)
            })
            .collect()
    }

    /// Detect and configure hardware acceleration
    pub fn configure_nvidia_early_kms(
        &mut self,
        gpu_name: &str,
        kernel_flavor: &str,
    ) -> GpuDriverConfig {
        let is_turing_or_newer = gpu_name.contains("RTX") || gpu_name.contains("GTX 16");
        let driver_pkg = if is_turing_or_newer {
            "nvidia-open-dkms".to_string()
        } else {
            "nvidia-dkms".to_string()
        };

        let headers = match kernel_flavor {
            "zen" => "linux-zen-headers".to_string(),
            "lts" => "linux-lts-headers".to_string(),
            "hardened" => "linux-hardened-headers".to_string(),
            _ => "linux-headers".to_string(),
        };

        let config = GpuDriverConfig {
            is_nvidia: true,
            driver_package: driver_pkg,
            kernel_headers: headers,
            early_kms_enabled: true,
            egl_wayland: true,
            vaapi_enabled: true,
        };

        self.gpu_config = Some(config.clone());
        config
    }
}

impl Default for OmarchyModernDesktopEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omarchy_theme_cycling() {
        let mut engine = OmarchyModernDesktopEngine::new();
        assert_eq!(engine.current_theme, OmarchyTheme::TokyoNight);
        let next = engine.cycle_next_theme();
        assert_eq!(next, OmarchyTheme::Catppuccin);
        assert_eq!(engine.current_theme, OmarchyTheme::Catppuccin);
    }

    #[test]
    fn test_omarchy_hyprland_config_gen() {
        let engine = OmarchyModernDesktopEngine::new();
        let config = engine.generate_hyprland_theme_config();
        assert!(config.contains("theme_name = tokyo-night"));
        assert!(config.contains("layout = dwindle"));
    }

    #[test]
    fn test_omarchy_web2app_registration() {
        let mut engine = OmarchyModernDesktopEngine::new();
        engine.register_webapp("Slack", "https://app.slack.com/", "https://example.com/slack.png");
        let desktop = engine.generate_desktop_entry("Slack").unwrap();
        assert!(desktop.contains("Name=Slack"));
        assert!(desktop.contains("--ozone-platform=wayland"));
    }

    #[test]
    fn test_omarchy_nvidia_early_kms() {
        let mut engine = OmarchyModernDesktopEngine::new();
        let gpu = engine.configure_nvidia_early_kms("NVIDIA GeForce RTX 4080", "zen");
        assert_eq!(gpu.driver_package, "nvidia-open-dkms");
        assert_eq!(gpu.kernel_headers, "linux-zen-headers");
        assert!(gpu.early_kms_enabled);
    }
}
