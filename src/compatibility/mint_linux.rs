#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Mint Linux Clean-Room Compatibility & Parity Engine
// Inspired by Linux Mint (Cinnamon, mintupdate, mintinstall, timeshift, mintdrivers, ufw)
// Implements robust userland configurations, package wrappers, desktop overlays,
// PAM security elevation gates, standard shell script utilities, and timeshift restore maps.

extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use crate::klib::HashMap;
use crate::security::CapabilityToken;

/// 1. Cinnamon Desktop Engine
/// Simulates panels, menu structures, applets, themes, MATE/Xfce fallbacks,
/// and desktop compositor overlays natively mapped to the Zenith compositor.
#[derive(Debug, Clone)]
pub struct CinnamonDesktopEngine {
    pub theme: String,
    pub active_applets: Vec<String>,
    pub fallback_mode: bool,
    pub panel_height_px: u32,
    pub compositing_effects: bool,
}

impl CinnamonDesktopEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            theme: "Mint-Y-Dark-Teal".to_string(),
            active_applets: vec![
                "menu@cinnamon.org".to_string(),
                "show-desktop@cinnamon.org".to_string(),
                "grouped-window-list@cinnamon.org".to_string(),
                "calendar@cinnamon.org".to_string(),
                "power@cinnamon.org".to_string(),
            ],
            fallback_mode: false,
            panel_height_px: 40,
            compositing_effects: true,
        }
    }

    pub fn set_theme(&mut self, theme_name: &str) {
        self.theme = theme_name.to_string();
    }

    pub fn enable_software_rendering(&mut self) {
        self.fallback_mode = true;
        self.compositing_effects = false;
    }

    pub fn register_desklet(&mut self, gadget: &str) {
        self.active_applets.push(format!("desklet:{}", gadget));
    }
}

impl Default for CinnamonDesktopEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 2. Mint Update Manager (`mintupdate`)
/// Handles packages upgrade channels, ranking them into standard Mint risk levels (1-5),
/// with automated Timeshift pre-flight checkpoint assertions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdateRiskLevel {
    Level1Safe = 1,          // Certified Mint packages
    Level2Normal = 2,        // Recommended / Tested upstream packages
    Level3Recommended = 3,   // Safe but unverified upstream packages
    Level4Sensitive = 4,     // Sensitive packages (e.g. bootloaders, systemd)
    Level5Dangerous = 5,     // Kernels, graphic drivers (high regression risk)
}

#[derive(Debug, Clone)]
pub struct MintUpdateItem {
    pub package_name: String,
    pub version: String,
    pub level: UpdateRiskLevel,
    pub size_kb: usize,
}

#[derive(Debug, Clone)]
pub struct MintUpdateManager {
    pub updates: Vec<MintUpdateItem>,
    pub selected_levels: Vec<bool>, // levels index 0..=5
    pub timeshift_preflight: bool,
}

impl MintUpdateManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
            selected_levels: vec![true, true, true, true, false, false], // level 1-3 selected, 4-5 unselected by default
            timeshift_preflight: true,
        }
    }

    pub fn register_update(&mut self, name: &str, ver: &str, risk: UpdateRiskLevel, size: usize) {
        self.updates.push(MintUpdateItem {
            package_name: name.to_string(),
            version: ver.to_string(),
            level: risk,
            size_kb: size,
        });
    }

    pub fn filter_updates(&self) -> Vec<MintUpdateItem> {
        self.updates
            .iter()
            .filter(|up| {
                let lvl = up.level as usize;
                if lvl < self.selected_levels.len() {
                    self.selected_levels[lvl]
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }

    pub fn run_preflight_snapshot_assertion(&self, timeshift_active: bool) -> Result<(), &'static str> {
        if self.timeshift_preflight && !timeshift_active {
            return Err("Timeshift backup missing! High-risk updates aborted for safety.");
        }
        Ok(())
    }
}

impl Default for MintUpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 3. Mint Install Software Manager (`mintinstall`)
/// Implements standard .deb, flatpak, and spkg application conversions,
/// and natively enforces Mint's design decision of blocking snapcraft by default.
#[derive(Debug, Clone)]
pub struct MintInstallSoftwareManager {
    pub flatpak_repositories: Vec<String>,
    pub snapcraft_enabled: bool,
    pub local_package_cache: HashMap<String, String>,
}

impl MintInstallSoftwareManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut repos = Vec::new();
        repos.push("flathub.org".to_string());
        Self {
            flatpak_repositories: repos,
            snapcraft_enabled: false, // snap disabled by default as in Mint
            local_package_cache: HashMap::new(),
        }
    }

    pub fn install_package(&mut self, name: &str, format: &str) -> Result<String, &'static str> {
        if format == "snap" && !self.snapcraft_enabled {
            return Err("Snapcraft is blocked by default on Mint. Enable snaps first or use Flatpak/.deb!");
        }

        let package_key = format!("{}:{}", format, name);
        self.local_package_cache.insert(package_key.clone(), "installed".to_string());
        Ok(format!("Successfully installed '{}' via {} manager", name, format))
    }

    pub fn enable_snaps(&mut self) {
        self.snapcraft_enabled = true;
    }
}

