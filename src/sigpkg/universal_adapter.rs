extern crate alloc;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::klib::collections::HashMap;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::security::Permission;
use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};

/// Debian-style package priority levels (DFSG and APT standard)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackagePriority {
    Optional = 0,
    Standard = 1,
    Important = 2,
    Required = 3,
    Essential = 4, // Systems block removing these (e.g. init, libc, kernel)
}

pub trait PackageFormatAdapter {
    fn format_name(&self) -> &str;
    fn parse_manifest(&self, raw: &[u8]) -> Result<Package, String>;
    fn parse_package(&self, raw: &[u8]) -> Result<Package, String> { self.parse_manifest(raw) }
    fn validate_permissions(&self, raw: &[u8]) -> Result<Vec<Permission>, String>;
    fn validate(&self, _raw: &[u8]) -> Result<bool, String> { Ok(true) }
    fn process_hook(&self, _hook: &str) -> Result<(), String> { Ok(()) }
    fn serialize_package(&self, _pkg: &Package) -> Result<Vec<u8>, String> { Ok(Vec::new()) }
}

#[derive(Debug, Clone)]
pub struct AptDebManifest {
    pub package: String,
    pub version: String,
    pub depends: Vec<String>,
    pub description: String,
    pub priority: PackagePriority,
}

#[derive(Debug, Clone)]
pub struct PacmanPkgbuild {
    pub pkgname: String,
    pub pkgver: String,
    pub depends: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SnapcraftManifest {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub confinement: String, // "strict", "classic", "devmode"
    pub plugs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FlatpakManifest {
    pub id: String,
    pub app_id: String,
    pub command: String,
    pub finish_args: Vec<String>, // Sandboxed permissions like "--share=network", "--share=ipc"
}

pub struct UniversalPackageAdapter;

impl UniversalPackageAdapter {
    pub fn new() -> Self {
        UniversalPackageAdapter
    }

    /// Parses raw Debian control file text (Apt)
    pub fn parse_apt_control(&self, text: &str) -> Result<AptDebManifest, &'static str> {
        let mut package = String::new();
        let mut version = String::new();
        let mut depends = Vec::new();
        let mut description = String::new();
        let mut priority = PackagePriority::Optional;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                match key {
                    "Package" => package = val.to_string(),
                    "Version" => version = val.to_string(),
                    "Depends" => {
                        for dep in val.split(',') {
                            depends.push(dep.trim().to_string());
                        }
                    }
                    "Description" => description = val.to_string(),
                    "Priority" => {
                        priority = match val.to_lowercase().as_str() {
                            "essential" => PackagePriority::Essential,
                            "required" => PackagePriority::Required,
                            "important" => PackagePriority::Important,
                            "standard" => PackagePriority::Standard,
                            _ => PackagePriority::Optional,
                        };
                    }
                    _ => {}
                }
            }
        }

        if package.is_empty() || version.is_empty() {
            return Err("Invalid Debian control manifest: missing Package or Version");
        }

