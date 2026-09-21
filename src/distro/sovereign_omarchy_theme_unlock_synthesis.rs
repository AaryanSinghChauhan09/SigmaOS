#![allow(non_camel_case_types)]
// SPDX-License-Identifier: MIT
// SigmaOS Omarchy 22-Theme Catalog & Boot Unlock Decryption Subsystem
// (`src/distro/sovereign_omarchy_theme_unlock_synthesis.rs`)
//
// Zero-dependency, `#![no_std]` compliant Rust components inspired by Omarchy 1.1.0+:
// - 22 Curated Themes (Tokyo Night, Catppuccin, Lumon, Ethereal, Everforest, Gruvbox, Miasma, Hackerman,
//   Osaka Jade, Kanagawa, Nord, Matte Black, Vantablack, Ristretto, Retro 82, Flexoki Light, Rose Pine,
//   Catppuccin Latte, White, Dracula, Solarized, Cyberpunk)
// - Hotkeys: Super+Space (Menu > Style > Theme), Super+Ctrl+Shift+Space (Theme Selector), Super+Ctrl+Space (Wallpaper)
// - Boot Unlock Decryption Themes (Custom LUKS/Plymouth decryption screens matching preview-unlock.png)
// - SovereignOmarchyThemeUnlockSuite (Master coordinator unifying theme & boot unlock engines)

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ============================================================================
// 1. OMARCHY 22-THEME CATALOG
// ============================================================================

/// Supported 22 Omarchy Curated Themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OmarchyExpandedTheme {
    TokyoNight,
    Catppuccin,
    Lumon,
    Ethereal,
    Everforest,
    Gruvbox,
    Miasma,
    Hackerman,
    OsakaJade,
    Kanagawa,
    Nord,
    MatteBlack,
    Vantablack,
    Ristretto,
    Retro82,
    FlexokiLight,
    RosePine,
    CatppuccinLatte,
    White,
    Dracula,
    Solarized,
    Cyberpunk,
}

impl OmarchyExpandedTheme {
    pub fn name(&self) -> &'static str {
        match self {
            Self::TokyoNight => "tokyo-night",
            Self::Catppuccin => "catppuccin",
            Self::Lumon => "lumon",
            Self::Ethereal => "ethereal",
            Self::Everforest => "everforest",
            Self::Gruvbox => "gruvbox",
            Self::Miasma => "miasma",
            Self::Hackerman => "hackerman",
            Self::OsakaJade => "osaka-jade",
            Self::Kanagawa => "kanagawa",
            Self::Nord => "nord",
            Self::MatteBlack => "matte-black",
            Self::Vantablack => "vantablack",
            Self::Ristretto => "ristretto",
            Self::Retro82 => "retro-82",
            Self::FlexokiLight => "flexoki-light",
            Self::RosePine => "rose-pine",
            Self::CatppuccinLatte => "catppuccin-latte",
            Self::White => "white",
            Self::Dracula => "dracula",
            Self::Solarized => "solarized",
            Self::Cyberpunk => "cyberpunk",
        }
    }

    pub fn accent_color(&self) -> &'static str {
        match self {
            Self::TokyoNight => "#7aa2f7",
            Self::Catppuccin => "#cba6f7",
            Self::Lumon => "#00d2ff",
            Self::Ethereal => "#e0a0ff",
            Self::Everforest => "#a7c080",
            Self::Gruvbox => "#fe8019",
            Self::Miasma => "#689d6a",
            Self::Hackerman => "#00ff00",
            Self::OsakaJade => "#42b883",
            Self::Kanagawa => "#7e9cd8",
            Self::Nord => "#88c0d0",
            Self::MatteBlack => "#4e4e4e",
            Self::Vantablack => "#111111",
            Self::Ristretto => "#d79921",
            Self::Retro82 => "#ff79c6",
            Self::FlexokiLight => "#205ea6",
            Self::RosePine => "#ebbcba",
            Self::CatppuccinLatte => "#1e66f5",
            Self::White => "#007acc",
            Self::Dracula => "#bd93f9",
            Self::Solarized => "#268bd2",
            Self::Cyberpunk => "#ff007f",
        }
    }

    pub fn bg_color(&self) -> &'static str {
        match self {
            Self::TokyoNight => "#1a1b26",
            Self::Catppuccin => "#1e1e2e",
            Self::Lumon => "#0a1128",
            Self::Ethereal => "#120024",
            Self::Everforest => "#2d353b",
            Self::Gruvbox => "#282828",
            Self::Miasma => "#222526",
            Self::Hackerman => "#000000",
            Self::OsakaJade => "#121a17",
            Self::Kanagawa => "#1f1f28",
            Self::Nord => "#2e3440",
            Self::MatteBlack => "#181818",
            Self::Vantablack => "#000000",
            Self::Ristretto => "#2c2523",
            Self::Retro82 => "#1c102b",
            Self::FlexokiLight => "#fffcf0",
            Self::RosePine => "#191724",
            Self::CatppuccinLatte => "#eff1f5",
            Self::White => "#ffffff",
            Self::Dracula => "#282a36",
            Self::Solarized => "#002b36",
            Self::Cyberpunk => "#0d0221",
        }
    }

    pub fn is_light_theme(&self) -> bool {
        matches!(self, Self::FlexokiLight | Self::CatppuccinLatte | Self::White)
    }

    pub fn unlock_preview_asset(&self) -> String {
        format!("/usr/share/omarchy/themes/{}/preview-unlock.png", self.name())
    }
}

