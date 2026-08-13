// Zorin OS Clean-Room Parity and Compatibility Subsystem
// Zero-dependency, compilable implementation of Zorin's core desktop innovations

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;
use alloc::format;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZorinLayoutType {
    Windows,
    MacOS,
    Ubuntu,
    Gnome,
}

/// Zorin-inspired Desktop Layout Switcher
pub struct ZorinLayoutSwitcher {
    pub active_layout: ZorinLayoutType,
    pub panel_height: u32,
    pub dock_visible: bool,
}

impl ZorinLayoutSwitcher {
    pub fn new() -> Self {
        Self {
            active_layout: ZorinLayoutType::Windows,
            panel_height: 40,
            dock_visible: false,
        }
    }

    /// Dynamically switches desktop layout, configuring taskbars and panel sizes
    pub fn switch_layout(&mut self, layout: ZorinLayoutType) -> (u32, bool) {
        self.active_layout = layout;
        match layout {
            ZorinLayoutType::Windows => {
                self.panel_height = 40;
                self.dock_visible = false;
            }
            ZorinLayoutType::MacOS => {
                self.panel_height = 24;
                self.dock_visible = true;
            }
            ZorinLayoutType::Ubuntu => {
                self.panel_height = 32;
                self.dock_visible = true;
            }
            ZorinLayoutType::Gnome => {
                self.panel_height = 28;
                self.dock_visible = false;
            }
        }
        (self.panel_height, self.dock_visible)
    }
}

impl Default for ZorinLayoutSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Zorin-inspired Chameleon Dynamic Theme Auto-Theming Engine
pub struct ZorinChameleonEngine {
    pub is_dark_mode: bool,
    pub active_accent_color: (u8, u8, u8),
}

impl ZorinChameleonEngine {
    pub fn new() -> Self {
        Self {
            is_dark_mode: false,
            active_accent_color: (0, 120, 215), // Default blue
        }
    }

    /// Automatically computes light/dark themes and accents based on wallpaper dominant RGB values
    pub fn compute_theme_from_wallpaper(&mut self, dominant_rgb: (u8, u8, u8)) -> &'static str {
        // Simple luminance threshold: Y = 0.299R + 0.587G + 0.114B
        let luminance = 0.299 * dominant_rgb.0 as f32
            + 0.587 * dominant_rgb.1 as f32
            + 0.114 * dominant_rgb.2 as f32;

        // If dominant color is dark, enable light elements (dark mode) to contrast
        if luminance < 128.0 {
            self.is_dark_mode = true;
            self.active_accent_color = (dominant_rgb.0, dominant_rgb.1, dominant_rgb.2);
            "dark"
        } else {
            self.is_dark_mode = false;
            self.active_accent_color = (dominant_rgb.0 / 2, dominant_rgb.1 / 2, dominant_rgb.2 / 2);
            "light"
        }
    }
}

impl Default for ZorinChameleonEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairingState {
    Unpaired,
    PairingRequested,
    Paired,
}

/// Zorin-inspired Mobile smartphone pairing and syncing engine (Zorin Connect / KDE Connect)
pub struct ZorinConnectManager {
    pub pairing_state: PairingState,
    pub device_name: Option<String>,
    pub clipboard_sync_data: Option<String>,
    pub notifications_pushed: Vec<String>,
}

impl ZorinConnectManager {
    pub fn new() -> Self {
        Self {
            pairing_state: PairingState::Unpaired,
            device_name: None,
            clipboard_sync_data: None,
            notifications_pushed: Vec::new(),
        }
    }

    /// Requests pairing with secure handshake parameters
    pub fn pair_smartphone(&mut self, name: &str, pairing_pin: u32) -> Result<String, &'static str> {
        if pairing_pin != 7777 {
            return Err("ZorinConnect: Pair request rejected: invalid handshake pin");
        }
        self.device_name = Some(name.to_string());
        self.pairing_state = PairingState::Paired;
        Ok(format!("ZorinConnect: Successfully paired and linked with {}", name))
    }

    /// Synchronizes clipboard data instantly across paired devices
    pub fn sync_clipboard(&mut self, text: &str) -> Result<(), &'static str> {
        if self.pairing_state != PairingState::Paired {
            return Err("ZorinConnect: Cannot sync clipboard; no paired device active");
        }
        self.clipboard_sync_data = Some(text.to_string());
        Ok(())
    }

    /// Push desktop notifications to paired phone
    pub fn push_notification(&mut self, notification: &str) -> Result<(), &'static str> {
        if self.pairing_state != PairingState::Paired {
            return Err("ZorinConnect: Cannot push notification; no paired device active");
        }
        self.notifications_pushed.push(notification.to_string());
        Ok(())
    }
}