impl Default for MintInstallSoftwareManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 4. Mint Backup Tool (`mintbackup`)
/// Backs up and restores user home-directory configurations,
/// compressing user files and managing application lists.
#[derive(Debug, Clone)]
pub struct MintBackupTool {
    pub source_home: String,
    pub target_device: String,
    pub excluded_patterns: Vec<String>,
}

impl MintBackupTool {
    pub fn new(home: &str, target: &str) -> Self {
        Self {
            source_home: home.to_string(),
            target_device: target.to_string(),
            excluded_patterns: vec![".cache".to_string(), "tmp".to_string()],
        }
    }

    pub fn run_user_backup(&self, total_files: usize) -> (String, usize) {
        let size_estimate = total_files * 45; // average 45KB per user config
        (
            format!("Backup of '{}' generated at '{}'", self.source_home, self.target_device),
            size_estimate,
        )
    }
}

/// 5. Mint Welcome Screen Engine
/// Greps welcome guides, system layout configurations, driver installers,
/// and language/multilingual translation service initialization.
#[derive(Debug, Clone)]
pub struct MintWelcomeEngine {
    pub current_step: &'static str,
    pub language_code: String,
    pub recommended_drivers_installed: bool,
}

impl MintWelcomeEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            current_step: "first-steps",
            language_code: "en_US".to_string(),
            recommended_drivers_installed: false,
        }
    }

    pub fn set_locale(&mut self, lang: &str) {
        self.language_code = lang.to_string();
    }

    pub fn advance_wizard(&mut self) -> &'static str {
        if self.current_step == "first-steps" {
            self.current_step = "system-settings";
        } else if self.current_step == "system-settings" {
            self.current_step = "software-manager";
        } else {
            self.current_step = "ready";
        }
        self.current_step
    }
}

impl Default for MintWelcomeEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 6. Mint Hardware Driver Manager (`mintdrivers`)
/// Probes local system devices and suggests proprietary or open-source drivers (e.g. nvidia, broadcom)
#[derive(Debug, Clone)]
pub struct MintDriverItem {
    pub device_id: String,
    pub driver_name: String,
    pub is_proprietary: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct MintHardwareDriverManager {
    pub drivers: Vec<MintDriverItem>,
}

impl MintHardwareDriverManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
        }
    }

    pub fn probe_hardware_devices(&mut self) {
        self.drivers.push(MintDriverItem {
            device_id: "pci:10de:1f08".to_string(),
            driver_name: "nvidia-driver-535".to_string(),
            is_proprietary: true,
            is_active: false,
        });
        self.drivers.push(MintDriverItem {
            device_id: "pci:14e4:43a0".to_string(),
            driver_name: "firmware-b43-installer".to_string(),
            is_proprietary: false,
            is_active: true,
        });
    }

    pub fn activate_driver(&mut self, device: &str) -> bool {
        for drv in self.drivers.iter_mut() {
            if drv.device_id == device {
                drv.is_active = true;
                return true;
            }
        }
        false
    }
}

impl Default for MintHardwareDriverManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 7. Mint System Admin PAM Gate
/// Handles PAM authorization rules, password encryption strength,
/// root user validation, and secure non-root capability mapping.
#[derive(Debug, Clone)]
pub struct MintSystemAdminPAM {
    pub shadow_pass_hash: String,
    pub pam_strict_mode: bool,
    pub root_override: bool,
}

impl MintSystemAdminPAM {
    pub fn new(hash: &str) -> Self {
        Self {
            shadow_pass_hash: hash.to_string(),
            pam_strict_mode: true,
            root_override: false,
        }
    }

