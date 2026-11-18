// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Session Idle & Screen Locker Engine
//!
//! Inspired by Omarchy's Hypridle + Hyprlock integration and
//! Arch Linux's swaylock / wayland ext-session-lock-v1 protocol.
//! Automatically monitors user inactivity timeouts, dims screens, triggers lockscreen
//! authentication dialogs, and manages system suspend / sleep inhibitors.

use std::fmt;

/// Power state or timeout trigger phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdleStage {
    DimScreen,
    LockSession,
    TurnOffDisplays,
    SuspendSystem,
}

impl fmt::Display for IdleStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimScreen => write!(f, "dim-screen"),
            Self::LockSession => write!(f, "lock-session"),
            Self::TurnOffDisplays => write!(f, "turn-off-displays"),
            Self::SuspendSystem => write!(f, "suspend-system"),
        }
    }
}

/// A configured idle timeout listener
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdleListener {
    pub timeout_seconds: u32,
    pub on_timeout: String,
    pub on_resume: Option<String>,
    pub stage: IdleStage,
}

impl IdleListener {
    pub fn new(timeout: u32, on_timeout: &str, on_resume: Option<&str>, stage: IdleStage) -> Self {
        Self {
            timeout_seconds: timeout,
            on_timeout: on_timeout.to_string(),
            on_resume: on_resume.map(|s| s.to_string()),
            stage,
        }
    }

    /// Render Hypridle listener directive
    pub fn to_hypridle_directive(&self) -> String {
        let mut out = format!("listener {{\n    timeout = {}\n    on-timeout = {}\n", self.timeout_seconds, self.on_timeout);
        if let Some(res) = &self.on_resume {
            out.push_str(&format!("    on-resume = {}\n", res));
        }
        out.push_str("}\n");
        out
    }
}

/// Complete session idle configuration generator
#[derive(Debug, Default)]
pub struct SessionIdleConfig {
    pub lock_cmd: String,
    pub before_sleep_cmd: String,
    pub listeners: Vec<IdleListener>,
}

impl SessionIdleConfig {
    pub fn omarchy_defaults() -> Self {
        let mut listeners = Vec::new();
        // 2.5 mins: dim backlight
        listeners.push(IdleListener::new(150, "brightnessctl -s set 10", Some("brightnessctl -r"), IdleStage::DimScreen));
        // 5 mins: lock session
        listeners.push(IdleListener::new(300, "hyprlock", None, IdleStage::LockSession));
        // 5.5 mins: DPMS off
        listeners.push(IdleListener::new(330, "hyprctl dispatch dpms off", Some("hyprctl dispatch dpms on"), IdleStage::TurnOffDisplays));
        // 10 mins: suspend
        listeners.push(IdleListener::new(600, "systemctl suspend", None, IdleStage::SuspendSystem));

        Self {
            lock_cmd: "pidof hyprlock || hyprlock".to_string(),
            before_sleep_cmd: "loginctl lock-session".to_string(),
            listeners,
        }
    }

    pub fn to_conf(&self) -> String {
        let mut out = String::from("# SigmaOS Omarchy Hypridle Configuration\n");
        out.push_str(&format!("general {{\n    lock_cmd = {}\n    before_sleep_cmd = {}\n}}\n\n", self.lock_cmd, self.before_sleep_cmd));
        for l in &self.listeners {
            out.push_str(&l.to_hypridle_directive());
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idle_stage_display_aaa() {
        assert_eq!(IdleStage::DimScreen.to_string(), "dim-screen");
        assert_eq!(IdleStage::LockSession.to_string(), "lock-session");
        assert_eq!(IdleStage::SuspendSystem.to_string(), "suspend-system");
    }

    #[test]
    fn test_hypridle_listener_directive_aaa() {
        let listener = IdleListener::new(300, "hyprlock", None, IdleStage::LockSession);
        let directive = listener.to_hypridle_directive();
        assert!(directive.contains("timeout = 300"));
        assert!(directive.contains("on-timeout = hyprlock"));
        assert!(!directive.contains("on-resume"));
    }

    #[test]
    fn test_omarchy_defaults_generation_aaa() {
        let conf = SessionIdleConfig::omarchy_defaults();
        let rendered = conf.to_conf();
        assert!(rendered.contains("lock_cmd = pidof hyprlock || hyprlock"));
        assert!(rendered.contains("before_sleep_cmd = loginctl lock-session"));
        assert!(rendered.contains("timeout = 150"));
        assert!(rendered.contains("timeout = 300"));
        assert!(rendered.contains("timeout = 600"));
    }
}