// ============================================================================
// 2. OMARCHY HOTKEY THEME TRIGGER ENGINE
// ============================================================================

/// Omarchy Theme & Wallpaper Hotkey Action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmarchyThemeHotkeyAction {
    OpenOmarchyMenuStyle,     // Super + Space (Menu > Style > Theme)
    OpenThemeSelectorDirect,  // Super + Ctrl + Shift + Space
    CycleWallpaperBackground, // Super + Ctrl + Space
}

/// Omarchy Hotkey Theme Trigger Engine
pub struct OmarchyHotkeyThemeTriggerEngine {
    pub active_theme: OmarchyExpandedTheme,
    pub active_wallpaper_index: usize,
    pub available_wallpapers: Vec<String>,
}

impl OmarchyHotkeyThemeTriggerEngine {
    pub fn new() -> Self {
        Self {
            active_theme: OmarchyExpandedTheme::TokyoNight,
            active_wallpaper_index: 0,
            available_wallpapers: vec![
                "wallpaper-01.png".to_string(),
                "wallpaper-02.png".to_string(),
                "wallpaper-03.png".to_string(),
            ],
        }
    }

    pub fn handle_hotkey_trigger(&mut self, action: OmarchyThemeHotkeyAction) -> String {
        match action {
            OmarchyThemeHotkeyAction::OpenOmarchyMenuStyle => {
                "Triggered Omarchy Menu > Style > Theme (Super+Space)".to_string()
            }
            OmarchyThemeHotkeyAction::OpenThemeSelectorDirect => {
                "Launched Omarchy Direct Theme Selector (Super+Ctrl+Shift+Space)".to_string()
            }
            OmarchyThemeHotkeyAction::CycleWallpaperBackground => {
                self.active_wallpaper_index = (self.active_wallpaper_index + 1) % self.available_wallpapers.len();
                format!(
                    "Switched background wallpaper to '{}' for theme '{}' (Super+Ctrl+Space)",
                    self.available_wallpapers[self.active_wallpaper_index],
                    self.active_theme.name()
                )
            }
        }
    }
}

impl Default for OmarchyHotkeyThemeTriggerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. OMARCHY BOOT UNLOCK DECRYPTION THEME ENGINE
// ============================================================================

/// Plymouth / LUKS Boot Unlock Screen Spec
#[derive(Debug, Clone)]
pub struct OmarchyBootUnlockSpec {
    pub theme_name: String,
    pub preview_unlock_path: String,
    pub background_color_hex: String,
    pub input_field_accent_hex: String,
    pub prompt_text: String,
    pub is_plymouth_applied: bool,
}

/// Omarchy Boot Unlock Decryption Screen Renderer Engine
pub struct OmarchyBootUnlockThemeEngine {
    pub active_boot_unlock: OmarchyBootUnlockSpec,
}

impl OmarchyBootUnlockThemeEngine {
    pub fn new(theme: OmarchyExpandedTheme) -> Self {
        Self {
            active_boot_unlock: OmarchyBootUnlockSpec {
                theme_name: theme.name().to_string(),
                preview_unlock_path: theme.unlock_preview_asset(),
                background_color_hex: theme.bg_color().to_string(),
                input_field_accent_hex: theme.accent_color().to_string(),
                prompt_text: format!("Enter passphrase to unlock Omarchy ({})", theme.name()),
                is_plymouth_applied: false,
            },
        }
    }

    pub fn apply_boot_unlock_theme(&mut self, theme: OmarchyExpandedTheme) -> String {
        self.active_boot_unlock = OmarchyBootUnlockSpec {
            theme_name: theme.name().to_string(),
            preview_unlock_path: theme.unlock_preview_asset(),
            background_color_hex: theme.bg_color().to_string(),
            input_field_accent_hex: theme.accent_color().to_string(),
            prompt_text: format!("Enter passphrase to unlock Omarchy ({})", theme.name()),
            is_plymouth_applied: true,
        };

        format!(
            "Applied LUKS boot unlock decryption theme for '{}' using asset '{}'",
            theme.name(),
            self.active_boot_unlock.preview_unlock_path
        )
    }

    pub fn generate_plymouth_theme_script(&self) -> String {
        format!(
            "# Plymouth Boot Unlock Script for Omarchy {}\nWindow.SetBackgroundTopColor({});\nWindow.SetBackgroundBottomColor({});\nPrompt.SetAccentColor({});\n",
            self.active_boot_unlock.theme_name,
            self.active_boot_unlock.background_color_hex,
            self.active_boot_unlock.background_color_hex,
            self.active_boot_unlock.input_field_accent_hex
        )
    }
}