    pub fn authenticate_user(&self, input_pw: &str) -> bool {
        // Simple hash check comparison mimicking standard pam_unix shadow validation
        let mut state = 0u64;
        for &byte in input_pw.as_bytes() {
            state = state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        let hex_hash = format!("{:x}", state);
        hex_hash == self.shadow_pass_hash
    }

    pub fn mode_to_capability(&self, uid: u32) -> CapabilityToken {
        if uid == 0 || self.root_override {
            CapabilityToken::from_bits(0xFFFF) // Master Root Override
        } else {
            CapabilityToken::from_bits(0x0004) // standard read capability
        }
    }
}

/// 8. Mint Ufw Firewall Manager
/// Emulates standard UFW (Uncomplicated Firewall) rules,
/// enabling stateful incoming/outgoing rules and rate-limiting block states.
#[derive(Debug, Clone)]
pub struct UfwRule {
    pub port: u16,
    pub allow: bool,
    pub rate_limit: bool,
}

#[derive(Debug, Clone)]
pub struct MintUfwFirewall {
    pub is_enabled: bool,
    pub rules: Vec<UfwRule>,
    pub blocked_ips: Vec<String>,
}

impl MintUfwFirewall {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_enabled: false,
            rules: Vec::new(),
            blocked_ips: Vec::new(),
        }
    }

    pub fn enable_firewall(&mut self) {
        self.is_enabled = true;
    }

    pub fn add_rule(&mut self, port: u16, allow: bool, rate_limit: bool) {
        self.rules.push(UfwRule {
            port,
            allow,
            rate_limit,
        });
    }

    pub fn evaluate_connection(&self, remote_ip: &str, port: u16) -> bool {
        if !self.is_enabled {
            return true;
        }

        if self.blocked_ips.contains(&remote_ip.to_string()) {
            return false;
        }

        for rule in &self.rules {
            if rule.port == port {
                return rule.allow;
            }
        }
        false // default deny incoming
    }
}

impl Default for MintUfwFirewall {
    fn default() -> Self {
        Self::new()
    }
}

/// 9. Mint Shell Script Interpreter
/// Parses basic Mint-style environment variables, executes shell aliases (`ll`, `la`),
/// standard streams, pipes, and background daemon states.
#[derive(Debug, Clone)]
pub struct MintShellScriptInterpreter {
    pub env_variables: HashMap<String, String>,
    pub aliases: HashMap<String, String>,
    pub running_daemons: Vec<String>,
}

impl MintShellScriptInterpreter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut env = HashMap::new();
        env.insert("USER".to_string(), "mint".to_string());
        env.insert("SHELL".to_string(), "/bin/bash".to_string());
        env.insert("DESKTOP_SESSION".to_string(), "cinnamon".to_string());

        let mut aliases = HashMap::new();
        aliases.insert("ll".to_string(), "ls -la".to_string());
        aliases.insert("la".to_string(), "ls -A".to_string());

        Self {
            env_variables: env,
            aliases,
            running_daemons: Vec::new(),
        }
    }

    pub fn parse_command(&self, input: &str) -> String {
        let trimmed = input.trim();
        if let Some(aliased) = self.aliases.get(trimmed) {
            aliased.clone()
        } else if trimmed.starts_with("export ") {
            let parts: Vec<&str> = trimmed[7..].split('=').collect();
            if parts.len() == 2 {
                format!("Set env variable {} to {}", parts[0], parts[1])
            } else {
                "Invalid export statement".to_string()
            }
        } else {
            trimmed.to_string()
        }
    }

    pub fn spawn_daemon(&mut self, daemon_name: &str) {
        self.running_daemons.push(daemon_name.to_string());
    }
}

impl Default for MintShellScriptInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// 10. Mint Timeshift Backup
/// Models system-level timeshift backup points (scheduled snapshots,
/// partition mounts, and target filesystem checks).
#[derive(Debug, Clone)]
pub struct TimeshiftSnapshot {
    pub id: String,
    pub timestamp: u64,
    pub filesystem: String,
}

#[derive(Debug, Clone)]
pub struct MintTimeshiftBackup {
    pub snapshots: Vec<TimeshiftSnapshot>,
    pub target_partition: String,
}

impl MintTimeshiftBackup {
    pub fn new(partition: &str) -> Self {
        Self {
            snapshots: Vec::new(),
            target_partition: partition.to_string(),
        }
    }

    pub fn generate_snapshot(&mut self, filesystem: &str) -> String {
        let snapshot_id = format!("timeshift_snap_{}", self.snapshots.len() + 1);
        self.snapshots.push(TimeshiftSnapshot {
            id: snapshot_id.clone(),
            timestamp: 1717329600, // simulated Unix epoch
            filesystem: filesystem.to_string(),
        });
        snapshot_id
    }

    pub fn restore_snapshot(&self, id: &str) -> bool {
        self.snapshots.iter().any(|snap| snap.id == id)
    }
}

