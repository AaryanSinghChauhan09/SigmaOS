use alloc::vec;
extern crate alloc;
// SigmaOS openSUSE & Slackware Competitor Parity Subsystem
// Independent, zero-dependency implementations of openSUSE YaST and Slackware pkgtools

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

// =========================================================================
// 1. OPENSUSE YAST (YET ANOTHER SETUP TOOL) CONTROL CENTER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YastModuleType {
    Network,
    Security,
    Users,
    Hardware,
}

pub struct YastCentralControlCenter {
    pub active_modules: BTreeMap<String, YastModuleType>,
    pub configuration_steps_completed: usize,
    pub hostname: String,
    pub firewall_enabled: bool,
}

impl YastCentralControlCenter {
    pub fn new() -> Self {
        let mut modules = BTreeMap::new();
        modules.insert("yast-network".to_string(), YastModuleType::Network);
        modules.insert("yast-firewall".to_string(), YastModuleType::Security);
        modules.insert("yast-users".to_string(), YastModuleType::Users);

        Self {
            active_modules: modules,
            configuration_steps_completed: 0,
            hostname: "opensuse-localhost".to_string(),
            firewall_enabled: true,
        }
    }

    pub fn register_setup_module(&mut self, label: &str, module_type: YastModuleType) {
        self.active_modules.insert(label.to_string(), module_type);
    }

    pub fn configure_network_hostname(&mut self, new_hostname: &str) -> Result<(), &'static str> {
        if new_hostname.is_empty() {
            return Err("YaST: Hostname cannot be empty");
        }
        self.hostname = new_hostname.to_string();
        self.configuration_steps_completed += 1;
        Ok(())
    }

    pub fn configure_security_firewall(&mut self, enable: bool) {
        self.firewall_enabled = enable;
        self.configuration_steps_completed += 1;
    }
}

impl Default for YastCentralControlCenter {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. SLACKWARE PKGTOOLS & SLACKPKG MINIMALIST PACKAGE MANAGER
// =========================================================================

pub struct SlackwarePackage {
    pub name: String,
    pub unpacked_files: Vec<String>,
    pub description_line: String,
}

pub struct SlackwarePkgTools {
    pub installed_packages_db: BTreeMap<String, SlackwarePackage>, // package_name -> SlackwarePackage log
}

impl SlackwarePkgTools {
    pub fn new() -> Self {
        Self {
            installed_packages_db: BTreeMap::new(),
        }
    }

    /// Replicates the famous Slackware 'installpkg' process. Unpacks tarball paths
    /// and logs files in '/var/log/packages/package-name'.
    pub fn installpkg(
        &mut self,
        package_name: &str,
        raw_tar_contents: &[&str],
        slack_desc: &str,
    ) -> Result<(), &'static str> {
        if self
            .installed_packages_db
            .contains_key(&package_name.to_string())
        {
            return Err("installpkg: Package already installed on Slackware system");
        }

        let mut unpacked_files = Vec::new();
        for &file in raw_tar_contents {
            unpacked_files.push(file.to_string());
        }

        let pkg = SlackwarePackage {
            name: package_name.to_string(),
            unpacked_files,
            description_line: slack_desc.to_string(),
        };

        // Write package metadata to Slackware's system registry
        self.installed_packages_db
            .insert(package_name.to_string(), pkg);
        Ok(())
    }

    /// Replicates the famous Slackware 'removepkg' process, removing tracked files from root.
    pub fn removepkg(&mut self, package_name: &str) -> Result<usize, &'static str> {
        let pkg = self
            .installed_packages_db
            .remove(&package_name.to_string())
            .ok_or("removepkg: Package not found in Slackware database")?;

        Ok(pkg.unpacked_files.len())
    }

    pub fn parse_slack_desc_field(&self, raw_desc: &str) -> String {
        // Slackware's slack-desc file format requires a structured 'package: description' line
        for line in raw_desc.lines() {
            if line.contains(":") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    return parts[1].trim().to_string();
                }
            }
        }
        "No Slackware package description found".to_string()
    }
}

impl Default for SlackwarePkgTools {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS MODULE
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yast_central_control_center() {
        let mut yast = YastCentralControlCenter::new();
        assert_eq!(yast.active_modules.len(), 3);
        assert_eq!(yast.hostname, "opensuse-localhost");

        yast.register_setup_module("yast-hardware", YastModuleType::Hardware);
        assert_eq!(yast.active_modules.len(), 4);

        assert!(yast.configure_network_hostname("gecko-station").is_ok());
        assert_eq!(yast.hostname, "gecko-station");

        yast.configure_security_firewall(false);
        assert!(!yast.firewall_enabled);
        assert_eq!(yast.configuration_steps_completed, 2);
    }

    #[test]
    fn test_slackware_pkgtools_install_remove() {
        let mut pkgtools = SlackwarePkgTools::new();
        let tar_contents = vec!["/usr/bin/slackpkg", "/etc/slackpkg/slackpkg.conf"];

        let slack_desc = "
            slackpkg: slackpkg (automated package manager)
            slackpkg: An automated tool to install or upgrade packages
        ";

        // Install Slackware Package
        assert!(pkgtools
            .installpkg("slackpkg", &tar_contents, slack_desc)
            .is_ok());
        assert_eq!(pkgtools.installed_packages_db.len(), 1);

        // Parse slack-desc description line
        let desc = pkgtools.parse_slack_desc_field(slack_desc);
        assert_eq!(desc, "slackpkg (automated package manager)");

        // Try installing duplicate (fails)
        assert!(pkgtools
            .installpkg("slackpkg", &tar_contents, slack_desc)
            .is_err());

        // Remove Slackware Package
        let count = pkgtools.removepkg("slackpkg").unwrap();
        assert_eq!(count, 2);
        assert_eq!(pkgtools.installed_packages_db.len(), 0);
    }
}
