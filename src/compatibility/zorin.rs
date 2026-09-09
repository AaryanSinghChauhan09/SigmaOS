/// Zorin OS Compatibility Subsystem for SigmaOS
/// Implements familiarity-first layout switching, Chameleon dynamic auto-theming,
/// Zorin Connect smartphone integration, and Windows App support.
use std::string::String;
use std::string::ToString;
use std::vec::Vec;

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

/// Zorin OS Grid Desktop Tile & Window Snap Layout Manager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZorinSnapPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    LeftHalf,
    RightHalf,
    Maximize,
}

pub struct ZorinGridDesktopManager {
    pub active_monitors_count: usize,
    pub is_grid_snap_enabled: bool,
}

impl ZorinGridDesktopManager {
    pub fn new(monitors: usize) -> Self {
        Self {
            active_monitors_count: monitors,
            is_grid_snap_enabled: true,
        }
    }

    pub fn calculate_window_bounds(
        &self,
        snap: ZorinSnapPosition,
        screen_width: u32,
        screen_height: u32,
    ) -> (u32, u32, u32, u32) {
        match snap {
            ZorinSnapPosition::LeftHalf => (0, 0, screen_width / 2, screen_height),
            ZorinSnapPosition::RightHalf => (screen_width / 2, 0, screen_width / 2, screen_height),
            ZorinSnapPosition::TopLeft => (0, 0, screen_width / 2, screen_height / 2),
            ZorinSnapPosition::TopRight => {
                (screen_width / 2, 0, screen_width / 2, screen_height / 2)
            }
            ZorinSnapPosition::BottomLeft => {
                (0, screen_height / 2, screen_width / 2, screen_height / 2)
            }
            ZorinSnapPosition::BottomRight => (
                screen_width / 2,
                screen_height / 2,
                screen_width / 2,
                screen_height / 2,
            ),
            ZorinSnapPosition::Maximize => (0, 0, screen_width, screen_height),
        }
    }
}

impl Default for ZorinGridDesktopManager {
    fn default() -> Self {
        Self::new(1)
    }
}

/// Zorin OS Intellihide Taskbar & Panel Manager
pub struct ZorinIntellihideTaskbar {
    pub is_hidden: bool,
    pub notification_badge_count: AtomicUsize,
    pub active_windows_count: AtomicUsize,
}

impl ZorinIntellihideTaskbar {
    pub fn new() -> Self {
        Self {
            is_hidden: false,
            notification_badge_count: AtomicUsize::new(0),
            active_windows_count: AtomicUsize::new(0),
        }
    }

    pub fn update_window_overlap(&mut self, window_intersects_panel: bool) {
        self.is_hidden = window_intersects_panel;
    }

    pub fn add_notification_badge(&self) {
        self.notification_badge_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn clear_badges(&self) {
        self.notification_badge_count.store(0, Ordering::SeqCst);
    }
}

impl Default for ZorinIntellihideTaskbar {
    fn default() -> Self {
        Self::new()
    }
}

/// Zorin OS Education Focus Mode & Screen Time Control
pub struct ZorinEducationFocusMode {
    pub is_focus_active: bool,
    pub allowed_educational_apps: Vec<String>,
    pub screen_time_limit_minutes: u32,
    pub elapsed_minutes: AtomicUsize,
}

impl ZorinEducationFocusMode {
    pub fn new(time_limit: u32) -> Self {
        let mut apps = Vec::new();
        apps.push(String::from("gcompris"));
        apps.push(String::from("ktouch"));
        apps.push(String::from("stellarium"));
        apps.push(String::from("libreoffice"));

        Self {
            is_focus_active: false,
            allowed_educational_apps: apps,
            screen_time_limit_minutes: time_limit,
            elapsed_minutes: AtomicUsize::new(0),
        }
    }

    pub fn enable_focus_mode(&mut self, enable: bool) {
        self.is_focus_active = enable;
    }

    pub fn is_app_allowed(&self, app_name: &str) -> bool {
        if !self.is_focus_active {
            return true;
        }
        let lower = app_name.to_lowercase();
        self.allowed_educational_apps
            .iter()
            .any(|allowed| lower.contains(allowed))
    }

