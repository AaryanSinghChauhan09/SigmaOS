#![allow(dead_code)]

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

/// Display Power Management Signaling (DPMS) state inspired by X11 / Wayland / BSD xset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpmsState {
    On,
    Standby,
    Suspend,
    Off,
}

/// Linux (xscreensaver/gnome-screensaver) and BSD screensaver animation modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreenSaverMode {
    Blank,
    MatrixRain,
    Starfield,
    ColorCycles,
    Custom(String),
}

/// Screen locking state including Wayland ext-session-lock-v1 protocol parity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockState {
    Unlocked,
    Locked,
    Authenticating,
}

/// DBus org.freedesktop.ScreenSaver Inhibit record
#[derive(Debug, Clone)]
pub struct ScreenSaverInhibitor {
    pub cookie: u32,
    pub app_name: String,
    pub reason: String,
}

/// Configuration settings for the ScreenSaver Engine
#[derive(Debug, Clone)]
pub struct ScreenSaverConfig {
    pub screensaver_timeout_secs: u64,
    pub lock_timeout_secs: u64,
    pub dpms_standby_secs: u64,
    pub dpms_suspend_secs: u64,
    pub dpms_off_secs: u64,
    pub mode: ScreenSaverMode,
    pub show_clock_on_lock: bool,
    pub user_name: String,
    pub hashed_passphrase: String, // Mock hashed passphrase
    pub max_failed_auth_attempts: u32,
}

impl Default for ScreenSaverConfig {
    fn default() -> Self {
        Self {
            screensaver_timeout_secs: 300, // 5 minutes
            lock_timeout_secs: 600,        // 10 minutes
            dpms_standby_secs: 900,        // 15 minutes
            dpms_suspend_secs: 1200,       // 20 minutes
            dpms_off_secs: 1800,           // 30 minutes
            mode: ScreenSaverMode::MatrixRain,
            show_clock_on_lock: true,
            user_name: String::from("sigma_user"),
            hashed_passphrase: String::from("passphrase123"),
            max_failed_auth_attempts: 5,
        }
    }
}

/// Rendered frame description for display backends
#[derive(Debug, Clone)]
pub struct ScreenSaverFrame {
    pub active_mode: ScreenSaverMode,
    pub dpms_state: DpmsState,
    pub lock_state: LockState,
    pub status_text: String,
    pub is_inhibited: bool,
    pub failed_attempts: u32,
}

/// Linux & BSD-inspired ScreenSaver and Display Power Management Engine
pub struct ScreenSaverEngine {
    pub config: ScreenSaverConfig,
    pub idle_time_secs: u64,
    pub is_active: bool,
    pub lock_state: LockState,
    pub dpms_state: DpmsState,
    pub frame_counter: u64,
    pub failed_auth_attempts: u32,
    pub inhibitors: HashMap<u32, ScreenSaverInhibitor>,
    pub next_cookie: u32,
}

impl ScreenSaverEngine {
    pub fn new(config: ScreenSaverConfig) -> Self {
        Self {
            config,
            idle_time_secs: 0,
            is_active: false,
            lock_state: LockState::Unlocked,
            dpms_state: DpmsState::On,
            frame_counter: 0,
            failed_auth_attempts: 0,
            inhibitors: HashMap::new(),
            next_cookie: 1000,
        }
    }

    /// Check if screensaver or lock screen activation is currently inhibited by DBus callers
    pub fn is_inhibited(&self) -> bool {
        !self.inhibitors.is_empty()
    }

    /// Register a DBus `org.freedesktop.ScreenSaver.Inhibit` request (e.g. video playback / presentations)
    pub fn inhibit(&mut self, app_name: &str, reason: &str) -> u32 {
        let cookie = self.next_cookie;
        self.next_cookie += 1;
        self.inhibitors.insert(
            cookie,
            ScreenSaverInhibitor {
                cookie,
                app_name: String::from(app_name),
                reason: String::from(reason),
            },
        );
        cookie
    }

