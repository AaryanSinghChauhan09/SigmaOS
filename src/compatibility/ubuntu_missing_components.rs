// SigmaOS Ubuntu Linux Ecosystem Parity & Missing Infrastructure Subsystem
// Zero-dependency, `#![no_std]` compliant implementations of core Ubuntu infrastructure components:
// 1. Ubuntu Netplan Declarative Network Configuration Engine
// 2. Ubuntu AppArmor Security Profile Parsing & Policy Enforcement Engine
// 3. Ubuntu Landscape Fleet Telemetry & Management Client Engine
// 4. Ubuntu Subiquity Autoinstall & Storage Layout Provisioning Engine
// 5. Ubuntu Unattended-Upgrades Automatic Security Patch Scheduler Engine

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec;
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// =========================================================================
// 1. UBUNTU NETPLAN DECLARATIVE NETWORK CONFIGURATION ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct NetplanNetdef {
    pub name: String,
    pub renderer: String, // "networkd" or "NetworkManager"
    pub dhcp4: bool,
    pub addresses: Vec<String>,
    pub gateway4: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UbuntuNetplanEngine {
    pub netdefs: BTreeMap<String, NetplanNetdef>,
}

impl UbuntuNetplanEngine {
    pub fn new() -> Self {
        Self {
            netdefs: BTreeMap::new(),
        }
    }

    pub fn add_ethernet(
        &mut self,
        ifname: &str,
        renderer: &str,
        dhcp4: bool,
        addresses: &[&str],
        gateway4: Option<&str>,
    ) {
        let netdef = NetplanNetdef {
            name: ifname.to_string(),
            renderer: renderer.to_string(),
            dhcp4,
            addresses: addresses.iter().map(|s| s.to_string()).collect(),
            gateway4: gateway4.map(|s| s.to_string()),
        };
        self.netdefs.insert(ifname.to_string(), netdef);
    }

    pub fn render_networkd_config(&self, ifname: &str) -> Result<String, &'static str> {
        let netdef = self
            .netdefs
            .get(ifname)
            .ok_or("Netplan: Interface not found")?;
        let mut conf = format!("[Match]\nName={}\n\n[Network]\n", netdef.name);
        if netdef.dhcp4 {
            conf.push_str("DHCP=ipv4\n");
        }
        for addr in &netdef.addresses {
            conf.push_str(&format!("Address={}\n", addr));
        }
        if let Some(gw) = &netdef.gateway4 {
            conf.push_str(&format!("Gateway={}\n", gw));
        }
        Ok(conf)
    }
}

impl Default for UbuntuNetplanEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ubuntu_netplan_engine() {
        let mut netplan = UbuntuNetplanEngine::new();
        netplan.add_ethernet(
            "eth0",
            "networkd",
            true,
            &["192.168.1.50/24"],
            Some("192.168.1.1"),
        );
        let rendered = netplan.render_networkd_config("eth0").expect("Render ok");
        assert!(rendered.contains("Name=eth0"));
        assert!(rendered.contains("DHCP=ipv4"));
        assert!(rendered.contains("Address=192.168.1.50/24"));
        assert!(rendered.contains("Gateway=192.168.1.1"));
    }

    #[test]
    fn test_ubuntu_apparmor_engine() {
        let mut apparmor = UbuntuAppArmorEngine::new();
        apparmor.load_profile(
            "usr.bin.firefox",
            AppArmorMode::Enforce,
            &["/home/user/", "/tmp/"],
            &["net_bind_service"],
        );
        assert!(apparmor.query_file_access("usr.bin.firefox", "/home/user/downloads"));
        assert!(!apparmor.query_file_access("usr.bin.firefox", "/etc/shadow"));

        apparmor.load_profile(
            "usr.bin.tcpdump",
            AppArmorMode::Complain,
            &["/var/log/"],
            &[],
        );
        assert!(apparmor.query_file_access("usr.bin.tcpdump", "/etc/shadow"));
    }

    #[test]
    fn test_ubuntu_landscape_engine() {
        let mut landscape =
            UbuntuLandscapeEngine::new("acc-123", "https://landscape.canonical.com");
        assert!(landscape.register_client());
        let report = landscape
            .generate_report("srv01", "6.8.0-generic", 3600, 120, 3)
            .expect("Report ok");
        assert_eq!(report.hostname, "srv01");
        assert_eq!(report.security_updates_pending, 3);
    }

    #[test]
    fn test_ubuntu_subiquity_engine() {
        let mut subiquity = UbuntuSubiquityEngine::new();
        assert!(!subiquity.validate_and_enable_autoinstall());
        subiquity.configure_storage_layout("/dev/sda", "ext4", "/", 100);
        assert!(subiquity.validate_and_enable_autoinstall());
        assert_eq!(subiquity.layouts[0].mountpoint, "/");
    }

    #[test]
    fn test_ubuntu_unattended_upgrades_engine() {
        let mut unattended = UbuntuUnattendedUpgradesEngine::new();
        unattended.add_origin_rule("Ubuntu:noble-security", true);
        assert!(unattended.should_upgrade_package("Ubuntu:noble-security/main"));
        assert!(!unattended.should_upgrade_package("Ubuntu:noble-backports/main"));
    }
}

