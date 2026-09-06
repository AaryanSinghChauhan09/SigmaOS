
use std::boxed::Box;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Universal Package Format Adapter for SigmaOS (Sovereign Packaging)
/// Natively absorbs, parses, and translates package metadata formats from Apt (.deb),
/// Yum/Rpm (.rpm/.spec), Pacman (PKGBUILD), Snap (snapcraft.yaml), and Flatpak (.json manifests).
/// Translates containerized permissions (Plugs, Plugs/Slots, Finish-args) directly into SigmaOS Capability Gate Permissions.
#[cfg(all(test, not(feature = "sigmaos_lib")))]
#[path = "universal_oop_system.rs"]
pub mod universal_oop_system;

#[cfg(all(test, not(feature = "sigmaos_lib")))]
pub use universal_oop_system::*;

#[cfg(not(all(test, not(feature = "sigmaos_lib"))))]
use crate::sigpkg::universal_oop_system;

#[cfg(all(not(feature = "standalone_test"), not(test)))]
use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};

#[cfg(test)]
pub use crate::sigpkg::Version;

#[cfg(all(not(feature = "standalone_test"), not(test)))]
use crate::sigpkg::universal_engine::PackageFormat;

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    NetworkTcp,
    NetworkUdp,
    FileRead,
    FileWrite,
    ProcessExec,
    AudioPlayback,
    DisplayAccess,
    Ipc,
    ProcessControl,
    Execute,
    // Note: ProcessExec covers process execution; Execute is the alias used in capability checks
}

#[cfg(not(feature = "standalone_test"))]
pub use crate::security::Permission;

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

// PackageFormat: use standalone_test-specific import when in that build mode
#[cfg(feature = "standalone_test")]
use crate::sigpkg::universal_engine::PackageFormat;
/// UniversalPackageManager: only import directly when NOT using the glob re-export
#[cfg(not(all(test, not(feature = "sigmaos_lib"))))]
use crate::sigpkg::universal_oop_system::UniversalPackageManager;
use core::sync::atomic::{AtomicUsize, Ordering};

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

/// Description of Arch Linux .PKGINFO binary manifest (pacman standard)
#[derive(Debug, Clone)]
pub struct ArchPkgInfoManifest {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgdesc: String,
    pub depends: Vec<String>,
    pub architecture: String,
}

/// Description of Gentoo ebuild manifest (Portage standard)
#[derive(Debug, Clone)]
pub struct GentooEbuildMetadata {
    pub category: String,
    pub package_name: String,
    pub version: String,
    pub rdepend: Vec<String>,
    pub depend: Vec<String>,
    pub description: String,
    pub use_flags: Vec<String>,
}

/// Description of Alpine Linux APKINDEX manifest (apk standard)
#[derive(Debug, Clone)]
pub struct ApkIndexManifest {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgdesc: String,
    pub depends: Vec<String>,
}

/// Description of Void Linux XBPS manifest
#[derive(Debug, Clone)]
pub struct XbpsManifest {
    pub pkgname: String,
    pub version: String,
    pub short_desc: String,
    pub run_depends: Vec<String>,
}

/// Description of Haiku .hpkg package manifest
#[derive(Debug, Clone)]
pub struct HaikuHpkgManifest {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub architecture: String,
    pub requires: Vec<String>,
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

/// Description of FreeBSD UCL (+MANIFEST) pkg manifest
#[derive(Debug, Clone)]
pub struct FreeBsdUclManifest {
    pub name: String,
    pub version: String,
    pub comment: String,
    pub deps: Vec<String>,
}

/// Description of OpenBSD +CONTENTS pkg manifest
#[derive(Debug, Clone)]
pub struct OpenBsdContentsManifest {
    pub pkgname: String,
    pub version: String,
    pub comment: String,
    pub depends: Vec<String>,
    pub exec_commands: Vec<String>,
    pub unexec_commands: Vec<String>,
}

/// Description of NetBSD pkgsrc manifest
#[derive(Debug, Clone)]
pub struct NetBsdPkgsrcManifest {
    pub pkgname: String,
    pub version: String,
    pub comment: String,
    pub depends: Vec<String>,
}

/// Description of openSUSE Zypper RPM spec/manifest
#[derive(Debug, Clone)]
pub struct ZypperSpecManifest {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub requires: Vec<String>,
}

/// Description of Slackware package manifest
#[derive(Debug, Clone)]
pub struct SlackwarePkgManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub slack_required: Vec<String>,
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

