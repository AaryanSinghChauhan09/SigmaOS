// #![no_std]
// #![no_main]

/// Zorin OS Compatibility Subsystem for SigmaOS
/// Implements familiarity-first layout switching, Chameleon dynamic auto-theming,
/// Zorin Connect smartphone integration, and Windows App support.

extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Switchable desktop layout personas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZorinLayout {
    TraditionalWindows = 0,
    MacLike = 1,
    UbuntuLike = 2,
    GnomeStyle = 3,
}

/// Dynamic desktop metrics representing the panel/dock geometries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZorinLayoutMetrics {
    pub taskbar_height: u32,
    pub dock_width: u32,
    pub panel_position_bottom: bool,
    pub has_start_menu: bool,
}

/// Layout switcher matching active persona structures
pub struct ZorinLayoutSwitcher {
    pub active: ZorinLayout,
}

impl ZorinLayoutSwitcher {
    pub fn new() -> Self {
        ZorinLayoutSwitcher {
            active: ZorinLayout::TraditionalWindows,
        }
    }

    pub fn set_layout(&mut self, layout: ZorinLayout) {
        self.active = layout;
    }

    pub fn active_layout(&self) -> ZorinLayout {
        self.active
    }

    pub fn get_metrics(&self) -> ZorinLayoutMetrics {
        match self.active {
            ZorinLayout::TraditionalWindows => ZorinLayoutMetrics {
                taskbar_height: 48,
                dock_width: 0,
                panel_position_bottom: true,
                has_start_menu: true,
            },
            ZorinLayout::MacLike => ZorinLayoutMetrics {
                taskbar_height: 0,
                dock_width: 64,
                panel_position_bottom: true,
                has_start_menu: false,
            },
            ZorinLayout::UbuntuLike => ZorinLayoutMetrics {
                taskbar_height: 40,
                dock_width: 48,
                panel_position_bottom: false, // left panel
                has_start_menu: true,
            },
            ZorinLayout::GnomeStyle => ZorinLayoutMetrics {
                taskbar_height: 32,
                dock_width: 0,
                panel_position_bottom: false, // top panel
                has_start_menu: false,
            },
        }
    }
}

impl Default for ZorinLayoutSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic RGB color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZorinChameleonColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Chameleon Engine adapting theme colors automatically based on wallpapers or environmental lights
pub struct ZorinChameleonEngine;

impl ZorinChameleonEngine {
    /// Adapts the active accent hue based on dominant wallpaper color samples
    pub fn calculate_accent_from_wallpaper(rgb_samples: &[(u8, u8, u8)]) -> ZorinChameleonColor {
        if rgb_samples.is_empty() {
            return ZorinChameleonColor { r: 18, g: 119, b: 211 }; // Zorin OS blue
        }

        let mut sum_r: u32 = 0;
        let mut sum_g: u32 = 0;
        let mut sum_b: u32 = 0;

        for &(r, g, b) in rgb_samples {
            sum_r += r as u32;
            sum_g += g as u32;
            sum_b += b as u32;
        }

        let len = rgb_samples.len() as u32;
        let avg_r = (sum_r / len) as u8;
        let avg_g = (sum_g / len) as u8;
        let avg_b = (sum_b / len) as u8;

        // Enhance color saturation to calculate optimal vibrant accent
        let max_val = avg_r.max(avg_g).max(avg_b);
        if max_val == 0 {
            return ZorinChameleonColor { r: 18, g: 119, b: 211 };
        }

        let scale = 255.0 / (max_val as f32);
        ZorinChameleonColor {
            r: ((avg_r as f32) * scale).min(255.0) as u8,
            g: ((avg_g as f32) * scale).min(255.0) as u8,
            b: ((avg_b as f32) * scale).min(255.0) as u8,
        }
    }

    /// Evaluates readability using W3C contrast ratio formula
    pub fn adaptive_contrast_ratio(background: ZorinChameleonColor, accent: ZorinChameleonColor) -> f32 {
        let l1 = 0.2126 * (background.r as f32 / 255.0) + 0.7152 * (background.g as f32 / 255.0) + 0.0722 * (background.b as f32 / 255.0);
        let l2 = 0.2126 * (accent.r as f32 / 255.0) + 0.7152 * (accent.g as f32 / 255.0) + 0.0722 * (accent.b as f32 / 255.0);

        let max_l = l1.max(l2);
        let min_l = l1.min(l2);

        (max_l + 0.05) / (min_l + 0.05)
    }
}

/// State-machine representation of Zorin Connect pairing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZorinConnectState {
    Unpaired = 0,
    PairingRequested = 1,
    Paired = 2,
    PairingTimedOut = 3,
}

/// Zorin Connect smartphone synchronization manager
pub struct ZorinConnectManager {
    pub state: ZorinConnectState,
    pub clipboard: String,
    pub notifications: Vec<String>,
}