    /// Register a DBus `org.freedesktop.ScreenSaver.Uninhibit` request
    pub fn uninhibit(&mut self, cookie: u32) -> bool {
        self.inhibitors.remove(&cookie).is_some()
    }

    /// Called on system timer tick to update user idle time
    pub fn update_idle_time(&mut self, idle_seconds: u64) {
        self.idle_time_secs = idle_seconds;

        // If inhibited by media or browser, prevent screensaver/lock activation and keep DPMS On
        if self.is_inhibited() && self.lock_state == LockState::Unlocked {
            self.is_active = false;
            self.dpms_state = DpmsState::On;
            return;
        }

        // Check if screensaver should activate
        if self.idle_time_secs >= self.config.screensaver_timeout_secs {
            self.is_active = true;
        } else if self.lock_state == LockState::Unlocked {
            self.is_active = false;
        }

        // Check if screen should lock automatically
        if self.idle_time_secs >= self.config.lock_timeout_secs {
            self.lock_state = LockState::Locked;
        }

        // Update DPMS power states based on idle duration
        if self.idle_time_secs >= self.config.dpms_off_secs {
            self.dpms_state = DpmsState::Off;
        } else if self.idle_time_secs >= self.config.dpms_suspend_secs {
            self.dpms_state = DpmsState::Suspend;
        } else if self.idle_time_secs >= self.config.dpms_standby_secs {
            self.dpms_state = DpmsState::Standby;
        } else {
            self.dpms_state = DpmsState::On;
        }
    }

    /// Register user input activity (mouse move, keypress)
    pub fn register_user_activity(&mut self) {
        self.idle_time_secs = 0;
        self.dpms_state = DpmsState::On;

        if self.lock_state == LockState::Unlocked {
            self.is_active = false;
        }
    }

    /// Manually lock the screen (e.g. shortcut Ctrl+Alt+L / Wayland ext-session-lock-v1)
    pub fn lock_screen(&mut self) {
        self.is_active = true;
        self.lock_state = LockState::Locked;
    }

    /// Authenticate passphrase (PAM / BSD auth parity) with zeroing scrub
    pub fn authenticate(&mut self, passphrase: &mut str) -> bool {
        self.lock_state = LockState::Authenticating;

        let matches = passphrase == self.config.hashed_passphrase.as_str();

        // BSD / OpenBSD-inspired zeroing memory scrub on input buffer
        unsafe {
            let bytes = passphrase.as_bytes_mut();
            for byte in bytes.iter_mut() {
                core::ptr::write_volatile(byte, 0);
            }
        }

        if matches {
            self.lock_state = LockState::Unlocked;
            self.is_active = false;
            self.idle_time_secs = 0;
            self.failed_auth_attempts = 0;
            true
        } else {
            self.failed_auth_attempts += 1;
            self.lock_state = LockState::Locked;
            false
        }
    }

    /// Set screen saver animation mode
    pub fn set_mode(&mut self, mode: ScreenSaverMode) {
        self.config.mode = mode;
    }

    /// Set explicit DPMS state
    pub fn set_dpms_state(&mut self, state: DpmsState) {
        self.dpms_state = state;
    }

    /// Render next frame state
    pub fn render_frame(&mut self) -> ScreenSaverFrame {
        self.frame_counter += 1;

        let status_text = match self.lock_state {
            LockState::Locked => {
                if self.failed_auth_attempts > 0 {
                    format!(
                        "Locked: User {} (Failed attempts: {})",
                        self.config.user_name, self.failed_auth_attempts
                    )
                } else {
                    format!("Locked: User {}", self.config.user_name)
                }
            }
            LockState::Authenticating => String::from("Verifying passphrase..."),
            LockState::Unlocked if self.is_active => {
                format!("Screensaver Active: Mode {:?}", self.config.mode)
            }
            LockState::Unlocked if self.is_inhibited() => String::from("Inhibited by application"),
            LockState::Unlocked => String::from("System Active"),
        };

        ScreenSaverFrame {
            active_mode: self.config.mode.clone(),
            dpms_state: self.dpms_state,
            lock_state: self.lock_state,
            status_text,
            is_inhibited: self.is_inhibited(),
            failed_attempts: self.failed_auth_attempts,
        }
    }
}

