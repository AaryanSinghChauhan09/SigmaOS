//! Omarchy Server Edition Engine & BBS Front Door Interface for SigmaOS
//!
//! Implements a headless Omarchy Server edition with BBS front-door capabilities:
//! - `OmarchyEdition`: `Server` vs `Desktop` edition enum & `OmarchyEditionGuard` file reader (`/etc/omarchy-edition`)
//! - `OmarchyServerBbsMenuEngine`: BBS door system routing (`Status` -> btop, `Docker` -> lazydocker, `Logs` -> lazyjournal, `Update` -> omarchy-update, `Backup`, `Network`, `Theme`, `Quit`)
//! - `OmarchyServerLoginGreetingEngine`: Pre-login `/etc/issue` generator and login splash (`Splash`, `Menu`, `Off` greet modes, node callers, vitals summary, MOTD)
//! - `OmarchyServerPackagesSpec`: Headless server package list filtering out Hyprland, Quickshell, audio, and GUI dependencies

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Omarchy Edition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmarchyEdition {
    Server,
    Desktop,
}

impl OmarchyEdition {
    pub fn as_str(&self) -> &'static str {
        match self {
            OmarchyEdition::Server => "server",
            OmarchyEdition::Desktop => "desktop",
        }
    }
}

/// Edition Gating Guard
#[derive(Debug, Clone)]
pub struct OmarchyEditionGuard {
    pub current_edition: OmarchyEdition,
    pub edition_file_path: String,
}

impl OmarchyEditionGuard {
    pub fn new(edition: OmarchyEdition) -> Self {
        Self {
            current_edition: edition,
            edition_file_path: "/etc/omarchy-edition".to_string(),
        }
    }

    pub fn is_server(&self) -> bool {
        self.current_edition == OmarchyEdition::Server
    }

    pub fn is_desktop(&self) -> bool {
        self.current_edition == OmarchyEdition::Desktop
    }

    pub fn render_edition_file_content(&self) -> String {
        format!("OMARCHY_EDITION={}\n", self.current_edition.as_str())
    }
}

impl Default for OmarchyEditionGuard {
    fn default() -> Self {
        Self::new(OmarchyEdition::Server)
    }
}

/// BBS Greeting Mode for SSH / Getty interactive logins
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BbsGreetMode {
    Splash,
    Menu,
    Off,
}

/// BBS Door Menu Entry
#[derive(Debug, Clone)]
pub struct BbsDoorEntry {
    pub hotkey: char,
    pub label: String,
    pub target_command: String,
    pub description: String,
}

/// Omarchy Server BBS Menu Engine
pub struct OmarchyServerBbsMenuEngine {
    pub doors: BTreeMap<char, BbsDoorEntry>,
    pub node_number: u32,
    pub sysop_name: String,
}

impl OmarchyServerBbsMenuEngine {
    pub fn new(node_num: u32, sysop: &str) -> Self {
        let mut engine = Self {
            doors: BTreeMap::new(),
            node_number: node_num,
            sysop_name: sysop.to_string(),
        };

        engine.add_door('S', "Status", "btop", "System vitals and process monitor");
        engine.add_door('D', "Docker", "lazydocker", "Container management console");
        engine.add_door('L', "Logs", "lazyjournal", "Journald log analyzer");
        engine.add_door('U', "Update", "omarchy-update", "System update with pre-flight snapshot");
        engine.add_door('B', "Backup", "omarchy-backup status", "Btrfs/Snapper backup controls");
        engine.add_door('N', "Network", "ufw status verbose", "UFW firewall and interface status");
        engine.add_door('T', "Theme", "omarchy-theme-set", "Switch system terminal theme");
        engine.add_door('Q', "Quit", "exit", "Exit to interactive shell");
        engine
    }

    pub fn add_door(&mut self, key: char, label: &str, cmd: &str, desc: &str) {
        self.doors.insert(
            key,
            BbsDoorEntry {
                hotkey: key,
                label: label.to_string(),
                target_command: cmd.to_string(),
                description: desc.to_string(),
            },
        );
    }

    pub fn execute_door(&self, key: char) -> Result<String, &'static str> {
        if let Some(door) = self.doors.get(&key.to_ascii_uppercase()) {
            Ok(format!("Launching BBS Door [{}]: {}", door.label, door.target_command))
        } else {
            Err("Invalid BBS door selection")
        }
    }

    pub fn render_bbs_menu_art(&self) -> String {
        let mut art = format!("=== OMARCHY SERVER BBS (Node #{}) - Sysop: {} ===\n\n", self.node_number, self.sysop_name);
        for door in self.doors.values() {
            art.push_str(&format!(" [{}] {:<10} - {}\n", door.hotkey, door.label, door.description));
        }
        art.push_str("\nSelect door [S/D/L/U/B/N/T/Q]: ");
        art
    }
}

impl Default for OmarchyServerBbsMenuEngine {
    fn default() -> Self {
        Self::new(1, "sysop")
    }
}

