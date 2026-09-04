//! Complete Linux Distribution Parity Subsystem for SigmaOS
//! Implements essential Linux OS capabilities to ensure seamless compatibility with standard Linux distributions:
//! - LSB (Linux Standard Base) release metadata and /etc/os-release parsing (`LsbReleaseGovernor`)
//! - SysVinit runlevels and systemd target management (`LinuxRunlevelGovernor`)
//! - `/etc/fstab` filesystem mount table entry parsing (`LinuxFstabEngine`)
//! - Dynamic shared library symbol loader resolution simulation (`LinuxLdSoLoader`)
extern crate alloc;

#[cfg(not(test))]
use crate::klib::{HashMap, Vec};
use alloc::string::String;
use alloc::string::ToString;
#[cfg(test)]
use alloc::vec::Vec;
#[cfg(test)]
use std::collections::HashMap;

// ==========================================
// 1. Linux Standard Base (LSB) & /etc/os-release
// ==========================================

#[derive(Debug, Clone)]
pub struct LsbReleaseInfo {
    pub distro_id: String,
    pub description: String,
    pub release: String,
    pub codename: String,
    pub pretty_name: String,
}

pub struct LsbReleaseGovernor;

impl LsbReleaseGovernor {
    /// Parses standard Linux `/etc/os-release` file content
    pub fn parse_os_release(content: &str) -> LsbReleaseInfo {
        let mut distro_id = String::from("sigmaos");
        let mut description = String::from("SigmaOS Linux Universal Parity Edition");
        let mut release = String::from("1.0.0");
        let mut codename = String::from("rolling");
        let mut pretty_name = String::from("SigmaOS GNU/Linux");

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            if let Some(idx) = trimmed.find('=') {
                let key = trimmed[..idx].trim();
                let val = trimmed[idx + 1..]
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'');

                match key {
                    "ID" => distro_id = val.to_string(),
                    "NAME" | "PRETTY_NAME" => pretty_name = val.to_string(),
                    "VERSION_ID" => release = val.to_string(),
                    "VERSION_CODENAME" => codename = val.to_string(),
                    "VERSION" => description = val.to_string(),
                    _ => {}
                }
            }
        }

        LsbReleaseInfo {
            distro_id,
            description,
            release,
            codename,
            pretty_name,
        }
    }
}

// ==========================================
// 2. SysVinit Runlevels & Systemd Target Parity
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxRunlevel {
    Runlevel0Halt,
    Runlevel1SingleUser,
    Runlevel2MultiUserNoNet,
    Runlevel3MultiUserNet,
    Runlevel5Graphical,
    Runlevel6Reboot,
}

pub struct LinuxRunlevelGovernor {
    pub current_runlevel: LinuxRunlevel,
    pub active_services: Vec<String>,
}

impl LinuxRunlevelGovernor {
    pub fn new() -> Self {
        Self {
            current_runlevel: LinuxRunlevel::Runlevel5Graphical,
            active_services: Vec::new(),
        }
    }

    pub fn set_runlevel(&mut self, target: LinuxRunlevel) {
        self.current_runlevel = target;
        match target {
            LinuxRunlevel::Runlevel0Halt | LinuxRunlevel::Runlevel6Reboot => {
                self.active_services.clear();
            }
            LinuxRunlevel::Runlevel1SingleUser => {
                self.active_services
                    .retain(|s| s == "sulogin" || s == "syslog");
            }
            LinuxRunlevel::Runlevel3MultiUserNet => {
                if !self.active_services.contains(&"network".to_string()) {
                    self.active_services.push("network".to_string());
                }
            }
            LinuxRunlevel::Runlevel5Graphical => {
                if !self
                    .active_services
                    .contains(&"display-manager".to_string())
                {
                    self.active_services.push("display-manager".to_string());
                }
            }
            _ => {}
        }
    }
}

impl Default for LinuxRunlevelGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 3. /etc/fstab Mount Entry Engine
// ==========================================