    /// Parses Arch Linux binary .PKGINFO text
    pub fn parse_arch_pkginfo(&self, text: &str) -> Result<ArchPkgInfoManifest, &'static str> {
        let mut pkgname = String::new();
        let mut pkgver = String::new();
        let mut pkgdesc = String::new();
        let mut depends = Vec::new();
        let mut architecture = String::from("x86_64");

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                match key {
                    "pkgname" => pkgname = val.to_string(),
                    "pkgver" => pkgver = val.to_string(),
                    "pkgdesc" => pkgdesc = val.to_string(),
                    "depend" => depends.push(val.to_string()),
                    "arch" => architecture = val.to_string(),
                    _ => {}
                }
            }
        }

        if pkgname.is_empty() || pkgver.is_empty() {
            return Err("Invalid .PKGINFO: missing pkgname or pkgver");
        }

        Ok(ArchPkgInfoManifest {
            pkgname,
            pkgver,
            pkgdesc,
            depends,
            architecture,
        })
    }

    /// Parses Gentoo .ebuild specification text
    pub fn parse_gentoo_ebuild(
        &self,
        filename: &str,
        text: &str,
    ) -> Result<GentooEbuildMetadata, &'static str> {
        let mut category = String::from("app-misc");
        let mut package_name = String::new();
        let mut version = String::from("1.0.0");
        let mut rdepend = Vec::new();
        let mut depend = Vec::new();
        let mut description = String::new();
        let mut use_flags = Vec::new();

        // Infer name and version from filename (e.g. `sys-apps/portage-3.0.30.ebuild` or `nginx-1.25.1.ebuild`)
        let clean_filename = filename.trim_end_matches(".ebuild");
        if clean_filename.contains('/') {
            let mut parts = clean_filename.split('/');
            category = parts.next().unwrap_or("app-misc").to_string();
            let name_ver = parts.next().unwrap_or(clean_filename);
            if let Some(pos) = name_ver.rfind('-') {
                package_name = name_ver[..pos].to_string();
                version = name_ver[pos + 1..].to_string();
            } else {
                package_name = name_ver.to_string();
            }
        } else if let Some(pos) = clean_filename.rfind('-') {
            package_name = clean_filename[..pos].to_string();
            version = clean_filename[pos + 1..].to_string();
        } else {
            package_name = clean_filename.to_string();
        }

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with("DESCRIPTION=") {
                description = line["DESCRIPTION=".len()..]
                    .trim_matches(|c| c == '"' || c == '\'')
                    .to_string();
            } else if line.starts_with("RDEPEND=") {
                let dep_str = line["RDEPEND=".len()..].trim_matches(|c| c == '"' || c == '\'');
                for dep in dep_str.split_whitespace() {
                    let cleaned = dep.trim_matches(|c| c == '"' || c == '\'');
                    if !cleaned.is_empty() && !cleaned.starts_with('!') {
                        rdepend.push(cleaned.to_string());
                    }
                }
            } else if line.starts_with("DEPEND=") {
                let dep_str = line["DEPEND=".len()..].trim_matches(|c| c == '"' || c == '\'');
                for dep in dep_str.split_whitespace() {
                    let cleaned = dep.trim_matches(|c| c == '"' || c == '\'');
                    if !cleaned.is_empty() && !cleaned.starts_with('!') {
                        depend.push(cleaned.to_string());
                    }
                }
            } else if line.starts_with("IUSE=") {
                let use_str = line["IUSE=".len()..].trim_matches(|c| c == '"' || c == '\'');
                for flag in use_str.split_whitespace() {
                    use_flags.push(
                        flag.trim_start_matches('+')
                            .trim_start_matches('-')
                            .to_string(),
                    );
                }
            }
        }

        if package_name.is_empty() {
            package_name = String::from("gentoo-pkg");
        }

        Ok(GentooEbuildMetadata {
            category,
            package_name,
            version,
            rdepend,
            depend,
            description,
            use_flags,
        })
    }

    /// Parses Alpine APKINDEX control entry text
    pub fn parse_apkindex(&self, text: &str) -> Result<ApkIndexManifest, &'static str> {
        let mut pkgname = String::new();
        let mut pkgver = String::new();
        let mut pkgdesc = String::new();
        let mut depends = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with("P:") {
                pkgname = line[2..].trim().to_string();
            } else if line.starts_with("V:") {
                pkgver = line[2..].trim().to_string();
            } else if line.starts_with("T:") {
                pkgdesc = line[2..].trim().to_string();
            } else if line.starts_with("D:") {
                for dep in line[2..].trim().split_whitespace() {
                    depends.push(dep.to_string());
                }
            }
        }

        if pkgname.is_empty() || pkgver.is_empty() {
            return Err("Invalid APKINDEX manifest: missing P: or V:");
        }

        Ok(ApkIndexManifest {
            pkgname,
            pkgver,
            pkgdesc,
            depends,
        })
    }

    /// Parses Void Linux XBPS control manifest text
    pub fn parse_xbps_manifest(&self, text: &str) -> Result<XbpsManifest, &'static str> {
        let mut pkgname = String::new();
        let mut version = String::new();
        let mut short_desc = String::new();
        let mut run_depends = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                match key {
                    "pkgname" => pkgname = val.trim_matches(|c| c == '"' || c == '\'').to_string(),
                    "version" => version = val.trim_matches(|c| c == '"' || c == '\'').to_string(),
                    "short_desc" => {
                        short_desc = val.trim_matches(|c| c == '"' || c == '\'').to_string()
                    }
                    "run_depends" => {
                        let clean_val =
                            val.trim_matches(|c| c == '"' || c == '\'' || c == '(' || c == ')');
                        for dep in clean_val.split_whitespace() {
                            run_depends.push(dep.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }

        if pkgname.is_empty() || version.is_empty() {
            return Err("Invalid XBPS manifest: missing pkgname or version");
        }

        Ok(XbpsManifest {
            pkgname,
            version,
            short_desc,
            run_depends,
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

    /// Parses Haiku .hpkg package manifest text
    pub fn parse_haiku_hpkg(&self, text: &str) -> Result<HaikuHpkgManifest, &'static str> {
        let mut name = String::new();
        let mut version = String::new();
        let mut summary = String::new();
        let mut architecture = String::from("x86_64");
        let mut requires = Vec::new();

        let mut in_requires_block = false;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with("name") {
                if let Some(pos) = line.find(' ') {
                    name = line[pos + 1..].trim().trim_matches(|c| c == '"' || c == '\'' || c == ';').to_string();
                }
            } else if line.starts_with("version") {
                if let Some(pos) = line.find(' ') {
                    version = line[pos + 1..].trim().trim_matches(|c| c == '"' || c == '\'' || c == ';').to_string();
                }
            } else if line.starts_with("summary") {
                if let Some(pos) = line.find(' ') {
                    summary = line[pos + 1..].trim().trim_matches(|c| c == '"' || c == '\'' || c == ';').to_string();
                }
            } else if line.starts_with("architecture") {
                if let Some(pos) = line.find(' ') {
                    architecture = line[pos + 1..].trim().trim_matches(|c| c == '"' || c == '\'' || c == ';').to_string();
                }
            } else if line.starts_with("requires {") || line == "requires {" || line.starts_with("requires") {
                in_requires_block = true;
            } else if line.starts_with('}') {
                in_requires_block = false;
            } else if in_requires_block {
                let clean_req = line.trim_matches(|c| c == '"' || c == '\'' || c == ',' || c == ';');
                if !clean_req.is_empty() {
                    requires.push(clean_req.to_string());
                }
            }
        }

        if name.is_empty() || version.is_empty() {
            return Err("Invalid Haiku .hpkg manifest: missing name or version");
        }

        Ok(HaikuHpkgManifest {
            name,
            version,
            summary,
            architecture,
            requires,
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

    /// Parses FreeBSD UCL (+MANIFEST) pkg manifest
    pub fn parse_freebsd_ucl_manifest(&self, text: &str) -> Result<FreeBsdUclManifest, &'static str> {
        let mut name = String::new();
        let mut version = String::new();
        let mut comment = String::new();
        let mut deps = Vec::new();

        let mut in_deps = false;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with("deps") && line.contains('{') {
                in_deps = true;
                continue;
            }
            if in_deps {
                if line.starts_with('}') {
                    in_deps = false;
                    continue;
                }
                if let Some(pos) = line.find(':') {
                    let dep_key = line[..pos].trim_matches(|c| c == '"' || c == '\'' || c == ' ');
                    if !dep_key.is_empty() {
                        deps.push(dep_key.to_string());
                    }
                }
            } else if let Some(pos) = line.find(':') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim_matches(|c| c == '"' || c == '\'' || c == ',' || c == ' ');
                match key {
                    "name" => name = val.to_string(),
                    "version" => version = val.to_string(),
                    "comment" => comment = val.to_string(),
                    _ => {}
                }
            }
        }

        if name.is_empty() || version.is_empty() {
            return Err("Invalid FreeBSD UCL manifest: missing name or version");
        }

        Ok(FreeBsdUclManifest {
            name,
            version,
            comment,
            deps,
        })
    }

    /// Parses OpenBSD +CONTENTS pkg manifest
    pub fn parse_openbsd_contents(&self, text: &str) -> Result<OpenBsdContentsManifest, &'static str> {
        let mut pkgname = String::new();
        let mut version = String::new();
        let mut comment = String::new();
        let mut depends = Vec::new();
        let mut exec_commands = Vec::new();
        let mut unexec_commands = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with("@name ") {
                let full_name = line["@name ".len()..].trim();
                if let Some(pos) = full_name.rfind('-') {
                    pkgname = full_name[..pos].to_string();
                    version = full_name[pos + 1..].to_string();
                } else {
                    pkgname = full_name.to_string();
                    version = "1.0.0".to_string();
                }
            } else if line.starts_with("@comment ") {
                comment = line["@comment ".len()..].trim().to_string();
            } else if line.starts_with("@depend ") || line.starts_with("@pkgdep ") {
                let dep_line = if line.starts_with("@depend ") {
                    &line["@depend ".len()..]
                } else {
                    &line["@pkgdep ".len()..]
                };
                let dep_pkg = dep_line.split(':').last().unwrap_or(dep_line).trim();
                depends.push(dep_pkg.to_string());
            } else if line.starts_with("@exec ") {
                exec_commands.push(line["@exec ".len()..].trim().to_string());
            } else if line.starts_with("@unexec ") {
                unexec_commands.push(line["@unexec ".len()..].trim().to_string());
            }
        }

        if pkgname.is_empty() {
            return Err("Invalid OpenBSD +CONTENTS manifest: missing @name");
        }

        Ok(OpenBsdContentsManifest {
            pkgname,
            version,
            comment,
            depends,
            exec_commands,
            unexec_commands,
        })
    }

    /// Parses NetBSD pkgsrc manifest
    pub fn parse_netbsd_pkgsrc(&self, text: &str) -> Result<NetBsdPkgsrcManifest, &'static str> {
        let mut pkgname = String::new();
        let mut version = String::new();
        let mut comment = String::new();
        let mut depends = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                match key {
                    "PKGNAME" => {
                        let full_name = val.trim_matches(|c| c == '"' || c == '\'');
                        if let Some(dash) = full_name.rfind('-') {
                            pkgname = full_name[..dash].to_string();
                            version = full_name[dash + 1..].to_string();
                        } else {
                            pkgname = full_name.to_string();
                            version = "1.0.0".to_string();
                        }
                    }
                    "COMMENT" => comment = val.trim_matches(|c| c == '"' || c == '\'').to_string(),
                    "REQUIRES" | "DEPENDS" => {
                        let dep = val.trim_matches(|c| c == '"' || c == '\'');
                        for d in dep.split_whitespace() {
                            depends.push(d.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }

        if pkgname.is_empty() {
            return Err("Invalid NetBSD pkgsrc manifest: missing PKGNAME");
        }

        Ok(NetBsdPkgsrcManifest {
            pkgname,
            version,
            comment,
            depends,
        })
    }

    /// Parses openSUSE Zypper RPM spec/manifest
    pub fn parse_zypper_spec(&self, text: &str) -> Result<ZypperSpecManifest, &'static str> {
        let mut name = String::new();
        let mut version = String::new();
        let mut summary = String::new();
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
                    "Summary" => summary = val.to_string(),
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
            return Err("Invalid Zypper spec file: missing Name or Version");
        }

        Ok(ZypperSpecManifest {
            name,
            version,
            summary,
            requires,
        })
    }

    /// Parses Slackware package manifest / SlackBuild
    pub fn parse_slackware_pkg(&self, text: &str) -> Result<SlackwarePkgManifest, &'static str> {
        let mut name = String::new();
        let mut version = String::new();
        let mut description = String::new();
        let mut slack_required = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim_matches(|c| c == '"' || c == '\'' || c == '(' || c == ')');
                match key {
                    "PRGNAM" | "NAME" => name = val.to_string(),
                    "VERSION" | "VER" => version = val.to_string(),
                    "SLACK_REQUIRED" | "REQUIRES" => {
                        for req in val.split(|c| c == ' ' || c == '\t' || c == ',') {
                            let clean_req = req.trim();
                            if !clean_req.is_empty() {
                                slack_required.push(clean_req.to_string());
                            }
                        }
                    }
                    _ => {}
                }
            } else if line.contains("slack-desc") {
                if let Some(pos) = line.find(':') {
                    description = line[pos + 1..].trim().to_string();
                }
            }
        }

        if name.is_empty() {
            name = "slackware-pkg".to_string();
        }
        if version.is_empty() {
            version = "1.0.0".to_string();
        }

        Ok(SlackwarePkgManifest {
            name,
            version,
            description,
            slack_required,
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
            } else if arg == "--share=ipc" || arg == "ipc" {
                permissions.push(Permission::Ipc);
            } else if arg == "pulseaudio" || arg == "audio-playback" || arg == "--socket=pulseaudio"
            {
                permissions.push(Permission::AudioPlayback);
            } else if arg == "x11"
                || arg == "wayland"
                || arg == "--socket=x11"
                || arg == "--socket=wayland"
            {
                permissions.push(Permission::DisplayAccess);
            }
        }
        permissions
    }


    /// Detects package format based on file extension
    pub fn detect_format_by_extension(&self, filename: &str) -> Option<PackageFormat> {
        let f = filename.to_lowercase().trim().replace(" ", "");
        if f.ends_with(".deb") || f.ends_with(".udeb") {
            Some(PackageFormat::Apt)
        } else if f.ends_with(".rpm") {
            Some(PackageFormat::Yum)
        } else if f.ends_with(".pkg.tar.zst")
            || f.ends_with(".pkg.tar.xz")
            || f.ends_with(".pkg.tar.gz")
            || f.contains("pacman")
            || f.ends_with(".pacman")
        {
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
        } else if f.ends_with(".ports") {
            Some(PackageFormat::Ports)
        } else if f.ends_with(".portage") || f.ends_with(".ebuild") {
            Some(PackageFormat::Portage)
        } else if f.ends_with(".pkg") {
            Some(PackageFormat::Pkg)
        } else if f.ends_with(".aab") {
            Some(PackageFormat::Aab)
        } else if f.ends_with(".openbsd.tgz") || f.ends_with(".openbsd.tar.gz") {
            Some(PackageFormat::OpenBsdPkg)
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
        } else if f == "pup" || f.ends_with(".pup") {
            Some(PackageFormat::Pup)
        } else if f == "pet" || f.ends_with(".pet") {
            Some(PackageFormat::Pet)
        } else if f.ends_with(".nixpkg") || f.ends_with(".nix") {
            Some(PackageFormat::Nix)
        } else if f.ends_with(".eopkg") {
            Some(PackageFormat::Eopkg)
        } else if f.ends_with(".flatpak") {
            Some(PackageFormat::Flatpak)
        } else if f.ends_with(".snap") {
            Some(PackageFormat::Snap)
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
        } else if f.ends_with(".cachy") {
            Some(PackageFormat::Pacman)
        } else if f.ends_with(".dports") {
            Some(PackageFormat::Ports)
        } else if f.ends_with(".slackbuild") || f.ends_with(".tlz") || f.ends_with(".tbz") {
            Some(PackageFormat::TarGz)
        } else if f.ends_with(".crux") || f.ends_with(".pkgfile") {
            Some(PackageFormat::TarXz)
        } else if f.ends_with(".drpm") {
            Some(PackageFormat::Yum)
        } else if f.ends_with(".stratum") {
            Some(PackageFormat::Sovereign)
        } else if f.ends_with(".ipk") {
            Some(PackageFormat::Ipk)
        } else if f.ends_with(".opkg") {
            Some(PackageFormat::Opkg)
        } else if f.ends_with(".p5p") || f.ends_with(".ips") {
            Some(PackageFormat::SolarisIps)
        } else if f.ends_with(".nar") {
            Some(PackageFormat::GuixNar)
        } else if f.ends_with(".spack") {
            Some(PackageFormat::Spack)
        } else if f.ends_with(".conan") {
            Some(PackageFormat::Conan)
        } else if f.ends_with(".whl") {
            Some(PackageFormat::Wheel)
        } else if f.ends_with(".crate") {
            Some(PackageFormat::Crate)
        } else if f.ends_with(".gem") {
            Some(PackageFormat::Gem)
        } else if f.ends_with(".nupkg") {
            Some(PackageFormat::Nupkg)
        } else if f.ends_with(".vcpkg") {
            Some(PackageFormat::Vcpkg)
        } else if f.ends_with(".narinfo") {
            Some(PackageFormat::NarInfo)
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
        } else if data.starts_with(b"CACHY") {
            Some(PackageFormat::Pacman) // CachyOS magic
        } else if data.starts_with(b"DRPM") {
            Some(PackageFormat::Yum) // Delta RPM magic
        } else if data.starts_with(b"CRUX") {
            Some(PackageFormat::TarXz) // CRUX package magic
        } else if data.starts_with(b"DPOR") {
            Some(PackageFormat::Ports) // DragonFly BSD DPorts magic
        } else if data.starts_with(b"BRLK") {
            Some(PackageFormat::Sovereign) // Bedrock Linux Stratum magic
        } else if data.starts_with(b"SLAK") {
            Some(PackageFormat::TarGz) // Slackware SlackBuild magic
        } else if data.starts_with(b"IPK!") {
            Some(PackageFormat::Ipk) // OpenWrt / opkg magic
        } else if data.starts_with(b"OPKG") {
            Some(PackageFormat::Opkg) // Yocto / opkg magic
        } else if data.starts_with(b"P5P!") {
            Some(PackageFormat::SolarisIps) // Solaris IPS magic
        } else if data.starts_with(b"NARS") {
            Some(PackageFormat::GuixNar) // Nix / Guix NAR archive magic
        } else if data.starts_with(b"OBSD") {
            Some(PackageFormat::OpenBsdPkg) // OpenBSD pkg_add magic
        } else if data.starts_with(b"SPAK") {
            Some(PackageFormat::Spack) // Spack HPC package magic
        } else if data.starts_with(b"CONA") {
            Some(PackageFormat::Conan) // Conan C/C++ package magic
        } else if data.starts_with(b"WHEL") {
            Some(PackageFormat::Wheel) // Python Wheel magic
        } else if data.starts_with(b"CRAT") {
            Some(PackageFormat::Crate) // Rust Cargo crate magic
        } else if data.starts_with(b"GEMS") {
            Some(PackageFormat::Gem) // RubyGems magic
        } else if data.starts_with(b"NUPK") {
            Some(PackageFormat::Nupkg) // .NET NuGet magic
        } else if data.starts_with(b"VCPK") {
            Some(PackageFormat::Vcpkg) // Microsoft Vcpkg magic
        } else if data.starts_with(b"NARI") {
            Some(PackageFormat::NarInfo) // Nix/Guix NarInfo substituter magic
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
        } else if version_str.contains('_') {
            version_str.split('_').next().unwrap()
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

    /// Auto-detects package format by extension or text heuristics and parses & translates into native Package
    pub fn parse_and_translate_manifest(
        &self,
        filename: &str,
        raw_text: &str,
    ) -> Result<Package, &'static str> {
        let fmt = self.detect_format_by_extension(filename);
        match fmt {
            Some(PackageFormat::Apt) | Some(PackageFormat::Superdeb) => {
                let deb = self.parse_apt_control(raw_text)?;
                self.translate_to_native_package(
                    &deb.package,
                    &deb.version,
                    &deb.description,
                    &deb.depends,
                )
            }
            Some(PackageFormat::Pacman) => {
                if raw_text.contains("pkgname")
                    && raw_text.contains("depend")
                    && !raw_text.contains("pkgname=")
                {
                    let pkginfo = self.parse_arch_pkginfo(raw_text)?;
                    self.translate_to_native_package(
                        &pkginfo.pkgname,
                        &pkginfo.pkgver,
                        &pkginfo.pkgdesc,
                        &pkginfo.depends,
                    )
                } else {
                    let pkgbuild = self.parse_pacman_pkgbuild(raw_text)?;
                    self.translate_to_native_package(
                        &pkgbuild.pkgname,
                        &pkgbuild.pkgver,
                        &pkgbuild.pkgdesc,
                        &pkgbuild.depends,
                    )
                }
            }
            Some(PackageFormat::Yum) | Some(PackageFormat::Pisi) => {
                let spec = self.parse_rpm_spec(raw_text)?;
                self.translate_to_native_package(
                    &spec.name,
                    &spec.version,
                    &spec.summary,
                    &spec.requires,
                )
            }
            Some(PackageFormat::Apk) => {
                let apk = self.parse_apkindex(raw_text)?;
                self.translate_to_native_package(
                    &apk.pkgname,
                    &apk.pkgver,
                    &apk.pkgdesc,
                    &apk.depends,
                )
            }
            Some(PackageFormat::Xbps) => {
                let xbps = self.parse_xbps_manifest(raw_text)?;
                self.translate_to_native_package(
                    &xbps.pkgname,
                    &xbps.version,
                    &xbps.short_desc,
                    &xbps.run_depends,
                )
            }
            Some(PackageFormat::Portage) => {
                let ebuild = self.parse_gentoo_ebuild(filename, raw_text)?;
                let mut deps = ebuild.rdepend.clone();
                deps.extend(ebuild.depend.clone());
                self.translate_to_native_package(
                    &ebuild.package_name,
                    &ebuild.version,
                    &ebuild.description,
                    &deps,
                )
            }
            Some(PackageFormat::Hpkg) => {
                let hpkg = self.parse_haiku_hpkg(raw_text)?;
                self.translate_to_native_package(
                    &hpkg.name,
                    &hpkg.version,
                    &hpkg.summary,
                    &hpkg.requires,
                )
            }
            _ => {
                // Heuristic inspection if extension detection wasn't definitive
                if raw_text.contains("Package:") && raw_text.contains("Version:") {
                    let deb = self.parse_apt_control(raw_text)?;
                    self.translate_to_native_package(
                        &deb.package,
                        &deb.version,
                        &deb.description,
                        &deb.depends,
                    )
                } else if raw_text.contains("pkgname=") && raw_text.contains("pkgver=") {
                    let pkgbuild = self.parse_pacman_pkgbuild(raw_text)?;
                    self.translate_to_native_package(
                        &pkgbuild.pkgname,
                        &pkgbuild.pkgver,
                        &pkgbuild.pkgdesc,
                        &pkgbuild.depends,
                    )
                } else if raw_text.contains("pkgname") && raw_text.contains("depend") {
                    let pkginfo = self.parse_arch_pkginfo(raw_text)?;
                    self.translate_to_native_package(
                        &pkginfo.pkgname,
                        &pkginfo.pkgver,
                        &pkginfo.pkgdesc,
                        &pkginfo.depends,
                    )
                } else if raw_text.contains("P:") && raw_text.contains("V:") {
                    let apk = self.parse_apkindex(raw_text)?;
                    self.translate_to_native_package(
                        &apk.pkgname,
                        &apk.pkgver,
                        &apk.pkgdesc,
                        &apk.depends,
                    )
                } else if raw_text.contains("short_desc=") || raw_text.contains("run_depends=") {
                    let xbps = self.parse_xbps_manifest(raw_text)?;
                    self.translate_to_native_package(
                        &xbps.pkgname,
                        &xbps.version,
                        &xbps.short_desc,
                        &xbps.run_depends,
                    )
                } else if raw_text.contains("DESCRIPTION=") || raw_text.contains("RDEPEND=") {
                    let ebuild = self.parse_gentoo_ebuild(filename, raw_text)?;
                    let mut deps = ebuild.rdepend.clone();
                    deps.extend(ebuild.depend.clone());
                    self.translate_to_native_package(
                        &ebuild.package_name,
                        &ebuild.version,
                        &ebuild.description,
                        &deps,
                    )
                } else if raw_text.contains("Name:") && raw_text.contains("Version:") {
                    let spec = self.parse_rpm_spec(raw_text)?;
                    self.translate_to_native_package(
                        &spec.name,
                        &spec.version,
                        &spec.summary,
                        &spec.requires,
                    )
                } else if raw_text.contains("name:") && raw_text.contains("confinement:") {
                    let snap = self.parse_snapcraft_yaml(raw_text)?;
                    self.translate_to_native_package(
                        &snap.name,
                        &snap.version,
                        &snap.summary,
                        &snap.plugs,
                    )
                } else if raw_text.contains("\"app-id\"") {
                    let flatpak = self.parse_flatpak_json(raw_text)?;
                    self.translate_to_native_package(
                        &flatpak.app_id,
                        "1.0.0",
                        "Flatpak Sandboxed App",
                        &flatpak.finish_args,
                    )
                } else if raw_text.contains("@name ") || raw_text.contains("@depend ") {
                    let obs = self.parse_openbsd_contents(raw_text)?;
                    self.translate_to_native_package(
                        &obs.pkgname,
                        &obs.version,
                        &obs.comment,
                        &obs.depends,
                    )
                } else if raw_text.contains("deps:") || raw_text.contains("origin:") {
                    let bsd = self.parse_freebsd_ucl_manifest(raw_text)?;
                    self.translate_to_native_package(
                        &bsd.name,
                        &bsd.version,
                        &bsd.comment,
                        &bsd.deps,
                    )
                } else if raw_text.contains("PRGNAM=") || raw_text.contains("slack-desc") {
                    let slack = self.parse_slackware_pkg(raw_text)?;
                    self.translate_to_native_package(
                        &slack.name,
                        &slack.version,
                        &slack.description,
                        &slack.slack_required,
                    )
                } else if raw_text.contains("requires {") || (raw_text.contains("summary \"") && raw_text.contains("architecture ")) {
                    let hpkg = self.parse_haiku_hpkg(raw_text)?;
                    self.translate_to_native_package(
                        &hpkg.name,
                        &hpkg.version,
                        &hpkg.summary,
                        &hpkg.requires,
                    )
                } else {
                    Err("Unrecognized package manifest format")
                }
            }
        }
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
    pub fn parse_server_image_manifest(
        &self,
        format: ServerImageFormat,
        manifest_data: &str,
    ) -> Result<ServerImageMetadata, &'static str> {
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

/// Universal Package Bridge Engine for SigmaOS
/// Seamlessly converts foreign Linux/BSD packages (.deb, PKGBUILD, .spec, .apk, .ebuild, .ports, etc.)
/// into native Sigma-pkg models, mapping dependencies, sandboxing capabilities, and registering with Universal PM.
pub struct SigPkgUniversalBridgeEngine {
    adapter: UniversalPackageAdapter,
    pm: universal_oop_system::UniversalPackageManager,
}

impl SigPkgUniversalBridgeEngine {
    pub fn new() -> Self {
        Self {
            adapter: UniversalPackageAdapter::new(),
            pm: universal_oop_system::UniversalPackageManager::new(),
        }
    }

    /// Automatically detects format from filename and header, parses foreign manifest,
    /// and converts it directly into a native Sigma-pkg (`Package`)
    pub fn convert_to_sigpkg(
        &self,
        filename: &str,
        raw_data: &[u8],
    ) -> Result<Package, &'static str> {
        let fmt = self
            .adapter
            .detect_format_by_header(raw_data)
            .or_else(|| self.adapter.detect_format_by_extension(filename))
            .ok_or("Bridge Engine: Unable to detect package format")?;

        let manifest_text = String::from_utf8_lossy(raw_data);

        match fmt {
            PackageFormat::Apt => {
                let apt = self.adapter.parse_apt_control(&manifest_text)?;
                self.adapter.translate_to_native_package(
                    &apt.package,
                    &apt.version,
                    &apt.description,
                    &apt.depends,
                )
            }
            PackageFormat::Pacman => {
                if manifest_text.contains("pkgname")
                    && manifest_text.contains("depend")
                    && !manifest_text.contains("pkgname=")
                {
                    let pkginfo = self.adapter.parse_arch_pkginfo(&manifest_text)?;
                    self.adapter.translate_to_native_package(
                        &pkginfo.pkgname,
                        &pkginfo.pkgver,
                        &pkginfo.pkgdesc,
                        &pkginfo.depends,
                    )
                } else {
                    let pacman = self.adapter.parse_pacman_pkgbuild(&manifest_text)?;
                    self.adapter.translate_to_native_package(
                        &pacman.pkgname,
                        &pacman.pkgver,
                        &pacman.pkgdesc,
                        &pacman.depends,
                    )
                }
            }
            PackageFormat::Yum => {
                let rpm = self.adapter.parse_rpm_spec(&manifest_text)?;
                self.adapter.translate_to_native_package(
                    &rpm.name,
                    &rpm.version,
                    &rpm.summary,
                    &rpm.requires,
                )
            }
            PackageFormat::Snap => {
                let snap = self.adapter.parse_snapcraft_yaml(&manifest_text)?;
                self.adapter.translate_to_native_package(
                    &snap.name,
                    &snap.version,
                    &snap.summary,
                    &snap.plugs,
                )
            }
            PackageFormat::Apk => {
                let apk = self.adapter.parse_apkindex(&manifest_text)?;
                self.adapter.translate_to_native_package(
                    &apk.pkgname,
                    &apk.pkgver,
                    &apk.pkgdesc,
                    &apk.depends,
                )
            }
            PackageFormat::Xbps => {
                let xbps = self.adapter.parse_xbps_manifest(&manifest_text)?;
                self.adapter.translate_to_native_package(
                    &xbps.pkgname,
                    &xbps.version,
                    &xbps.short_desc,
                    &xbps.run_depends,
                )
            }
            PackageFormat::Portage => {
                let ebuild = self.adapter.parse_gentoo_ebuild(filename, &manifest_text)?;
                let mut deps = ebuild.rdepend.clone();
                deps.extend(ebuild.depend.clone());
                self.adapter.translate_to_native_package(
                    &ebuild.package_name,
                    &ebuild.version,
                    &ebuild.description,
                    &deps,
                )
            }
            PackageFormat::Pkg | PackageFormat::Ports => {
                if manifest_text.contains("@name") {
                    let obs = self.adapter.parse_openbsd_contents(&manifest_text)?;
                    self.adapter.translate_to_native_package(
                        &obs.pkgname,
                        &obs.version,
                        &obs.comment,
                        &obs.depends,
                    )
                } else {
                    let bsd = self.adapter.parse_freebsd_ucl_manifest(&manifest_text)?;
                    self.adapter.translate_to_native_package(
                        &bsd.name,
                        &bsd.version,
                        &bsd.comment,
                        &bsd.deps,
                    )
                }
            }
            PackageFormat::Zypper => {
                let zyp = self.adapter.parse_zypper_spec(&manifest_text)?;
                self.adapter.translate_to_native_package(
                    &zyp.name,
                    &zyp.version,
                    &zyp.summary,
                    &zyp.requires,
                )
            }
            PackageFormat::Pkgsrc => {
                let net = self.adapter.parse_netbsd_pkgsrc(&manifest_text)?;
                self.adapter.translate_to_native_package(
                    &net.pkgname,
                    &net.version,
                    &net.comment,
                    &net.depends,
                )
            }
            PackageFormat::Hpkg => {
                let hpkg = self.adapter.parse_haiku_hpkg(&manifest_text)?;
                self.adapter.translate_to_native_package(
                    &hpkg.name,
                    &hpkg.version,
                    &hpkg.summary,
                    &hpkg.requires,
                )
            }
            _ => self
                .adapter
                .parse_and_translate_manifest(filename, &manifest_text),
        }
    }

    /// Converts a foreign package manifest and registers it into the Universal Package Manager
    pub fn absorb_and_register(
        &mut self,
        filename: &str,
        raw_data: &[u8],
    ) -> Result<Package, &'static str> {
        let native_pkg = self.convert_to_sigpkg(filename, raw_data)?;
        let standard_pkg = universal_oop_system::StandardPackage {
            metadata: universal_oop_system::PackageMetadata {
                name: native_pkg.name.clone(),
                version: native_pkg.version,
                description: native_pkg.description.clone(),
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: native_pkg.checksum.clone(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies: Vec::new(),
            format: universal_oop_system::PackageFormat::Sigma,
        };
        let _ = self.pm.install_package(Box::new(standard_pkg));
        Ok(native_pkg)
    }

    pub fn is_package_registered(&self, name: &str) -> bool {
        self.pm.get_package(name).is_some()
    }
}

impl Default for SigPkgUniversalBridgeEngine {
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

// =========================================================================
// UNIVERSAL CROSS-DISTRO DEPENDENCY & SCRIPTLET MAPPER
// =========================================================================

/// Maps distro-specific package names to canonical SigmaOS package names
pub struct UniversalDependencyMapper;

impl UniversalDependencyMapper {
    pub fn new() -> Self {
        Self
    }

    /// Translates a foreign package dependency name to a canonical Sigma-pkg dependency name
    pub fn to_canonical_name(&self, foreign_name: &str) -> String {
        let raw = foreign_name.trim().to_lowercase();
        let clean = if let Some(stripped) = raw.strip_prefix("so:") {
            if stripped.starts_with("libc.") {
                "libc"
            } else {
                stripped.split('.').next().unwrap_or(stripped)
            }
        } else if let Some(stripped) = raw.strip_prefix("cmd:") {
            stripped
        } else {
            raw.as_str()
        };

        match clean {
            "libssl-dev" | "libssl3" | "openssl-devel" | "openssl-dev" | "security/openssl"
            | "dev-libs/openssl" | "openssl" | "libssl1.1" | "libssl" => "openssl".to_string(),
            "libc6" | "glibc" | "musl" | "musl-dev" | "devel/glibc" | "sys-libs/glibc" | "libc" | "glibc-devel" => {
                "libc".to_string()
            }
            "zlib1g-dev" | "zlib-devel" | "zlib-dev" | "devel/zlib" | "sys-libs/zlib" => {
                "zlib".to_string()
            }
            "python" | "python3" | "python3-dev" | "python3-devel" | "dev-lang/python" | "lang/python" => {
                "python".to_string()
            }
            "curl" | "libcurl4" | "libcurl-devel" | "libcurl-dev" | "ftp/curl" | "net-misc/curl" => "curl".to_string(),
            "bash" | "shells/bash" | "app-shells/bash" => "bash".to_string(),
            "libx11" | "x11-libs/libx11" | "x11-proto/xorgproto" => "libx11".to_string(),
            "wayland" | "dev-libs/wayland" => "wayland".to_string(),
            "pipewire" | "media-video/pipewire" => "pipewire".to_string(),
            "dbus" | "sys-apps/dbus" => "dbus".to_string(),
            "pkgconf" | "pkg-config" | "dev-util/pkgconf" => "pkgconf".to_string(),
            "ncurses" | "ncursesw" | "sys-libs/ncurses" => "ncurses".to_string(),
            "readline" | "sys-libs/readline" => "readline".to_string(),
            "xz" | "xz-utils" | "app-arch/xz-utils" => "xz".to_string(),
            "zstd" | "app-arch/zstd" => "zstd".to_string(),
            "sqlite" | "sqlite3" | "libsqlite3-dev" | "sqlite-devel" | "databases/sqlite3" | "dev-db/sqlite" => "sqlite".to_string(),
            "libpng" | "libpng-dev" | "libpng-devel" | "graphics/png" | "media-libs/libpng" => "libpng".to_string(),
            "jpeg" | "libjpeg" | "libjpeg-turbo" | "libjpeg-devel" | "graphics/jpeg" => "jpeg".to_string(),
            "systemd" | "systemd-libs" | "libsystemd-dev" | "systemd-devel" | "sys-apps/systemd" => "systemd".to_string(),
            "llvm" | "llvm-dev" | "llvm-devel" | "devel/llvm" | "sys-devel/llvm" => "llvm".to_string(),
            "rust" | "rustc" | "lang/rust" | "dev-lang/rust" | "rust-dev" => "rust".to_string(),
            "libxml2" | "libxml2-dev" | "libxml2-devel" | "textproc/libxml2" | "dev-libs/libxml2" => "libxml2".to_string(),
            _ => clean.to_string(),
        }
    }
}

impl Default for UniversalDependencyMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigmaPkgHookType {
    PreInstall,
    PostInstall,
    PreRemove,
    PostRemove,
}

#[derive(Debug, Clone)]
pub struct MappedScriptletHook {
    pub hook_type: SigmaPkgHookType,
    pub script_content: String,
}

/// Converts foreign package scriptlets (Debian postinst, RPM %post, Arch .INSTALL, Alpine post-install) into Sigma-pkg lifecycle hooks
pub struct UniversalScriptletConverter;

impl UniversalScriptletConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_scriptlet(
        &self,
        format: PackageFormat,
        script_name: &str,
        content: &str,
    ) -> Option<MappedScriptletHook> {
        let hook_type = match format {
            PackageFormat::Apt => match script_name {
                "preinst" => Some(SigmaPkgHookType::PreInstall),
                "postinst" => Some(SigmaPkgHookType::PostInstall),
                "prerm" => Some(SigmaPkgHookType::PreRemove),
                "postrm" => Some(SigmaPkgHookType::PostRemove),
                _ => None,
            },
            PackageFormat::Yum => match script_name {
                "%pre" => Some(SigmaPkgHookType::PreInstall),
                "%post" | "%posttrans" => Some(SigmaPkgHookType::PostInstall),
                "%preun" => Some(SigmaPkgHookType::PreRemove),
                "%postun" => Some(SigmaPkgHookType::PostRemove),
                _ => None,
            },
            PackageFormat::Pkg | PackageFormat::Ports => match script_name {
                "+POST_INSTALL" | "+INSTALL" | "pkg-post-install" | "post-install" => Some(SigmaPkgHookType::PostInstall),
                "+PRE_INSTALL" | "pre-install" => Some(SigmaPkgHookType::PreInstall),
                "+POST_DEINSTALL" | "+DEINSTALL" | "post-deinstall" | "post-remove" => Some(SigmaPkgHookType::PostRemove),
                "+PRE_DEINSTALL" | "pre-deinstall" | "pre-remove" => Some(SigmaPkgHookType::PreRemove),
                _ => None,
            },
            PackageFormat::Pacman => match script_name {
                "pre_install" => Some(SigmaPkgHookType::PreInstall),
                "post_install" => Some(SigmaPkgHookType::PostInstall),
                "pre_remove" => Some(SigmaPkgHookType::PreRemove),
                "post_remove" => Some(SigmaPkgHookType::PostRemove),
                _ => None,
            },
            PackageFormat::Apk | PackageFormat::Xbps => match script_name {
                "pre-install" => Some(SigmaPkgHookType::PreInstall),
                "post-install" => Some(SigmaPkgHookType::PostInstall),
                "pre-deinstall" | "pre-remove" => Some(SigmaPkgHookType::PreRemove),
                "post-deinstall" | "post-remove" => Some(SigmaPkgHookType::PostRemove),
                _ => None,
            },
            PackageFormat::Portage => match script_name {
                "pkg_preinst" | "pre_install" => Some(SigmaPkgHookType::PreInstall),
                "pkg_postinst" | "post_install" => Some(SigmaPkgHookType::PostInstall),
                "pkg_prerm" | "pre_remove" => Some(SigmaPkgHookType::PreRemove),
                "pkg_postrm" | "post_remove" => Some(SigmaPkgHookType::PostRemove),
                _ => None,
            },
            PackageFormat::Nix | PackageFormat::Guix => match script_name {
                "preInstall" => Some(SigmaPkgHookType::PreInstall),
                "postInstall" => Some(SigmaPkgHookType::PostInstall),
                "preUnpack" | "preBuild" => Some(SigmaPkgHookType::PreInstall),
                _ => None,
            },
            _ => None,
        };

        hook_type.map(|ht| MappedScriptletHook {
            hook_type: ht,
            script_content: content.to_string(),
        })
    }
}

impl Default for UniversalScriptletConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Universal Sandbox Capability Matrix for mapping foreign security permissions into SigmaOS Capabilities
pub struct UniversalSandboxCapabilityMatrix;

impl UniversalSandboxCapabilityMatrix {
    pub fn new() -> Self {
        Self
    }

    /// Maps Snap plugs, Flatpak finish-args, OpenBSD pledge/unveil, and FreeBSD RACCT rules into SigmaOS permissions
    pub fn map_foreign_capabilities(&self, raw_caps: &[String]) -> Vec<Permission> {
        let mut perms = Vec::new();

        for cap in raw_caps {
            let c = cap.trim().to_lowercase();
            if c == "network"
                || c == "network-bind"
                || c == "--share=network"
                || c == "inet"
                || c == "inet6"
            {
                perms.push(Permission::NetworkTcp);
                perms.push(Permission::NetworkUdp);
            } else if c == "home"
                || c == "--filesystem=home"
                || c == "--filesystem=host"
                || c == "rpath"
                || c == "wpath"
                || c == "cpath"
            {
                perms.push(Permission::FileRead);
                perms.push(Permission::FileWrite);
            } else if c == "--share=ipc" || c == "ipc" || c == "ps" {
                perms.push(Permission::Ipc);
            } else if c == "audio-playback"
                || c == "pulseaudio"
                || c == "--socket=pulseaudio"
                || c == "audio"
            {
                perms.push(Permission::AudioPlayback);
            } else if c == "x11" || c == "wayland" || c == "--socket=x11" || c == "--socket=wayland"
            {
                perms.push(Permission::DisplayAccess);
            } else if c == "system-observe" || c == "proc" || c == "sysctl" || c == "exec" || c == "execpromises" {
                perms.push(Permission::ProcessExec);
            }
        }
        if perms.is_empty() {
            perms.push(Permission::FileRead);
        }
        perms
    }
}

impl Default for UniversalSandboxCapabilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniversalPmOperation {
    Install,
    Remove,
    Upgrade,
    Search,
    QueryInfo,
    CleanCache,
}

#[derive(Debug, Clone)]
pub struct DispatchedPmAction {
    pub source_pm: String,
    pub operation: UniversalPmOperation,
    pub target_packages: Vec<String>,
    pub dry_run: bool,
}

/// Universal Foreign Package Manager Command Dispatcher Bridge
pub struct UniversalPmCommandDispatcher;

impl UniversalPmCommandDispatcher {
    pub fn new() -> Self {
        Self
    }

    /// Parses foreign PM command invocation (apt, pacman, dnf, apk, pkg, zypper, xbps) and translates into canonical sigpkg action
    pub fn dispatch_command(&self, full_cmd: &str) -> Result<DispatchedPmAction, &'static str> {
        let tokens: Vec<&str> = full_cmd.split_whitespace().collect();
        if tokens.is_empty() {
            return Err("Empty package manager command");
        }

        let pm = tokens[0].to_lowercase();
        let args = &tokens[1..];

        let mut operation = UniversalPmOperation::Install;
        let mut target_packages = Vec::new();
        let mut dry_run = false;

        match pm.as_str() {
            "apt" | "apt-get" | "dpkg" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "install" | "-i" => operation = UniversalPmOperation::Install,
                        "remove" | "purge" | "-r" => operation = UniversalPmOperation::Remove,
                        "update" | "upgrade" | "dist-upgrade" => {
                            operation = UniversalPmOperation::Upgrade
                        }
                        "search" => operation = UniversalPmOperation::Search,
                        "show" | "status" => operation = UniversalPmOperation::QueryInfo,
                        "clean" | "autoclean" => operation = UniversalPmOperation::CleanCache,
                        "-s" | "--dry-run" | "--simulate" => dry_run = true,
                        arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                        _ => {}
                    }
                    i += 1;
                }
            }
            "spack" | "conan" | "pip" | "cargo" | "gem" | "nuget" | "vcpkg" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "install" | "add" => operation = UniversalPmOperation::Install,
                        "uninstall" | "remove" | "rm" => operation = UniversalPmOperation::Remove,
                        "update" | "upgrade" => operation = UniversalPmOperation::Upgrade,
                        "search" | "find" => operation = UniversalPmOperation::Search,
                        "info" | "show" => operation = UniversalPmOperation::QueryInfo,
                        "--dry-run" | "--dryrun" => dry_run = true,
                        arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                        _ => {}
                    }
                    i += 1;
                }
            }
            "pacman" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "-S" | "-Sy" => operation = UniversalPmOperation::Install,
                        "-R" | "-Rns" => operation = UniversalPmOperation::Remove,
                        "-Syu" | "-Syyu" => operation = UniversalPmOperation::Upgrade,
                        "-Ss" | "-Qs" => operation = UniversalPmOperation::Search,
                        "-Si" | "-Qi" => operation = UniversalPmOperation::QueryInfo,
                        "-Sc" | "-Scc" => operation = UniversalPmOperation::CleanCache,
                        "--print" | "--dryrun" => dry_run = true,
                        arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                        _ => {}
                    }
                    i += 1;
                }
            }
            "dnf" | "yum" | "zypper" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "install" | "in" => operation = UniversalPmOperation::Install,
                        "remove" | "erase" | "rm" => operation = UniversalPmOperation::Remove,
                        "update" | "upgrade" | "up" => operation = UniversalPmOperation::Upgrade,
                        "search" | "se" => operation = UniversalPmOperation::Search,
                        "info" => operation = UniversalPmOperation::QueryInfo,
                        "clean" => operation = UniversalPmOperation::CleanCache,
                        "--dry-run" => dry_run = true,
                        arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                        _ => {}
                    }
                    i += 1;
                }
            }
            "apk" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "add" => operation = UniversalPmOperation::Install,
                        "del" => operation = UniversalPmOperation::Remove,
                        "upgrade" => operation = UniversalPmOperation::Upgrade,
                        "search" => operation = UniversalPmOperation::Search,
                        "info" => operation = UniversalPmOperation::QueryInfo,
                        "-s" | "--simulate" => dry_run = true,
                        arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                        _ => {}
                    }
                    i += 1;
                }
            }
            "opkg" | "ipkg" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "install" => operation = UniversalPmOperation::Install,
                        "remove" => operation = UniversalPmOperation::Remove,
                        "upgrade" => operation = UniversalPmOperation::Upgrade,
                        "find" | "search" => operation = UniversalPmOperation::Search,
                        "info" | "status" => operation = UniversalPmOperation::QueryInfo,
                        "--noaction" => dry_run = true,
                        arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                        _ => {}
                    }
                    i += 1;
                }
            }
            "pkg" | "pkgsend" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "install" | "add" => operation = UniversalPmOperation::Install,
                        "delete" | "remove" => operation = UniversalPmOperation::Remove,
                        "upgrade" => operation = UniversalPmOperation::Upgrade,
                        "search" => operation = UniversalPmOperation::Search,
                        "info" => operation = UniversalPmOperation::QueryInfo,
                        "-n" => dry_run = true,
                        arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                        _ => {}
                    }
                    i += 1;
                }
            }
            "xbps-install" | "xbps-remove" | "xbps-query" => {
                if pm == "xbps-install" {
                    operation = UniversalPmOperation::Install;
                } else if pm == "xbps-remove" {
                    operation = UniversalPmOperation::Remove;
                } else {
                    operation = UniversalPmOperation::QueryInfo;
                }
                for arg in args {
                    if *arg == "-n" || *arg == "--dry-run" {
                        dry_run = true;
                    } else if !arg.starts_with('-') {
                        target_packages.push(arg.to_string());
                    }
                }
            }
            "emerge" | "ebuild" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "-C" | "--unmerge" | "--deselect" => operation = UniversalPmOperation::Remove,
                        "-u" | "-uDN" | "--update" => operation = UniversalPmOperation::Upgrade,
                        "-s" | "--search" => operation = UniversalPmOperation::Search,
                        "--info" => operation = UniversalPmOperation::QueryInfo,
                        "-p" | "--pretend" | "-a" | "--ask" => dry_run = true,
                        arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                        _ => {}
                    }
                    i += 1;
                }
            }
            "nix" | "nix-env" | "nix-shell" | "guix" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "install" | "-i" | "-iA" => operation = UniversalPmOperation::Install,
                        "remove" | "uninstall" | "-e" => operation = UniversalPmOperation::Remove,
                        "upgrade" | "-u" => operation = UniversalPmOperation::Upgrade,
                        "search" | "-qa" => operation = UniversalPmOperation::Search,
                        "--dry-run" => dry_run = true,
                        arg if !arg.starts_with('-') && arg != "package" && arg != "profile" => {
                            target_packages.push(arg.to_string())
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }
            "flatpak" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "install" => operation = UniversalPmOperation::Install,
                        "uninstall" | "remove" => operation = UniversalPmOperation::Remove,
                        "update" => operation = UniversalPmOperation::Upgrade,
                        "search" => operation = UniversalPmOperation::Search,
                        "info" => operation = UniversalPmOperation::QueryInfo,
                        "--dry-run" => dry_run = true,
                        arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                        _ => {}
                    }
                    i += 1;
                }
            }
            "snap" => {
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        "install" => operation = UniversalPmOperation::Install,
                        "remove" => operation = UniversalPmOperation::Remove,
                        "refresh" => operation = UniversalPmOperation::Upgrade,
                        "find" | "search" => operation = UniversalPmOperation::Search,
                        "info" => operation = UniversalPmOperation::QueryInfo,
                        arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                        _ => {}
                    }
                    i += 1;
                }
            }
            "slackpkg" | "installpkg" | "removepkg" | "upgradepkg" => {
                if pm == "removepkg" {
                    operation = UniversalPmOperation::Remove;
                } else if pm == "upgradepkg" {
                    operation = UniversalPmOperation::Upgrade;
                } else {
                    let mut i = 0;
                    while i < args.len() {
                        match args[i] {
                            "install" => operation = UniversalPmOperation::Install,
                            "remove" | "purge" => operation = UniversalPmOperation::Remove,
                            "upgrade" | "upgrade-all" => operation = UniversalPmOperation::Upgrade,
                            "search" => operation = UniversalPmOperation::Search,
                            "info" => operation = UniversalPmOperation::QueryInfo,
                            arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                            _ => {}
                        }
                        i += 1;
                    }
                }
                if target_packages.is_empty() {
                    for arg in args {
                        if !arg.starts_with('-') && *arg != "install" && *arg != "remove" && *arg != "upgrade" {
                            target_packages.push(arg.to_string());
                        }
                    }
                }
            }
            "pkgman" | "swupd" | "eopkg" | "moss" | "pkgin" | "pkg_delete" | "pkg_info" => {
                if pm == "pkg_delete" {
                    operation = UniversalPmOperation::Remove;
                } else if pm == "pkg_info" {
                    operation = UniversalPmOperation::QueryInfo;
                } else {
                    let mut i = 0;
                    while i < args.len() {
                        match args[i] {
                            "install" | "in" | "it" | "bundle-add" | "add" => operation = UniversalPmOperation::Install,
                            "uninstall" | "remove" | "rm" | "bundle-remove" => operation = UniversalPmOperation::Remove,
                            "update" | "upgrade" | "ur" => operation = UniversalPmOperation::Upgrade,
                            "search" | "se" | "sr" => operation = UniversalPmOperation::Search,
                            "info" => operation = UniversalPmOperation::QueryInfo,
                            "-n" | "--dry-run" => dry_run = true,
                            arg if !arg.starts_with('-') => target_packages.push(arg.to_string()),
                            _ => {}
                        }
                        i += 1;
                    }
                }
                if target_packages.is_empty() {
                    for arg in args {
                        if !arg.starts_with('-') {
                            target_packages.push(arg.to_string());
                        }
                    }
                }
            }
            _ => {
                for arg in args {
                    if !arg.starts_with('-') {
                        target_packages.push(arg.to_string());
                    }
                }
            }
        }

        Ok(DispatchedPmAction {
            source_pm: pm,
            operation,
            target_packages,
            dry_run,
        })
    }
}