    pub fn tick_minute(&self) -> bool {
        let current = self.elapsed_minutes.fetch_add(1, Ordering::SeqCst) + 1;
        current >= self.screen_time_limit_minutes as usize
    }
}

/// Zorin OS Adaptive System Sound Theme & Volume Amplification Manager
pub struct ZorinSoundThemeManager {
    pub master_volume_percent: AtomicUsize,
    pub amplification_boost_enabled: bool,
}

impl ZorinSoundThemeManager {
    pub fn new() -> Self {
        Self {
            master_volume_percent: AtomicUsize::new(100),
            amplification_boost_enabled: false,
        }
    }

    pub fn set_volume(&self, volume: usize) {
        let max_vol = if self.amplification_boost_enabled {
            150
        } else {
            100
        };
        self.master_volume_percent
            .store(volume.min(max_vol), Ordering::SeqCst);
    }

    pub fn enable_amplification_boost(&mut self, enable: bool) {
        self.amplification_boost_enabled = enable;
        if !enable && self.master_volume_percent.load(Ordering::SeqCst) > 100 {
            self.master_volume_percent.store(100, Ordering::SeqCst);
        }
    }
}

impl Default for ZorinSoundThemeManager {
    fn default() -> Self {
        Self::new()
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
            return ZorinChameleonColor {
                r: 18,
                g: 119,
                b: 211,
            }; // Zorin OS blue
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
            return ZorinChameleonColor {
                r: 18,
                g: 119,
                b: 211,
            };
        }

        let scale = 255.0 / (max_val as f32);
        ZorinChameleonColor {
            r: ((avg_r as f32) * scale).min(255.0) as u8,
            g: ((avg_g as f32) * scale).min(255.0) as u8,
            b: ((avg_b as f32) * scale).min(255.0) as u8,
        }
    }

    /// Evaluates readability using W3C contrast ratio formula
    pub fn adaptive_contrast_ratio(
        background: ZorinChameleonColor,
        accent: ZorinChameleonColor,
    ) -> f32 {
        let l1 = 0.2126 * (background.r as f32 / 255.0)
            + 0.7152 * (background.g as f32 / 255.0)
            + 0.0722 * (background.b as f32 / 255.0);
        let l2 = 0.2126 * (accent.r as f32 / 255.0)
            + 0.7152 * (accent.g as f32 / 255.0)
            + 0.0722 * (accent.b as f32 / 255.0);

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

/// Zorin OS Grid Window Tiling & Snap Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZorinGridWindowTilingEngine {
    pub screen_width: u32,
    pub screen_height: u32,
    pub enable_auto_tiling: bool,
}

impl ZorinGridWindowTilingEngine {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            screen_width,
            screen_height,
            enable_auto_tiling: true,
        }
    }

    pub fn calculate_snap_rect(&self, position: ZorinSnapPosition) -> (u32, u32, u32, u32) {
        let half_w = self.screen_width / 2;
        let half_h = self.screen_height / 2;

        match position {
            ZorinSnapPosition::LeftHalf => (0, 0, half_w, self.screen_height),
            ZorinSnapPosition::RightHalf => (half_w, 0, half_w, self.screen_height),
            ZorinSnapPosition::TopLeft => (0, 0, half_w, half_h),
            ZorinSnapPosition::TopRight => (half_w, 0, half_w, half_h),
            ZorinSnapPosition::BottomLeft => (0, half_h, half_w, half_h),
            ZorinSnapPosition::BottomRight => (half_w, half_h, half_w, half_h),
            ZorinSnapPosition::Maximize => (0, 0, self.screen_width, self.screen_height),
        }
    }
}

impl Default for ZorinGridWindowTilingEngine {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

/// Zorin OS Sound Theme & Event Audio Feedback Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZorinSoundManager {
    pub sound_theme: String,
    pub enable_event_sounds: bool,
    pub volume_level_percent: u8,
}

impl ZorinSoundManager {
    pub fn new() -> Self {
        Self {
            sound_theme: "zorin".to_string(),
            enable_event_sounds: true,
            volume_level_percent: 80,
        }
    }

    pub fn get_sound_file_path(&self, event_name: &str) -> String {
        format!(
            "/usr/share/sounds/{}/stereo/{}.ogg",
            self.sound_theme, event_name
        )
    }
}