        Ok(AptDebManifest {
            package,
            version,
            depends,
            description,
            priority,
        })
    }

    /// Parses raw PKGBUILD script text (Pacman)
    pub fn parse_pacman_pkgbuild(&self, text: &str) -> Result<PacmanPkgbuild, &'static str> {
        let mut pkgname = String::new();
        let mut pkgver = String::new();
        let mut depends = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with("pkgname=") {
                pkgname = line["pkgname=".len()..]
                    .trim_matches(|c| c == '"' || c == '\'' || c == ' ')
                    .to_string();
            } else if line.starts_with("pkgver=") {
                pkgver = line["pkgver=".len()..]
                    .trim_matches(|c| c == '"' || c == '\'' || c == ' ')
                    .to_string();
            } else if line.starts_with("depends=") {
                let dep_content =
                    line["depends=".len()..].trim_matches(|c| c == '(' || c == ')' || c == ' ');
                for dep in dep_content.split_whitespace() {
                    let cleaned = dep.trim_matches(|c| c == '\'' || c == '"');
                    depends.push(cleaned.to_string());
                }
            }
        }

        if pkgname.is_empty() || pkgver.is_empty() {
            return Err("Invalid PKGBUILD: missing pkgname or pkgver");
        }

        Ok(PacmanPkgbuild {
            pkgname,
            pkgver,
            depends,
        })
    }

    /// Parses raw snapcraft.yaml text (Snap)
    pub fn parse_snapcraft_yaml(&self, text: &str) -> Result<SnapcraftManifest, &'static str> {
        let mut name = String::new();
        let mut version = String::new();
        let mut summary = String::new();
        let mut confinement = String::new();
        let mut plugs = Vec::new();

        let mut in_plugs_block = false;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                in_plugs_block = false;
                match key {
                    "name" => name = val.trim_matches(|c| c == '"' || c == '\'').to_string(),
                    "version" => version = val.trim_matches(|c| c == '"' || c == '\'').to_string(),
                    "summary" => summary = val.trim_matches(|c| c == '"' || c == '\'').to_string(),
                    "confinement" => confinement = val.to_string(),
                    "plugs" => {
                        in_plugs_block = true;
                        if !val.is_empty() {
                            plugs.push(val.to_string());
                        }
                    }
                    _ => {}
                }
            } else if line.starts_with("- ") && in_plugs_block {
                let plug_name = line["- ".len()..].trim();
                plugs.push(plug_name.to_string());
            }
        }

        if name.is_empty() || version.is_empty() {
            return Err("Invalid snapcraft.yaml: missing name or version");
        }

        Ok(SnapcraftManifest {
            name,
            version,
            summary,
            confinement,
            plugs,
        })
    }

    /// Parses raw Flatpak JSON manifest text
    pub fn parse_flatpak_json(&self, text: &str) -> Result<FlatpakManifest, &'static str> {
        let mut app_id = String::new();
        let mut command = String::new();
        let mut finish_args = Vec::new();

        let mut in_finish_args = false;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            if line.starts_with("\"app-id\"") {
                if let Some(pos) = line.find(':') {
                    app_id = line[pos + 1..]
                        .trim_matches(|c| c == ',' || c == '"' || c == ' ' || c == '\n')
                        .to_string();
                }
            } else if line.starts_with("\"command\"") {
                if let Some(pos) = line.find(':') {
                    command = line[pos + 1..]
                        .trim_matches(|c| c == ',' || c == '"' || c == ' ' || c == '\n')
                        .to_string();
                }
            } else if line.starts_with("\"finish-args\"") {
                in_finish_args = true;
            } else if line.starts_with(']') {
                in_finish_args = false;
            } else if in_finish_args && line.starts_with('"') {
                let arg = line
                    .trim_matches(|c| c == ',' || c == '"' || c == ' ' || c == '\n')
                    .to_string();
                finish_args.push(arg);
            }
        }

        if app_id.is_empty() {
            return Err("Invalid Flatpak JSON: missing app-id");
        }

        Ok(FlatpakManifest {
            id: app_id.clone(),
            app_id,
            command,
            finish_args,
        })
    }

    /// Translates sandboxed containerized permissions (Flatpak/Snap) into SigmaOS native Capability permissions
    pub fn translate_sandbox_permissions(&self, plugs_or_args: &[String]) -> Vec<Permission> {
        let mut permissions = Vec::new();
        for arg in plugs_or_args {
            if arg == "network" || arg == "network-bind" || arg == "--share=network" {
                permissions.push(Permission::NetworkTcp);
                permissions.push(Permission::NetworkUdp);
            } else if arg == "home" || arg == "--filesystem=home" || arg == "--filesystem=host" {
                permissions.push(Permission::FileRead);
                permissions.push(Permission::FileWrite);
            } else if arg == "--share=ipc" {
                permissions.push(Permission::Ipc);
            }
        }
        permissions
    }

    /// Standardizes any foreign parsed manifest into SigmaOS native Package models
    pub fn translate_to_native_package(
        &self,
        name: &str,
        version_str: &str,
        desc: &str,
        raw_deps: &[String],
    ) -> Result<Package, &'static str> {
        let cleaned_ver = if version_str.contains('-') {
            version_str.split('-').next().unwrap()
        } else {
            version_str
        };

        let parsed_ver =
            Version::parse(cleaned_ver).map_err(|_| "Failed to parse semver representation")?;

        let mut dependencies = Vec::new();
        for dep in raw_deps {
            dependencies.push(Dependency {
                name: dep.clone(),
                version_constraint: VersionConstraint::Any,
            });
        }

        Ok(Package {
            name: name.to_string(),
            version: parsed_ver,
            description: desc.to_string(),
            dependencies,
            checksum: format!("SHA256:{}", name),
        })
    }
}