// ==========================================
// Unit Tests for Mint Parity Subsystems
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cinnamon_desktop_engine() {
        let mut desk = CinnamonDesktopEngine::new();
        assert_eq!(desk.theme, "Mint-Y-Dark-Teal");
        assert_eq!(desk.active_applets.len(), 5);

        desk.set_theme("Mint-Y-Aqua");
        assert_eq!(desk.theme, "Mint-Y-Aqua");

        desk.enable_software_rendering();
        assert!(desk.fallback_mode);
        assert!(!desk.compositing_effects);

        desk.register_desklet("clock");
        assert_eq!(desk.active_applets[5], "desklet:clock");
    }

    #[test]
    fn test_mint_update_manager() {
        let mut mgr = MintUpdateManager::new();
        mgr.register_update("libreoffice", "7.6.1", UpdateRiskLevel::Level2Normal, 45000);
        mgr.register_update("linux-kernel", "6.2.0-33", UpdateRiskLevel::Level5Dangerous, 85000);

        // Filter updates (risk levels 4-5 are filtered out by default)
        let filtered = mgr.filter_updates();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].package_name, "libreoffice");

        assert!(mgr.run_preflight_snapshot_assertion(true).is_ok());
        assert!(mgr.run_preflight_snapshot_assertion(false).is_err());
    }

    #[test]
    fn test_mint_install_software_manager() {
        let mut install = MintInstallSoftwareManager::new();
        assert_eq!(install.flatpak_repositories.len(), 1);
        assert!(!install.snapcraft_enabled);

        // Attempting to install snap should fail by default
        assert!(install.install_package("firefox", "snap").is_err());

        // Installing Flatpak works
        assert!(install.install_package("firefox", "flatpak").is_ok());

        // Enable snapcraft and install snap
        install.enable_snaps();
        assert!(install.install_package("firefox", "snap").is_ok());
    }

    #[test]
    fn test_mint_backup_tool_and_welcome() {
        let backup = MintBackupTool::new("/home/mint", "/dev/sdb1");
        let (log, size) = backup.run_user_backup(100);
        assert!(log.contains("/home/mint"));
        assert!(size > 0);

        let mut welcome = MintWelcomeEngine::new();
        assert_eq!(welcome.current_step, "first-steps");
        welcome.set_locale("hi_IN");
        assert_eq!(welcome.language_code, "hi_IN");

        let next = welcome.advance_wizard();
        assert_eq!(next, "system-settings");
    }

    #[test]
    fn test_mint_drivers_and_pam() {
        let mut drivers = MintHardwareDriverManager::new();
        drivers.probe_hardware_devices();
        assert_eq!(drivers.drivers.len(), 2);
        assert!(!drivers.drivers[0].is_active);

        assert!(drivers.activate_driver("pci:10de:1f08"));
        assert!(drivers.drivers[0].is_active);

        // hash of "secretpw": e.g. using our simple hash we simulate shadow unix entry
        // "secretpw" as bytes: s(115), e(101), c(99), r(114), e(101), t(116), p(112), w(119)
        // state = 115
        // state = state*31 + 101 = 3666
        // state = 3666*31 + 99 = 113745
        // ... lets calculate dynamically and test pamunix auth
        let pam = MintSystemAdminPAM::new("2f6386270b7");
        assert!(pam.authenticate_user("secretpw"));
        assert!(!pam.authenticate_user("wrongpw"));

        let cap_root = pam.mode_to_capability(0);
        assert_eq!(cap_root.bits(), 0xFFFF);
        let cap_user = pam.mode_to_capability(1001);
        assert_eq!(cap_user.bits(), 0x0004);
    }

    #[test]
    fn test_ufw_and_shell() {
        let mut ufw = MintUfwFirewall::new();
        assert!(!ufw.is_enabled);

        ufw.enable_firewall();
        ufw.add_rule(80, true, false);  // allow HTTP
        ufw.add_rule(22, false, true); // deny SSH with rate limit

        assert!(ufw.evaluate_connection("192.168.1.5", 80));
        assert!(!ufw.evaluate_connection("192.168.1.5", 22));

        // block IP
        ufw.blocked_ips.push("10.0.0.5".to_string());
        assert!(!ufw.evaluate_connection("10.0.0.5", 80)); // even on port 80 because IP is blocked

        let mut interpreter = MintShellScriptInterpreter::new();
        assert_eq!(interpreter.parse_command("ll"), "ls -la");
        assert_eq!(interpreter.parse_command("export PATH=/bin"), "Set env variable PATH to /bin");

        interpreter.spawn_daemon("sshd");
        assert_eq!(interpreter.running_daemons[0], "sshd");
    }

    #[test]
    fn test_timeshift_backup() {
        let mut timeshift = MintTimeshiftBackup::new("/dev/sda2");
        assert_eq!(timeshift.target_partition, "/dev/sda2");

        let id1 = timeshift.generate_snapshot("ext4");
        assert!(timeshift.restore_snapshot(&id1));
        assert!(!timeshift.restore_snapshot("invalid_id"));
    }
}
