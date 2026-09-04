#![allow(dead_code)]

use std::format;
use std::string::String;

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

/// Screen locking state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockState {
    Unlocked,
    Locked,
    Authenticating,
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
}

/// Linux & BSD-inspired ScreenSaver and Display Power Management Engine
pub struct ScreenSaverEngine {
    pub config: ScreenSaverConfig,
    pub idle_time_secs: u64,
    pub is_active: bool,
    pub lock_state: LockState,
    pub dpms_state: DpmsState,
    pub frame_counter: u64,
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
        }
    }

    /// Called on system timer tick to update user idle time
    pub fn update_idle_time(&mut self, idle_seconds: u64) {
        self.idle_time_secs = idle_seconds;

        // Check if screensaver should activate
        if self.idle_time_secs >= self.config.screensaver_timeout_secs {
            self.is_active = true;
        } else {
            if self.lock_state == LockState::Unlocked {
                self.is_active = false;
            }
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

    /// Manually lock the screen (e.g. shortcut Ctrl+Alt+L)
    pub fn lock_screen(&mut self) {
        self.is_active = true;
        self.lock_state = LockState::Locked;
    }

    /// Authenticate passphrase (PAM-style verification)
    pub fn authenticate(&mut self, passphrase: &str) -> bool {
        self.lock_state = LockState::Authenticating;
        if passphrase == self.config.hashed_passphrase {
            self.lock_state = LockState::Unlocked;
            self.is_active = false;
            self.idle_time_secs = 0;
            true
        } else {
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
            LockState::Locked => format!("Locked: User {}", self.config.user_name),
            LockState::Authenticating => String::from("Verifying passphrase..."),
            LockState::Unlocked if self.is_active => {
                format!("Screensaver Active: Mode {:?}", self.config.mode)
            }
            LockState::Unlocked => String::from("System Active"),
        };

        ScreenSaverFrame {
            active_mode: self.config.mode.clone(),
            dpms_state: self.dpms_state,
            lock_state: self.lock_state,
            status_text,
        }
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
    fn test_authentication() {
        let config = ScreenSaverConfig::default();
        let mut engine = ScreenSaverEngine::new(config);

        engine.lock_screen();
        assert_eq!(engine.lock_state, LockState::Locked);

        // Wrong passphrase
        assert!(!engine.authenticate("wrongpass"));
        assert_eq!(engine.lock_state, LockState::Locked);

        // Correct passphrase
        assert!(engine.authenticate("passphrase123"));
        assert_eq!(engine.lock_state, LockState::Unlocked);
        assert!(!engine.is_active);
    }
}