impl Default for ZorinSoundManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Zorin OS Taskbar & Panel Customizer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZorinTaskbarCustomizer {
    pub transparency_percent: u8,
    pub auto_hide: bool,
    pub icon_size_px: u32,
    pub show_window_previews: bool,
}

impl ZorinTaskbarCustomizer {
    pub fn new() -> Self {
        Self {
            transparency_percent: 20,
            auto_hide: false,
            icon_size_px: 32,
            show_window_previews: true,
        }
    }

    pub fn generate_panel_css(&self) -> String {
        format!(
            ".zorin-panel {{ background: rgba(0, 0, 0, {:.2}); icon-size: {}px; }}",
            1.0 - (self.transparency_percent as f32 / 100.0),
            self.icon_size_px
        )
    }
}

impl Default for ZorinTaskbarCustomizer {
    fn default() -> Self {
        Self::new()
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
        let samples = vec![(10, 10, 10), (20, 20, 20), (30, 30, 30)];
        let color = ZorinChameleonEngine::calculate_accent_from_wallpaper(&samples);
        assert_eq!(color.r, 255); // Scaled saturation
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);

        let bg = ZorinChameleonColor { r: 0, g: 0, b: 0 };
        let accent = ZorinChameleonColor {
            r: 255,
            g: 255,
            b: 255,
        };
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
    fn test_zorin_grid_desktop_manager() {
        let grid = ZorinGridDesktopManager::new(2);
        assert!(grid.is_grid_snap_enabled);
        let bounds = grid.calculate_window_bounds(ZorinSnapPosition::LeftHalf, 1920, 1080);
        assert_eq!(bounds, (0, 0, 960, 1080));
    }

    #[test]
    fn test_zorin_intellihide_taskbar() {
        let mut bar = ZorinIntellihideTaskbar::new();
        assert!(!bar.is_hidden);
        bar.update_window_overlap(true);
        assert!(bar.is_hidden);
        bar.add_notification_badge();
        assert_eq!(bar.notification_badge_count.load(Ordering::SeqCst), 1);
        bar.clear_badges();
        assert_eq!(bar.notification_badge_count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_zorin_education_focus_mode() {
        let mut edu = ZorinEducationFocusMode::new(60);
        assert!(edu.is_app_allowed("firefox"));
        edu.enable_focus_mode(true);
        assert!(edu.is_app_allowed("libreoffice_writer"));
        assert!(!edu.is_app_allowed("steam_game"));
        assert!(!edu.tick_minute());
    }

    #[test]
    fn test_zorin_sound_theme_manager() {
        let mut sound = ZorinSoundThemeManager::new();
        sound.set_volume(120);
        assert_eq!(sound.master_volume_percent.load(Ordering::SeqCst), 100);
        sound.enable_amplification_boost(true);
        sound.set_volume(140);
        assert_eq!(sound.master_volume_percent.load(Ordering::SeqCst), 140);
    }

    #[test]
    fn test_zorin_windows_app_recommendations() {
        let res_office =
            ZorinWindowsAppSupport::inspect_package_format("ms_office_installer.exe").unwrap();
        assert!(res_office.contains("libreoffice"));

        let res_generic = ZorinWindowsAppSupport::inspect_package_format("game.msi").unwrap();
        assert!(res_generic.contains("Sovereign Wine"));

        let res_none = ZorinWindowsAppSupport::inspect_package_format("native_pkg.sigpkg");
        assert!(res_none.is_none());
    }

    #[test]
    fn test_zorin_grid_tiling_sound_and_taskbar() {
        let grid = ZorinGridWindowTilingEngine::new(1920, 1080);
        let left_rect = grid.calculate_snap_rect(ZorinSnapPosition::LeftHalf);
        assert_eq!(left_rect, (0, 0, 960, 1080));

        let top_right_rect = grid.calculate_snap_rect(ZorinSnapPosition::TopRight);
        assert_eq!(top_right_rect, (960, 0, 960, 540));

        let sound_mgr = ZorinSoundManager::new();
        let login_sound = sound_mgr.get_sound_file_path("desktop-login");
        assert_eq!(
            login_sound,
            "/usr/share/sounds/zorin/stereo/desktop-login.ogg"
        );

        let taskbar = ZorinTaskbarCustomizer::new();
        let css = taskbar.generate_panel_css();
        assert!(css.contains(".zorin-panel"));
        assert!(css.contains("icon-size: 32px"));
    }
}
