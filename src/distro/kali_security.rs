//! Kali Linux Security Subsystem and Penetration Testing Framework for SigmaOS
//!
//! Inspired by Kali Linux:
//! - `KaliUndercoverEngine`: Stealth Windows 10/11 desktop theme switcher
//! - `KaliNetHunterEngine`: Mobile chroot, HID attack simulation, and Kismet wireless assessment
//! - `KaliWinKexEngine`: WSL2 / Remote GUI desktop session bridge
//! - `KaliMetapackageEngine`: Security tool categories manager (`kali-tools-top10`, `kali-tools-web`, `kali-tools-forensics`, `kali-tools-wireless`)
//! - `KaliLiveEncryptedPersistenceEngine`: Encrypted live USB persistence & LUKS emergency nuke key

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Undercover Theme Mode (Kali Undercover)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UndercoverThemeMode {
    KaliDefault,
    Windows10Stealth,
    Windows11Stealth,
}

/// Kali Undercover Desktop Theme Switcher
pub struct KaliUndercoverEngine {
    pub current_mode: UndercoverThemeMode,
    pub stealth_active: bool,
}

impl KaliUndercoverEngine {
    pub fn new() -> Self {
        Self {
            current_mode: UndercoverThemeMode::KaliDefault,
            stealth_active: false,
        }
    }

    pub fn toggle_undercover(&mut self, target_mode: UndercoverThemeMode) -> String {
        self.current_mode = target_mode;
        self.stealth_active = target_mode != UndercoverThemeMode::KaliDefault;
        format!(
            "Kali Undercover Mode set to {:?} (Stealth Active: {})",
            self.current_mode, self.stealth_active
        )
    }
}

impl Default for KaliUndercoverEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// NetHunter Mobile & HID Attack Orchestrator
pub struct KaliNetHunterEngine {
    pub hid_keyboard_injection_enabled: bool,
    pub kismet_wireless_monitor_active: bool,
    pub chroot_overlay_path: String,
}

impl KaliNetHunterEngine {
    pub fn new() -> Self {
        Self {
            hid_keyboard_injection_enabled: false,
            kismet_wireless_monitor_active: false,
            chroot_overlay_path: "/data/local/nhsystem/kali-arm64".to_string(),
        }
    }

    pub fn enable_hid_keyboard_injection(&mut self) -> Result<String, &'static str> {
        self.hid_keyboard_injection_enabled = true;
        Ok("NetHunter BadUSB/HID Keyboard Injection Emulation Enabled".to_string())
    }
}

impl Default for KaliNetHunterEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// WinKeX WSL2 / Remote Desktop GUI Session Bridge
pub struct KaliWinKexEngine {
    pub session_port: u16,
    pub seamless_mode_enabled: bool,
}

impl KaliWinKexEngine {
    pub fn new() -> Self {
        Self {
            session_port: 5901,
            seamless_mode_enabled: true,
        }
    }

    pub fn launch_kex_session(&self) -> String {
        format!(
            "WinKeX GUI Session Active on Port {} (Seamless Window Mode: {})",
            self.session_port, self.seamless_mode_enabled
        )
    }
}

impl Default for KaliWinKexEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Security Tools Metapackage Manager
pub struct KaliMetapackageEngine {
    pub metapackages: BTreeMap<String, Vec<String>>,
}

impl KaliMetapackageEngine {
    pub fn new() -> Self {
        let mut mgr = Self {
            metapackages: BTreeMap::new(),
        };

        mgr.register_metapackage(
            "kali-tools-top10",
            vec![
                "nmap".to_string(),
                "burpsuite".to_string(),
                "wireshark".to_string(),
                "john".to_string(),
                "aircrack-ng".to_string(),
                "sqlmap".to_string(),
                "metasploit-framework".to_string(),
                "hydra".to_string(),
                "autopsy".to_string(),
                "hashcat".to_string(),
            ],
        );

        mgr.register_metapackage(
            "kali-tools-web",
            vec![
                "burpsuite".to_string(),
                "sqlmap".to_string(),
                "nikto".to_string(),
                "gobuster".to_string(),
                "wpscan".to_string(),
            ],
        );

        mgr
    }

    pub fn register_metapackage(&mut self, name: &str, tools: Vec<String>) {
        self.metapackages.insert(name.to_string(), tools);
    }

    pub fn resolve_metapackage_tools(&self, name: &str) -> Option<&Vec<String>> {
        self.metapackages.get(name)
    }
}

impl Default for KaliMetapackageEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Live USB Encrypted Persistence & Emergency LUKS Nuke Engine
pub struct KaliLiveEncryptedPersistenceEngine {
    pub persistence_mounted: bool,
    pub luks_nuke_armed: bool,
}

impl KaliLiveEncryptedPersistenceEngine {
    pub fn new() -> Self {
        Self {
            persistence_mounted: true,
            luks_nuke_armed: true,
        }
    }

    pub fn execute_emergency_nuke(
        &mut self,
        nuke_passphrase_entered: bool,
    ) -> Result<String, &'static str> {
        if !self.luks_nuke_armed {
            return Err("LUKS nuke key is not armed");
        }
        if nuke_passphrase_entered {
            self.persistence_mounted = false;
            Ok("EMERGENCY NUKE EXECUTED: LUKS Header Destroyed permanently".to_string())
        } else {
            Err("Nuke passphrase validation failed")
        }
    }
}

impl Default for KaliLiveEncryptedPersistenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kali_undercover_toggle() {
        let mut undercover = KaliUndercoverEngine::new();
        assert!(!undercover.stealth_active);

        let res = undercover.toggle_undercover(UndercoverThemeMode::Windows10Stealth);
        assert!(res.contains("Stealth Active: true"));
        assert_eq!(
            undercover.current_mode,
            UndercoverThemeMode::Windows10Stealth
        );
    }

    #[test]
    fn test_kali_metapackages() {
        let mgr = KaliMetapackageEngine::new();
        let top10 = mgr.resolve_metapackage_tools("kali-tools-top10").unwrap();
        assert_eq!(top10.len(), 10);
        assert!(top10.contains(&"nmap".to_string()));
    }

    #[test]
    fn test_kali_luks_nuke() {
        let mut nuke_engine = KaliLiveEncryptedPersistenceEngine::new();
        let res = nuke_engine.execute_emergency_nuke(true).unwrap();
        assert!(res.contains("EMERGENCY NUKE EXECUTED"));
        assert!(!nuke_engine.persistence_mounted);
    }
}
