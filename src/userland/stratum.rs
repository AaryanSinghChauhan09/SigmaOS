// Bedrock Linux Inspired Userland Stratum Interoperability Engine
// Location: src/userland/stratum.rs
//
// Enables binaries and libraries from any Linux or BSD distribution stratum
// (Debian, Arch, Fedora, Void, FreeBSD, Alpine, Gentoo, Nix, OpenBSD)
// to execute side-by-side with full cross-stratum path and dynamic library resolution.

use std::collections::{BTreeMap, BTreeSet};
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StratumKind {
    NativeSigma,
    Debian,
    Arch,
    Fedora,
    Void,
    FreeBsd,
    Alpine,
    Gentoo,
    Nix,
    OpenBsd,
}

impl StratumKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::NativeSigma => "sigma",
            Self::Debian => "debian",
            Self::Arch => "arch",
            Self::Fedora => "fedora",
            Self::Void => "void",
            Self::FreeBsd => "freebsd",
            Self::Alpine => "alpine",
            Self::Gentoo => "gentoo",
            Self::Nix => "nix",
            Self::OpenBsd => "openbsd",
        }
    }

    pub fn default_root(&self) -> &'static str {
        match self {
            Self::NativeSigma => "/",
            Self::Debian => "/strata/debian",
            Self::Arch => "/strata/arch",
            Self::Fedora => "/strata/fedora",
            Self::Void => "/strata/void",
            Self::FreeBsd => "/strata/freebsd",
            Self::Alpine => "/strata/alpine",
            Self::Gentoo => "/strata/gentoo",
            Self::Nix => "/nix/store",
            Self::OpenBsd => "/strata/openbsd",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stratum {
    pub kind: StratumKind,
    pub name: String,
    pub root_prefix: String,
    pub binary_directories: Vec<String>,
    pub library_directories: Vec<String>,
    pub registered_commands: BTreeSet<String>,
    pub active: bool,
}

impl Stratum {
    pub fn new(kind: StratumKind) -> Self {
        let root = kind.default_root().to_string();
        let name = kind.name().to_string();

        let mut binary_directories = Vec::new();
        let mut library_directories = Vec::new();
        let mut registered_commands = BTreeSet::new();

        if kind == StratumKind::NativeSigma {
            binary_directories.push("/bin".to_string());
            binary_directories.push("/usr/bin".to_string());
            library_directories.push("/lib".to_string());
            library_directories.push("/usr/lib".to_string());
            library_directories.push("/lib64".to_string());

            registered_commands.insert("ls".to_string());
            registered_commands.insert("cat".to_string());
            registered_commands.insert("grep".to_string());
            registered_commands.insert("ps".to_string());
            registered_commands.insert("sh".to_string());
            registered_commands.insert("bash".to_string());
            registered_commands.insert("pwd".to_string());
            registered_commands.insert("echo".to_string());
        } else {
            binary_directories.push(format!("{}/bin", root));
            binary_directories.push(format!("{}/usr/bin", root));
            library_directories.push(format!("{}/lib", root));
            library_directories.push(format!("{}/usr/lib", root));
            library_directories.push(format!("{}/usr/lib64", root));

            match kind {
                StratumKind::Debian => {
                    registered_commands.insert("dpkg".to_string());
                    registered_commands.insert("apt".to_string());
                    registered_commands.insert("apt-get".to_string());
                }
                StratumKind::Arch => {
                    registered_commands.insert("pacman".to_string());
                    registered_commands.insert("makepkg".to_string());
                }
                StratumKind::Fedora => {
                    registered_commands.insert("dnf".to_string());
                    registered_commands.insert("rpm".to_string());
                }
                StratumKind::Void => {
                    registered_commands.insert("xbps-install".to_string());
                }
                StratumKind::FreeBsd => {
                    registered_commands.insert("pkg".to_string());
                    registered_commands.insert("jail".to_string());
                }
                StratumKind::Alpine => {
                    registered_commands.insert("apk".to_string());
                }
                StratumKind::Gentoo => {
                    registered_commands.insert("emerge".to_string());
                }
                StratumKind::Nix => {
                    registered_commands.insert("nix".to_string());
                }
                StratumKind::OpenBsd => {
                    registered_commands.insert("pkg_add".to_string());
                }
                _ => {}
            }
        }

        Self {
            kind,
            name,
            root_prefix: root,
            binary_directories,
            library_directories,
            registered_commands,
            active: true,
        }
    }

    pub fn register_command(&mut self, cmd: &str) {
        self.registered_commands.insert(cmd.to_string());
    }
}

pub struct StratumManager {
    pub strata: BTreeMap<StratumKind, Stratum>,
    pub priority_order: Vec<StratumKind>,
}

impl StratumManager {
    pub fn new() -> Self {
        let mut strata = BTreeMap::new();
        let priority_order = vec![
            StratumKind::NativeSigma,
            StratumKind::Arch,
            StratumKind::Debian,
            StratumKind::Fedora,
            StratumKind::Void,
            StratumKind::FreeBsd,
            StratumKind::Alpine,
            StratumKind::Gentoo,
            StratumKind::Nix,
            StratumKind::OpenBsd,
        ];

        for &kind in &priority_order {
            strata.insert(kind, Stratum::new(kind));
        }

        Self {
            strata,
            priority_order,
        }
    }

    pub fn register_stratum(&mut self, stratum: Stratum) {
        if !self.priority_order.contains(&stratum.kind) {
            self.priority_order.push(stratum.kind);
        }
        self.strata.insert(stratum.kind, stratum);
    }

    /// Resolves binary executable path across all registered strata transparently.
    pub fn resolve_command_path(&self, command: &str) -> Option<String> {
        if command.starts_with('/') {
            return Some(command.to_string());
        }

        for &kind in &self.priority_order {
            if let Some(stratum) = self.strata.get(&kind) {
                if !stratum.active {
                    continue;
                }
                if stratum.registered_commands.contains(command) {
                    if let Some(first_dir) = stratum.binary_directories.first() {
                        return Some(format!("{}/{}", first_dir, command));
                    }
                }
            }
        }

        // Default fallback to native sigma path
        if let Some(native) = self.strata.get(&StratumKind::NativeSigma) {
            if let Some(first_dir) = native.binary_directories.first() {
                return Some(format!("{}/{}", first_dir, command));
            }
        }

        None
    }

    /// Constructs unified dynamic library search paths across active strata.
    pub fn construct_unified_library_path(&self) -> String {
        let mut paths = Vec::new();
        for &kind in &self.priority_order {
            if let Some(stratum) = self.strata.get(&kind) {
                if stratum.active {
                    paths.extend(stratum.library_directories.clone());
                }
            }
        }
        paths.join(":")
    }

    /// Translates cross-stratum path execution context.
    pub fn cross_stratum_exec(&self, command: &str, target_stratum: StratumKind) -> Result<String, &'static str> {
        let stratum = self.strata.get(&target_stratum).ok_or("Target stratum not registered")?;
        if !stratum.active {
            return Err("Target stratum is disabled");
        }
        Ok(format!("{}/usr/bin/{}", stratum.root_prefix, command))
    }
}