// ============================================================================
// OMARCHY BACKGROUNDS & OWE VIDEO WALLPAPER ENGINE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperMediaFormat {
    StillImage, // png, jpg, webp, bmp
    VideoMp4,   // mp4, m4v
    VideoMkv,   // mkv, avi, mov
    VideoWebm,  // webm
    AnimatedGif,// gif
}

impl WallpaperMediaFormat {
    pub fn from_filename(file_path: &str) -> Self {
        let lower = file_path.to_lowercase();
        if lower.ends_with(".mp4") || lower.ends_with(".m4v") {
            Self::VideoMp4
        } else if lower.ends_with(".mkv") || lower.ends_with(".avi") || lower.ends_with(".mov") {
            Self::VideoMkv
        } else if lower.ends_with(".webm") {
            Self::VideoWebm
        } else if lower.ends_with(".gif") {
            Self::AnimatedGif
        } else {
            Self::StillImage
        }
    }

    pub fn is_video(&self) -> bool {
        matches!(self, Self::VideoMp4 | Self::VideoMkv | Self::VideoWebm)
    }
}

pub struct OmarchyBackgroundManager {
    pub active_theme: String,
    pub backgrounds_base_dir: String,
    pub available_wallpapers: Vec<String>,
    pub selected_wallpaper: String,
}

impl OmarchyBackgroundManager {
    pub fn new(theme: &str) -> Self {
        let base_dir = format!("~/.config/omarchy/backgrounds/{}", theme);
        Self {
            active_theme: theme.to_string(),
            backgrounds_base_dir: base_dir,
            available_wallpapers: Vec::new(),
            selected_wallpaper: String::new(),
        }
    }

    pub fn resolve_theme_background_directory(&self) -> String {
        format!("~/.config/omarchy/backgrounds/{}", self.active_theme)
    }

    pub fn open_theme_background_directory_cmd(&self) -> String {
        format!("nemo ~/.config/omarchy/backgrounds/{}", self.active_theme)
    }

    pub fn select_wallpaper(&mut self, wallpaper_filename: &str) -> String {
        self.selected_wallpaper = wallpaper_filename.to_string();
        format!(
            "Omarchy Wallpaper Switcher (Super+Ctrl+Space): Active background set to {}/{}",
            self.resolve_theme_background_directory(), wallpaper_filename
        )
    }
}

pub struct OweVideoWallpaperEngine {
    pub current_video_path: String,
    pub is_playing: bool,
    pub is_audio_muted: bool,
    pub shared_multihead_decode: bool,
    pub cached_lockscreen_still_path: String,
}

impl OweVideoWallpaperEngine {
    pub fn new(video_path: &str) -> Self {
        let format = WallpaperMediaFormat::from_filename(video_path);
        let cached_still = if format.is_video() || format == WallpaperMediaFormat::AnimatedGif {
            format!("{}.lock_cached_still.png", video_path)
        } else {
            video_path.to_string()
        };

        Self {
            current_video_path: video_path.to_string(),
            is_playing: true,
            is_audio_muted: false,
            shared_multihead_decode: true, // Decodes once for all monitors
            cached_lockscreen_still_path: cached_still,
        }
    }

    pub fn pause_playback_for_power_saving(&mut self) -> String {
        self.is_playing = false;
        "OWE Engine: Video playback paused to preserve battery/power".to_string()
    }

    pub fn render_lockscreen_frame(&mut self) -> String {
        self.is_audio_muted = true;
        format!(
            "OWE Lockscreen: Displaying cached still frame '{}' with muted audio decode",
            self.cached_lockscreen_still_path
        )
    }
}