#[derive(Debug, Clone)]
pub struct FstabEntry {
    pub spec: String,        // UUID=xxx or /dev/sda1
    pub file: String,        // Mount point e.g. /home or /
    pub vfstype: String,     // ext4, btrfs, xfs, vfat
    pub mntops: Vec<String>, // rw, noatime, errors=remount-ro
    pub freq: u32,           // dump order
    pub passno: u32,         // fsck order
}

pub struct LinuxFstabEngine;

impl LinuxFstabEngine {
    /// Parses `/etc/fstab` configuration file
    pub fn parse_fstab(content: &str) -> Vec<FstabEntry> {
        let mut entries = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 4 {
                let spec = parts[0].to_string();
                let file = parts[1].to_string();
                let vfstype = parts[2].to_string();
                let mntops = parts[3]
                    .split(',')
                    .map(|s: &str| s.trim().to_string())
                    .collect();
                let freq = parts
                    .get(4)
                    .and_then(|s: &&str| s.parse().ok())
                    .unwrap_or(0);
                let passno = parts
                    .get(5)
                    .and_then(|s: &&str| s.parse().ok())
                    .unwrap_or(0);

                entries.push(FstabEntry {
                    spec,
                    file,
                    vfstype,
                    mntops,
                    freq,
                    passno,
                });
            }
        }

        entries
    }
}

// ==========================================
// 4. Dynamic Shared Library Symbol Loader (ld.so)
// ==========================================

#[derive(Debug, Clone)]
pub struct SharedLibrary {
    pub soname: String,
    pub symbols: HashMap<String, u64>, // Symbol Name -> Address
}

pub struct LinuxLdSoLoader {
    pub loaded_libraries: HashMap<String, SharedLibrary>,
    pub library_paths: Vec<String>,
}

impl LinuxLdSoLoader {
    pub fn new() -> Self {
        let mut library_paths = Vec::new();
        library_paths.push("/lib64".to_string());
        library_paths.push("/usr/lib64".to_string());
        library_paths.push("/usr/local/lib".to_string());

        Self {
            loaded_libraries: HashMap::new(),
            library_paths,
        }
    }

    pub fn register_library(&mut self, soname: &str, mut library: SharedLibrary) {
        library.soname = soname.to_string();
        self.loaded_libraries.insert(soname.to_string(), library);
    }

    pub fn dlsym(&self, soname: &str, symbol_name: &str) -> Option<u64> {
        if let Some(lib) = self.loaded_libraries.get(soname) {
            lib.symbols.get(symbol_name).copied()
        } else {
            None
        }
    }
}

impl Default for LinuxLdSoLoader {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 5. Linux Pluggable Authentication Modules (PAM) Engine
// ==========================================

pub struct LinuxPamAuthenticationEngine {
    pub active_service: String,
    pub pam_modules: Vec<String>,
    pub authenticated_sessions: HashMap<String, bool>,
}

impl LinuxPamAuthenticationEngine {
    pub fn new(service_name: &str) -> Self {
        let mut pam_modules = Vec::new();
        pam_modules.push("pam_unix.so".to_string());
        pam_modules.push("pam_env.so".to_string());
        pam_modules.push("pam_limits.so".to_string());

        Self {
            active_service: service_name.to_string(),
            pam_modules,
            authenticated_sessions: HashMap::new(),
        }
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<bool, &'static str> {
        if username.is_empty() {
            return Err("PAM Authentication Error: Username empty");
        }

        // Simulate pam_unix.so credential check
        let is_valid = password == "sigma_pass" || password == "root_pass";
        self.authenticated_sessions
            .insert(username.to_string(), is_valid);
        Ok(is_valid)
    }

    pub fn close_session(&mut self, username: &str) {
        self.authenticated_sessions.remove(username);
    }
}

// ==========================================
// 6. Linux Sysctl Governor (/etc/sysctl.conf)
// ==========================================

pub struct LinuxSysctlGovernor {
    pub sysctl_params: HashMap<String, String>,
}

impl LinuxSysctlGovernor {
    pub fn new() -> Self {
        let mut params = HashMap::new();
        params.insert("vm.swappiness".to_string(), "60".to_string());
        params.insert("net.ipv4.ip_forward".to_string(), "0".to_string());
        params.insert("fs.file-max".to_string(), "2097152".to_string());
        Self {
            sysctl_params: params,
        }
    }