/// Server Login Greeting Engine
pub struct OmarchyServerLoginGreetingEngine {
    pub greet_mode: BbsGreetMode,
    pub callers_today: u32,
    pub motd_text: String,
}

impl OmarchyServerLoginGreetingEngine {
    pub fn new(motd: &str) -> Self {
        Self {
            greet_mode: BbsGreetMode::Splash,
            callers_today: 1,
            motd_text: motd.to_string(),
        }
    }

    pub fn set_greet_mode(&mut self, mode: BbsGreetMode) {
        self.greet_mode = mode;
    }

    pub fn record_caller_login(&mut self) -> u32 {
        self.callers_today += 1;
        self.callers_today
    }

    pub fn render_etc_issue(&self, hostname: &str, ip_addr: &str) -> String {
        format!(
            "Omarchy Server 4.0 ({})\nHost: {} | IP: {}\n\n",
            hostname, hostname, ip_addr
        )
    }

    pub fn render_login_splash(&self, hostname: &str, node_num: u32) -> String {
        match self.greet_mode {
            BbsGreetMode::Off => String::new(),
            BbsGreetMode::Splash | BbsGreetMode::Menu => {
                format!(
                    "====================================================\n\
                     Welcome to OMARCHY SERVER [{}]\n\
                     Node: #{} | Callers Today: {}\n\
                     MOTD: {}\n\
                     ====================================================\n\
                     Press [ENTER] for BBS Menu, or any key for Shell...\n",
                    hostname, node_num, self.callers_today, self.motd_text
                )
            }
        }
    }
}

impl Default for OmarchyServerLoginGreetingEngine {
    fn default() -> Self {
        Self::new("Welcome to Omarchy Server edition!")
    }
}

/// Omarchy Server Packages Specification
#[derive(Debug, Clone)]
pub struct OmarchyServerPackagesSpec {
    pub included_packages: Vec<String>,
    pub excluded_gui_packages: Vec<String>,
}

impl OmarchyServerPackagesSpec {
    pub fn new() -> Self {
        Self {
            included_packages: vec![
                "base".to_string(),
                "linux".to_string(),
                "linux-firmware".to_string(),
                "openssh".to_string(),
                "ufw".to_string(),
                "docker".to_string(),
                "docker-compose".to_string(),
                "snapper".to_string(),
                "btop".to_string(),
                "lazygit".to_string(),
                "lazydocker".to_string(),
                "lazyjournal".to_string(),
                "gum".to_string(),
                "starship".to_string(),
            ],
            excluded_gui_packages: vec![
                "hyprland".to_string(),
                "quickshell".to_string(),
                "waybar".to_string(),
                "pipewire".to_string(),
                "mako".to_string(),
                "walker".to_string(),
                "firefox".to_string(),
                "ghostty".to_string(),
            ],
        }
    }

    pub fn is_package_allowed(&self, pkg: &str) -> bool {
        !self.excluded_gui_packages.contains(&pkg.to_string())
    }

    pub fn filter_package_list(&self, raw_packages: &[&str]) -> Vec<String> {
        raw_packages
            .iter()
            .filter(|p| self.is_package_allowed(p))
            .map(|p| p.to_string())
            .collect()
    }
}

impl Default for OmarchyServerPackagesSpec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod omarchy_server_tests {
    use super::*;

    #[test]
    fn test_edition_guard() {
        let guard = OmarchyEditionGuard::new(OmarchyEdition::Server);
        assert!(guard.is_server());
        assert!(!guard.is_desktop());
        assert!(guard.render_edition_file_content().contains("OMARCHY_EDITION=server"));
    }

    #[test]
    fn test_bbs_menu_engine() {
        let menu = OmarchyServerBbsMenuEngine::new(1, "sysadmin");
        assert_eq!(menu.doors.len(), 8);
        let res = menu.execute_door('S').unwrap();
        assert!(res.contains("btop"));

        let art = menu.render_bbs_menu_art();
        assert!(art.contains("Node #1"));
        assert!(art.contains("Status"));
    }

    #[test]
    fn test_login_greeting_engine() {
        let mut greeting = OmarchyServerLoginGreetingEngine::new("Server online");
        assert_eq!(greeting.record_caller_login(), 2);

        let splash = greeting.render_login_splash("homelab", 1);
        assert!(splash.contains("OMARCHY SERVER [homelab]"));
        assert!(splash.contains("Callers Today: 2"));

        greeting.set_greet_mode(BbsGreetMode::Off);
        assert!(greeting.render_login_splash("homelab", 1).is_empty());
    }

    #[test]
    fn test_server_package_spec() {
        let spec = OmarchyServerPackagesSpec::new();
        assert!(spec.is_package_allowed("docker"));
        assert!(!spec.is_package_allowed("hyprland"));

        let input = vec!["docker", "hyprland", "btop", "waybar"];
        let filtered = spec.filter_package_list(&input);
        assert_eq!(filtered, vec!["docker".to_string(), "btop".to_string()]);
    }
}