impl ZorinConnectManager {
    pub fn new() -> Self {
        ZorinConnectManager {
            state: ZorinConnectState::Unpaired,
            clipboard: String::new(),
            notifications: Vec::new(),
        }
    }

    pub fn request_pairing(&mut self) {
        self.state = ZorinConnectState::PairingRequested;
    }

    /// Enforces pairing pairing timeouts preventing connection race conditions
    pub fn handle_pairing_timeout(&mut self, elapsed_seconds: u32) {
        if self.state == ZorinConnectState::PairingRequested && elapsed_seconds >= 30 {
            self.state = ZorinConnectState::PairingTimedOut;
        }
    }

    pub fn confirm_pairing(&mut self) {
        if self.state == ZorinConnectState::PairingRequested {
            self.state = ZorinConnectState::Paired;
        }
    }

    pub fn send_notification(&mut self, text: &str) {
        if self.state == ZorinConnectState::Paired {
            self.notifications.push(text.to_string());
        }
    }

    pub fn sync_clipboard(&mut self, text: &str) {
        if self.state == ZorinConnectState::Paired {
            self.clipboard = text.to_string();
        }
    }

    pub fn get_clipboard(&self) -> &str {
        &self.clipboard
    }
}

impl Default for ZorinConnectManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Intercepts Windows executables and offers native alternative hints
pub struct ZorinWindowsAppSupport;

impl ZorinWindowsAppSupport {
    pub fn inspect_package_format(filename: &str) -> Option<&'static str> {
        if filename.ends_with(".exe") || filename.ends_with(".msi") {
            // Suggest standard wine launchers or secure native containerized alternatives
            if filename.contains("office") {
                Some("Warning: Windows binary detected. Consider installing native alternative: 'sigpkg install libreoffice'.")
            } else if filename.contains("photoshop") {
                Some("Warning: Windows binary detected. Consider installing native alternative: 'sigpkg install gimp'.")
            } else {
                Some("Warning: Windows binary detected. We suggest running this securely via Sovereign Wine subsystem.")
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zorin_layout_switching() {
        let mut switcher = ZorinLayoutSwitcher::new();
        assert_eq!(switcher.active_layout(), ZorinLayout::TraditionalWindows);

        let win_metrics = switcher.get_metrics();
        assert!(win_metrics.has_start_menu);
        assert_eq!(win_metrics.taskbar_height, 48);

        switcher.set_layout(ZorinLayout::MacLike);
        assert_eq!(switcher.active_layout(), ZorinLayout::MacLike);

        let mac_metrics = switcher.get_metrics();
        assert!(!mac_metrics.has_start_menu);
        assert_eq!(mac_metrics.dock_width, 64);
    }

    #[test]
    fn test_zorin_chameleon_engine() {
        let samples = vec![
            (10, 10, 10),
            (20, 20, 20),
            (30, 30, 30),
        ];
        let color = ZorinChameleonEngine::calculate_accent_from_wallpaper(&samples);
        assert_eq!(color.r, 255); // Scaled saturation
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);

        let bg = ZorinChameleonColor { r: 0, g: 0, b: 0 };
        let accent = ZorinChameleonColor { r: 255, g: 255, b: 255 };
        let ratio = ZorinChameleonEngine::adaptive_contrast_ratio(bg, accent);
        assert!(ratio > 4.5); // Readable high-contrast ratio
    }

    #[test]
    fn test_zorin_connect_pairing_timeout() {
        let mut connect = ZorinConnectManager::new();
        assert_eq!(connect.state, ZorinConnectState::Unpaired);

        connect.request_pairing();
        assert_eq!(connect.state, ZorinConnectState::PairingRequested);

        // Under 30s pairing limit
        connect.handle_pairing_timeout(15);
        assert_eq!(connect.state, ZorinConnectState::PairingRequested);

        // Over 30s timeout trigger
        connect.handle_pairing_timeout(35);
        assert_eq!(connect.state, ZorinConnectState::PairingTimedOut);
    }

    #[test]
    fn test_zorin_connect_sync() {
        let mut connect = ZorinConnectManager::new();
        connect.request_pairing();
        connect.confirm_pairing();
        assert_eq!(connect.state, ZorinConnectState::Paired);

        connect.sync_clipboard("Synced Text");
        assert_eq!(connect.get_clipboard(), "Synced Text");

        connect.send_notification("Call from smartphone");
        assert_eq!(connect.notifications[0], "Call from smartphone");
    }

    #[test]
    fn test_zorin_windows_app_recommendations() {
        let res_office = ZorinWindowsAppSupport::inspect_package_format("ms_office_installer.exe").unwrap();
        assert!(res_office.contains("libreoffice"));

        let res_generic = ZorinWindowsAppSupport::inspect_package_format("game.msi").unwrap();
        assert!(res_generic.contains("Sovereign Wine"));

        let res_none = ZorinWindowsAppSupport::inspect_package_format("native_pkg.sigpkg");
        assert!(res_none.is_none());
    }
}