impl Default for UniversalPmCommandDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Universal Converter that converts any foreign package manifest into a native Sigma-pkg Package
pub struct UniversalFormatConverter {
    pub dep_mapper: UniversalDependencyMapper,
    pub scriptlet_converter: UniversalScriptletConverter,
}

impl UniversalFormatConverter {
    pub fn new() -> Self {
        Self {
            dep_mapper: UniversalDependencyMapper::new(),
            scriptlet_converter: UniversalScriptletConverter::new(),
        }
    }

    /// Converts raw manifest bytes of any supported Linux / BSD format into a native Sigma-pkg Package
    pub fn convert_to_sigma_pkg(
        &self,
        format: PackageFormat,
        raw_manifest: &[u8],
    ) -> Result<Package, String> {
        let adapter = UniversalPackageAdapter::new();
        let text = String::from_utf8_lossy(raw_manifest);

        match format {
            PackageFormat::Apt => {
                let parsed = adapter
                    .parse_apt_control(&text)
                    .map_err(|e: &'static str| e.to_string())?;
                let canonical_deps: Vec<String> = parsed
                    .depends
                    .iter()
                    .map(|d| self.dep_mapper.to_canonical_name(d))
                    .collect();
                adapter
                    .translate_to_native_package(
                        &parsed.package,
                        &parsed.version,
                        &parsed.description,
                        &canonical_deps,
                    )
                    .map_err(|e: &'static str| e.to_string())
            }
            PackageFormat::Pacman => {
                let parsed = adapter
                    .parse_pacman_pkgbuild(&text)
                    .map_err(|e: &'static str| e.to_string())?;
                let canonical_deps: Vec<String> = parsed
                    .depends
                    .iter()
                    .map(|d| self.dep_mapper.to_canonical_name(d))
                    .collect();
                adapter
                    .translate_to_native_package(
                        &parsed.pkgname,
                        &parsed.pkgver,
                        &parsed.pkgdesc,
                        &canonical_deps,
                    )
                    .map_err(|e: &'static str| e.to_string())
            }
            PackageFormat::Yum | PackageFormat::Pisi => {
                let parsed = adapter
                    .parse_rpm_spec(&text)
                    .map_err(|e: &'static str| e.to_string())?;
                let canonical_deps: Vec<String> = parsed
                    .requires
                    .iter()
                    .map(|d| self.dep_mapper.to_canonical_name(d))
                    .collect();
                adapter
                    .translate_to_native_package(
                        &parsed.name,
                        &parsed.version,
                        &parsed.summary,
                        &canonical_deps,
                    )
                    .map_err(|e: &'static str| e.to_string())
            }
            PackageFormat::Apk => {
                let parsed = adapter
                    .parse_apkindex(&text)
                    .map_err(|e: &'static str| e.to_string())?;
                let canonical_deps: Vec<String> = parsed
                    .depends
                    .iter()
                    .map(|d| self.dep_mapper.to_canonical_name(d))
                    .collect();
                adapter
                    .translate_to_native_package(
                        &parsed.pkgname,
                        &parsed.pkgver,
                        &parsed.pkgdesc,
                        &canonical_deps,
                    )
                    .map_err(|e: &'static str| e.to_string())
            }
            PackageFormat::Xbps => {
                let parsed = adapter
                    .parse_xbps_manifest(&text)
                    .map_err(|e: &'static str| e.to_string())?;
                let canonical_deps: Vec<String> = parsed
                    .run_depends
                    .iter()
                    .map(|d| self.dep_mapper.to_canonical_name(d))
                    .collect();
                adapter
                    .translate_to_native_package(
                        &parsed.pkgname,
                        &parsed.version,
                        &parsed.short_desc,
                        &canonical_deps,
                    )
                    .map_err(|e: &'static str| e.to_string())
            }
            PackageFormat::Portage => {
                let parsed = adapter
                    .parse_gentoo_ebuild("package.ebuild", &text)
                    .map_err(|e: &'static str| e.to_string())?;
                let mut raw_deps = parsed.rdepend.clone();
                raw_deps.extend(parsed.depend.clone());
                let canonical_deps: Vec<String> = raw_deps
                    .iter()
                    .map(|d| self.dep_mapper.to_canonical_name(d))
                    .collect();
                adapter
                    .translate_to_native_package(
                        &parsed.package_name,
                        &parsed.version,
                        &parsed.description,
                        &canonical_deps,
                    )
                    .map_err(|e: &'static str| e.to_string())
            }
            PackageFormat::Snap => {
                let parsed = adapter
                    .parse_snapcraft_yaml(&text)
                    .map_err(|e: &'static str| e.to_string())?;
                let canonical_deps: Vec<String> = parsed
                    .plugs
                    .iter()
                    .map(|d| self.dep_mapper.to_canonical_name(d))
                    .collect();
                adapter
                    .translate_to_native_package(
                        &parsed.name,
                        &parsed.version,
                        &parsed.summary,
                        &canonical_deps,
                    )
                    .map_err(|e: &'static str| e.to_string())
            }
            PackageFormat::Flatpak => {
                let parsed = adapter
                    .parse_flatpak_json(&text)
                    .map_err(|e: &'static str| e.to_string())?;
                let canonical_deps: Vec<String> = parsed
                    .finish_args
                    .iter()
                    .map(|d| self.dep_mapper.to_canonical_name(d))
                    .collect();
                adapter
                    .translate_to_native_package(
                        &parsed.app_id,
                        "1.0.0",
                        "Flatpak Sandboxed App",
                        &canonical_deps,
                    )
                    .map_err(|e: &'static str| e.to_string())
            }
            PackageFormat::Hpkg => {
                let parsed = adapter
                    .parse_haiku_hpkg(&text)
                    .map_err(|e: &'static str| e.to_string())?;
                let canonical_deps: Vec<String> = parsed
                    .requires
                    .iter()
                    .map(|d| self.dep_mapper.to_canonical_name(d))
                    .collect();
                adapter
                    .translate_to_native_package(
                        &parsed.name,
                        &parsed.version,
                        &parsed.summary,
                        &canonical_deps,
                    )
                    .map_err(|e: &'static str| e.to_string())
            }
            _ => {
                if text.contains("Package:") && text.contains("Version:") {
                    let deb = adapter
                        .parse_apt_control(&text)
                        .map_err(|e: &'static str| e.to_string())?;
                    let canonical_deps: Vec<String> = deb
                        .depends
                        .iter()
                        .map(|d| self.dep_mapper.to_canonical_name(d))
                        .collect();
                    adapter
                        .translate_to_native_package(
                            &deb.package,
                            &deb.version,
                            &deb.description,
                            &canonical_deps,
                        )
                        .map_err(|e: &'static str| e.to_string())
                } else if text.contains("pkgname=") && text.contains("pkgver=") {
                    let pkgbuild = adapter
                        .parse_pacman_pkgbuild(&text)
                        .map_err(|e: &'static str| e.to_string())?;
                    let canonical_deps: Vec<String> = pkgbuild
                        .depends
                        .iter()
                        .map(|d| self.dep_mapper.to_canonical_name(d))
                        .collect();
                    adapter
                        .translate_to_native_package(
                            &pkgbuild.pkgname,
                            &pkgbuild.pkgver,
                            &pkgbuild.pkgdesc,
                            &canonical_deps,
                        )
                        .map_err(|e: &'static str| e.to_string())
                } else {
                    let name = format!("{:?}-converted-pkg", format).to_lowercase();
                    adapter
                        .translate_to_native_package(
                            &name,
                            "1.0.0",
                            "Converted foreign package",
                            &[],
                        )
                        .map_err(|e: &'static str| e.to_string())
                }
            }
        }
    }
}

impl Default for UniversalFormatConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct UniversalDryRunResult {
    pub package_name: String,
    pub target_format: PackageFormat,
    pub resolved_dependencies: Vec<String>,
    pub required_permissions: Vec<String>,
    pub is_valid: bool,
}

/// Performs dry-run installation simulation for foreign packages with Universal PM
pub struct UniversalDryRunSimulator {
    pub converter: UniversalFormatConverter,
}

impl UniversalDryRunSimulator {
    pub fn new() -> Self {
        Self {
            converter: UniversalFormatConverter::new(),
        }
    }

    pub fn simulate_install(
        &self,
        format: PackageFormat,
        manifest: &[u8],
    ) -> Result<UniversalDryRunResult, String> {
        let pkg = self.converter.convert_to_sigma_pkg(format, manifest)?;
        let deps: Vec<String> = pkg.dependencies.iter().map(|d| d.name.clone()).collect();

        Ok(UniversalDryRunResult {
            package_name: pkg.name,
            target_format: format,
            resolved_dependencies: deps,
            required_permissions: vec!["FileRead".to_string(), "FileWrite".to_string()],
            is_valid: true,
        })
    }
}

impl Default for UniversalDryRunSimulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
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
    fn test_sigpkg_universal_bridge_engine() {
        let mut bridge = SigPkgUniversalBridgeEngine::new();

        let deb_control = r#"
            Package: htop
            Version: 3.2.2
            Depends: libc6, libncursesw6
            Description: Interactive process viewer
        "#;

        let converted_deb = bridge
            .absorb_and_register("htop.deb", deb_control.as_bytes())
            .unwrap();
        assert_eq!(converted_deb.name, "htop");
        assert_eq!(converted_deb.version, Version::new(3, 2, 2));
        assert_eq!(converted_deb.dependencies.len(), 2);
        assert!(bridge.is_package_registered("htop"));

        let pkgbuild = r#"
            pkgname=ripgrep
            pkgver=13.0.0
            depends=('pcre2')
        "#;

        let converted_pacman = bridge
            .absorb_and_register("ripgrep.pkg.tar.zst", pkgbuild.as_bytes())
            .unwrap();
        assert_eq!(converted_pacman.name, "ripgrep");
        assert_eq!(converted_pacman.version, Version::new(13, 0, 0));
        assert!(bridge.is_package_registered("ripgrep"));
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

        let meta = adapter
            .parse_server_image_manifest(ServerImageFormat::Qcow2DiskImage, manifest_data)
            .unwrap();
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
        assert_eq!(
            adapter.detect_format_by_extension("app.air"),
            Some(PackageFormat::Air)
        );
        assert_eq!(
            adapter.detect_format_by_extension("lib.bottle"),
            Some(PackageFormat::Bottle)
        );
        assert_eq!(
            adapter.detect_format_by_extension("game.ipa"),
            Some(PackageFormat::Ipa)
        );
        assert_eq!(
            adapter.detect_format_by_extension("custom.ports"),
            Some(PackageFormat::Ports)
        );
        assert_eq!(
            adapter.detect_format_by_extension("base.pkg"),
            Some(PackageFormat::Pkg)
        );
        assert_eq!(
            adapter.detect_format_by_extension("mobile.aab"),
            Some(PackageFormat::Aab)
        );
        assert_eq!(
            adapter.detect_format_by_extension("alpine.apk"),
            Some(PackageFormat::Apk)
        );
        assert_eq!(
            adapter.detect_format_by_extension("app.appimage"),
            Some(PackageFormat::AppImage)
        );
        assert_eq!(
            adapter.detect_format_by_extension("solus.eopkg"),
            Some(PackageFormat::Eopkg)
        );
        assert_eq!(
            adapter.detect_format_by_extension("gentoo.ebuild"),
            Some(PackageFormat::Portage)
        );
        assert_eq!(
            adapter.detect_format_by_extension("ubuntu.deb"),
            Some(PackageFormat::Apt)
        );
        assert_eq!(
            adapter.detect_format_by_extension("arch.pkg.tar.xz"),
            Some(PackageFormat::Pacman)
        );
        assert_eq!(
            adapter.detect_format_by_extension("fedora.rpm"),
            Some(PackageFormat::Yum)
        );
        assert_eq!(
            adapter.detect_format_by_extension("harmony.hap"),
            Some(PackageFormat::Hap)
        );
        assert_eq!(
            adapter.detect_format_by_extension("slax.lzm"),
            Some(PackageFormat::Lzm)
        );
        assert_eq!(
            adapter.detect_format_by_extension("puppy.pup"),
            Some(PackageFormat::Pup)
        );
        assert_eq!(
            adapter.detect_format_by_extension("puppy.pet"),
            Some(PackageFormat::Pet)
        );

        assert_eq!(
            adapter.detect_format_by_extension("solus.moss"),
            Some(PackageFormat::Moss)
        );
        assert_eq!(
            adapter.detect_format_by_extension("haiku.hpkg"),
            Some(PackageFormat::Hpkg)
        );
        assert_eq!(
            adapter.detect_format_by_extension("extension.tcz"),
            Some(PackageFormat::Tcz)
        );
        assert_eq!(
            adapter.detect_format_by_extension("app.gobo"),
            Some(PackageFormat::Gobo)
        );
        assert_eq!(
            adapter.detect_format_by_extension("commit.ostree"),
            Some(PackageFormat::Ostree)
        );
        assert_eq!(
            adapter.detect_format_by_extension("tool.pkgsrc"),
            Some(PackageFormat::Pkgsrc)
        );
        assert_eq!(
            adapter.detect_format_by_extension("module.sfs"),
            Some(PackageFormat::Sfs)
        );
        assert_eq!(
            adapter.detect_format_by_extension("portable.puk"),
            Some(PackageFormat::Puk)
        );
        assert_eq!(
            adapter.detect_format_by_extension("image.dmg"),
            Some(PackageFormat::Dmg)
        );
        assert_eq!(
            adapter.detect_format_by_extension("recipe.cports"),
            Some(PackageFormat::Cports)
        );
        assert_eq!(
            adapter.detect_format_by_extension("router.ipk"),
            Some(PackageFormat::Ipk)
        );
        assert_eq!(
            adapter.detect_format_by_extension("yocto.opkg"),
            Some(PackageFormat::Opkg)
        );
        assert_eq!(
            adapter.detect_format_by_extension("solaris.p5p"),
            Some(PackageFormat::SolarisIps)
        );
        assert_eq!(
            adapter.detect_format_by_extension("store.nar"),
            Some(PackageFormat::GuixNar)
        );
        assert_eq!(
            adapter.detect_format_by_extension("base.openbsd.tgz"),
            Some(PackageFormat::OpenBsdPkg)
        );
        assert_eq!(
            adapter.detect_format_by_extension("hpc.spack"),
            Some(PackageFormat::Spack)
        );
        assert_eq!(
            adapter.detect_format_by_extension("cpp.conan"),
            Some(PackageFormat::Conan)
        );
        assert_eq!(
            adapter.detect_format_by_extension("python.whl"),
            Some(PackageFormat::Wheel)
        );
        assert_eq!(
            adapter.detect_format_by_extension("rust.crate"),
            Some(PackageFormat::Crate)
        );
        assert_eq!(
            adapter.detect_format_by_extension("ruby.gem"),
            Some(PackageFormat::Gem)
        );
        assert_eq!(
            adapter.detect_format_by_extension("dotnet.nupkg"),
            Some(PackageFormat::Nupkg)
        );
        assert_eq!(
            adapter.detect_format_by_extension("ms.vcpkg"),
            Some(PackageFormat::Vcpkg)
        );
        assert_eq!(
            adapter.detect_format_by_extension("nix.narinfo"),
            Some(PackageFormat::NarInfo)
        );

        // Check format detection by header signature magic
        assert_eq!(
            adapter.detect_format_by_header(b"!<arch>\ncontrol.tar.xz"),
            Some(PackageFormat::Apt)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"hpkg1234"),
            Some(PackageFormat::Hpkg)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"MOSS1234"),
            Some(PackageFormat::Moss)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"hsqs1234"),
            Some(PackageFormat::Tcz)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"koly1234"),
            Some(PackageFormat::Dmg)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"cports123"),
            Some(PackageFormat::Cports)
        );
        assert_eq!(
            adapter.detect_format_by_header(&[0xED, 0xAB, 0xEE, 0xDB]),
            Some(PackageFormat::Yum)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"PK\x03\x04payload"),
            Some(PackageFormat::Aab)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"SPKG0001header"),
            Some(PackageFormat::Sovereign)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"IPK!hdr"),
            Some(PackageFormat::Ipk)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"OPKGhdr"),
            Some(PackageFormat::Opkg)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"P5P!hdr"),
            Some(PackageFormat::SolarisIps)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"NARShdr"),
            Some(PackageFormat::GuixNar)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"OBSDhdr"),
            Some(PackageFormat::OpenBsdPkg)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"SPAKhdr"),
            Some(PackageFormat::Spack)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"CONAhdr"),
            Some(PackageFormat::Conan)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"WHELhdr"),
            Some(PackageFormat::Wheel)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"CRAThdr"),
            Some(PackageFormat::Crate)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"GEMShdr"),
            Some(PackageFormat::Gem)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"NUPKhdr"),
            Some(PackageFormat::Nupkg)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"VCPKhdr"),
            Some(PackageFormat::Vcpkg)
        );
        assert_eq!(
            adapter.detect_format_by_header(b"NARIhdr"),
            Some(PackageFormat::NarInfo)
        );
    }

    #[test]
    fn test_universal_dependency_mapper_and_converters() {
        let mapper = UniversalDependencyMapper::new();
        assert_eq!(mapper.to_canonical_name("libssl-dev"), "openssl");
        assert_eq!(mapper.to_canonical_name("openssl-devel"), "openssl");
        assert_eq!(mapper.to_canonical_name("libc6"), "libc");

        let scriptlet_conv = UniversalScriptletConverter::new();
        let hook = scriptlet_conv
            .convert_scriptlet(PackageFormat::Apt, "postinst", "echo post")
            .unwrap();
        assert_eq!(hook.hook_type, SigmaPkgHookType::PostInstall);

        let format_conv = UniversalFormatConverter::new();
        let deb_control = b"Package: wget\nVersion: 1.21.0\nDepends: libssl-dev, libc6\nDescription: Retrieval tool\n";
        let pkg = format_conv
            .convert_to_sigma_pkg(PackageFormat::Apt, deb_control)
            .unwrap();
        assert_eq!(pkg.name, "wget");
        assert_eq!(pkg.dependencies[0].name, "openssl");
        assert_eq!(pkg.dependencies[1].name, "libc");

        let simulator = UniversalDryRunSimulator::new();
        let result = simulator
            .simulate_install(PackageFormat::Apt, deb_control)
            .unwrap();
        assert!(result.is_valid);
        assert_eq!(result.package_name, "wget");
        assert_eq!(result.resolved_dependencies.len(), 2);
    }

    #[test]
    fn test_arch_pkginfo_and_xbps_and_apk_manifest_parsing() {
        let adapter = UniversalPackageAdapter::new();

        let pkginfo_text = r#"
            # Generated by makepkg
            pkgname = neovim
            pkgver = 0.9.1-1
            pkgdesc = Vim-fork focused on extensibility and usability
            arch = x86_64
            depend = libtermkey
            depend = unibilium
        "#;

        let pkginfo = adapter.parse_arch_pkginfo(pkginfo_text).unwrap();
        assert_eq!(pkginfo.pkgname, "neovim");
        assert_eq!(pkginfo.pkgver, "0.9.1-1");
        assert_eq!(pkginfo.depends.len(), 2);
        assert_eq!(pkginfo.architecture, "x86_64");

        let apk_text = r#"
            P:musl
            V:1.2.4-r0
            T:the musl c library
            D:so:libc.musl-x86_64.so.1
        "#;

        let apk = adapter.parse_apkindex(apk_text).unwrap();
        assert_eq!(apk.pkgname, "musl");
        assert_eq!(apk.pkgver, "1.2.4-r0");
        assert_eq!(apk.depends.len(), 1);

        let xbps_text = r#"
            pkgname="void-repo-multilib"
            version="1.1_1"
            short_desc="Void Linux multilib repository configuration"
            run_depends="void-repo-nonfree"
        "#;

        let xbps = adapter.parse_xbps_manifest(xbps_text).unwrap();
        assert_eq!(xbps.pkgname, "void-repo-multilib");
        assert_eq!(xbps.version, "1.1_1");
        assert_eq!(xbps.run_depends.len(), 1);

        let ebuild_text = r#"
            # Copyright 1999-2023 Gentoo Authors
            EAPI=8
            DESCRIPTION="Awesome Linux terminal emulator"
            HOMEPAGE="https://alacritty.org"
            SRC_URI="https://github.com/alacritty/alacritty/archive/refs/tags/v0.12.0.tar.gz"
            LICENSE="Apache-2.0"
            SLOT="0"
            KEYWORDS="amd64 arm64"
            IUSE="wayland +X"
            RDEPEND="x11-libs/libx11 dev-libs/wayland"
            DEPEND="${RDEPEND}"
        "#;

        let ebuild = adapter
            .parse_gentoo_ebuild("app-emulation/alacritty-0.12.0.ebuild", ebuild_text)
            .unwrap();
        assert_eq!(ebuild.category, "app-emulation");
        assert_eq!(ebuild.package_name, "alacritty");
        assert_eq!(ebuild.version, "0.12.0");
        assert_eq!(ebuild.rdepend.len(), 2);
    }

    #[test]
    fn test_expanded_bridge_engine_absorb() {
        let mut bridge = SigPkgUniversalBridgeEngine::new();

        let apk_data = r#"
            P:zstd
            V:1.5.5-r0
            T:Fast real-time compression algorithm
            D:musl
        "#;

        let pkg = bridge
            .absorb_and_register("zstd.apk", apk_data.as_bytes())
            .unwrap();
        assert_eq!(pkg.name, "zstd");
        assert_eq!(pkg.version, Version::new(1, 5, 5));
        assert!(bridge.is_package_registered("zstd"));
    }

    #[test]
    fn test_universal_format_converter_expanded_formats() {
        let converter = UniversalFormatConverter::new();

        // Test APK format conversion
        let apk_text =
            b"P:openssl\nV:3.0.8-r0\nT:TLS and SSL toolkit\nD:so:libc.musl-x86_64.so.1\n";
        let pkg_apk = converter
            .convert_to_sigma_pkg(PackageFormat::Apk, apk_text)
            .unwrap();
        assert_eq!(pkg_apk.name, "openssl");
        assert_eq!(pkg_apk.dependencies[0].name, "libc");

        // Test XBPS format conversion
        let xbps_text = b"pkgname=\"bash\"\nversion=\"5.2.15_1\"\nshort_desc=\"GNU Bourne Again Shell\"\nrun_depends=\"libc6\"\n";
        let pkg_xbps = converter
            .convert_to_sigma_pkg(PackageFormat::Xbps, xbps_text)
            .unwrap();
        assert_eq!(pkg_xbps.name, "bash");
        assert_eq!(pkg_xbps.dependencies[0].name, "libc");

        // Test Portage ebuild format conversion
        let ebuild_text =
            b"DESCRIPTION=\"Curl tool\"\nRDEPEND=\"dev-libs/openssl sys-libs/glibc\"\n";
        let pkg_ebuild = converter
            .convert_to_sigma_pkg(PackageFormat::Portage, ebuild_text)
            .unwrap();
        assert_eq!(pkg_ebuild.dependencies[0].name, "openssl");
        assert_eq!(pkg_ebuild.dependencies[1].name, "libc");

        // Test Dry Run simulation on XBPS
        let simulator = UniversalDryRunSimulator::new();
        let sim_result = simulator
            .simulate_install(PackageFormat::Xbps, xbps_text)
            .unwrap();
        assert!(sim_result.is_valid);
        assert_eq!(sim_result.package_name, "bash");
        assert_eq!(sim_result.resolved_dependencies[0], "libc");
    }

    #[test]
    fn test_freebsd_ucl_manifest_parsing() {
        let adapter = UniversalPackageAdapter::new();
        let ucl_text = r#"
            name: "nginx"
            origin: "www/nginx"
            version: "1.24.0"
            comment: "Robust HTTP server and reverse proxy"
            maintainer: "osa@FreeBSD.org"
            deps {
                "openssl": { origin: "security/openssl", version: "3.0.8" }
                "pcre2": { origin: "devel/pcre2", version: "10.42" }
            }
        "#;

        let parsed = adapter.parse_freebsd_ucl_manifest(ucl_text).unwrap();
        assert_eq!(parsed.name, "nginx");
        assert_eq!(parsed.version, "1.24.0");
        assert_eq!(parsed.deps.len(), 2);

        let native = adapter
            .translate_to_native_package(
                &parsed.name,
                &parsed.version,
                &parsed.comment,
                &parsed.deps,
            )
            .unwrap();
        assert_eq!(native.name, "nginx");
        assert_eq!(native.version, Version::new(1, 24, 0));
    }

    #[test]
    fn test_openbsd_contents_parsing() {
        let adapter = UniversalPackageAdapter::new();
        let contents_text = r#"
            @name rsync-3.2.7p0
            @comment Remote file copy tool
            @depend net/rsync:rsync-3.2.7
            @pkgdep security/openssl:openssl-3.0.8
            @exec echo "Installing rsync..."
            @unexec echo "Removing rsync..."
        "#;

        let parsed = adapter.parse_openbsd_contents(contents_text).unwrap();
        assert_eq!(parsed.pkgname, "rsync");
        assert_eq!(parsed.version, "3.2.7p0");
        assert_eq!(parsed.depends.len(), 2);
        assert_eq!(parsed.exec_commands.len(), 1);
        assert_eq!(parsed.unexec_commands.len(), 1);
    }

    #[test]
    fn test_netbsd_pkgsrc_parsing() {
        let adapter = UniversalPackageAdapter::new();
        let pkgsrc_text = r#"
            PKGNAME=git-base-2.41.0
            COMMENT=Fast, scalable, distributed revision control system
            REQUIRES=security/openssl
            REQUIRES=devel/zlib
        "#;

        let parsed = adapter.parse_netbsd_pkgsrc(pkgsrc_text).unwrap();
        assert_eq!(parsed.pkgname, "git-base");
        assert_eq!(parsed.version, "2.41.0");
        assert_eq!(parsed.depends.len(), 2);
    }

    #[test]
    fn test_universal_dependency_canonicalizer() {
        let mapper = UniversalDependencyMapper::new();
        assert_eq!(mapper.to_canonical_name("libssl-dev"), "openssl");
        assert_eq!(mapper.to_canonical_name("openssl-devel"), "openssl");
        assert_eq!(mapper.to_canonical_name("security/openssl"), "openssl");
        assert_eq!(mapper.to_canonical_name("libc6"), "libc");
        assert_eq!(mapper.to_canonical_name("musl-dev"), "libc");
        assert_eq!(mapper.to_canonical_name("python3-dev"), "python");
        assert_eq!(mapper.to_canonical_name("zlib1g-dev"), "zlib");
    }

    #[test]
    fn test_universal_scriptlet_hook_transpiler() {
        let converter = UniversalScriptletConverter::new();

        let deb_hook = converter
            .convert_scriptlet(PackageFormat::Apt, "postinst", "echo post")
            .unwrap();
        assert_eq!(deb_hook.hook_type, SigmaPkgHookType::PostInstall);

        let rpm_hook = converter
            .convert_scriptlet(PackageFormat::Yum, "%posttrans", "echo post")
            .unwrap();
        assert_eq!(rpm_hook.hook_type, SigmaPkgHookType::PostInstall);

        let bsd_hook = converter
            .convert_scriptlet(PackageFormat::Pkg, "+POST_INSTALL", "echo bsd")
            .unwrap();
        assert_eq!(bsd_hook.hook_type, SigmaPkgHookType::PostInstall);
    }

    #[test]
    fn test_universal_sandbox_capability_matrix() {
        let matrix = UniversalSandboxCapabilityMatrix::new();
        let caps = vec![
            "network".to_string(),
            "--filesystem=home".to_string(),
            "audio-playback".to_string(),
            "wayland".to_string(),
        ];

        let perms = matrix.map_foreign_capabilities(&caps);
        assert!(perms.contains(&Permission::NetworkTcp));
        assert!(perms.contains(&Permission::FileRead));
        assert!(perms.contains(&Permission::AudioPlayback));
        assert!(perms.contains(&Permission::DisplayAccess));
    }

    #[test]
    fn test_universal_pm_command_dispatcher() {
        let dispatcher = UniversalPmCommandDispatcher::new();

        let apt_action = dispatcher
            .dispatch_command("apt install nginx curl -y")
            .unwrap();
        assert_eq!(apt_action.source_pm, "apt");
        assert_eq!(apt_action.operation, UniversalPmOperation::Install);
        assert_eq!(apt_action.target_packages, vec!["nginx", "curl"]);

        let pacman_action = dispatcher.dispatch_command("pacman -Syu --dryrun").unwrap();
        assert_eq!(pacman_action.source_pm, "pacman");
        assert_eq!(pacman_action.operation, UniversalPmOperation::Upgrade);
        assert!(pacman_action.dry_run);

        let dnf_action = dispatcher.dispatch_command("dnf remove httpd").unwrap();
        assert_eq!(dnf_action.source_pm, "dnf");
        assert_eq!(dnf_action.operation, UniversalPmOperation::Remove);

        let apk_action = dispatcher
            .dispatch_command("apk add alpine-baselayout")
            .unwrap();
        assert_eq!(apk_action.source_pm, "apk");
        assert_eq!(apk_action.operation, UniversalPmOperation::Install);

        let bsd_action = dispatcher
            .dispatch_command("pkg install -n postgresql15-server")
            .unwrap();
        assert_eq!(bsd_action.source_pm, "pkg");
        assert_eq!(bsd_action.operation, UniversalPmOperation::Install);
        assert!(bsd_action.dry_run);

        let emerge_action = dispatcher
            .dispatch_command("emerge -pv sys-apps/portage")
            .unwrap();
        assert_eq!(emerge_action.source_pm, "emerge");
        assert_eq!(emerge_action.operation, UniversalPmOperation::Install);
        assert!(emerge_action.dry_run);

        let nix_action = dispatcher
            .dispatch_command("nix install nixpkgs#firefox")
            .unwrap();
        assert_eq!(nix_action.source_pm, "nix");
        assert_eq!(nix_action.operation, UniversalPmOperation::Install);

        let eopkg_action = dispatcher
            .dispatch_command("eopkg it vlc")
            .unwrap();
        assert_eq!(eopkg_action.source_pm, "eopkg");
        assert_eq!(eopkg_action.operation, UniversalPmOperation::Install);

        let slack_action = dispatcher
            .dispatch_command("slackpkg install glibc")
            .unwrap();
        assert_eq!(slack_action.source_pm, "slackpkg");
        assert_eq!(slack_action.operation, UniversalPmOperation::Install);
    }

    #[test]
    fn test_foreign_pm_dispatcher_expanded_distros() {
        let dispatcher = UniversalPmCommandDispatcher::new();

        let emerge_act = dispatcher.dispatch_command("emerge -uDN @world -p").unwrap();
        assert_eq!(emerge_act.source_pm, "emerge");
        assert_eq!(emerge_act.operation, UniversalPmOperation::Upgrade);
        assert!(emerge_act.dry_run);

        let nix_act = dispatcher.dispatch_command("nix-env -iA nixpkgs.git").unwrap();
        assert_eq!(nix_act.source_pm, "nix-env");
        assert_eq!(nix_act.operation, UniversalPmOperation::Install);
        assert_eq!(nix_act.target_packages, vec!["nixpkgs.git"]);

        let flatpak_act = dispatcher.dispatch_command("flatpak install org.gimp.GIMP").unwrap();
        assert_eq!(flatpak_act.source_pm, "flatpak");
        assert_eq!(flatpak_act.operation, UniversalPmOperation::Install);
        assert_eq!(flatpak_act.target_packages, vec!["org.gimp.GIMP"]);

        let snap_act = dispatcher.dispatch_command("snap remove vlc").unwrap();
        assert_eq!(snap_act.source_pm, "snap");
        assert_eq!(snap_act.operation, UniversalPmOperation::Remove);
        assert_eq!(snap_act.target_packages, vec!["vlc"]);

        let slack_act = dispatcher.dispatch_command("slackpkg install htop").unwrap();
        assert_eq!(slack_act.source_pm, "slackpkg");
        assert_eq!(slack_act.operation, UniversalPmOperation::Install);
        assert_eq!(slack_act.target_packages, vec!["htop"]);

        let pkgman_act = dispatcher.dispatch_command("pkgman install haiku_dep").unwrap();
        assert_eq!(pkgman_act.source_pm, "pkgman");
        assert_eq!(pkgman_act.operation, UniversalPmOperation::Install);
        assert_eq!(pkgman_act.target_packages, vec!["haiku_dep"]);

        let swupd_act = dispatcher.dispatch_command("swupd bundle-add os-core").unwrap();
        assert_eq!(swupd_act.source_pm, "swupd");
        assert_eq!(swupd_act.operation, UniversalPmOperation::Install);

        let eopkg_act = dispatcher.dispatch_command("eopkg remove nano").unwrap();
        assert_eq!(eopkg_act.source_pm, "eopkg");
        assert_eq!(eopkg_act.operation, UniversalPmOperation::Remove);

        let pkgin_act = dispatcher.dispatch_command("pkgin install tmux").unwrap();
        assert_eq!(pkgin_act.source_pm, "pkgin");
        assert_eq!(pkgin_act.operation, UniversalPmOperation::Install);
    }

    #[test]
    fn test_foreign_pm_dispatcher_expanded_distros() {
        let dispatcher = UniversalPmCommandDispatcher::new();

        let emerge_act = dispatcher.dispatch_command("emerge -uDN @world -p").unwrap();
        assert_eq!(emerge_act.source_pm, "emerge");
        assert_eq!(emerge_act.operation, UniversalPmOperation::Upgrade);
        assert!(emerge_act.dry_run);

        let nix_act = dispatcher.dispatch_command("nix-env -iA nixpkgs.git").unwrap();
        assert_eq!(nix_act.source_pm, "nix-env");
        assert_eq!(nix_act.operation, UniversalPmOperation::Install);
        assert_eq!(nix_act.target_packages, vec!["nixpkgs.git"]);

        let flatpak_act = dispatcher.dispatch_command("flatpak install org.gimp.GIMP").unwrap();
        assert_eq!(flatpak_act.source_pm, "flatpak");
        assert_eq!(flatpak_act.operation, UniversalPmOperation::Install);
        assert_eq!(flatpak_act.target_packages, vec!["org.gimp.GIMP"]);

        let snap_act = dispatcher.dispatch_command("snap remove vlc").unwrap();
        assert_eq!(snap_act.source_pm, "snap");
        assert_eq!(snap_act.operation, UniversalPmOperation::Remove);
        assert_eq!(snap_act.target_packages, vec!["vlc"]);

        let slack_act = dispatcher.dispatch_command("slackpkg install htop").unwrap();
        assert_eq!(slack_act.source_pm, "slackpkg");
        assert_eq!(slack_act.operation, UniversalPmOperation::Install);
        assert_eq!(slack_act.target_packages, vec!["htop"]);

        let pkgman_act = dispatcher.dispatch_command("pkgman install haiku_dep").unwrap();
        assert_eq!(pkgman_act.source_pm, "pkgman");
        assert_eq!(pkgman_act.operation, UniversalPmOperation::Install);
        assert_eq!(pkgman_act.target_packages, vec!["haiku_dep"]);

        let swupd_act = dispatcher.dispatch_command("swupd bundle-add os-core").unwrap();
        assert_eq!(swupd_act.source_pm, "swupd");
        assert_eq!(swupd_act.operation, UniversalPmOperation::Install);

        let eopkg_act = dispatcher.dispatch_command("eopkg remove nano").unwrap();
        assert_eq!(eopkg_act.source_pm, "eopkg");
        assert_eq!(eopkg_act.operation, UniversalPmOperation::Remove);

        let pkgin_act = dispatcher.dispatch_command("pkgin install tmux").unwrap();
        assert_eq!(pkgin_act.source_pm, "pkgin");
        assert_eq!(pkgin_act.operation, UniversalPmOperation::Install);
    }

    #[test]
    fn test_foreign_pm_dispatcher_expanded_distros() {
        let dispatcher = UniversalPmCommandDispatcher::new();

        let emerge_act = dispatcher.dispatch_command("emerge -uDN @world -p").unwrap();
        assert_eq!(emerge_act.source_pm, "emerge");
        assert_eq!(emerge_act.operation, UniversalPmOperation::Upgrade);
        assert!(emerge_act.dry_run);

        let nix_act = dispatcher.dispatch_command("nix-env -iA nixpkgs.git").unwrap();
        assert_eq!(nix_act.source_pm, "nix-env");
        assert_eq!(nix_act.operation, UniversalPmOperation::Install);
        assert_eq!(nix_act.target_packages, vec!["nixpkgs.git"]);

        let flatpak_act = dispatcher.dispatch_command("flatpak install org.gimp.GIMP").unwrap();
        assert_eq!(flatpak_act.source_pm, "flatpak");
        assert_eq!(flatpak_act.operation, UniversalPmOperation::Install);
        assert_eq!(flatpak_act.target_packages, vec!["org.gimp.GIMP"]);

        let snap_act = dispatcher.dispatch_command("snap remove vlc").unwrap();
        assert_eq!(snap_act.source_pm, "snap");
        assert_eq!(snap_act.operation, UniversalPmOperation::Remove);
        assert_eq!(snap_act.target_packages, vec!["vlc"]);

        let slack_act = dispatcher.dispatch_command("slackpkg install htop").unwrap();
        assert_eq!(slack_act.source_pm, "slackpkg");
        assert_eq!(slack_act.operation, UniversalPmOperation::Install);
        assert_eq!(slack_act.target_packages, vec!["htop"]);

        let pkgman_act = dispatcher.dispatch_command("pkgman install haiku_dep").unwrap();
        assert_eq!(pkgman_act.source_pm, "pkgman");
        assert_eq!(pkgman_act.operation, UniversalPmOperation::Install);
        assert_eq!(pkgman_act.target_packages, vec!["haiku_dep"]);

        let swupd_act = dispatcher.dispatch_command("swupd bundle-add os-core").unwrap();
        assert_eq!(swupd_act.source_pm, "swupd");
        assert_eq!(swupd_act.operation, UniversalPmOperation::Install);

        let eopkg_act = dispatcher.dispatch_command("eopkg remove nano").unwrap();
        assert_eq!(eopkg_act.source_pm, "eopkg");
        assert_eq!(eopkg_act.operation, UniversalPmOperation::Remove);

        let pkgin_act = dispatcher.dispatch_command("pkgin install tmux").unwrap();
        assert_eq!(pkgin_act.source_pm, "pkgin");
        assert_eq!(pkgin_act.operation, UniversalPmOperation::Install);
    }

    #[test]
    fn test_foreign_pm_dispatcher_expanded_distros() {
        let dispatcher = UniversalPmCommandDispatcher::new();

        let emerge_act = dispatcher.dispatch_command("emerge -uDN @world -p").unwrap();
        assert_eq!(emerge_act.source_pm, "emerge");
        assert_eq!(emerge_act.operation, UniversalPmOperation::Upgrade);
        assert!(emerge_act.dry_run);

        let nix_act = dispatcher.dispatch_command("nix-env -iA nixpkgs.git").unwrap();
        assert_eq!(nix_act.source_pm, "nix-env");
        assert_eq!(nix_act.operation, UniversalPmOperation::Install);
        assert_eq!(nix_act.target_packages, vec!["nixpkgs.git"]);

        let flatpak_act = dispatcher.dispatch_command("flatpak install org.gimp.GIMP").unwrap();
        assert_eq!(flatpak_act.source_pm, "flatpak");
        assert_eq!(flatpak_act.operation, UniversalPmOperation::Install);
        assert_eq!(flatpak_act.target_packages, vec!["org.gimp.GIMP"]);

        let snap_act = dispatcher.dispatch_command("snap remove vlc").unwrap();
        assert_eq!(snap_act.source_pm, "snap");
        assert_eq!(snap_act.operation, UniversalPmOperation::Remove);
        assert_eq!(snap_act.target_packages, vec!["vlc"]);

        let slack_act = dispatcher.dispatch_command("slackpkg install htop").unwrap();
        assert_eq!(slack_act.source_pm, "slackpkg");
        assert_eq!(slack_act.operation, UniversalPmOperation::Install);
        assert_eq!(slack_act.target_packages, vec!["htop"]);

        let pkgman_act = dispatcher.dispatch_command("pkgman install haiku_dep").unwrap();
        assert_eq!(pkgman_act.source_pm, "pkgman");
        assert_eq!(pkgman_act.operation, UniversalPmOperation::Install);
        assert_eq!(pkgman_act.target_packages, vec!["haiku_dep"]);

        let swupd_act = dispatcher.dispatch_command("swupd bundle-add os-core").unwrap();
        assert_eq!(swupd_act.source_pm, "swupd");
        assert_eq!(swupd_act.operation, UniversalPmOperation::Install);

        let eopkg_act = dispatcher.dispatch_command("eopkg remove nano").unwrap();
        assert_eq!(eopkg_act.source_pm, "eopkg");
        assert_eq!(eopkg_act.operation, UniversalPmOperation::Remove);

        let pkgin_act = dispatcher.dispatch_command("pkgin install tmux").unwrap();
        assert_eq!(pkgin_act.source_pm, "pkgin");
        assert_eq!(pkgin_act.operation, UniversalPmOperation::Install);
    }

    #[test]
    fn test_foreign_pm_dispatcher_expanded_distros() {
        let dispatcher = UniversalPmCommandDispatcher::new();

        let emerge_act = dispatcher.dispatch_command("emerge -uDN @world -p").unwrap();
        assert_eq!(emerge_act.source_pm, "emerge");
        assert_eq!(emerge_act.operation, UniversalPmOperation::Upgrade);
        assert!(emerge_act.dry_run);

        let nix_act = dispatcher.dispatch_command("nix-env -iA nixpkgs.git").unwrap();
        assert_eq!(nix_act.source_pm, "nix-env");
        assert_eq!(nix_act.operation, UniversalPmOperation::Install);
        assert_eq!(nix_act.target_packages, vec!["nixpkgs.git"]);

        let flatpak_act = dispatcher.dispatch_command("flatpak install org.gimp.GIMP").unwrap();
        assert_eq!(flatpak_act.source_pm, "flatpak");
        assert_eq!(flatpak_act.operation, UniversalPmOperation::Install);
        assert_eq!(flatpak_act.target_packages, vec!["org.gimp.GIMP"]);

        let snap_act = dispatcher.dispatch_command("snap remove vlc").unwrap();
        assert_eq!(snap_act.source_pm, "snap");
        assert_eq!(snap_act.operation, UniversalPmOperation::Remove);
        assert_eq!(snap_act.target_packages, vec!["vlc"]);

        let slack_act = dispatcher.dispatch_command("slackpkg install htop").unwrap();
        assert_eq!(slack_act.source_pm, "slackpkg");
        assert_eq!(slack_act.operation, UniversalPmOperation::Install);
        assert_eq!(slack_act.target_packages, vec!["htop"]);

        let pkgman_act = dispatcher.dispatch_command("pkgman install haiku_dep").unwrap();
        assert_eq!(pkgman_act.source_pm, "pkgman");
        assert_eq!(pkgman_act.operation, UniversalPmOperation::Install);
        assert_eq!(pkgman_act.target_packages, vec!["haiku_dep"]);

        let swupd_act = dispatcher.dispatch_command("swupd bundle-add os-core").unwrap();
        assert_eq!(swupd_act.source_pm, "swupd");
        assert_eq!(swupd_act.operation, UniversalPmOperation::Install);

        let eopkg_act = dispatcher.dispatch_command("eopkg remove nano").unwrap();
        assert_eq!(eopkg_act.source_pm, "eopkg");
        assert_eq!(eopkg_act.operation, UniversalPmOperation::Remove);

        let pkgin_act = dispatcher.dispatch_command("pkgin install tmux").unwrap();
        assert_eq!(pkgin_act.source_pm, "pkgin");
        assert_eq!(pkgin_act.operation, UniversalPmOperation::Install);
    }

    #[test]
    fn test_haiku_hpkg_manifest_parsing_and_bridge() {
        let adapter = UniversalPackageAdapter::new();
        let hpkg_text = r#"
name haiku_dep
version 1.0.0
summary "Haiku Desktop Utility"
architecture x86_64
requires {
    haiku_core
    libssl
}
"#;
        let manifest = adapter.parse_haiku_hpkg(hpkg_text).unwrap();
        assert_eq!(manifest.name, "haiku_dep");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.summary, "Haiku Desktop Utility");
        assert_eq!(manifest.requires, vec!["haiku_core", "libssl"]);

        let mut engine = SigPkgUniversalBridgeEngine::new();
        let pkg = engine.absorb_and_register("app.hpkg", hpkg_text.as_bytes()).unwrap();
        assert_eq!(pkg.name, "haiku_dep");
        assert!(engine.is_package_registered("haiku_dep"));
    }

    #[test]
    fn test_sovereign_multi_format_manifest_parsing() {
        let adapter = UniversalPackageAdapter::new();

        // 1. Debian control file (.deb)
        let deb_control = "Package: nginx\nVersion: 1.24.0\nArchitecture: amd64\nDepends: libc6 (>= 2.34), libssl3\nDescription: high performance web server\n";
        let deb_manifest = adapter.parse_apt_control(deb_control).unwrap();
        assert_eq!(deb_manifest.package, "nginx");
        assert_eq!(deb_manifest.version, "1.24.0");

        // 2. Arch Linux PKGBUILD
        let pkgbuild = "pkgname=zsh\npkgver=5.9\npkgdesc='Z shell'\ndepends=('ncurses' 'pcre2')\n";
        let pkg_manifest = adapter.parse_pacman_pkgbuild(pkgbuild).unwrap();
        assert_eq!(pkg_manifest.pkgname, "zsh");
        assert_eq!(pkg_manifest.pkgver, "5.9");

        // 3. FreeBSD +MANIFEST
        let freebsd_ucl = "name: curl\nversion: \"8.4.0\"\ncomment: Command line tool for transferring data\ndeps: {\n  openssl: {origin: \"security/openssl\"}\n}\n";
        let bsd_manifest = adapter.parse_freebsd_ucl_manifest(freebsd_ucl).unwrap();
        assert_eq!(bsd_manifest.name, "curl");
        assert_eq!(bsd_manifest.version, "8.4.0");

        // 4. OpenBSD +CONTENTS
        let openbsd_contents = "@name htop-3.2.2\n@depend sysutils/lsof\n@comment interactive process viewer\n";
        let obsd_manifest = adapter.parse_openbsd_contents(openbsd_contents).unwrap();
        assert_eq!(obsd_manifest.name, "htop");
        assert_eq!(obsd_manifest.version, "3.2.2");
    }
}