impl Default for OmarchyBackgroundManager {
    fn default() -> Self {
        Self::new("nord")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screensaver_idle_activation() {
        let mut config = ScreenSaverConfig::default();
        config.screensaver_timeout_secs = 10;
        config.lock_timeout_secs = 20;
        config.dpms_off_secs = 30;

        let mut engine = ScreenSaverEngine::new(config);
        assert!(!engine.is_active);

        // Update idle to 15s (screensaver active, unlocked)
        engine.update_idle_time(15);
        assert!(engine.is_active);
        assert_eq!(engine.lock_state, LockState::Unlocked);

        // Update idle to 25s (locked)
        engine.update_idle_time(25);
        assert!(engine.is_active);
        assert_eq!(engine.lock_state, LockState::Locked);

        // Update idle to 35s (DPMS Off)
        engine.update_idle_time(35);
        assert_eq!(engine.dpms_state, DpmsState::Off);
    }

    #[test]
    fn test_authentication_and_memory_zeroing() {
        let config = ScreenSaverConfig::default();
        let mut engine = ScreenSaverEngine::new(config);

        engine.lock_screen();
        assert_eq!(engine.lock_state, LockState::Locked);

        // Wrong passphrase
        let mut wrong_pass = String::from("wrongpass");
        assert!(!engine.authenticate(&mut wrong_pass));
        assert_eq!(engine.lock_state, LockState::Locked);
        assert_eq!(engine.failed_auth_attempts, 1);
        assert_eq!(wrong_pass, "\0\0\0\0\0\0\0\0\0"); // Memory scrubbed

        // Correct passphrase
        let mut correct_pass = String::from("passphrase123");
        assert!(engine.authenticate(&mut correct_pass));
        assert_eq!(engine.lock_state, LockState::Unlocked);
        assert!(!engine.is_active);
        assert_eq!(engine.failed_auth_attempts, 0);
        assert_eq!(correct_pass, "\0\0\0\0\0\0\0\0\0\0\0\0\0"); // Memory scrubbed
    }

    #[test]
    fn test_dbus_inhibit_interface() {
        let mut config = ScreenSaverConfig::default();
        config.screensaver_timeout_secs = 10;
        let mut engine = ScreenSaverEngine::new(config);

        assert!(!engine.is_inhibited());

        let cookie = engine.inhibit("mpv", "Playing 4K Movie");
        assert!(engine.is_inhibited());

        // Update idle to 20s while inhibited -> should remain inactive and DPMS On
        engine.update_idle_time(20);
        assert!(!engine.is_active);
        assert_eq!(engine.dpms_state, DpmsState::On);

        // Uninhibit
        assert!(engine.uninhibit(cookie));
        assert!(!engine.is_inhibited());

        // Update idle again -> now screensaver activates
        engine.update_idle_time(20);
        assert!(engine.is_active);
    }

    #[test]
    fn test_omarchy_background_manager() {
        let mut mgr = OmarchyBackgroundManager::new("nord");
        assert_eq!(mgr.resolve_theme_background_directory(), "~/.config/omarchy/backgrounds/nord");

        let open_cmd = mgr.open_theme_background_directory_cmd();
        assert!(open_cmd.contains("nemo"));
        assert!(open_cmd.contains("nord"));

        let select_msg = mgr.select_wallpaper("cyberpunk_city.mp4");
        assert!(select_msg.contains("Super+Ctrl+Space"));
        assert_eq!(mgr.selected_wallpaper, "cyberpunk_city.mp4");
    }

    #[test]
    fn test_owe_video_wallpaper_engine() {
        let mut owe = OweVideoWallpaperEngine::new("matrix_loop.mp4");
        assert!(owe.shared_multihead_decode);
        assert!(owe.cached_lockscreen_still_path.contains("lock_cached_still.png"));

        let pause_msg = owe.pause_playback_for_power_saving();
        assert!(pause_msg.contains("paused"));
        assert!(!owe.is_playing);

        let lock_msg = owe.render_lockscreen_frame();
        assert!(lock_msg.contains("muted audio"));
        assert!(owe.is_audio_muted);
    }
}