impl Default for StratumManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stratum_manager_initialization() {
        let mgr = StratumManager::new();
        assert!(mgr.strata.contains_key(&StratumKind::NativeSigma));
        assert!(mgr.strata.contains_key(&StratumKind::Debian));
        assert!(mgr.strata.contains_key(&StratumKind::Arch));
        assert!(mgr.strata.contains_key(&StratumKind::FreeBsd));

        // Command resolution for debian stratum command
        let apt_path = mgr.resolve_command_path("apt-get").unwrap();
        assert_eq!(apt_path, "/strata/debian/bin/apt-get");

        // Command resolution for arch stratum command
        let pacman_path = mgr.resolve_command_path("pacman").unwrap();
        assert_eq!(pacman_path, "/strata/arch/bin/pacman");
    }

    #[test]
    fn test_unified_library_path_construction() {
        let mgr = StratumManager::new();
        let lib_path = mgr.construct_unified_library_path();
        assert!(lib_path.contains("/lib"));
        assert!(lib_path.contains("/strata/debian/lib"));
        assert!(lib_path.contains("/strata/freebsd/lib"));
    }

    #[test]
    fn test_cross_stratum_exec() {
        let mgr = StratumManager::new();
        let exec_path = mgr.cross_stratum_exec("pacman", StratumKind::Arch).unwrap();
        assert_eq!(exec_path, "/strata/arch/usr/bin/pacman");
    }
}