impl Default for UniversalPackageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// RedHat/Yum RPM SPEC manifest structure
#[derive(Debug, Clone)]
pub struct RpmSpecManifest {
    pub name: String,
    pub version: String,
    pub release: String,
    pub summary: String,
    pub license: String,
    pub requires: Vec<String>, // Dependencies list
}

/// AppImage single-file containerized loop-mounted layout
#[derive(Debug, Clone)]
pub struct AppImageContainer {
    pub file_name: String,
    pub payload_offset_bytes: u64,
    pub entry_point_cmd: String,
    pub mounted: bool,
}

impl AppImageContainer {
    pub fn new(file_name: &str, entry_point_cmd: &str) -> Self {
        AppImageContainer {
            file_name: file_name.to_string(),
            payload_offset_bytes: 0x20000, // standard SquashFS offset
            entry_point_cmd: entry_point_cmd.to_string(),
            mounted: false,
        }
    }

    /// Mounts the SquashFS payload of the AppImage dynamically (simulated)
    pub fn mount_and_run(&mut self, mount_point: &str) -> Result<String, &'static str> {
        if mount_point.is_empty() {
            return Err("AppImage: Invalid mount point.");
        }
        self.mounted = true;
        let mut exec_path = mount_point.to_string();
        exec_path.push_str("/");
        exec_path.push_str(&self.entry_point_cmd);
        Ok(exec_path)
    }
}

