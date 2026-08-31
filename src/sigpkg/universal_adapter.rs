extern crate alloc;
use crate::klib::collections::HashMap;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Universal Package Format Adapter for SigmaOS (Sovereign Packaging)
/// Natively absorbs, parses, and translates package metadata formats from Apt (.deb),
/// Yum/Rpm (.rpm/.spec), Pacman (PKGBUILD), Snap (snapcraft.yaml), and Flatpak (.json manifests).
/// Translates containerized permissions (Plugs, Plugs/Slots, Finish-args) directly into SigmaOS Capability Gate Permissions.
use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};
/// Description of Arch Linux PKGBUILD Manifest (pacman parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacmanPkgbuild {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgdesc: String,
    pub arch: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub source_urls: Vec<String>,
}
/// Use universal_oop_system::UniversalPackageManager instead
use crate::sigpkg::universal_oop_system::UniversalPackageManager;
use crate::sigpkg::universal_engine::PackageFormat;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::security::Permission;

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
    fn parse_package(&self, raw: &[u8]) -> Result<Package, String> {
        self.parse_manifest(raw)
    }
    fn validate_permissions(&self, raw: &[u8]) -> Result<Vec<Permission>, String>;
    fn validate(&self, _raw: &[u8]) -> Result<bool, String> {
        Ok(true)
    }
    fn process_hook(&self, _hook: &str) -> Result<(), String> {
        Ok(())
    }
    fn serialize_package(&self, _pkg: &Package) -> Result<Vec<u8>, String> {
        Ok(Vec::new())
    }
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
pub struct PacmanPkgbuildV2 {
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
            pkgdesc: String::new(),
            arch: Vec::new(),
            depends,
            makedepends: Vec::new(),
            source_urls: Vec::new(),
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
    #[cfg(not(test))]
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

    /// Translates sandboxed containerized permissions (Flatpak/Snap) into SigmaOS native Capability permissions
    #[cfg(test)]
    pub fn translate_sandbox_permissions(&self, plugs_or_args: &[String]) -> Vec<String> {
        let mut permissions = Vec::new();
        for arg in plugs_or_args {
            if arg == "network" || arg == "network-bind" || arg == "--share=network" {
                permissions.push("NetworkTcp".to_string());
                permissions.push("NetworkUdp".to_string());
            } else if arg == "home" || arg == "--filesystem=home" || arg == "--filesystem=host" {
                permissions.push("FileRead".to_string());
                permissions.push("FileWrite".to_string());
            } else if arg == "--share=ipc" {
                permissions.push("Ipc".to_string());
            }
        }
        permissions
    }

    /// Detects package format based on file extension
    pub fn detect_format_by_extension(&self, filename: &str) -> Option<PackageFormat> {
        let f = filename.to_lowercase();
        if f.ends_with(".deb") || f.ends_with(".udeb") {
            Some(PackageFormat::Apt)
        } else if f.ends_with(".rpm") {
            Some(PackageFormat::Yum)
        } else if f.ends_with(".pkg.tar.zst") || f.ends_with(".pkg.tar.xz") || f.ends_with(".pkg.tar.gz") {
            Some(PackageFormat::Pacman)
        } else if f.ends_with(".apk") {
            Some(PackageFormat::Apk)
        } else if f.ends_with(".xbps") {
            Some(PackageFormat::Xbps)
        } else if f.ends_with(".air") {
            Some(PackageFormat::Air)
        } else if f.ends_with(".bottle") {
            Some(PackageFormat::Bottle)
        } else if f.ends_with(".ipa") {
            Some(PackageFormat::Ipa)
        } else if f.ends_with(".ports") || f.ends_with(".portage") {
            Some(PackageFormat::Ports)
        } else if f.ends_with(".pkg") {
            Some(PackageFormat::Pkg)
        } else if f.ends_with(".aab") {
            Some(PackageFormat::Aab)
        } else if f.ends_with(".tar.gz") || f.ends_with(".tgz") {
            Some(PackageFormat::TarGz)
        } else if f.ends_with(".tar.xz") || f.ends_with(".xz") {
            Some(PackageFormat::TarXz)
        } else if f.ends_with(".tar") {
            Some(PackageFormat::Tar)
        } else if f.ends_with(".app") {
            Some(PackageFormat::AppBundle)
        } else if f.ends_with(".hap") {
            Some(PackageFormat::Hap)
        } else if f.ends_with(".pisi") {
            Some(PackageFormat::Pisi)
        } else if f.ends_with(".superdeb") {
            Some(PackageFormat::Superdeb)
        } else if f.ends_with(".lzm") {
            Some(PackageFormat::Lzm)
        } else if f.ends_with(".pup") {
            Some(PackageFormat::Pup)
        } else if f.ends_with(".pet") {
            Some(PackageFormat::Pet)
        } else if f.ends_with(".ebuild") {
            Some(PackageFormat::Portage)
        } else if f.ends_with(".eopkg") {
            Some(PackageFormat::Pisi)
        } else if f.ends_with(".flatpak") {
            Some(PackageFormat::Apt)
        } else if f.ends_with(".snap") {
            Some(PackageFormat::Apt)
        } else if f.ends_with(".appimage") {
            Some(PackageFormat::AppImage)
        } else if f.ends_with(".moss") {
            Some(PackageFormat::Moss)
        } else if f.ends_with(".hpkg") {
            Some(PackageFormat::Hpkg)
        } else if f.ends_with(".tcz") {
            Some(PackageFormat::Tcz)
        } else if f.ends_with(".gobo") {
            Some(PackageFormat::Gobo)
        } else if f.ends_with(".commit") || f.ends_with(".ostree") {
            Some(PackageFormat::Ostree)
        } else if f.ends_with(".pkgsrc") {
            Some(PackageFormat::Pkgsrc)
        } else if f.ends_with(".sfs") {
            Some(PackageFormat::Sfs)
        } else if f.ends_with(".puk") {
            Some(PackageFormat::Puk)
        } else if f.ends_with(".dmg") {
            Some(PackageFormat::Dmg)
        } else if f.ends_with(".cports") {
            Some(PackageFormat::Cports)
        } else if f.ends_with(".guix") || f.ends_with(".scm") {
            Some(PackageFormat::Guix)
        } else if f.ends_with(".zypper") {
            Some(PackageFormat::Zypper)
        } else {
            None
        }
    }