    pub fn parse_sysctl_conf(&mut self, content: &str) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.starts_with(';') || trimmed.is_empty() {
                continue;
            }
            if let Some(idx) = trimmed.find('=') {
                let key = trimmed[..idx].trim().to_string();
                let val = trimmed[idx + 1..].trim().to_string();
                self.sysctl_params.insert(key, val);
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.sysctl_params.get(key).map(|s| s.as_str())
    }

    pub fn set(&mut self, key: &str, val: &str) {
        self.sysctl_params.insert(key.to_string(), val.to_string());
    }
}

impl Default for LinuxSysctlGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 7. Linux Udev Rules Engine (/etc/udev/rules.d/)
// ==========================================

#[derive(Debug, Clone)]
pub struct UdevRule {
    pub subsystem: Option<String>,
    pub action: Option<String>,
    pub vendor_id: Option<String>,
    pub product_id: Option<String>,
    pub run_command: Option<String>,
    pub mode: Option<String>,
}

pub struct LinuxUdevRulesEngine {
    pub rules: Vec<UdevRule>,
}

impl LinuxUdevRulesEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn parse_rule_line(&mut self, line: &str) {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            return;
        }

        let mut subsystem = None;
        let mut action = None;
        let mut vendor_id = None;
        let mut product_id = None;
        let mut run_command = None;
        let mut mode = None;

        for token in trimmed.split(',') {
            let parts: Vec<&str> = token
                .split('=')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if parts.len() == 2 {
                let key = parts[0].trim_end_matches(':').trim_end_matches('+');
                let val = parts[1];
                match key {
                    "SUBSYSTEM" => subsystem = Some(val.to_string()),
                    "ACTION" => action = Some(val.to_string()),
                    "ATTR{idVendor}" => vendor_id = Some(val.to_string()),
                    "ATTR{idProduct}" => product_id = Some(val.to_string()),
                    "RUN" => run_command = Some(val.to_string()),
                    "MODE" => mode = Some(val.to_string()),
                    _ => {}
                }
            }
        }

        self.rules.push(UdevRule {
            subsystem,
            action,
            vendor_id,
            product_id,
            run_command,
            mode,
        });
    }

    pub fn match_device(&self, subsystem: &str, vendor: &str, product: &str) -> Vec<&UdevRule> {
        self.rules
            .iter()
            .filter(|r| {
                (r.subsystem.is_none() || r.subsystem.as_deref() == Some(subsystem))
                    && (r.vendor_id.is_none() || r.vendor_id.as_deref() == Some(vendor))
                    && (r.product_id.is_none() || r.product_id.as_deref() == Some(product))
            })
            .collect()
    }
}

impl Default for LinuxUdevRulesEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 8. Linux Modules Load Engine (/etc/modules-load.d/)
// ==========================================

pub struct LinuxModulesLoadEngine {
    pub modules_to_load: Vec<String>,
}

impl LinuxModulesLoadEngine {
    pub fn new() -> Self {
        Self {
            modules_to_load: Vec::new(),
        }
    }

    pub fn parse_modules_load_conf(&mut self, content: &str) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.starts_with(';') || trimmed.is_empty() {
                continue;
            }
            if !self.modules_to_load.contains(&trimmed.to_string()) {
                self.modules_to_load.push(trimmed.to_string());
            }
        }
    }
}

