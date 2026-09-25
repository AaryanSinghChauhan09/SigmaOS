// SigmaOS GoboLinux Decoupled Non-FHS Symlink Hierarchy Subsystem
// Incorporates GoboLinux inspirations:
// - /Programs/<AppName>/<Version>/ decoupled application store
// - /Programs/<AppName>/Current dynamic symlink pointing to active version
// - /System/Index/{bin,lib,include,share} unified symlink index tree
// - Settings/ (/etc alternative) & Variable/ (/var alternative) per-app directories
// - /Users and /Data top-level root directories
// - POSIX FHS (/usr/bin, /lib, /etc, /var) <-> GoboLinux path translation engine

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoboProgramVersion {
    pub version: String,
    pub binaries: Vec<String>,
    pub libraries: Vec<String>,
    pub headers: Vec<String>,
    pub shared_data: Vec<String>,
    pub settings: Vec<String>, // equivalent to /etc
    pub variable: Vec<String>, // equivalent to /var
}

#[derive(Debug, Clone)]
pub struct GoboProgramEntry {
    pub name: String,
    pub versions: BTreeMap<String, GoboProgramVersion>,
    pub current_version: String,
}

#[derive(Debug, Clone)]
pub struct GoboSystemIndex {
    pub bin_index: BTreeMap<String, String>,     // binary_name -> /Programs/App/Current/bin/binary
    pub lib_index: BTreeMap<String, String>,     // lib_name -> /Programs/App/Current/lib/lib_name
    pub include_index: BTreeMap<String, String>, // header_name -> /Programs/App/Current/include/header_name
    pub share_index: BTreeMap<String, String>,   // data_name -> /Programs/App/Current/share/data_name
}

pub struct GoboLinuxSymlinkHierarchyEngine {
    pub programs: BTreeMap<String, GoboProgramEntry>,
    pub index: GoboSystemIndex,
    pub users_root: String, // /Users
    pub data_root: String,  // /Data
}

impl GoboLinuxSymlinkHierarchyEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            programs: BTreeMap::new(),
            index: GoboSystemIndex {
                bin_index: BTreeMap::new(),
                lib_index: BTreeMap::new(),
                include_index: BTreeMap::new(),
                share_index: BTreeMap::new(),
            },
            users_root: "/Users".to_string(),
            data_root: "/Data".to_string(),
        };

        engine.register_core_defaults();
        engine
    }

    fn register_core_defaults(&mut self) {
        let gcc_v13 = GoboProgramVersion {
            version: "13.2.0".to_string(),
            binaries: vec!["gcc".to_string(), "g++".to_string(), "cpp".to_string()],
            libraries: vec!["libgcc_s.so".to_string(), "libstdc++.so".to_string()],
            headers: vec!["gcc.h".to_string(), "g++.h".to_string()],
            shared_data: vec!["man/gcc.1".to_string()],
            settings: vec!["gcc.conf".to_string()],
            variable: vec!["log/gcc.log".to_string()],
        };

        self.install_program_version("GCC", gcc_v13);

        let bash_v5 = GoboProgramVersion {
            version: "5.2.15".to_string(),
            binaries: vec!["bash".to_string(), "sh".to_string()],
            libraries: vec![],
            headers: vec![],
            shared_data: vec!["man/bash.1".to_string()],
            settings: vec!["bashrc".to_string()],
            variable: vec![],
        };

        self.install_program_version("Bash", bash_v5);
    }

    pub fn install_program_version(&mut self, app_name: &str, version_info: GoboProgramVersion) {
        let ver = version_info.version.clone();
        if let Some(entry) = self.programs.get_mut(app_name) {
            entry.versions.insert(ver.clone(), version_info);
            entry.current_version = ver;
        } else {
            let mut versions = BTreeMap::new();
            versions.insert(ver.clone(), version_info);
            self.programs.insert(
                app_name.to_string(),
                GoboProgramEntry {
                    name: app_name.to_string(),
                    versions,
                    current_version: ver,
                },
            );
        }

        self.rebuild_system_index(app_name);
    }

    pub fn switch_current_version(&mut self, app_name: &str, target_version: &str) -> Result<(), &'static str> {
        let entry = self
            .programs
            .get_mut(app_name)
            .ok_or("GoboLinux: Program not found")?;

        if !entry.versions.contains_key(target_version) {
            return Err("GoboLinux: Target version not found");
        }

        entry.current_version = target_version.to_string();
        self.rebuild_system_index(app_name);
        Ok(())
    }

    pub fn rebuild_system_index(&mut self, app_name: &str) {
        if let Some(entry) = self.programs.get(app_name) {
            if let Some(version_info) = entry.versions.get(&entry.current_version) {
                // Binaries -> /System/Index/bin
                for bin in &version_info.binaries {
                    let path = format!("/Programs/{}/Current/bin/{}", app_name, bin);
                    self.index.bin_index.insert(bin.clone(), path);
                }

                // Libraries -> /System/Index/lib
                for lib in &version_info.libraries {
                    let path = format!("/Programs/{}/Current/lib/{}", app_name, lib);
                    self.index.lib_index.insert(lib.clone(), path);
                }

                // Headers -> /System/Index/include
                for header in &version_info.headers {
                    let path = format!("/Programs/{}/Current/include/{}", app_name, header);
                    self.index.include_index.insert(header.clone(), path);
                }

                // Share -> /System/Index/share
                for share in &version_info.shared_data {
                    let path = format!("/Programs/{}/Current/share/{}", app_name, share);
                    self.index.share_index.insert(share.clone(), path);
                }
            }
        }
    }

    pub fn translate_fhs_to_gobolinux(&self, fhs_path: &str) -> String {
        if fhs_path.starts_with("/usr/bin/") || fhs_path.starts_with("/bin/") {
            let bin_name = fhs_path
                .trim_start_matches("/usr/bin/")
                .trim_start_matches("/bin/");
            if let Some(target) = self.index.bin_index.get(bin_name) {
                return target.clone();
            }
        } else if fhs_path.starts_with("/usr/lib/") || fhs_path.starts_with("/lib/") {
            let lib_name = fhs_path
                .trim_start_matches("/usr/lib/")
                .trim_start_matches("/lib/");
            if let Some(target) = self.index.lib_index.get(lib_name) {
                return target.clone();
            }
        } else if fhs_path.starts_with("/home/") {
            let user_rel = fhs_path.trim_start_matches("/home/");
            return format!("{}/{}", self.users_root, user_rel);
        }

        fhs_path.to_string()
    }

    pub fn translate_gobolinux_to_fhs(&self, gobo_path: &str) -> String {
        if gobo_path.starts_with("/System/Index/bin/") {
            let bin_name = gobo_path.trim_start_matches("/System/Index/bin/");
            return format!("/usr/bin/{}", bin_name);
        } else if gobo_path.starts_with("/System/Index/lib/") {
            let lib_name = gobo_path.trim_start_matches("/System/Index/lib/");
            return format!("/usr/lib/{}", lib_name);
        } else if gobo_path.starts_with("/Users/") {
            let user_rel = gobo_path.trim_start_matches("/Users/");
            return format!("/home/{}", user_rel);
        }

        gobo_path.to_string()
    }

    pub fn resolve_symlink(&self, virtual_path: &str) -> Option<String> {
        if virtual_path.contains("/Current/") {
            for (app_name, entry) in &self.programs {
                let current_prefix = format!("/Programs/{}/Current/", app_name);
                if virtual_path.starts_with(&current_prefix) {
                    let subpath = virtual_path.trim_start_matches(&current_prefix);
                    return Some(format!(
                        "/Programs/{}/{}/{}",
                        app_name, entry.current_version, subpath
                    ));
                }
            }
        }
        None
    }
}