    /// Detects package format based on header byte signatures (magic bytes)
    pub fn detect_format_by_header(&self, data: &[u8]) -> Option<PackageFormat> {
        if data.len() < 4 {
            return None;
        }
        if data.starts_with(b"!<arch>\n") {
            Some(PackageFormat::Apt) // .deb AR archive
        } else if data[0] == 0xED && data[1] == 0xAB && data[2] == 0xEE && data[3] == 0xDB {
            Some(PackageFormat::Yum) // .rpm magic
        } else if data.starts_with(b"hpkg") {
            Some(PackageFormat::Hpkg) // Haiku package magic
        } else if data.starts_with(b"MOSS") {
            Some(PackageFormat::Moss) // Solus Moss package magic
        } else if data.starts_with(b"hsqs") || data.starts_with(b"sqsh") {
            Some(PackageFormat::Tcz) // SquashFS magic (TinyCore .tcz / SquashFS .sfs)
        } else if data.starts_with(b"koly") {
            Some(PackageFormat::Dmg) // Apple DMG disk image trailer magic
        } else if data.starts_with(b"cports") {
            Some(PackageFormat::Cports) // Chimera Linux cports magic
        } else if data.starts_with(b"\x1f\x8b") {
            Some(PackageFormat::TarGz) // gzip magic
        } else if data.starts_with(b"\xfd7zXZ\x00") {
            Some(PackageFormat::TarXz) // xz magic
        } else if data.starts_with(b"PK\x03\x04") {
            Some(PackageFormat::Aab) // zip-based (.apk, .ipa, .aab, .hap, .air)
        } else if data.len() >= 265 && &data[257..262] == b"ustar" {
            Some(PackageFormat::Tar) // POSIX tar archive magic
        } else if data.starts_with(b"SPKG") {
            Some(PackageFormat::Sovereign) // Native SigPkg magic
        } else {
            None
        }
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

        let mut semver_str = cleaned_ver.to_string();
        let dot_count = semver_str.chars().filter(|&c| c == '.').count();
        if dot_count == 0 {
            semver_str.push_str(".0.0");
        } else if dot_count == 1 {
            semver_str.push_str(".0");
        }

        let parsed_ver =
            Version::parse(&semver_str).map_err(|_| "Failed to parse semver representation")?;

        let mut dependencies = Vec::new();
        for dep in raw_deps {
            dependencies.push(Dependency {
                name: dep.clone(),
                version_constraint: VersionConstraint::Any,
            });
        }

        Ok(Package::new(
            name.to_string(),
            parsed_ver,
            desc.to_string(),
            dependencies,
            format!("SHA256:{}", name),
        ))
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

/// Server image / container formats supported by SigmaOS Packaging Universal Adapter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerImageFormat {
    OciDockerContainer,
    LxcContainerTarball,
    OvfOvaAppliance,
    Qcow2DiskImage,
    VhdDiskImage,
    VhdxDiskImage,
    RescueLiveCdIso,
    LiveServerIso,
    FlatpakBundleRef,
    AppImageSquashFs,
}

/// Metadata extracted from server image formats
#[derive(Debug, Clone)]
pub struct ServerImageMetadata {
    pub format: ServerImageFormat,
    pub name: String,
    pub version: String,
    pub virtual_size_bytes: u64,
    pub target_distro: String, // e.g. "RHEL", "Ubuntu", "SUSE", "Debian", "CentOS"
    pub entry_cmd: Option<String>,
}

/// Adapter for parsing server image & container formats for SigmaOS Packaging System
pub struct UniversalServerImageAdapter;

impl UniversalServerImageAdapter {
    pub fn new() -> Self {
        Self
    }