// =========================================================================
// 2. UBUNTU APPARMOR SECURITY PROFILE ENFORCEMENT ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppArmorMode {
    Enforce,
    Complain,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct AppArmorProfile {
    pub name: String,
    pub mode: AppArmorMode,
    pub allowed_paths: Vec<String>,
    pub allowed_capabilities: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct UbuntuAppArmorEngine {
    pub profiles: BTreeMap<String, AppArmorProfile>,
}

impl UbuntuAppArmorEngine {
    pub fn new() -> Self {
        Self {
            profiles: BTreeMap::new(),
        }
    }

    pub fn load_profile(&mut self, name: &str, mode: AppArmorMode, paths: &[&str], caps: &[&str]) {
        let profile = AppArmorProfile {
            name: name.to_string(),
            mode,
            allowed_paths: paths.iter().map(|s| s.to_string()).collect(),
            allowed_capabilities: caps.iter().map(|s| s.to_string()).collect(),
        };
        self.profiles.insert(name.to_string(), profile);
    }

    pub fn query_file_access(&self, profile_name: &str, path: &str) -> bool {
        if let Some(prof) = self.profiles.get(profile_name) {
            if prof.mode == AppArmorMode::Complain || prof.mode == AppArmorMode::Disabled {
                return true;
            }
            prof.allowed_paths.iter().any(|p| path.starts_with(p))
        } else {
            true // Unconfined
        }
    }
}

impl Default for UbuntuAppArmorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. UBUNTU LANDSCAPE FLEET TELEMETRY CLIENT ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct LandscapeSystemReport {
    pub hostname: String,
    pub kernel_version: String,
    pub uptime_secs: u64,
    pub active_processes: usize,
    pub security_updates_pending: usize,
}

#[derive(Debug, Clone)]
pub struct UbuntuLandscapeEngine {
    pub account_id: String,
    pub server_url: String,
    pub is_registered: bool,
}

impl UbuntuLandscapeEngine {
    pub fn new(account_id: &str, server_url: &str) -> Self {
        Self {
            account_id: account_id.to_string(),
            server_url: server_url.to_string(),
            is_registered: false,
        }
    }

    pub fn register_client(&mut self) -> bool {
        self.is_registered = !self.account_id.is_empty();
        self.is_registered
    }

    pub fn generate_report(
        &self,
        hostname: &str,
        kernel: &str,
        uptime: u64,
        procs: usize,
        updates: usize,
    ) -> Result<LandscapeSystemReport, &'static str> {
        if !self.is_registered {
            return Err("Landscape: Client not registered with management server");
        }
        Ok(LandscapeSystemReport {
            hostname: hostname.to_string(),
            kernel_version: kernel.to_string(),
            uptime_secs: uptime,
            active_processes: procs,
            security_updates_pending: updates,
        })
    }
}

// =========================================================================
// 4. UBUNTU SUBIQUITY AUTOINSTALL & STORAGE PROVISIONING ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct SubiquityStorageLayout {
    pub disk_path: String,
    pub fstype: String, // ext4, xfs, btrfs, zfs
    pub mountpoint: String,
    pub size_gb: u64,
}

#[derive(Debug, Clone)]
pub struct UbuntuSubiquityEngine {
    pub autoinstall_enabled: bool,
    pub layouts: Vec<SubiquityStorageLayout>,
}

impl UbuntuSubiquityEngine {
    pub fn new() -> Self {
        Self {
            autoinstall_enabled: false,
            layouts: Vec::new(),
        }
    }

    pub fn configure_storage_layout(
        &mut self,
        disk: &str,
        fstype: &str,
        mountpoint: &str,
        size_gb: u64,
    ) {
        self.layouts.push(SubiquityStorageLayout {
            disk_path: disk.to_string(),
            fstype: fstype.to_string(),
            mountpoint: mountpoint.to_string(),
            size_gb,
        });
    }

    pub fn validate_and_enable_autoinstall(&mut self) -> bool {
        self.autoinstall_enabled = !self.layouts.is_empty();
        self.autoinstall_enabled
    }
}

impl Default for UbuntuSubiquityEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. UBUNTU UNATTENDED-UPGRADES SECURITY PATCH SCHEDULER ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct UnattendedUpgradeRule {
    pub origin_pattern: String, // e.g. "${distro_id}:${distro_codename}-security"
    pub allowed: bool,
}

#[derive(Debug, Clone)]
pub struct UbuntuUnattendedUpgradesEngine {
    pub allowed_origins: Vec<UnattendedUpgradeRule>,
    pub auto_reboot: bool,
    pub reboot_time: String,
}

impl UbuntuUnattendedUpgradesEngine {
    pub fn new() -> Self {
        Self {
            allowed_origins: Vec::new(),
            auto_reboot: false,
            reboot_time: "02:00".to_string(),
        }
    }

    pub fn add_origin_rule(&mut self, origin_pattern: &str, allowed: bool) {
        self.allowed_origins.push(UnattendedUpgradeRule {
            origin_pattern: origin_pattern.to_string(),
            allowed,
        });
    }

    pub fn should_upgrade_package(&self, origin: &str) -> bool {
        self.allowed_origins
            .iter()
            .any(|rule| rule.allowed && origin.contains(&rule.origin_pattern))
    }
}

impl Default for UbuntuUnattendedUpgradesEngine {
    fn default() -> Self {
        Self::new()
    }
}