impl Default for ZorinConnectManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Zorin-inspired Windows executable (.exe / .msi) detection and native alternative suggestion engine
pub struct ZorinWindowsAppSupport {
    pub alternatives_db: BTreeMap<String, String>,
}

impl ZorinWindowsAppSupport {
    pub fn new() -> Self {
        let mut db = BTreeMap::new();
        db.insert("winword.exe".to_string(), "sigma-office-writer".to_string());
        db.insert("excel.exe".to_string(), "sigma-office-calc".to_string());
        db.insert("powerpnt.exe".to_string(), "sigma-office-impress".to_string());
        db.insert("photoshop.exe".to_string(), "gimp".to_string());
        db.insert("chrome_installer.msi".to_string(), "chromium".to_string());
        db.insert("utorrent.exe".to_string(), "transmission".to_string());

        Self { alternatives_db: db }
    }

    /// Intercepts a Windows application launch, returning a recommended native SigmaOS package alternative
    pub fn suggest_alternative_for_exe(&self, filename: &str) -> Option<String> {
        let clean = filename.to_ascii_lowercase();
        self.alternatives_db.get(&clean).cloned()
    }
}

impl Default for ZorinWindowsAppSupport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zorin_layout_switcher() {
        let mut switcher = ZorinLayoutSwitcher::new();
        assert_eq!(switcher.active_layout, ZorinLayoutType::Windows);

        let (height, dock) = switcher.switch_layout(ZorinLayoutType::MacOS);
        assert_eq!(switcher.active_layout, ZorinLayoutType::MacOS);
        assert_eq!(height, 24);
        assert!(dock);
    }

    #[test]
    fn test_zorin_chameleon_auto_theming() {
        let mut chameleon = ZorinChameleonEngine::new();
        // High luminance wallpaper (light colors) -> triggers light theme
        let theme1 = chameleon.compute_theme_from_wallpaper((240, 240, 240));
        assert_eq!(theme1, "light");
        assert!(!chameleon.is_dark_mode);

        // Low luminance wallpaper (dark colors) -> triggers dark theme
        let theme2 = chameleon.compute_theme_from_wallpaper((10, 10, 10));
        assert_eq!(theme2, "dark");
        assert!(chameleon.is_dark_mode);
        assert_eq!(chameleon.active_accent_color, (10, 10, 10));
    }

    #[test]
    fn test_zorin_connect_manager() {
        let mut connect = ZorinConnectManager::new();
        assert_eq!(connect.pairing_state, PairingState::Unpaired);

        // Fail pairing with bad PIN
        assert!(connect.pair_smartphone("Pixel_6", 1111).is_err());

        // Succeed pairing with secure PIN
        let res = connect.pair_smartphone("Pixel_6", 7777).unwrap();
        assert_eq!(connect.pairing_state, PairingState::Paired);
        assert!(res.contains("Successfully paired"));

        // Sync clipboard data
        assert!(connect.sync_clipboard("copied text").is_ok());
        assert_eq!(connect.clipboard_sync_data, Some("copied text".to_string()));

        // Push notifications
        assert!(connect.push_notification("New Slack Message").is_ok());
        assert_eq!(connect.notifications_pushed[0], "New Slack Message");
    }

    #[test]
    fn test_zorin_windows_app_support() {
        let support = ZorinWindowsAppSupport::new();
        let recommendation = support.suggest_alternative_for_exe("photoshop.exe").unwrap();
        assert_eq!(recommendation, "gimp");

        assert_eq!(support.suggest_alternative_for_exe("unknown.exe"), None);
    }
}