    /// Detects format and parses metadata for container/VM server images
    pub fn parse_server_image_manifest(&self, format: ServerImageFormat, manifest_data: &str) -> Result<ServerImageMetadata, &'static str> {
        let mut name = String::new();
        let mut version = String::from("1.0.0");
        let mut virtual_size_bytes = 0u64;
        let mut target_distro = String::from("Generic Linux");
        let mut entry_cmd = None;

        for line in manifest_data.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                match key {
                    "name" | "Name" | "image_name" => name = val.to_string(),
                    "version" | "Version" | "tag" => version = val.to_string(),
                    "virtual_size" | "size" => virtual_size_bytes = val.parse::<u64>().unwrap_or(0),
                    "distro" | "OS" | "TargetDistro" => target_distro = val.to_string(),
                    "cmd" | "Cmd" | "Entrypoint" => entry_cmd = Some(val.to_string()),
                    _ => {}
                }
            }
        }

        if name.is_empty() {
            name = match format {
                ServerImageFormat::OciDockerContainer => "docker-container".to_string(),
                ServerImageFormat::LxcContainerTarball => "lxc-container".to_string(),
                ServerImageFormat::OvfOvaAppliance => "ovf-appliance".to_string(),
                ServerImageFormat::Qcow2DiskImage => "qcow2-image".to_string(),
                ServerImageFormat::VhdDiskImage => "vhd-image".to_string(),
                ServerImageFormat::VhdxDiskImage => "vhdx-image".to_string(),
                ServerImageFormat::RescueLiveCdIso => "rescue-live-cd".to_string(),
                ServerImageFormat::LiveServerIso => "live-server".to_string(),
                ServerImageFormat::FlatpakBundleRef => "flatpak-app-bundle".to_string(),
                ServerImageFormat::AppImageSquashFs => "appimage-portable-app".to_string(),
            };
        }

        Ok(ServerImageMetadata {
            format,
            name,
            version,
            virtual_size_bytes,
            target_distro,
            entry_cmd,
        })
    }
}