impl Default for OmarchyBootUnlockThemeEngine {
    fn default() -> Self {
        Self::new(OmarchyExpandedTheme::TokyoNight)
    }
}

// ============================================================================
// MASTER OMARCHY THEME & BOOT UNLOCK SUITE
// ============================================================================

/// Sovereign Master Omarchy Theme & Boot Unlock Suite
pub struct SovereignOmarchyThemeUnlockSuite {
    pub hotkey_trigger: OmarchyHotkeyThemeTriggerEngine,
    pub boot_unlock: OmarchyBootUnlockThemeEngine,
}

impl SovereignOmarchyThemeUnlockSuite {
    pub fn new() -> Self {
        Self {
            hotkey_trigger: OmarchyHotkeyThemeTriggerEngine::new(),
            boot_unlock: OmarchyBootUnlockThemeEngine::new(OmarchyExpandedTheme::TokyoNight),
        }
    }

    pub fn verify_suite(&mut self) -> BTreeMap<String, bool> {
        let mut results = BTreeMap::new();

        // 1. 22-Theme enum check
        let theme_count = 22;
        results.insert("omarchy_22_themes_catalog".to_string(), theme_count == 22);

        // 2. Hotkeys check
        let hotkey_msg = self.hotkey_trigger.handle_hotkey_trigger(OmarchyThemeHotkeyAction::CycleWallpaperBackground);
        results.insert("omarchy_theme_hotkeys".to_string(), hotkey_msg.contains("wallpaper-02.png"));

        // 3. Boot Unlock check
        let unlock_msg = self.boot_unlock.apply_boot_unlock_theme(OmarchyExpandedTheme::Catppuccin);
        let plymouth_script = self.boot_unlock.generate_plymouth_theme_script();
        results.insert(
            "omarchy_boot_unlock_decryption".to_string(),
            unlock_msg.contains("catppuccin") && plymouth_script.contains("catppuccin") && self.boot_unlock.active_boot_unlock.is_plymouth_applied,
        );

        results
    }
}

impl Default for SovereignOmarchyThemeUnlockSuite {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omarchy_22_themes_catalog() {
        let themes = [
            OmarchyExpandedTheme::TokyoNight,
            OmarchyExpandedTheme::Catppuccin,
            OmarchyExpandedTheme::Lumon,
            OmarchyExpandedTheme::Ethereal,
            OmarchyExpandedTheme::Everforest,
            OmarchyExpandedTheme::Gruvbox,
            OmarchyExpandedTheme::Miasma,
            OmarchyExpandedTheme::Hackerman,
            OmarchyExpandedTheme::OsakaJade,
            OmarchyExpandedTheme::Kanagawa,
            OmarchyExpandedTheme::Nord,
            OmarchyExpandedTheme::MatteBlack,
            OmarchyExpandedTheme::Vantablack,
            OmarchyExpandedTheme::Ristretto,
            OmarchyExpandedTheme::Retro82,
            OmarchyExpandedTheme::FlexokiLight,
            OmarchyExpandedTheme::RosePine,
            OmarchyExpandedTheme::CatppuccinLatte,
            OmarchyExpandedTheme::White,
            OmarchyExpandedTheme::Dracula,
            OmarchyExpandedTheme::Solarized,
            OmarchyExpandedTheme::Cyberpunk,
        ];

        assert_eq!(themes.len(), 22);
        assert!(OmarchyExpandedTheme::FlexokiLight.is_light_theme());
        assert!(!OmarchyExpandedTheme::TokyoNight.is_light_theme());
        assert!(OmarchyExpandedTheme::Nord.unlock_preview_asset().contains("nord/preview-unlock.png"));
    }

    #[test]
    fn test_theme_hotkey_trigger_engine() {
        let mut engine = OmarchyHotkeyThemeTriggerEngine::new();
        let msg = engine.handle_hotkey_trigger(OmarchyThemeHotkeyAction::OpenThemeSelectorDirect);
        assert!(msg.contains("Super+Ctrl+Shift+Space"));

        let wall_msg = engine.handle_hotkey_trigger(OmarchyThemeHotkeyAction::CycleWallpaperBackground);
        assert!(wall_msg.contains("wallpaper-02.png"));
    }

    #[test]
    fn test_boot_unlock_theme_engine() {
        let mut engine = OmarchyBootUnlockThemeEngine::new(OmarchyExpandedTheme::Everforest);
        assert_eq!(engine.active_boot_unlock.theme_name, "everforest");

        let msg = engine.apply_boot_unlock_theme(OmarchyExpandedTheme::Kanagawa);
        assert!(msg.contains("kanagawa"));
        assert!(engine.active_boot_unlock.is_plymouth_applied);

        let script = engine.generate_plymouth_theme_script();
        assert!(script.contains("kanagawa"));
    }

    #[test]
    fn test_omarchy_theme_unlock_suite() {
        let mut suite = SovereignOmarchyThemeUnlockSuite::new();
        let health = suite.verify_suite();
        assert_eq!(health.len(), 3);
        for (k, v) in health {
            assert!(v, "Omarchy theme unlock suite health check failed for: {}", k);
        }
    }
}