impl UniversalPackageAdapter {
    /// Parses RedHat/Yum .spec files for RPM metadata translation
    pub fn parse_rpm_spec(&self, text: &str) -> Result<RpmSpecManifest, &'static str> {
        let mut name = String::new();
        let mut version = String::new();
        let mut release = String::new();
        let mut summary = String::new();
        let mut license = String::new();
        let mut requires = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                match key {
                    "Name" => name = val.to_string(),
                    "Version" => version = val.to_string(),
                    "Release" => release = val.to_string(),
                    "Summary" => summary = val.to_string(),
                    "License" => license = val.to_string(),
                    "Requires" => {
                        for req in val.split(',') {
                            requires.push(req.trim().to_string());
                        }
                    }
                    _ => {}
                }
            }
        }

        if name.is_empty() || version.is_empty() {
            return Err("Invalid RPM spec file: missing Name or Version");
        }

        Ok(RpmSpecManifest {
            name,
            version,
            release,
            summary,
            license,
            requires,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apt_control_parsing_and_translation() {
        let adapter = UniversalPackageAdapter::new();
        let manifest_text = r#"
            Package: curl
            Version: 8.2.1
            Depends: libcurl4, libssl3, zlib1g
            Description: Command line tool for transferring data
            Priority: standard
        "#;

        let parsed = adapter.parse_apt_control(manifest_text).unwrap();
        assert_eq!(parsed.package, "curl");
        assert_eq!(parsed.version, "8.2.1");
        assert_eq!(parsed.depends.len(), 3);
        assert_eq!(parsed.priority, PackagePriority::Standard);

        // Test parsing system essential priority (Debian-style)
        let essential_text = r#"
            Package: sigma-init
            Version: 1.0.0
            Priority: essential
        "#;
        let parsed_essential = adapter.parse_apt_control(essential_text).unwrap();
        assert_eq!(parsed_essential.priority, PackagePriority::Essential);

        let native = adapter
            .translate_to_native_package(
                &parsed.package,
                &parsed.version,
                &parsed.description,
                parsed.depends.as_slice(),
            )
            .unwrap();
        assert_eq!(native.name, "curl");
        assert_eq!(native.version, Version::new(8, 2, 1));
    }

    #[test]
    fn test_pacman_pkgbuild_parsing() {
        let adapter = UniversalPackageAdapter::new();
        let pkgbuild_text = r#"
            # Maintainer: Sigma Team
            pkgname=nginx
            pkgver=1.25.1
            depends=('openssl' 'zlib' 'pcre')
        "#;

        let parsed = adapter.parse_pacman_pkgbuild(pkgbuild_text).unwrap();
        assert_eq!(parsed.pkgname, "nginx");
        assert_eq!(parsed.pkgver, "1.25.1");
        assert_eq!(parsed.depends.len(), 3);
        assert_eq!(parsed.depends[0], "openssl");
    }

    #[test]
    fn test_snap_manifest_and_permissions() {
        let adapter = UniversalPackageAdapter::new();
        let yaml_text = r#"
            name: vlc
            version: '3.0.18'
            summary: VLC media player
            confinement: strict
            plugs:
              - network
              - home
        "#;

        let parsed = adapter.parse_snapcraft_yaml(yaml_text).unwrap();
        assert_eq!(parsed.name, "vlc");
        assert_eq!(parsed.version, "3.0.18");
        assert_eq!(parsed.plugs.len(), 2);

        // Verify container permissions map perfectly to SigmaOS capability permissions
        let perms = adapter.translate_sandbox_permissions(parsed.plugs.as_slice());
        assert!(perms.contains(&Permission::NetworkTcp));
        assert!(perms.contains(&Permission::FileRead));
    }

    #[test]
    fn test_flatpak_manifest_and_permissions() {
        let adapter = UniversalPackageAdapter::new();
        let json_text = r#"
            {
                "app-id": "org.mozilla.Firefox",
                "command": "firefox",
                "finish-args": [
                    "--share=ipc",
                    "--share=network",
                    "--filesystem=home"
                ]
            }
        "#;

        let parsed = adapter.parse_flatpak_json(json_text).unwrap();
        assert_eq!(parsed.app_id, "org.mozilla.Firefox");
        assert_eq!(parsed.finish_args.len(), 3);

        let perms = adapter.translate_sandbox_permissions(parsed.finish_args.as_slice());
        assert!(perms.contains(&Permission::Ipc));
        assert!(perms.contains(&Permission::NetworkTcp));
        assert!(perms.contains(&Permission::FileWrite));
    }

    #[test]
    fn test_rpm_spec_parsing_and_native_translation() {
        let adapter = UniversalPackageAdapter::new();
        let spec_text = r#"
            Name: custom_service
            Version: 2.1
            Release: 1%{?dist}
            Summary: High performance backend service
            License: GPL-3.0
            Requires: bash, glibc >= 2.17
        "#;

        let parsed = adapter.parse_rpm_spec(spec_text).unwrap();
        assert_eq!(parsed.name, "custom_service");
        assert_eq!(parsed.version, "2.1");
        assert_eq!(parsed.license, "GPL-3.0");
        assert_eq!(parsed.requires.len(), 2);
        assert_eq!(parsed.requires[0], "bash");

        let native = adapter.translate_to_native_package(
            &parsed.name,
            &parsed.version,
            &parsed.summary,
            parsed.requires.as_slice(),
        ).unwrap();

        assert_eq!(native.name, "custom_service");
        assert_eq!(native.version, Version::new(2, 1, 0));
    }

    #[test]
    fn test_appimage_single_file_loop_mounting() {
        let mut appimage = AppImageContainer::new("Vlc-3.0.18-x86_64.AppImage", "vlc");
        assert!(!appimage.mounted);

        let exec_path = appimage.mount_and_run("/tmp/.mount_vlc").unwrap();
        assert_eq!(exec_path, "/tmp/.mount_vlc/vlc");
        assert!(appimage.mounted);
    }
}