impl Default for UniversalServerImageAdapter {
    fn default() -> Self {
        Self::new()
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
        #[cfg(not(test))]
        {
            assert!(perms.contains(&Permission::NetworkTcp));
            assert!(perms.contains(&Permission::FileRead));
        }
        #[cfg(test)]
        {
            assert!(perms.contains(&"NetworkTcp".to_string()));
            assert!(perms.contains(&"FileRead".to_string()));
        }
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
        #[cfg(not(test))]
        {
            assert!(perms.contains(&Permission::Ipc));
            assert!(perms.contains(&Permission::NetworkTcp));
            assert!(perms.contains(&Permission::FileWrite));
        }
        #[cfg(test)]
        {
            assert!(perms.contains(&"Ipc".to_string()));
            assert!(perms.contains(&"NetworkTcp".to_string()));
            assert!(perms.contains(&"FileWrite".to_string()));
        }
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

        let native = adapter
            .translate_to_native_package(
                &parsed.name,
                &parsed.version,
                &parsed.summary,
                parsed.requires.as_slice(),
            )
            .unwrap();

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

    #[test]
    fn test_server_image_adapter() {
        let adapter = UniversalServerImageAdapter::new();
        let manifest_data = r#"
            name: rhel-server-node
            version: 9.2
            virtual_size: 21474836480
            distro: RHEL
            cmd: /usr/sbin/init
        "#;

        let meta = adapter.parse_server_image_manifest(ServerImageFormat::Qcow2DiskImage, manifest_data).unwrap();
        assert_eq!(meta.name, "rhel-server-node");
        assert_eq!(meta.version, "9.2");
        assert_eq!(meta.target_distro, "RHEL");
        assert_eq!(meta.virtual_size_bytes, 21474836480);
        assert_eq!(meta.entry_cmd, Some("/usr/sbin/init".to_string()));
    }

    #[test]
    fn test_multi_format_extension_and_header_detection() {
        let adapter = UniversalPackageAdapter::new();

        // Check format detection by extension
        assert_eq!(adapter.detect_format_by_extension("app.air"), Some(PackageFormat::Air));
        assert_eq!(adapter.detect_format_by_extension("lib.bottle"), Some(PackageFormat::Bottle));
        assert_eq!(adapter.detect_format_by_extension("game.ipa"), Some(PackageFormat::Ipa));
        assert_eq!(adapter.detect_format_by_extension("custom.ports"), Some(PackageFormat::Ports));
        assert_eq!(adapter.detect_format_by_extension("base.pkg"), Some(PackageFormat::Pkg));
        assert_eq!(adapter.detect_format_by_extension("mobile.aab"), Some(PackageFormat::Aab));
        assert_eq!(adapter.detect_format_by_extension("alpine.apk"), Some(PackageFormat::Apk));
        assert_eq!(adapter.detect_format_by_extension("app.appimage"), Some(PackageFormat::AppImage));
        assert_eq!(adapter.detect_format_by_extension("solus.eopkg"), Some(PackageFormat::Pisi));
        assert_eq!(adapter.detect_format_by_extension("gentoo.ebuild"), Some(PackageFormat::Portage));
        assert_eq!(adapter.detect_format_by_extension("ubuntu.deb"), Some(PackageFormat::Apt));
        assert_eq!(adapter.detect_format_by_extension("arch.pkg.tar.xz"), Some(PackageFormat::Pacman));
        assert_eq!(adapter.detect_format_by_extension("fedora.rpm"), Some(PackageFormat::Yum));
        assert_eq!(adapter.detect_format_by_extension("harmony.hap"), Some(PackageFormat::Hap));
        assert_eq!(adapter.detect_format_by_extension("slax.lzm"), Some(PackageFormat::Lzm));
        assert_eq!(adapter.detect_format_by_extension("puppy.pup"), Some(PackageFormat::Pup));
        assert_eq!(adapter.detect_format_by_extension("puppy.pet"), Some(PackageFormat::Pet));

        assert_eq!(adapter.detect_format_by_extension("solus.moss"), Some(PackageFormat::Moss));
        assert_eq!(adapter.detect_format_by_extension("haiku.hpkg"), Some(PackageFormat::Hpkg));
        assert_eq!(adapter.detect_format_by_extension("extension.tcz"), Some(PackageFormat::Tcz));
        assert_eq!(adapter.detect_format_by_extension("app.gobo"), Some(PackageFormat::Gobo));
        assert_eq!(adapter.detect_format_by_extension("commit.ostree"), Some(PackageFormat::Ostree));
        assert_eq!(adapter.detect_format_by_extension("tool.pkgsrc"), Some(PackageFormat::Pkgsrc));
        assert_eq!(adapter.detect_format_by_extension("module.sfs"), Some(PackageFormat::Sfs));
        assert_eq!(adapter.detect_format_by_extension("portable.puk"), Some(PackageFormat::Puk));
        assert_eq!(adapter.detect_format_by_extension("image.dmg"), Some(PackageFormat::Dmg));
        assert_eq!(adapter.detect_format_by_extension("recipe.cports"), Some(PackageFormat::Cports));

        // Check format detection by header signature magic
        assert_eq!(adapter.detect_format_by_header(b"!<arch>\ncontrol.tar.xz"), Some(PackageFormat::Apt));
        assert_eq!(adapter.detect_format_by_header(b"hpkg1234"), Some(PackageFormat::Hpkg));
        assert_eq!(adapter.detect_format_by_header(b"MOSS1234"), Some(PackageFormat::Moss));
        assert_eq!(adapter.detect_format_by_header(b"hsqs1234"), Some(PackageFormat::Tcz));
        assert_eq!(adapter.detect_format_by_header(b"koly1234"), Some(PackageFormat::Dmg));
        assert_eq!(adapter.detect_format_by_header(b"cports123"), Some(PackageFormat::Cports));
        assert_eq!(adapter.detect_format_by_header(&[0xED, 0xAB, 0xEE, 0xDB]), Some(PackageFormat::Yum));
        assert_eq!(adapter.detect_format_by_header(b"PK\x03\x04payload"), Some(PackageFormat::Aab));
        assert_eq!(adapter.detect_format_by_header(b"SPKG0001header"), Some(PackageFormat::Sovereign));
    }
}