impl Default for GoboLinuxSymlinkHierarchyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gobolinux_symlink_hierarchy() {
        let mut gobo = GoboLinuxSymlinkHierarchyEngine::new();

        // Check defaults
        assert!(gobo.programs.contains_key("GCC"));
        assert!(gobo.programs.contains_key("Bash"));

        // Translate FHS /usr/bin/gcc -> GoboLinux path
        let gcc_gobo = gobo.translate_fhs_to_gobolinux("/usr/bin/gcc");
        assert_eq!(gcc_gobo, "/Programs/GCC/Current/bin/gcc");

        // Translate GoboLinux /System/Index/bin/gcc -> FHS
        let gcc_fhs = gobo.translate_gobolinux_to_fhs("/System/Index/bin/gcc");
        assert_eq!(gcc_fhs, "/usr/bin/gcc");

        // Resolve /Programs/GCC/Current/bin/gcc -> /Programs/GCC/13.2.0/bin/gcc
        let resolved = gobo.resolve_symlink("/Programs/GCC/Current/bin/gcc");
        assert_eq!(resolved, Some("/Programs/GCC/13.2.0/bin/gcc".to_string()));

        // Install new GCC version 14.1.0
        let gcc_v14 = GoboProgramVersion {
            version: "14.1.0".to_string(),
            binaries: vec!["gcc".to_string(), "g++".to_string()],
            libraries: vec!["libgcc_s.so".to_string()],
            headers: vec!["gcc.h".to_string()],
            shared_data: vec![],
            settings: vec![],
            variable: vec![],
        };

        gobo.install_program_version("GCC", gcc_v14);
        assert_eq!(
            gobo.programs.get("GCC").unwrap().current_version,
            "14.1.0"
        );

        let resolved_v14 = gobo.resolve_symlink("/Programs/GCC/Current/bin/gcc");
        assert_eq!(resolved_v14, Some("/Programs/GCC/14.1.0/bin/gcc".to_string()));

        // Switch back to 13.2.0
        assert!(gobo.switch_current_version("GCC", "13.2.0").is_ok());
        let resolved_v13 = gobo.resolve_symlink("/Programs/GCC/Current/bin/gcc");
        assert_eq!(resolved_v13, Some("/Programs/GCC/13.2.0/bin/gcc".to_string()));
    }

    #[test]
    fn test_gobolinux_users_and_data_translation() {
        let gobo = GoboLinuxSymlinkHierarchyEngine::new();
        assert_eq!(
            gobo.translate_fhs_to_gobolinux("/home/alice/docs"),
            "/Users/alice/docs"
        );
        assert_eq!(
            gobo.translate_gobolinux_to_fhs("/Users/bob/code"),
            "/home/bob/code"
        );
    }
}
