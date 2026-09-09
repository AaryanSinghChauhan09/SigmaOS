//! antiX Linux and Zorin OS Subsystem Innovations for SigmaOS
//!
//! Inspired by antiX Linux and Zorin OS:
//! - `AntiXSysVInitEngine`: Systemd-free lightweight init, runlevel, and Runit/SysV service supervisor
//! - `ZorinAppearanceSwitcher`: Adaptive desktop layout engine (Windows 11, Windows Classic, macOS, GNOME, Ubuntu)

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// antiX Service State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntiXServiceState {
    Stopped,
    Running,
    Disabled,
}

/// antiX Systemd-Free Lightweight Init & Service Manager
pub struct AntiXSysVInitEngine {
    pub services: BTreeMap<String, AntiXServiceState>,
}

impl AntiXSysVInitEngine {
    pub fn new() -> Self {
        let mut mgr = Self {
            services: BTreeMap::new(),
        };
        mgr.register_service("syslogd", AntiXServiceState::Running);
        mgr.register_service("slim", AntiXServiceState::Running);
        mgr.register_service("dbus", AntiXServiceState::Running);
        mgr
    }

    pub fn register_service(&mut self, name: &str, state: AntiXServiceState) {
        self.services.insert(name.to_string(), state);
    }

    pub fn set_service_state(
        &mut self,
        name: &str,
        state: AntiXServiceState,
    ) -> Result<String, &'static str> {
        if let Some(svc) = self.services.get_mut(name) {
            *svc = state;
            Ok(format!("antiX init: Service '{}' set to {:?}", name, state))
        } else {
            Err("Service not found in antiX init database")
        }
    }
}

impl Default for AntiXSysVInitEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Zorin OS Desktop Layout Target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZorinDesktopLayout {
    Windows11,
    WindowsClassic,
    MacOs,
    GnomeStandard,
    UbuntuUnity,
}

/// Zorin Appearance Desktop Switcher Engine
pub struct ZorinAppearanceSwitcher {
    pub active_layout: ZorinDesktopLayout,
    pub accent_color: String,
    pub dark_theme: bool,
}

impl ZorinAppearanceSwitcher {
    pub fn new() -> Self {
        Self {
            active_layout: ZorinDesktopLayout::Windows11,
            accent_color: "Blue".to_string(),
            dark_theme: true,
        }
    }

    pub fn set_layout(&mut self, layout: ZorinDesktopLayout) -> String {
        self.active_layout = layout;
        format!(
            "Zorin Appearance: Switched desktop layout to {:?} (Accent: {}, Dark: {})",
            self.active_layout, self.accent_color, self.dark_theme
        )
    }
}

impl Default for ZorinAppearanceSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antix_init_engine() {
        let mut init = AntiXSysVInitEngine::new();
        assert_eq!(
            init.services.get("syslogd"),
            Some(&AntiXServiceState::Running)
        );

        let res = init
            .set_service_state("syslogd", AntiXServiceState::Stopped)
            .unwrap();
        assert!(res.contains("Service 'syslogd' set to Stopped"));
        assert_eq!(
            init.services.get("syslogd"),
            Some(&AntiXServiceState::Stopped)
        );
    }

    #[test]
    fn test_zorin_appearance_switcher() {
        let mut zorin = ZorinAppearanceSwitcher::new();
        assert_eq!(zorin.active_layout, ZorinDesktopLayout::Windows11);

        let res = zorin.set_layout(ZorinDesktopLayout::MacOs);
        assert!(res.contains("Switched desktop layout to MacOs"));
        assert_eq!(zorin.active_layout, ZorinDesktopLayout::MacOs);
    }
}