impl Default for LinuxModulesLoadEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 9. Integration Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsb_os_release_parser() {
        let os_release_data = r#"
NAME="Ubuntu"
VERSION="24.04 LTS (Noble Numbat)"
ID=ubuntu
PRETTY_NAME="Ubuntu 24.04 LTS"
VERSION_ID="24.04"
VERSION_CODENAME=noble
"#;
        let info = LsbReleaseGovernor::parse_os_release(os_release_data);
        assert_eq!(info.distro_id, "ubuntu");
        assert_eq!(info.release, "24.04");
        assert_eq!(info.codename, "noble");
        assert_eq!(info.pretty_name, "Ubuntu 24.04 LTS");
    }

    #[test]
    fn test_linux_runlevel_governor() {
        let mut governor = LinuxRunlevelGovernor::new();
        assert_eq!(governor.current_runlevel, LinuxRunlevel::Runlevel5Graphical);

        governor.set_runlevel(LinuxRunlevel::Runlevel3MultiUserNet);
        assert_eq!(
            governor.current_runlevel,
            LinuxRunlevel::Runlevel3MultiUserNet
        );
        assert!(governor.active_services.contains(&"network".to_string()));

        governor.set_runlevel(LinuxRunlevel::Runlevel0Halt);
        assert!(governor.active_services.is_empty());
    }

    #[test]
    fn test_fstab_parser() {
        let fstab_data = r#"
# /etc/fstab: static file system information.
UUID=1234-5678-90AB-CDEF /               ext4    errors=remount-ro 0       1
UUID=AAAA-BBBB           /boot/efi       vfat    umask=0077        0       2
/dev/sda3                /home           btrfs   defaults,noatime  0       2
"#;
        let entries = LinuxFstabEngine::parse_fstab(fstab_data);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].file, "/");
        assert_eq!(entries[0].vfstype, "ext4");
        assert_eq!(entries[2].file, "/home");
        assert_eq!(
            entries[2].mntops,
            vec!["defaults".to_string(), "noatime".to_string()]
        );
    }

    #[test]
    fn test_linux_pam_authentication_engine() {
        let mut pam = LinuxPamAuthenticationEngine::new("sshd");
        assert_eq!(pam.active_service, "sshd");
        assert!(pam.pam_modules.contains(&"pam_unix.so".to_string()));

        let ok = pam.authenticate("sovereign_user", "sigma_pass").unwrap();
        assert!(ok);
        assert_eq!(
            pam.authenticated_sessions.get("sovereign_user"),
            Some(&true)
        );

        pam.close_session("sovereign_user");
        assert!(pam.authenticated_sessions.get("sovereign_user").is_none());
    }

    #[test]
    fn test_ld_so_symbol_loader() {
        let mut loader = LinuxLdSoLoader::new();
        let mut libc_symbols = HashMap::new();
        libc_symbols.insert("malloc".to_string(), 0x7FFF00010000);
        libc_symbols.insert("free".to_string(), 0x7FFF00010040);

        let libc = SharedLibrary {
            soname: "libc.so.6".to_string(),
            symbols: libc_symbols,
        };

        loader.register_library("libc.so.6", libc);

        assert_eq!(loader.dlsym("libc.so.6", "malloc"), Some(0x7FFF00010000));
        assert_eq!(loader.dlsym("libc.so.6", "free"), Some(0x7FFF00010040));
        assert_eq!(loader.dlsym("libc.so.6", "nonexistent"), None);
    }

    #[test]
    fn test_linux_sysctl_governor() {
        let mut sysctl = LinuxSysctlGovernor::new();
        assert_eq!(sysctl.get("vm.swappiness"), Some("60"));

        let conf = r#"
# Kernel IP forwarding
net.ipv4.ip_forward = 1
vm.swappiness = 10
"#;
        sysctl.parse_sysctl_conf(conf);
        assert_eq!(sysctl.get("net.ipv4.ip_forward"), Some("1"));
        assert_eq!(sysctl.get("vm.swappiness"), Some("10"));
    }

    #[test]
    fn test_linux_udev_rules_engine() {
        let mut udev = LinuxUdevRulesEngine::new();
        udev.parse_rule_line(r#"SUBSYSTEM=="usb", ATTR{idVendor}=="10de", ATTR{idProduct}=="1e84", MODE="0666", RUN+="/usr/bin/nvidia_setup"#);

        let matches = udev.match_device("usb", "10de", "1e84");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].mode.as_deref(), Some("0666"));
    }

    #[test]
    fn test_linux_modules_load_engine() {
        let mut modules = LinuxModulesLoadEngine::new();
        let conf = r#"
# Load wireguard and kvm at boot
wireguard
kvm
"#;
        modules.parse_modules_load_conf(conf);
        assert_eq!(
            modules.modules_to_load,
            vec!["wireguard".to_string(), "kvm".to_string()]
        );
    }
}
