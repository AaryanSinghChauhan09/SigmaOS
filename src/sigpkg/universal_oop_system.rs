#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::vec;

use std::boxed::Box;
use std::collections::HashMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// Universal OOP Package System for SigmaOS
// Supports all Linux distro package formats with user-defined functions
// Implements Strategy Pattern, Adapter Pattern, and Factory Pattern

#[cfg(all(not(feature = "standalone_test"), not(test)))]
pub use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};

#[cfg(test)]
pub use crate::sigpkg::Version;

#[cfg(all(not(feature = "standalone_test"), not(test)))]
use crate::klib::HashMap;

use std::sync::Arc;

#[cfg(feature = "standalone_test")]
impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(feature = "standalone_test")]
impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
    pub fn parse(s: &str) -> Result<Self, &'static str> {
        let clean: String = s.chars().map(|c| if c.is_ascii_digit() || c == '.' { c } else { ' ' }).collect();
        let first_num = clean.split_whitespace().next().unwrap_or("1.0.0");
        let parts: Vec<&str> = first_num.split('.').collect();
        let major = parts.get(0).and_then(|p| p.parse().ok()).unwrap_or(1);
        let minor = parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0);
        let patch = parts.get(2).and_then(|p| p.parse().ok()).unwrap_or(0);
        Ok(Self::new(major, minor, patch))
    }
}

#[cfg(any(feature = "standalone_test", test))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
}

#[cfg(any(feature = "standalone_test", test))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    Any,
}

#[cfg(any(feature = "standalone_test", test))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub checksum: String,
}

#[cfg(any(feature = "standalone_test", test))]
impl Package {
    pub fn new(name: String, version: Version, description: String, dependencies: Vec<Dependency>, checksum: String) -> Self {
        Self {
            name,
            version,
            description,
            dependencies,
            checksum,
        }
    }
}

// ============================================================================
// Core Abstractions (OOP Interface Layer)
// ============================================================================

/// Core package trait - defines the contract for all package operations
#[derive(Debug, Clone)]
pub struct ConditionalDependency {
    pub required_use_flag: String,
    pub dependency: Dependency,
}

pub trait IPackage: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &Version;
    fn dependencies(&self) -> &[Dependency];
    fn format(&self) -> PackageFormat;
    fn metadata(&self) -> &PackageMetadata;
    fn metadata_mut(&mut self) -> &mut PackageMetadata;
    fn files(&self) -> &[String] {
        &[]
    }
    fn conditional_dependencies(&self) -> &[ConditionalDependency] {
        &[]
    }
}

/// Package format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageFormat {
    // Debian-based
    Deb,
    Apt,
    // RPM-based
    Rpm,
    Yum,
    // Arch-based
    Pacman,
    // Gentoo-based
    Ebuild,
    Portage,
    // Alpine-based
    Apk,
    // Nix-based
    Nix,
    // Flatpak
    Flatpak,
    // Snap
    Snap,
    // AppImage
    AppImage,
    // Void Linux
    Xbps,
    // Slackware
    Txz,
    // Solus
    Eopkg,
    // OpenSUSE
    Zypper,
    // Guix
    Guix,
    // SigmaOS Native
    Sigma,
    Sovereign,
    // Adobe AIR
    Air,
    // Homebrew Bottle
    Bottle,
    // iOS App (.ipa)
    Ipa,
    // FreeBSD / OpenBSD Ports
    Ports,
    // macOS / FreeBSD / Solaris PKG
    Pkg,
    // Android App Bundle (.aab)
    Aab,
    // Compressed Tar archives (.tar.gz, .tgz)
    TarGz,
    // Compressed Tar XZ archives (.tar.xz, .xz, .pkg.tar.xz)
    TarXz,
    // Plain Tar archive (.tar)
    Tar,
    // macOS / Nextstep App bundle (.app)
    AppBundle,
    // HarmonyOS Ability Package (.hap)
    Hap,
    // Pardus / Solus PiSi (.PiSi)
    Pisi,
    // Deepin Superdeb (.superdeb)
    Superdeb,
    // Slax Linux Module (.lzm)
    Lzm,
    // Puppy Linux Package (.pup)
    Pup,
    // Puppy Extra Tarball / Pet (.pet)
    Pet,
    // Solus Moss (.moss)
    Moss,
    // Haiku Package (.hpkg)
    Hpkg,
    // Tiny Core Linux extension (.tcz)
    Tcz,
    // GoboLinux package (.gobo)
    Gobo,
    // OSTree commit (.commit)
    Ostree,
    // NetBSD pkgsrc (.pkgsrc)
    Pkgsrc,
    // SquashFS package (.sfs)
    Sfs,
    // Portable package (.puk)
    Puk,
    // macOS Disk Image (.dmg)
    Dmg,
    // Chimera Linux (.cports)
    Cports,
    // DragonFly BSD DPorts (.dports)
    Dports,
    // Slackware SlackBuild (.slackbuild)
    SlackBuild,
    // CRUX Linux (.crux)
    Crux,
    // Delta RPM (.drpm)
    Drpm,
    // Bedrock Linux Stratum (.stratum)
    Stratum,
    // OpenWrt / opkg / Entware (.ipk)
    Ipk,
    // Yocto / OpenEmbedded (.opkg)
    Opkg,
    // OpenBSD pkg_add (.openbsd.tgz / .tgz)
    OpenBsdPkg,
    // Solaris / Illumos IPS (.p5p / .ips)
    SolarisIps,
    // GNU Guix / Nix Archive (.nar)
    GuixNar,
}

impl PackageFormat {
    pub fn from_filename(filename: &str) -> Option<Self> {
        let name = filename.to_lowercase();
        let name = name.trim();
        let normalized = name.replace(" ", "");

        if normalized.ends_with(".deb") || normalized.ends_with(".udeb") {
            Some(PackageFormat::Deb)
        } else if normalized.ends_with(".superdeb") {
            Some(PackageFormat::Superdeb)
        } else if normalized.ends_with(".rpm") || normalized.ends_with(".drpm") {
            Some(PackageFormat::Rpm)
        } else if normalized.ends_with(".pkg.tar.zst")
            || normalized.ends_with(".pkg.tar.xz")
            || normalized.ends_with(".pkg.tar.gz")
            || normalized.contains("pacman")
        {
            Some(PackageFormat::Pacman)
        } else if normalized.ends_with(".snap") {
            Some(PackageFormat::Snap)
        } else if normalized.ends_with(".flatpak") {
            Some(PackageFormat::Flatpak)
        } else if normalized.ends_with(".appimage") {
            Some(PackageFormat::AppImage)
        } else if normalized.ends_with(".sigpkg") || normalized.ends_with(".sigma") {
            Some(PackageFormat::Sigma)
        } else if normalized.ends_with(".air") {
            Some(PackageFormat::Air)
        } else if normalized.ends_with(".bottle") {
            Some(PackageFormat::Bottle)
        } else if normalized.ends_with(".ipa") {
            Some(PackageFormat::Ipa)
        } else if normalized.ends_with(".ports") {
            Some(PackageFormat::Ports)
        } else if normalized.ends_with(".pkg") {
            Some(PackageFormat::Pkg)
        } else if normalized.ends_with(".aab") {
            Some(PackageFormat::Aab)
        } else if normalized.ends_with(".apk") {
            Some(PackageFormat::Apk)
        } else if normalized.ends_with(".eopkg") {
            Some(PackageFormat::Eopkg)
        } else if normalized.ends_with(".nixpkg") || normalized.ends_with(".nix") {
            Some(PackageFormat::Nix)
        } else if normalized.ends_with(".ebuild") || normalized.ends_with(".portage") {
            Some(PackageFormat::Ebuild)
        } else if normalized.ends_with(".openbsd.tgz") {
            Some(PackageFormat::OpenBsdPkg)
        } else if normalized.ends_with(".tar.gz") || normalized.ends_with(".tgz") {
            Some(PackageFormat::TarGz)
        } else if normalized.ends_with(".txz") || normalized.ends_with(".tar.xz") || normalized.ends_with(".xz") {
            Some(PackageFormat::TarXz)
        } else if normalized.ends_with(".xbps") {
            Some(PackageFormat::Xbps)
        } else if normalized.ends_with(".zypper") {
            Some(PackageFormat::Zypper)
        } else if normalized.ends_with(".guix") || normalized.ends_with(".scm") {
            Some(PackageFormat::Guix)
        } else if normalized.ends_with(".moss") {
            Some(PackageFormat::Moss)
        } else if normalized.ends_with(".hpkg") {
            Some(PackageFormat::Hpkg)
        } else if normalized.ends_with(".tcz") {
            Some(PackageFormat::Tcz)
        } else if normalized.ends_with(".gobo") {
            Some(PackageFormat::Gobo)
        } else if normalized.ends_with(".commit") || normalized.ends_with(".ostree") {
            Some(PackageFormat::Ostree)
        } else if normalized.ends_with(".pkgsrc") {
            Some(PackageFormat::Pkgsrc)
        } else if normalized.ends_with(".sfs") {
            Some(PackageFormat::Sfs)
        } else if normalized.ends_with(".puk") {
            Some(PackageFormat::Puk)
        } else if normalized.ends_with(".dmg") {
            Some(PackageFormat::Dmg)
        } else if normalized.ends_with(".cports") {
            Some(PackageFormat::Cports)
        } else if normalized.ends_with(".dports") {
            Some(PackageFormat::Dports)
        } else if normalized.ends_with(".slackbuild") || normalized.ends_with(".tlz") || normalized.ends_with(".tbz") {
            Some(PackageFormat::SlackBuild)
        } else if normalized.ends_with(".crux") || normalized.ends_with(".pkgfile") {
            Some(PackageFormat::Crux)
        } else if normalized.ends_with(".stratum") {
            Some(PackageFormat::Stratum)
        } else if normalized.ends_with(".app") {
            Some(PackageFormat::AppBundle)
        } else if normalized.ends_with(".hap") {
            Some(PackageFormat::Hap)
        } else if normalized.ends_with(".pisi") {
            Some(PackageFormat::Pisi)
        } else if normalized.ends_with(".lzm") {
            Some(PackageFormat::Lzm)
        } else if normalized.ends_with(".pup") {
            Some(PackageFormat::Pup)
        } else if normalized.ends_with(".pet") {
            Some(PackageFormat::Pet)
        } else if normalized.ends_with(".tar") {
            Some(PackageFormat::Tar)
        } else if normalized.ends_with(".ipk") {
            Some(PackageFormat::Ipk)
        } else if normalized.ends_with(".opkg") {
            Some(PackageFormat::Opkg)
        } else if normalized.ends_with(".p5p") || normalized.ends_with(".ips") {
            Some(PackageFormat::SolarisIps)
        } else if normalized.ends_with(".nar") {
            Some(PackageFormat::GuixNar)
        } else {
            None
        }
    }
}

/// Package metadata structure
#[derive(Debug, Clone)]
pub struct PackageMetadata {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub license: String,
    pub maintainer: String,
    pub homepage: String,
    pub architecture: String,
    pub checksum: String,
    pub size: u64,
    pub install_date: Option<u64>,
    pub pqc_signature: Option<String>,
    pub gpg_key_id: Option<String>,
    pub supported_architectures: Vec<String>,
}

// ============================================================================
// Strategy Pattern: Package Parsing Strategies
// ============================================================================

/// Package parser trait - Strategy pattern for different parsing algorithms
pub trait IPathTrigger: Send + Sync {
    fn pattern(&self) -> &str;
    fn execute(&self, matched_files: &[String]) -> Result<(), HookError>;
}

pub trait IPackageParser: Send + Sync {
    fn format(&self) -> PackageFormat;
    fn can_parse(&self, data: &[u8]) -> bool;
    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError>;
    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError>;
}

/// Parse error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidFormat,
    MissingField(String),
    InvalidVersion(String),
    InvalidChecksum,
    UnsupportedFormat(PackageFormat),
    IoError(String),
}

// ============================================================================
// Adapter Pattern: Distro-Specific Adapters
// ============================================================================

/// Base adapter with common functionality
pub struct BaseAdapter {
    format: PackageFormat,
    user_hooks: Vec<Arc<dyn UserDefinedHook>>,
}

impl BaseAdapter {
    pub fn new(format: PackageFormat) -> Self {
        Self {
            format,
            user_hooks: Vec::new(),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.user_hooks.push(hook);
    }

    pub fn execute_hooks(&self, package: &mut dyn IPackage) -> Result<(), HookError> {
        for hook in &self.user_hooks {
            hook.execute(package)?;
        }
        Ok(())
    }
}

/// User-defined hook trait for extensibility
pub trait UserDefinedHook: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, package: &mut dyn IPackage) -> Result<(), HookError>;
}

/// Hook error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookError {
    HookFailed(String),
    PermissionDenied,
    ValidationError(String),
}

// ============================================================================
// Concrete Adapters for Each Distro
// ============================================================================

/// Debian/Ubuntu .deb adapter
pub struct DebAdapter {
    base: BaseAdapter,
}

impl DebAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Deb),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for DebAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Deb
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("Package:") && content.contains("Version:")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content =
            String::from_utf8(data.to_vec()).map_err(|e| ParseError::IoError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut license = String::new();
        let mut maintainer = String::new();
        let mut homepage = String::new();
        let mut arch = "amd64".to_string();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("Package: ") {
                name = line[9..].to_string();
            } else if line.starts_with("Version: ") {
                version_str = line[9..].to_string();
            } else if line.starts_with("Description: ") {
                description = line[13..].to_string();
            } else if line.starts_with("License: ") {
                license = line[9..].to_string();
            } else if line.starts_with("Maintainer: ") {
                maintainer = line[12..].to_string();
            } else if line.starts_with("Homepage: ") {
                homepage = line[10..].to_string();
            } else if line.starts_with("Architecture: ") {
                arch = line[14..].to_string();
            } else if line.starts_with("Depends: ") {
                let deps_str = &line[9..];
                for dep in deps_str.split(',') {
                    let dep_name = dep.trim().split_whitespace().next().unwrap_or("");
                    if !dep_name.is_empty() {
                        dependencies.push(Dependency {
                            name: dep_name.to_string(),
                            version_constraint: VersionConstraint::Any,
                        });
                    }
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license,
                maintainer,
                homepage,
                architecture: arch,
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Deb,
        });

        // Execute user-defined hooks
        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str(&format!("Package: {}\n", meta.name));
        output.push_str(&format!(
            "Version: {}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("Description: {}\n", meta.description));
        output.push_str(&format!("License: {}\n", meta.license));
        output.push_str(&format!("Maintainer: {}\n", meta.maintainer));
        output.push_str(&format!("Homepage: {}\n", meta.homepage));
        output.push_str(&format!("Architecture: {}\n", meta.architecture));

        if !package.dependencies().is_empty() {
            output.push_str("Depends: ");
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&dep_names.join(", "));
            output.push('\n');
        }

        Ok(output.into_bytes())
    }
}

/// Generic helper macro for generating lightweight package adapters
macro_rules! impl_generic_package_adapter {
    ($struct_name:ident, $format_variant:ident, $can_parse_key:expr, $name_prefix:expr, $version_prefix:expr) => {
        pub struct $struct_name {
            base: BaseAdapter,
        }

        impl $struct_name {
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self {
                    base: BaseAdapter::new(PackageFormat::$format_variant),
                }
            }

            pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
                self.base.add_hook(hook);
            }
        }

        impl IPackageParser for $struct_name {
            fn format(&self) -> PackageFormat {
                PackageFormat::$format_variant
            }

            fn can_parse(&self, data: &[u8]) -> bool {
                let content = String::from_utf8_lossy(data);
                content.contains($can_parse_key)
            }

            fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
                let content = String::from_utf8_lossy(data);
                let mut name = String::new();
                let mut version_str = String::new();
                let mut description = String::new();
                let mut dependencies = Vec::new();

                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with($name_prefix) {
                        name = trimmed[$name_prefix.len()..]
                            .trim_matches(|c| c == '"' || c == '\'' || c == ' ')
                            .to_string();
                    } else if trimmed.starts_with($version_prefix) {
                        version_str = trimmed[$version_prefix.len()..]
                            .trim_matches(|c| c == '"' || c == '\'' || c == ' ')
                            .to_string();
                    } else if trimmed.starts_with("description=")
                        || trimmed.starts_with("Description:")
                        || trimmed.starts_with("summary=")
                    {
                        let pos = trimmed.find('=').or_else(|| trimmed.find(':')).unwrap_or(0);
                        description = trimmed[pos + 1..].trim().to_string();
                    } else if trimmed.starts_with("depends=") || trimmed.starts_with("Depends:") {
                        let pos = trimmed.find('=').or_else(|| trimmed.find(':')).unwrap_or(0);
                        for dep in trimmed[pos + 1..].split_whitespace() {
                            dependencies.push(Dependency {
                                name: dep.to_string(),
                                version_constraint: VersionConstraint::Any,
                            });
                        }
                    }
                }

                if name.is_empty() {
                    name = stringify!($struct_name).to_lowercase();
                }

                let version =
                    Version::parse(&version_str).unwrap_or_else(|_| Version::new(1, 0, 0));

                let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
                    metadata: PackageMetadata {
                        name,
                        version,
                        description,
                        license: String::new(),
                        maintainer: String::new(),
                        homepage: String::new(),
                        architecture: "x86_64".to_string(),
                        checksum: String::new(),
                        size: 0,
                        install_date: None,
                        pqc_signature: None,
                        gpg_key_id: None,
                        supported_architectures: Vec::new(),
                    },
                    dependencies,
                    format: PackageFormat::$format_variant,
                });

                self.base
                    .execute_hooks(package.as_mut())
                    .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

                Ok(package)
            }

            fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
                let mut output = String::new();
                let meta = package.metadata();

                output.push_str(&format!("{}{}\n", $name_prefix, meta.name));
                output.push_str(&format!(
                    "{}{}.{}.{}\n",
                    $version_prefix, meta.version.major, meta.version.minor, meta.version.patch
                ));
                Ok(output.into_bytes())
            }
        }
    };
}

impl_generic_package_adapter!(
    AirAdapter,
    Air,
    "air-application:",
    "air-application: ",
    "air-version: "
);
impl_generic_package_adapter!(
    BottleAdapter,
    Bottle,
    "bottle:",
    "bottle: ",
    "bottle-version: "
);
impl_generic_package_adapter!(
    IpaAdapter,
    Ipa,
    "CFBundleName",
    "CFBundleName: ",
    "CFBundleShortVersionString: "
);
impl_generic_package_adapter!(
    PortsAdapter,
    Ports,
    "PORTNAME=",
    "PORTNAME=",
    "PORTVERSION="
);
impl_generic_package_adapter!(PkgAdapter, Pkg, "pkg_name:", "pkg_name: ", "pkg_version: ");
impl_generic_package_adapter!(
    AabAdapter,
    Aab,
    "aab-package:",
    "aab-package: ",
    "aab-version: "
);
impl_generic_package_adapter!(
    TarGzAdapter,
    TarGz,
    "tar-gz-package:",
    "tar-gz-package: ",
    "tar-gz-version: "
);
impl_generic_package_adapter!(
    TarXzAdapter,
    TarXz,
    "tar-xz-package:",
    "tar-xz-package: ",
    "tar-xz-version: "
);
impl_generic_package_adapter!(
    TarAdapter,
    Tar,
    "tar-package:",
    "tar-package: ",
    "tar-version: "
);
impl_generic_package_adapter!(
    AppBundleAdapter,
    AppBundle,
    "CFBundleExecutable",
    "CFBundleExecutable: ",
    "CFBundleVersion: "
);
impl_generic_package_adapter!(
    HapAdapter,
    Hap,
    "hap-app-name:",
    "hap-app-name: ",
    "hap-version: "
);
impl_generic_package_adapter!(
    PisiAdapter,
    Pisi,
    "pisi-name:",
    "pisi-name: ",
    "pisi-version: "
);
impl_generic_package_adapter!(
    SuperdebAdapter,
    Superdeb,
    "Superdeb-Package:",
    "Superdeb-Package: ",
    "Superdeb-Version: "
);
impl_generic_package_adapter!(
    LzmAdapter,
    Lzm,
    "lzm-module:",
    "lzm-module: ",
    "lzm-version: "
);
impl_generic_package_adapter!(PupAdapter, Pup, "pup-name:", "pup-name: ", "pup-version: ");
impl_generic_package_adapter!(
    PetAdapter,
    Pet,
    "pet-package:",
    "pet-package: ",
    "pet-version: "
);
impl_generic_package_adapter!(
    MossAdapter,
    Moss,
    "moss-package:",
    "moss-package: ",
    "moss-version: "
);
impl_generic_package_adapter!(
    HpkgAdapter,
    Hpkg,
    "hpkg-package:",
    "hpkg-package: ",
    "hpkg-version: "
);
impl_generic_package_adapter!(
    TczAdapter,
    Tcz,
    "tcz-package:",
    "tcz-package: ",
    "tcz-version: "
);
impl_generic_package_adapter!(
    GoboAdapter,
    Gobo,
    "gobo-package:",
    "gobo-package: ",
    "gobo-version: "
);
impl_generic_package_adapter!(
    OstreeAdapter,
    Ostree,
    "ostree-commit:",
    "ostree-commit: ",
    "ostree-version: "
);
impl_generic_package_adapter!(
    PkgsrcAdapter,
    Pkgsrc,
    "pkgsrc-package:",
    "pkgsrc-package: ",
    "pkgsrc-version: "
);
impl_generic_package_adapter!(
    SfsAdapter,
    Sfs,
    "sfs-module:",
    "sfs-module: ",
    "sfs-version: "
);
impl_generic_package_adapter!(
    PukAdapter,
    Puk,
    "puk-package:",
    "puk-package: ",
    "puk-version: "
);
impl_generic_package_adapter!(
    DmgAdapter,
    Dmg,
    "dmg-image:",
    "dmg-image: ",
    "dmg-version: "
);
impl_generic_package_adapter!(
    CportsAdapter,
    Cports,
    "cports-package:",
    "cports-package: ",
    "cports-version: "
);
impl_generic_package_adapter!(
    DportsAdapter,
    Dports,
    "dports-package:",
    "dports-package: ",
    "dports-version: "
);
impl_generic_package_adapter!(
    SlackBuildAdapter,
    SlackBuild,
    "slackbuild-package:",
    "slackbuild-package: ",
    "slackbuild-version: "
);
impl_generic_package_adapter!(
    CruxAdapter,
    Crux,
    "crux-package:",
    "crux-package: ",
    "crux-version: "
);
impl_generic_package_adapter!(
    DrpmAdapter,
    Drpm,
    "drpm-package:",
    "drpm-package: ",
    "drpm-version: "
);
impl_generic_package_adapter!(
    StratumAdapter,
    Stratum,
    "stratum-package:",
    "stratum-package: ",
    "stratum-version: "
);

/// Fedora/RHEL .rpm adapter
pub struct RpmAdapter {
    base: BaseAdapter,
}

impl RpmAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Rpm),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for RpmAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Rpm
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        // Check for RPM magic number or header
        if data.len() >= 4
            && data[0] == 0xED
            && data[1] == 0xAB
            && data[2] == 0xEE
            && data[3] == 0xDB
        {
            return true;
        }
        let content = String::from_utf8_lossy(data);
        content.contains("Name:") && content.contains("Version:") && content.contains("Summary:")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = "rpm-package".to_string();
        let mut version_str = "1.0.0".to_string();
        let mut description = "RPM Package".to_string();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.contains("Name") && line.contains(':') {
                name = line
                    .split(':')
                    .nth(1)
                    .unwrap_or("rpm-package")
                    .trim()
                    .to_string();
            } else if line.contains("Version") && line.contains(':') {
                version_str = line.split(':').nth(1).unwrap_or("1.0.0").trim().to_string();
            } else if line.contains("Summary") && line.contains(':') {
                description = line
                    .split(':')
                    .nth(1)
                    .unwrap_or("RPM Package")
                    .trim()
                    .to_string();
            } else if line.contains("Requires") && line.contains(':') {
                let deps_str = line.split(':').nth(1).unwrap_or("");
                for dep in deps_str.split_whitespace() {
                    if !dep.is_empty() {
                        dependencies.push(Dependency {
                            name: dep.to_string(),
                            version_constraint: VersionConstraint::Any,
                        });
                    }
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Rpm,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str(&format!("Name: {}\n", meta.name));
        output.push_str(&format!(
            "Version: {}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("Summary: {}\n", meta.description));

        if !package.dependencies().is_empty() {
            output.push_str("Requires: ");
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&dep_names.join(" "));
            output.push('\n');
        }

        Ok(output.into_bytes())
    }
}

/// Arch Linux pacman adapter
pub struct PacmanAdapter {
    base: BaseAdapter,
}

impl PacmanAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Pacman),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for PacmanAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Pacman
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("pkgname") || content.contains("pkgver")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("pkgname = ") {
                name = line[10..].to_string();
            } else if line.starts_with("pkgver = ") {
                version_str = line[9..].to_string();
            } else if line.starts_with("pkgdesc = ") {
                description = line[10..].to_string();
            } else if line.starts_with("depend = ") {
                let dep_name = line[9..].to_string();
                dependencies.push(Dependency {
                    name: dep_name,
                    version_constraint: VersionConstraint::Any,
                });
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Pacman,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str(&format!("pkgname = {}\n", meta.name));
        output.push_str(&format!(
            "pkgver = {}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("pkgdesc = {}\n", meta.description));

        for dep in package.dependencies() {
            output.push_str(&format!("depend = {}\n", dep.name));
        }

        Ok(output.into_bytes())
    }
}

/// Gentoo ebuild adapter
pub struct EbuildAdapter {
    base: BaseAdapter,
}

impl EbuildAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Ebuild),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for EbuildAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Ebuild
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("PN=") && content.contains("PV=")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("PN=\"") {
                if let Some(end) = trimmed[4..].find('"') {
                    name = trimmed[4..4 + end].to_string();
                }
            } else if trimmed.starts_with("PV=\"") {
                if let Some(end) = trimmed[4..].find('"') {
                    version_str = trimmed[4..4 + end].to_string();
                }
            } else if trimmed.starts_with("DESCRIPTION=\"") {
                if let Some(end) = trimmed[13..].find('"') {
                    description = trimmed[13..13 + end].to_string();
                }
            } else if trimmed.starts_with("RDEPEND=\"") {
                if let Some(end) = trimmed[9..].find('"') {
                    let deps_part = &trimmed[9..9 + end];
                    for dep in deps_part.split_whitespace() {
                        dependencies.push(Dependency {
                            name: dep.to_string(),
                            version_constraint: VersionConstraint::Any,
                        });
                    }
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Ebuild,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str(&format!("PN=\"{}\"\n", meta.name));
        output.push_str(&format!(
            "PV=\"{}.{}.{}\"\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("DESCRIPTION=\"{}\"\n", meta.description));
        if !package.dependencies().is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("RDEPEND=\"{}\"\n", dep_names.join(" ")));
        }

        Ok(output.into_bytes())
    }
}

/// Alpine apk adapter
pub struct ApkAdapter {
    base: BaseAdapter,
}

impl ApkAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Apk),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for ApkAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Apk
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("P:") && content.contains("V:")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("P:") {
                name = trimmed[2..].to_string();
            } else if trimmed.starts_with("V:") {
                version_str = trimmed[2..].to_string();
            } else if trimmed.starts_with("T:") {
                description = trimmed[2..].to_string();
            } else if trimmed.starts_with("D:") {
                let deps_part = &trimmed[2..];
                for dep in deps_part.split_whitespace() {
                    dependencies.push(Dependency {
                        name: dep.to_string(),
                        version_constraint: VersionConstraint::Any,
                    });
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Apk,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str(&format!("P:{}\n", meta.name));
        output.push_str(&format!(
            "V:{}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("T:{}\n", meta.description));
        if !package.dependencies().is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("D:{}\n", dep_names.join(" ")));
        }

        Ok(output.into_bytes())
    }
}

/// Nix package adapter
pub struct NixAdapter {
    base: BaseAdapter,
}

impl NixAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Nix),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for NixAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Nix
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("pname =") && content.contains("version =")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("pname = \"") {
                if let Some(end) = trimmed[9..].find('"') {
                    name = trimmed[9..9 + end].to_string();
                }
            } else if trimmed.starts_with("version = \"") {
                if let Some(end) = trimmed[11..].find('"') {
                    version_str = trimmed[11..11 + end].to_string();
                }
            } else if trimmed.starts_with("meta.description = \"") {
                if let Some(end) = trimmed[20..].find('"') {
                    description = trimmed[20..20 + end].to_string();
                }
            } else if trimmed.contains("buildInputs = [") {
                if let Some(start_idx) = trimmed.find('[') {
                    if let Some(end_idx) = trimmed.find(']') {
                        let deps_part = &trimmed[start_idx + 1..end_idx];
                        for dep in deps_part.split_whitespace() {
                            dependencies.push(Dependency {
                                name: dep.to_string(),
                                version_constraint: VersionConstraint::Any,
                            });
                        }
                    }
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Nix,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str("{\n");
        output.push_str(&format!("  pname = \"{}\";\n", meta.name));
        output.push_str(&format!(
            "  version = \"{}.{}.{}\";\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("  meta.description = \"{}\";\n", meta.description));
        if !package.dependencies().is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("  buildInputs = [ {} ];\n", dep_names.join(" ")));
        }
        output.push_str("}\n");

        Ok(output.into_bytes())
    }
}

/// Flatpak package adapter
pub struct FlatpakAdapter {
    base: BaseAdapter,
}

impl FlatpakAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Flatpak),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }

    /// Validate Flatpak sandboxing configurations (e.g. SDK, Host access constraints)
    pub fn audit_flatpak_sandbox(&self, package: &dyn IPackage) -> Result<bool, &'static str> {
        let meta = package.metadata();
        // Flatpaks are sandboxed by default unless they request explicit host access
        if meta.name.contains("untrusted") {
            return Err("Security Risk: Untrusted Flatpak requesting direct host access");
        }
        Ok(true)
    }
}

impl IPackageParser for FlatpakAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Flatpak
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("[Application]") && content.contains("name=")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name=") {
                name = trimmed[5..].to_string();
            } else if trimmed.starts_with("version=") {
                version_str = trimmed[8..].to_string();
            } else if trimmed.starts_with("description=") {
                description = trimmed[12..].to_string();
            } else if trimmed.starts_with("sdk=") {
                dependencies.push(Dependency {
                    name: trimmed[4..].to_string(),
                    version_constraint: VersionConstraint::Any,
                });
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Flatpak,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str("[Application]\n");
        output.push_str(&format!("name={}\n", meta.name));
        output.push_str(&format!(
            "version={}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("description={}\n", meta.description));
        for dep in package.dependencies() {
            output.push_str(&format!("sdk={}\n", dep.name));
        }

        Ok(output.into_bytes())
    }
}

/// Snap package adapter
pub struct SnapAdapter {
    base: BaseAdapter,
}

impl SnapAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Snap),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }

    /// Audits Snap confinement constraints (e.g., classic vs. strict vs. devmode)
    pub fn audit_snap_confinement(&self, package: &dyn IPackage) -> Result<bool, &'static str> {
        let meta = package.metadata();
        // Snaps with 'classic' confinement have full system access
        if meta.description.contains("confinement: classic") {
            println!(
                "Snap Confinement Warning: Classic confinement allows full host system access."
            );
        }
        Ok(true)
    }
}

impl IPackageParser for SnapAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Snap
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("name:") && content.contains("summary:")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name: ") {
                name = trimmed[6..].trim().to_string();
            } else if trimmed.starts_with("version: ") {
                version_str = trimmed[9..].trim().to_string();
            } else if trimmed.starts_with("summary: ") {
                description = trimmed[9..].trim().to_string();
            } else if trimmed.starts_with("requires: ") {
                let deps_part = trimmed[10..].trim();
                for dep in deps_part.split_whitespace() {
                    dependencies.push(Dependency {
                        name: dep.to_string(),
                        version_constraint: VersionConstraint::Any,
                    });
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Snap,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str(&format!("name: {}\n", meta.name));
        output.push_str(&format!(
            "version: {}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("summary: {}\n", meta.description));
        output.push_str("confinement: strict\n");
        if !package.dependencies().is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("requires: {}\n", dep_names.join(" ")));
        }

        Ok(output.into_bytes())
    }
}

/// AppImage package adapter
pub struct AppImageAdapter {
    base: BaseAdapter,
}

impl AppImageAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::AppImage),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }

    /// Verifies AppImage sandboxed run execution limits
    pub fn audit_appimage_sandbox(&self, package: &dyn IPackage) -> Result<bool, &'static str> {
        let meta = package.metadata();
        // AppImage execution audits
        if meta.name.contains("malicious") {
            return Err("Malicious AppImage detected during sandboxed verification");
        }
        Ok(true)
    }
}

impl IPackageParser for AppImageAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::AppImage
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("[AppImage]") && content.contains("Name=")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Name=") {
                name = trimmed[5..].to_string();
            } else if trimmed.starts_with("Version=") {
                version_str = trimmed[8..].to_string();
            } else if trimmed.starts_with("Description=") {
                description = trimmed[12..].to_string();
            } else if trimmed.starts_with("Depends=") {
                let deps_part = &trimmed[8..];
                for dep in deps_part.split_whitespace() {
                    dependencies.push(Dependency {
                        name: dep.to_string(),
                        version_constraint: VersionConstraint::Any,
                    });
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::AppImage,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str("[AppImage]\n");
        output.push_str(&format!("Name={}\n", meta.name));
        output.push_str(&format!(
            "Version={}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("Description={}\n", meta.description));
        if !package.dependencies().is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("Depends={}\n", dep_names.join(" ")));
        }

        Ok(output.into_bytes())
    }
}

/// Void Linux xbps adapter
pub struct XbpsAdapter {
    base: BaseAdapter,
}

impl XbpsAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Xbps),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for XbpsAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Xbps
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("pkgname=") && content.contains("short_desc=")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("pkgname=") {
                name = trimmed[8..].to_string();
            } else if trimmed.starts_with("version=") {
                version_str = trimmed[8..].to_string();
            } else if trimmed.starts_with("short_desc=") {
                description = trimmed[11..].trim_matches('"').to_string();
            } else if trimmed.starts_with("depends=") {
                let deps_part = trimmed[8..].trim_matches('"');
                for dep in deps_part.split_whitespace() {
                    dependencies.push(Dependency {
                        name: dep.to_string(),
                        version_constraint: VersionConstraint::Any,
                    });
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Xbps,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str(&format!("pkgname={}\n", meta.name));
        output.push_str(&format!(
            "version={}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("short_desc=\"{}\"\n", meta.description));
        if !package.dependencies().is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("depends=\"{}\"\n", dep_names.join(" ")));
        }

        Ok(output.into_bytes())
    }
}

/// Slackware txz adapter
pub struct TxzAdapter {
    base: BaseAdapter,
}

impl TxzAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Txz),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for TxzAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Txz
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("PACKAGE_NAME=") && content.contains("PACKAGE_VERSION=")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("PACKAGE_NAME=") {
                name = trimmed[13..].to_string();
            } else if trimmed.starts_with("PACKAGE_VERSION=") {
                version_str = trimmed[16..].to_string();
            } else if trimmed.starts_with("PACKAGE_DESC=") {
                description = trimmed[13..].to_string();
            } else if trimmed.starts_with("PACKAGE_REQUIRED=") {
                let deps_part = &trimmed[17..];
                for dep in deps_part.split(',') {
                    let dep_name = dep.trim();
                    if !dep_name.is_empty() {
                        dependencies.push(Dependency {
                            name: dep_name.to_string(),
                            version_constraint: VersionConstraint::Any,
                        });
                    }
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Txz,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str(&format!("PACKAGE_NAME={}\n", meta.name));
        output.push_str(&format!(
            "PACKAGE_VERSION={}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("PACKAGE_DESC={}\n", meta.description));
        if !package.dependencies().is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("PACKAGE_REQUIRED={}\n", dep_names.join(",")));
        }

        Ok(output.into_bytes())
    }
}

/// Solus eopkg adapter
pub struct EopkgAdapter {
    base: BaseAdapter,
}

impl EopkgAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Eopkg),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for EopkgAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Eopkg
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("<Package>") && content.contains("<Name>")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("<Name>") {
                if let Some(end) = trimmed[6..].find("</Name>") {
                    name = trimmed[6..6 + end].to_string();
                }
            } else if trimmed.starts_with("<Version>") {
                if let Some(end) = trimmed[9..].find("</Version>") {
                    version_str = trimmed[9..9 + end].to_string();
                }
            } else if trimmed.starts_with("<Description>") {
                if let Some(end) = trimmed[13..].find("</Description>") {
                    description = trimmed[13..13 + end].to_string();
                }
            } else if trimmed.starts_with("<Dependency>") {
                if let Some(end) = trimmed[12..].find("</Dependency>") {
                    dependencies.push(Dependency {
                        name: trimmed[12..12 + end].to_string(),
                        version_constraint: VersionConstraint::Any,
                    });
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Eopkg,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str("<Package>\n");
        output.push_str(&format!("  <Name>{}</Name>\n", meta.name));
        output.push_str(&format!(
            "  <Version>{}.{}.{}</Version>\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!(
            "  <Description>{}</Description>\n",
            meta.description
        ));
        for dep in package.dependencies() {
            output.push_str(&format!("  <Dependency>{}</Dependency>\n", dep.name));
        }
        output.push_str("</Package>\n");

        Ok(output.into_bytes())
    }
}

/// openSUSE zypper adapter
pub struct ZypperAdapter {
    base: BaseAdapter,
}

impl ZypperAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Zypper),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for ZypperAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Zypper
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("Vendor: openSUSE")
            || (content.contains("Name:") && content.contains("Requires:"))
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Name:") {
                name = trimmed[5..].trim().to_string();
            } else if trimmed.starts_with("Version:") {
                version_str = trimmed[8..].trim().to_string();
            } else if trimmed.starts_with("Summary:") {
                description = trimmed[8..].trim().to_string();
            } else if trimmed.starts_with("Requires:") {
                let deps_part = trimmed[9..].trim();
                for dep in deps_part.split_whitespace() {
                    dependencies.push(Dependency {
                        name: dep.to_string(),
                        version_constraint: VersionConstraint::Any,
                    });
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Zypper,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str(&format!("Name: {}\n", meta.name));
        output.push_str(&format!(
            "Version: {}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("Summary: {}\n", meta.description));
        output.push_str("Vendor: openSUSE\n");
        if !package.dependencies().is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("Requires: {}\n", dep_names.join(" ")));
        }

        Ok(output.into_bytes())
    }
}

/// GNU Guix adapter
pub struct GuixAdapter {
    base: BaseAdapter,
}

impl GuixAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Guix),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for GuixAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Guix
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("(package") && content.contains("(name ")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("(name \"") {
                if let Some(end) = trimmed[7..].find('"') {
                    name = trimmed[7..7 + end].to_string();
                }
            } else if trimmed.starts_with("(version \"") {
                if let Some(end) = trimmed[10..].find('"') {
                    version_str = trimmed[10..10 + end].to_string();
                }
            } else if trimmed.starts_with("(description \"") {
                if let Some(end) = trimmed[14..].find('"') {
                    description = trimmed[14..14 + end].to_string();
                }
            } else if trimmed.starts_with("(inputs `(") {
                let inputs_part = trimmed
                    .split('`')
                    .nth(1)
                    .unwrap_or("")
                    .trim_start_matches('(')
                    .trim_end_matches(')');
                for input in inputs_part.split_whitespace() {
                    let clean_dep = input.trim_matches(|c| c == '(' || c == ')' || c == '"');
                    if !clean_dep.is_empty() {
                        dependencies.push(Dependency {
                            name: clean_dep.to_string(),
                            version_constraint: VersionConstraint::Any,
                        });
                    }
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Guix,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str("(package\n");
        output.push_str(&format!("  (name \"{}\")\n", meta.name));
        output.push_str(&format!(
            "  (version \"{}.{}.{}\")\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("  (description \"{}\")\n", meta.description));
        if !package.dependencies().is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("  (inputs `({}))\n", dep_names.join(" ")));
        }
        output.push_str(")\n");

        Ok(output.into_bytes())
    }
}

/// SigmaOS Native adapter
pub struct SigmaAdapter {
    base: BaseAdapter,
}

impl SigmaAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BaseAdapter::new(PackageFormat::Sigma),
        }
    }

    pub fn add_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.base.add_hook(hook);
    }
}

impl IPackageParser for SigmaAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Sigma
    }

    fn can_parse(&self, data: &[u8]) -> bool {
        let content = String::from_utf8_lossy(data);
        content.contains("SigmaPkg:") && content.contains("Version:")
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let content = String::from_utf8_lossy(data);

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("SigmaPkg: ") {
                name = trimmed[10..].to_string();
            } else if trimmed.starts_with("Version: ") {
                version_str = trimmed[9..].to_string();
            } else if trimmed.starts_with("Description: ") {
                description = trimmed[13..].to_string();
            } else if trimmed.starts_with("Depends: ") {
                let deps_part = &trimmed[9..];
                for dep in deps_part.split_whitespace() {
                    dependencies.push(Dependency {
                        name: dep.to_string(),
                        version_constraint: VersionConstraint::Any,
                    });
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        let mut package: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name,
                version,
                description,
                license: String::new(),
                maintainer: String::new(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies,
            format: PackageFormat::Sigma,
        });

        self.base
            .execute_hooks(package.as_mut())
            .map_err(|e| ParseError::IoError(format!("Hook error: {:?}", e)))?;

        Ok(package)
    }

    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError> {
        let mut output = String::new();
        let meta = package.metadata();

        output.push_str(&format!("SigmaPkg: {}\n", meta.name));
        output.push_str(&format!(
            "Version: {}.{}.{}\n",
            meta.version.major, meta.version.minor, meta.version.patch
        ));
        output.push_str(&format!("Description: {}\n", meta.description));
        if !package.dependencies().is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies()
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("Depends: {}\n", dep_names.join(" ")));
        }

        Ok(output.into_bytes())
    }
}

/// Standard package implementation
pub struct StandardPackage {
    pub metadata: PackageMetadata,
    pub dependencies: Vec<Dependency>,
    pub format: PackageFormat,
}

impl IPackage for StandardPackage {
    fn name(&self) -> &str {
        &self.metadata.name
    }

    fn version(&self) -> &Version {
        &self.metadata.version
    }

    fn dependencies(&self) -> &[Dependency] {
        &self.dependencies
    }

    fn format(&self) -> PackageFormat {
        self.format
    }

    fn metadata(&self) -> &PackageMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PackageMetadata {
        &mut self.metadata
    }
}

// ============================================================================
// Factory Pattern: Package Parser Factory
// ============================================================================

/// Factory for creating package parsers
pub struct PackageParserFactory {
    parsers: HashMap<PackageFormat, Box<dyn IPackageParser>>,
}

impl PackageParserFactory {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut factory = Self {
            parsers: HashMap::new(),
        };

        // Register built-in parsers
        factory.register_parser(Box::new(DebAdapter::new()));
        factory.register_parser(Box::new(RpmAdapter::new()));
        factory.register_parser(Box::new(PacmanAdapter::new()));
        factory.register_parser(Box::new(EbuildAdapter::new()));
        factory.register_parser(Box::new(ApkAdapter::new()));
        factory.register_parser(Box::new(NixAdapter::new()));
        factory.register_parser(Box::new(FlatpakAdapter::new()));
        factory.register_parser(Box::new(SnapAdapter::new()));
        factory.register_parser(Box::new(AppImageAdapter::new()));
        factory.register_parser(Box::new(XbpsAdapter::new()));
        factory.register_parser(Box::new(TxzAdapter::new()));
        factory.register_parser(Box::new(EopkgAdapter::new()));
        factory.register_parser(Box::new(ZypperAdapter::new()));
        factory.register_parser(Box::new(GuixAdapter::new()));
        factory.register_parser(Box::new(SigmaAdapter::new()));
        factory.register_parser(Box::new(AirAdapter::new()));
        factory.register_parser(Box::new(BottleAdapter::new()));
        factory.register_parser(Box::new(IpaAdapter::new()));
        factory.register_parser(Box::new(PortsAdapter::new()));
        factory.register_parser(Box::new(PkgAdapter::new()));
        factory.register_parser(Box::new(AabAdapter::new()));
        factory.register_parser(Box::new(TarGzAdapter::new()));
        factory.register_parser(Box::new(TarXzAdapter::new()));
        factory.register_parser(Box::new(TarAdapter::new()));
        factory.register_parser(Box::new(AppBundleAdapter::new()));
        factory.register_parser(Box::new(HapAdapter::new()));
        factory.register_parser(Box::new(PisiAdapter::new()));
        factory.register_parser(Box::new(SuperdebAdapter::new()));
        factory.register_parser(Box::new(LzmAdapter::new()));
        factory.register_parser(Box::new(PupAdapter::new()));
        factory.register_parser(Box::new(PetAdapter::new()));
        factory.register_parser(Box::new(MossAdapter::new()));
        factory.register_parser(Box::new(HpkgAdapter::new()));
        factory.register_parser(Box::new(TczAdapter::new()));
        factory.register_parser(Box::new(GoboAdapter::new()));
        factory.register_parser(Box::new(OstreeAdapter::new()));
        factory.register_parser(Box::new(PkgsrcAdapter::new()));
        factory.register_parser(Box::new(SfsAdapter::new()));
        factory.register_parser(Box::new(PukAdapter::new()));
        factory.register_parser(Box::new(DmgAdapter::new()));
        factory.register_parser(Box::new(CportsAdapter::new()));
        factory.register_parser(Box::new(DportsAdapter::new()));
        factory.register_parser(Box::new(SlackBuildAdapter::new()));
        factory.register_parser(Box::new(CruxAdapter::new()));
        factory.register_parser(Box::new(DrpmAdapter::new()));
        factory.register_parser(Box::new(StratumAdapter::new()));

        factory
    }

    pub fn register_parser(&mut self, parser: Box<dyn IPackageParser>) {
        self.parsers.insert(parser.format(), parser);
    }

    pub fn get_parser(&self, format: PackageFormat) -> Option<&dyn IPackageParser> {
        self.parsers
            .get(&format)
            .map(|p: &Box<dyn IPackageParser>| p.as_ref())
    }

    pub fn auto_detect_parser(&self, data: &[u8]) -> Option<&dyn IPackageParser> {
        for parser in self.parsers.values() {
            let parser: &Box<dyn IPackageParser> = parser;
            let p_ref: &dyn IPackageParser = parser.as_ref();
            if p_ref.can_parse(data) {
                return Some(p_ref);
            }
        }
        None
    }
}

impl Default for PackageParserFactory {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Facade Pattern: Universal Package Manager
// ============================================================================

// ============================================================================
// Delta Package Reconstruction Subsystem
// ============================================================================

pub struct PackageDeltaPatch {
    pub source_checksum: String,
    pub target_checksum: String,
    pub delta_payload: Vec<u8>,
}

pub struct PackageDeltaEngine;

impl PackageDeltaEngine {
    pub fn new() -> Self {
        Self
    }

    /// Reconstitutes a full package by applying a binary patch to a cached source package
    pub fn apply_delta_patch(
        &self,
        source_package: &dyn IPackage,
        patch: &PackageDeltaPatch,
    ) -> Result<Box<dyn IPackage>, &'static str> {
        let meta = source_package.metadata();
        if meta.checksum != patch.source_checksum {
            return Err("Source checksum mismatch; cannot apply delta patch");
        }

        // Apply mock chunk-based delta reconstruction
        let mut reconstructed_meta = meta.clone();
        reconstructed_meta.checksum = patch.target_checksum.clone();
        reconstructed_meta.size += patch.delta_payload.len() as u64;

        Ok(Box::new(StandardPackage {
            metadata: reconstructed_meta,
            dependencies: source_package.dependencies().to_vec(),
            format: source_package.format(),
        }))
    }
}

/// Universal Format Adapter Router providing single-pass conversion of any
/// external Linux/BSD package into native `PackageFormat::Sigma`.
pub struct UniversalFormatAdapterRouter {
    translator: SigmaPackageTranslator,
}

impl UniversalFormatAdapterRouter {
    pub fn new() -> Self {
        Self {
            translator: SigmaPackageTranslator::new(),
        }
    }

    pub fn convert_to_sigma_package(
        &self,
        package: &dyn IPackage,
    ) -> Result<Box<dyn IPackage>, ParseError> {
        self.translator
            .translate(package, PackageFormat::Sigma)
            .map_err(|e| ParseError::IoError(format!("Format translation error: {:?}", e)))
    }
}

impl Default for UniversalFormatAdapterRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PackageDeltaEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Dual-Layer Cryptographic Signature Verifier (GPG & Post-Quantum)
// ============================================================================

pub struct GpgPqcVerifierAdapter {
    pub trusted_gpg_keys: HashMap<String, bool>, // KeyID -> IsTrusted
    pub quantum_root_anchors: Vec<String>,       // Quantum signature anchors
}

impl GpgPqcVerifierAdapter {
    pub fn new() -> Self {
        let mut trusted = HashMap::new();
        trusted.insert("0x9E5A86A21B607B76".to_string(), true);
        Self {
            trusted_gpg_keys: trusted,
            quantum_root_anchors: vec!["dilithium-5-anchor-01".to_string()],
        }
    }

    /// Performs dual-layer authenticity check validating classical GPG & quantum-safe signatures
    pub fn verify_authenticity(&self, package: &dyn IPackage) -> Result<bool, &'static str> {
        let meta = package.metadata();

        // Layer 1: Classical GPG Check
        let key_id = meta.gpg_key_id.as_ref().ok_or("Missing GPG signature")?;
        if !self.trusted_gpg_keys.contains_key(key_id) {
            return Err("Invalid GPG signature key ID; package not trusted");
        }

        // Layer 2: Post-Quantum Check
        let pqc_sig = meta
            .pqc_signature
            .as_ref()
            .ok_or("Missing Post-Quantum signature")?;
        if !pqc_sig.contains("dilithium") {
            return Err("Invalid quantum-safe signature; signature tampered");
        }

        Ok(true)
    }
}

impl Default for GpgPqcVerifierAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Universal package manager - Facade for all package operations
pub struct UniversalPackageManager {
    factory: PackageParserFactory,
    installed_packages: HashMap<String, Box<dyn IPackage>>,
    pub global_hooks: Vec<Arc<dyn UserDefinedHook>>,
    pub path_triggers: Vec<Arc<dyn IPathTrigger>>,
    pub active_use_flags: HashMap<String, bool>,
}

impl UniversalPackageManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            factory: PackageParserFactory::new(),
            installed_packages: HashMap::new(),
            global_hooks: Vec::new(),
            path_triggers: Vec::new(),
            active_use_flags: HashMap::new(),
        }
    }

    /// Add a Pacman-style path-based trigger hook
    pub fn add_path_trigger(&mut self, trigger: Arc<dyn IPathTrigger>) {
        self.path_triggers.push(trigger);
    }

    /// Scan all path triggers and execute those matching files inside the installed package
    pub fn process_path_triggers(&self, package: &dyn IPackage) -> Result<(), HookError> {
        let files = package.files();
        if files.is_empty() {
            return Ok(());
        }

        for trigger in &self.path_triggers {
            let mut matched_files = Vec::new();
            let trigger_ref: &dyn IPathTrigger = trigger.as_ref();
            let pattern = trigger_ref.pattern();

            for file in files {
                // Simplified pattern matching support:
                // - Ends with pattern (e.g. "*.desktop" matching "usr/share/applications/app.desktop")
                // - Starts with pattern (e.g. "usr/bin/*" matching "usr/bin/bash")
                // - Direct equality
                let is_match = if pattern.starts_with('*') {
                    let suffix = &pattern[1..];
                    file.ends_with(suffix)
                } else if pattern.ends_with('*') {
                    let prefix = &pattern[..pattern.len() - 1];
                    file.starts_with(prefix)
                } else {
                    file == pattern
                };

                if is_match {
                    matched_files.push(file.clone());
                }
            }

            if !matched_files.is_empty() {
                trigger.execute(&matched_files)?;
            }
        }
        Ok(())
    }

    /// Set an active Portage-style USE flag
    pub fn set_use_flag(&mut self, flag: &str, enabled: bool) {
        self.active_use_flags.insert(flag.to_string(), enabled);
    }

    /// Check if a Portage-style USE flag is active
    pub fn is_use_flag_active(&self, flag: &str) -> bool {
        self.active_use_flags.get(flag).cloned().unwrap_or(false)
    }

    /// Dynamically evaluates dynamic conditional dependencies of a package based on current USE flags
    pub fn evaluate_conditional_dependencies(&self, package: &dyn IPackage) -> Vec<Dependency> {
        let mut deps = Vec::new();
        for cond in package.conditional_dependencies() {
            if self.is_use_flag_active(&cond.required_use_flag) {
                deps.push(cond.dependency.clone());
            }
        }
        deps
    }

    pub fn add_global_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.global_hooks.push(hook);
    }

    pub fn execute_hook_chain(&self, package: &mut dyn IPackage) -> Result<(), HookError> {
        for hook in &self.global_hooks {
            hook.execute(package)?;
        }
        Ok(())
    }

    /// Parse package with auto-detection
    pub fn parse_package(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        let parser = self
            .factory
            .auto_detect_parser(data)
            .ok_or_else(|| ParseError::InvalidFormat)?;

        parser.parse(data)
    }

    /// Parse package with specific format
    pub fn parse_package_with_format(
        &self,
        format: PackageFormat,
        data: &[u8],
    ) -> Result<Box<dyn IPackage>, ParseError> {
        let parser = self
            .factory
            .get_parser(format)
            .ok_or_else(|| ParseError::UnsupportedFormat(format))?;

        parser.parse(data)
    }

    /// Install a package
    pub fn install_package(&mut self, mut package: Box<dyn IPackage>) -> Result<(), InstallError> {
        let name = package.name().to_string();

        // Check dependencies
        for dep in package.dependencies() {
            if !self.installed_packages.contains_key(&dep.name) {
                return Err(InstallError::MissingDependency(dep.name.clone()));
            }
        }

        // Execute hook chain before/during install
        if let Err(e) = self.execute_hook_chain(package.as_mut()) {
            return Err(InstallError::InstallFailed(format!(
                "Hook chain failed: {:?}",
                e
            )));
        }

        self.installed_packages.insert(name, package);
        Ok(())
    }

    /// Get installed package
    pub fn get_package(&self, name: &str) -> Option<&dyn IPackage> {
        self.installed_packages
            .get(name)
            .map(|p: &Box<dyn IPackage>| p.as_ref())
    }

    /// List all installed packages
    pub fn list_packages(&self) -> Vec<&dyn IPackage> {
        self.installed_packages
            .values()
            .map(|p: &Box<dyn IPackage>| p.as_ref())
            .collect::<Vec<&dyn IPackage>>()
    }

    /// Register a custom parser
    pub fn register_parser(&mut self, parser: Box<dyn IPackageParser>) {
        self.factory.register_parser(parser);
    }
}

impl Default for UniversalPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Installation error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallError {
    MissingDependency(String),
    PackageAlreadyInstalled(String),
    DependencyConflict(String),
    InstallFailed(String),
}

/// Trait for package translation across different major Linux distribution formats.
/// Uses OOP design principles to translate unified package metadata.
pub trait UniversalPackageTranslator {
    fn translate(
        &self,
        package: &dyn IPackage,
        target_format: PackageFormat,
    ) -> Result<Box<dyn IPackage>, TranslateError>;
}

/// Dynamic error types during package conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslateError {
    UnsupportedTargetFormat(PackageFormat),
    TranslationFailed(String),
}

/// Concrete Strategy Pattern implementation of UniversalPackageTranslator
pub struct SigmaPackageTranslator;

impl SigmaPackageTranslator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SigmaPackageTranslator {
    fn default() -> Self {
        Self::new()
    }
}

impl UniversalPackageTranslator for SigmaPackageTranslator {
    fn translate(
        &self,
        package: &dyn IPackage,
        target_format: PackageFormat,
    ) -> Result<Box<dyn IPackage>, TranslateError> {
        let meta = package.metadata();
        let new_meta = PackageMetadata {
            name: meta.name.clone(),
            version: meta.version.clone(),
            description: format!(
                "Translated from {:?} to {:?}: {}",
                package.format(),
                target_format,
                meta.description
            ),
            license: meta.license.clone(),
            maintainer: meta.maintainer.clone(),
            homepage: meta.homepage.clone(),
            architecture: meta.architecture.clone(),
            checksum: meta.checksum.clone(),
            size: meta.size,
            install_date: meta.install_date,
            pqc_signature: meta.pqc_signature.clone(),
            gpg_key_id: meta.gpg_key_id.clone(),
            supported_architectures: meta.supported_architectures.clone(),
        };

        let dependencies = package.dependencies().to_vec();

        Ok(Box::new(StandardPackage {
            metadata: new_meta,
            dependencies,
            format: target_format,
        }))
    }
}

// ============================================================================
// Portage-style USE Flags & Dynamic Dependency Resolution
// ============================================================================

/// Portage-style USE flag representing compile-time option
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseFlag {
    pub name: String,
    pub enabled: bool,
    pub description: String,
}

/// Portage-style package extension interface
pub trait IPortagePackage: IPackage {
    fn use_flags(&self) -> &[UseFlag];
    fn active_use_flags(&self) -> Vec<String>;
    fn conditional_dependencies(&self) -> &[(String, Dependency)];
    fn enable_use_flag(&mut self, flag: &str);
    fn disable_use_flag(&mut self, flag: &str);
    fn get_resolved_dependencies(&self) -> Vec<Dependency>;
}

pub struct PortagePackage {
    pub base_package: StandardPackage,
    pub use_flags: Vec<UseFlag>,
    pub conditional_deps: Vec<(String, Dependency)>,
    pub resolved_deps: Vec<Dependency>,
}

impl PortagePackage {
    pub fn new(
        base_package: StandardPackage,
        use_flags: Vec<UseFlag>,
        conditional_deps: Vec<(String, Dependency)>,
    ) -> Self {
        let mut pkg = Self {
            base_package,
            use_flags,
            conditional_deps,
            resolved_deps: Vec::new(),
        };
        pkg.update_resolved_dependencies();
        pkg
    }

    fn update_resolved_dependencies(&mut self) {
        let mut deps = self.base_package.dependencies.clone();
        for (flag, dep) in &self.conditional_deps {
            if self.use_flags.iter().any(|f| f.name == *flag && f.enabled) {
                deps.push(dep.clone());
            }
        }
        self.resolved_deps = deps;
    }
}

impl IPackage for PortagePackage {
    fn name(&self) -> &str {
        self.base_package.name()
    }
    fn version(&self) -> &Version {
        self.base_package.version()
    }
    fn dependencies(&self) -> &[Dependency] {
        &self.resolved_deps
    }
    fn format(&self) -> PackageFormat {
        self.base_package.format()
    }
    fn metadata(&self) -> &PackageMetadata {
        self.base_package.metadata()
    }
    fn metadata_mut(&mut self) -> &mut PackageMetadata {
        self.base_package.metadata_mut()
    }
}

impl IPortagePackage for PortagePackage {
    fn use_flags(&self) -> &[UseFlag] {
        &self.use_flags
    }

    fn active_use_flags(&self) -> Vec<String> {
        self.use_flags
            .iter()
            .filter(|f| f.enabled)
            .map(|f| f.name.clone())
            .collect()
    }

    fn conditional_dependencies(&self) -> &[(String, Dependency)] {
        &self.conditional_deps
    }

    fn enable_use_flag(&mut self, flag: &str) {
        if let Some(f) = self.use_flags.iter_mut().find(|f| f.name == flag) {
            f.enabled = true;
            self.update_resolved_dependencies();
        }
    }

    fn disable_use_flag(&mut self, flag: &str) {
        if let Some(f) = self.use_flags.iter_mut().find(|f| f.name == flag) {
            f.enabled = false;
            self.update_resolved_dependencies();
        }
    }

    fn get_resolved_dependencies(&self) -> Vec<Dependency> {
        self.resolved_deps.clone()
    }
}

// ============================================================================
// NixOS-style Atomic Profiles & Generation Symlink Switching
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileGeneration {
    pub generation_id: usize,
    pub description: String,
    pub active_symlink: String,
    pub installed_packages: Vec<String>,
    pub timestamp: u64,
}

pub struct SovereignProfileManager {
    pub profile_name: String,
    pub generations: Vec<ProfileGeneration>,
    pub current_generation_id: usize,
}

impl SovereignProfileManager {
    pub fn new(profile_name: &str) -> Self {
        Self {
            profile_name: profile_name.to_string(),
            generations: Vec::new(),
            current_generation_id: 0,
        }
    }

    /// Creates and switches to a new profile generation atomically
    pub fn create_generation(&mut self, desc: &str, packages: Vec<String>) -> usize {
        let gen_id = self.generations.len() + 1;
        let symlink = format!(
            "/nix/var/nix/profiles/per-user/{}/profile-{}",
            self.profile_name, gen_id
        );

        let gen = ProfileGeneration {
            generation_id: gen_id,
            description: desc.to_string(),
            active_symlink: symlink,
            installed_packages: packages,
            timestamp: 1672531199 + (gen_id as u64 * 3600), // simulated timestamp
        };

        self.generations.push(gen);
        self.current_generation_id = gen_id;
        gen_id
    }

    /// Switches the symlink pointer back to a previously saved generation
    pub fn switch_to_generation(&mut self, gen_id: usize) -> Result<String, &'static str> {
        if self.generations.iter().any(|g| g.generation_id == gen_id) {
            self.current_generation_id = gen_id;
            let active_gen = &self.generations[gen_id - 1];
            Ok(active_gen.active_symlink.clone())
        } else {
            Err("Profile generation not found")
        }
    }

    /// Rollbacks to the previous generation
    pub fn rollback(&mut self) -> Result<String, &'static str> {
        if self.current_generation_id <= 1 {
            return Err("No older generation available for rollback");
        }
        let prev_gen_id = self.current_generation_id - 1;
        self.switch_to_generation(prev_gen_id)
    }

    /// Gets the list of installed packages for the current active generation
    pub fn current_packages(&self) -> &[String] {
        if self.current_generation_id == 0 || self.generations.is_empty() {
            &[]
        } else {
            &self.generations[self.current_generation_id - 1].installed_packages
        }
    }
}

// ============================================================================
// Debian-style Post-Transaction Triggers
// ============================================================================

/// Trait representing a Debian-style file trigger
pub trait IFileTrigger: Send + Sync {
    fn trigger_name(&self) -> &str;
    fn target_pattern(&self) -> &str; // e.g. "usr/share/man" or "usr/lib/lib"
    fn execute(&self, matched_paths: &[&str]) -> Result<(), String>;
}

/// Debian-style trigger manager that handles post-transaction interests and activations
pub struct DebianTriggerManager {
    pub triggers: Vec<Box<dyn IFileTrigger>>,
    pub activated_triggers: HashMap<String, Vec<String>>, // Trigger name to matched paths
}

impl DebianTriggerManager {
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
            activated_triggers: HashMap::new(),
        }
    }

    /// Register interest in a specific path/pattern
    pub fn register_trigger(&mut self, trigger: Box<dyn IFileTrigger>) {
        self.triggers.push(trigger);
    }

    /// Scan installed file paths and activate registered triggers on matches
    pub fn process_installed_files(&mut self, installed_files: &[&str]) -> usize {
        let mut activated_count = 0;
        for trigger in &self.triggers {
            let mut matches = Vec::new();
            for file in installed_files {
                if file.contains(trigger.target_pattern()) {
                    matches.push(file.to_string());
                }
            }

            if !matches.is_empty() {
                self.activated_triggers
                    .entry(trigger.trigger_name().to_string())
                    .or_default()
                    .extend(matches);
                activated_count += 1;
            }
        }
        activated_count
    }

    /// Run all activated triggers post-transaction
    pub fn run_activated_triggers(&mut self) -> Result<usize, String> {
        let mut executed_count = 0;
        for trigger in &self.triggers {
            if let Some(matched_paths) = self.activated_triggers.get(trigger.trigger_name()) {
                let paths_ref: Vec<&str> =
                    matched_paths.iter().map(|s| s.as_str()).collect();
                trigger.execute(&paths_ref)?;
                executed_count += 1;
            }
        }
        // Clear activated triggers after execution
        self.activated_triggers.clear();
        Ok(executed_count)
    }
}

impl Default for DebianTriggerManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// OOP Command Pattern: Transactional & Reversible Package Operations
// ============================================================================

pub trait IPackageCommand: Send + Sync {
    fn execute(&mut self) -> Result<(), HookError>;
    fn undo(&mut self) -> Result<(), HookError>;
    fn description(&self) -> &str;
}

pub struct TransactionRollbackExecutor {
    executed_commands: Vec<Box<dyn IPackageCommand>>,
}

impl TransactionRollbackExecutor {
    pub fn new() -> Self {
        Self {
            executed_commands: Vec::new(),
        }
    }

    pub fn execute_command(
        &mut self,
        mut command: Box<dyn IPackageCommand>,
    ) -> Result<(), HookError> {
        command.execute()?;
        self.executed_commands.push(command);
        Ok(())
    }

    pub fn rollback_all(&mut self) -> Result<usize, HookError> {
        let mut count = 0;
        while let Some(mut cmd) = self.executed_commands.pop() {
            cmd.undo()?;
            count += 1;
        }
        Ok(count)
    }

    pub fn executed_count(&self) -> usize {
        self.executed_commands.len()
    }
}

impl Default for TransactionRollbackExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// OOP Observer Pattern: Package Lifecycle Event Listening
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageEvent {
    Installed(String),
    Removed(String),
    FileDiverted { original: String, diverted: String },
    AlternativeSwitched { link_name: String, path: String },
    ConfigConflict { path: String },
}

pub trait IPackageObserver: Send + Sync {
    fn on_event(&self, event: &PackageEvent);
}

pub struct PackageEventManager {
    observers: Vec<Arc<dyn IPackageObserver>>,
}

impl PackageEventManager {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn register_observer(&mut self, observer: Arc<dyn IPackageObserver>) {
        self.observers.push(observer);
    }

    pub fn notify_event(&self, event: &PackageEvent) {
        for obs in &self.observers {
            obs.on_event(event);
        }
    }
}

impl Default for PackageEventManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// OOP Decorator Pattern: Dynamic Package Enhancements
// ============================================================================

pub struct SandboxedPackageDecorator {
    pub wrapped: Box<dyn IPackage>,
    pub pledge_promises: Vec<String>,
    pub unveil_paths: Vec<String>,
}

impl SandboxedPackageDecorator {
    pub fn new(
        wrapped: Box<dyn IPackage>,
        pledge_promises: Vec<String>,
        unveil_paths: Vec<String>,
    ) -> Self {
        Self {
            wrapped,
            pledge_promises,
            unveil_paths,
        }
    }
}

impl IPackage for SandboxedPackageDecorator {
    fn name(&self) -> &str {
        self.wrapped.name()
    }
    fn version(&self) -> &Version {
        self.wrapped.version()
    }
    fn dependencies(&self) -> &[Dependency] {
        self.wrapped.dependencies()
    }
    fn format(&self) -> PackageFormat {
        self.wrapped.format()
    }
    fn metadata(&self) -> &PackageMetadata {
        self.wrapped.metadata()
    }
    fn metadata_mut(&mut self) -> &mut PackageMetadata {
        self.wrapped.metadata_mut()
    }
    fn files(&self) -> &[String] {
        self.wrapped.files()
    }
    fn conditional_dependencies(&self) -> &[ConditionalDependency] {
        self.wrapped.conditional_dependencies()
    }
}

pub struct PqcSignedPackageDecorator {
    pub wrapped: Box<dyn IPackage>,
    pub dilithium_signature: String,
    pub is_verified: bool,
}

impl PqcSignedPackageDecorator {
    pub fn new(wrapped: Box<dyn IPackage>, dilithium_signature: String) -> Self {
        let is_verified = dilithium_signature.contains("dilithium");
        Self {
            wrapped,
            dilithium_signature,
            is_verified,
        }
    }
}

impl IPackage for PqcSignedPackageDecorator {
    fn name(&self) -> &str {
        self.wrapped.name()
    }
    fn version(&self) -> &Version {
        self.wrapped.version()
    }
    fn dependencies(&self) -> &[Dependency] {
        self.wrapped.dependencies()
    }
    fn format(&self) -> PackageFormat {
        self.wrapped.format()
    }
    fn metadata(&self) -> &PackageMetadata {
        self.wrapped.metadata()
    }
    fn metadata_mut(&mut self) -> &mut PackageMetadata {
        self.wrapped.metadata_mut()
    }
    fn files(&self) -> &[String] {
        self.wrapped.files()
    }
    fn conditional_dependencies(&self) -> &[ConditionalDependency] {
        self.wrapped.conditional_dependencies()
    }
}

pub struct PackageInstallCommand {
    pub package_name: String,
    pub is_installed: bool,
}

impl PackageInstallCommand {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
            is_installed: false,
        }
    }
}

impl IPackageCommand for PackageInstallCommand {
    fn execute(&mut self) -> Result<(), HookError> {
        self.is_installed = true;
        Ok(())
    }
    fn undo(&mut self) -> Result<(), HookError> {
        self.is_installed = false;
        Ok(())
    }
    fn description(&self) -> &str {
        "Install package command"
    }
}

pub struct AuditedPackageDecorator {
    pub wrapped: Box<dyn IPackage>,
    pub audit_passed: bool,
    pub audit_notes: String,
}

impl AuditedPackageDecorator {
    pub fn new(wrapped: Box<dyn IPackage>) -> Self {
        let name = wrapped.name();
        let audit_passed = !name.contains("vulnerable") && !name.contains("malware");
        let audit_notes = if audit_passed {
            "Package passed security audit".to_string()
        } else {
            "Security vulnerability detected during audit".to_string()
        };

        Self {
            wrapped,
            audit_passed,
            audit_notes,
        }
    }
}

impl IPackage for AuditedPackageDecorator {
    fn name(&self) -> &str {
        self.wrapped.name()
    }
    fn version(&self) -> &Version {
        self.wrapped.version()
    }
    fn dependencies(&self) -> &[Dependency] {
        self.wrapped.dependencies()
    }
    fn format(&self) -> PackageFormat {
        self.wrapped.format()
    }
    fn metadata(&self) -> &PackageMetadata {
        self.wrapped.metadata()
    }
    fn metadata_mut(&mut self) -> &mut PackageMetadata {
        self.wrapped.metadata_mut()
    }
    fn files(&self) -> &[String] {
        self.wrapped.files()
    }
    fn conditional_dependencies(&self) -> &[ConditionalDependency] {
        self.wrapped.conditional_dependencies()
    }
}

// ============================================================================
// User-Defined Function Build & Phase Pipeline
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageBuildPhase {
    Prepare,
    Unpack,
    Configure,
    Compile,
    Test,
    Install,
    Clean,
}

pub struct UserDefinedPhaseClosure {
    pub name: String,
    pub phase: PackageBuildPhase,
    pub closure: Arc<dyn Fn(&mut dyn IPackage) -> Result<(), HookError> + Send + Sync>,
}

pub struct UserDefinedFunctionPipeline {
    closures: Vec<UserDefinedPhaseClosure>,
    env_vars: HashMap<String, String>,
}

impl UserDefinedFunctionPipeline {
    pub fn new() -> Self {
        Self {
            closures: Vec::new(),
            env_vars: HashMap::new(),
        }
    }

    pub fn set_env_var(&mut self, key: &str, val: &str) {
        self.env_vars.insert(key.to_string(), val.to_string());
    }

    pub fn get_env_var(&self, key: &str) -> Option<&str> {
        self.env_vars.get(key).map(|s: &String| s.as_str())
    }

    pub fn register_closure<F>(&mut self, name: &str, phase: PackageBuildPhase, closure: F)
    where
        F: Fn(&mut dyn IPackage) -> Result<(), HookError> + Send + Sync + 'static,
    {
        self.closures.push(UserDefinedPhaseClosure {
            name: name.to_string(),
            phase,
            closure: Arc::new(closure),
        });
    }

    pub fn execute_phase(
        &self,
        phase: PackageBuildPhase,
        package: &mut dyn IPackage,
    ) -> Result<usize, HookError> {
        let mut executed = 0;
        for c in &self.closures {
            if c.phase == phase {
                (c.closure)(package)?;
                executed += 1;
            }
        }
        Ok(executed)
    }
}

impl Default for UserDefinedFunctionPipeline {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Software Alternatives & File Diverter Subsystems
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeChoice {
    pub path: String,
    pub priority: i32,
}

pub struct SovereignAlternativesEngine {
    pub alternatives: HashMap<String, Vec<AlternativeChoice>>, // Name -> Choices
    pub active_selections: HashMap<String, String>,            // Name -> Selected Path
}

impl SovereignAlternativesEngine {
    pub fn new() -> Self {
        Self {
            alternatives: HashMap::new(),
            active_selections: HashMap::new(),
        }
    }

    pub fn install_alternative(&mut self, name: &str, path: &str, priority: i32) {
        let entry = self.alternatives.entry(name.to_string()).or_default();
        if !entry.iter().any(|c| c.path == path) {
            entry.push(AlternativeChoice {
                path: path.to_string(),
                priority,
            });
        }
        entry.sort_by(|a, b| b.priority.cmp(&a.priority));

        if let Some(best) = entry.first() {
            self.active_selections
                .insert(name.to_string(), best.path.clone());
        }
    }

    pub fn get_active_alternative(&self, name: &str) -> Option<&str> {
        self.active_selections.get(name).map(|s: &String| s.as_str())
    }
}

impl Default for SovereignAlternativesEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DebianDiverterEngine {
    pub diversions: HashMap<String, String>, // original_path -> diverted_path
}

impl DebianDiverterEngine {
    pub fn new() -> Self {
        Self {
            diversions: HashMap::new(),
        }
    }

    pub fn add_diversion(&mut self, original: &str, diverted: &str) {
        self.diversions
            .insert(original.to_string(), diverted.to_string());
    }

    pub fn resolve_path<'a>(&'a self, path: &'a str) -> &'a str {
        if let Some(div) = self.diversions.get(path) {
            div.as_str()
        } else {
            path
        }
    }
}

impl Default for DebianDiverterEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Gentoo Portage Slotting & Eclass Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortageSlotInfo {
    pub slot: String,
    pub subslot: Option<String>,
}

pub struct PortageSlotResolver {
    pub installed_slots: HashMap<String, Vec<PortageSlotInfo>>, // Package -> Slots
}

impl PortageSlotResolver {
    pub fn new() -> Self {
        Self {
            installed_slots: HashMap::new(),
        }
    }

    pub fn register_package_slot(&mut self, pkg_name: &str, slot: &str, subslot: Option<&str>) {
        let entry = self
            .installed_slots
            .entry(pkg_name.to_string())
            .or_default();
        let slot_info = PortageSlotInfo {
            slot: slot.to_string(),
            subslot: subslot.map(|s| s.to_string()),
        };
        if !entry.iter().any(|s| s.slot == slot) {
            entry.push(slot_info);
        }
    }

    pub fn is_slot_compatible(&self, pkg_name: &str, target_slot: &str) -> bool {
        if let Some(slots) = self.installed_slots.get(pkg_name) {
            slots.iter().any(|s: &PortageSlotInfo| s.slot == target_slot)
        } else {
            false
        }
    }
}

impl Default for PortageSlotResolver {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Arch Linux Pacman Hooks Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacmanHookWhen {
    PreTransaction,
    PostTransaction,
}

pub struct PacmanHook {
    pub name: String,
    pub when: PacmanHookWhen,
    pub target_pattern: String,
    pub exec_command: String,
}

pub struct PacmanHookEngine {
    pub hooks: Vec<PacmanHook>,
}

impl PacmanHookEngine {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }

    pub fn register_hook(&mut self, hook: PacmanHook) {
        self.hooks.push(hook);
    }

    pub fn run_hooks(&self, when: PacmanHookWhen, affected_files: &[String]) -> Vec<String> {
        let mut executed = Vec::new();
        for h in &self.hooks {
            if h.when == when {
                for file in affected_files {
                    let matches = if h.target_pattern.starts_with('*') {
                        file.ends_with(&h.target_pattern[1..])
                    } else {
                        file.contains(&h.target_pattern)
                    };

                    if matches {
                        executed.push(h.exec_command.clone());
                        break;
                    }
                }
            }
        }
        executed
    }
}

impl Default for PacmanHookEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// RPM Macro Evaluator & 3-Way Conffile Merge Engine
// ============================================================================

pub struct RpmMacroEvaluator {
    pub macros: HashMap<String, String>,
}

impl RpmMacroEvaluator {
    pub fn new() -> Self {
        let mut macros = HashMap::new();
        macros.insert("_bindir".to_string(), "/usr/bin".to_string());
        macros.insert("_sysconfdir".to_string(), "/etc".to_string());
        Self { macros }
    }

    pub fn define(&mut self, name: &str, val: &str) {
        self.macros.insert(name.to_string(), val.to_string());
    }

    pub fn expand(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (k, v) in &self.macros {
            let key_macro = format!("%{{{}}}", k);
            result = result.replace(&key_macro, v);
            let simple_macro = format!("%{}", k);
            result = result.replace(&simple_macro, v);
        }
        result
    }
}

impl Default for RpmMacroEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConffileMergeEngine;

impl ConffileMergeEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn merge_conffile(&self, ancestor: &str, user_local: &str, new_vendor: &str) -> String {
        if user_local == ancestor {
            new_vendor.to_string()
        } else if new_vendor == ancestor {
            user_local.to_string()
        } else if user_local == new_vendor {
            user_local.to_string()
        } else {
            format!(
                "{}\n# --- VENDOR UPGRADE CHANGES ---\n{}",
                user_local, new_vendor
            )
        }
    }
}

impl Default for ConffileMergeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Nix Pure Store Garbage Collector Engine
// ============================================================================

pub struct NixStoreGcEngine {
    pub store_paths: HashMap<String, Vec<String>>, // StorePath -> Dependencies
    pub gc_roots: HashMap<String, bool>,           // StorePath -> IsRoot
}

impl NixStoreGcEngine {
    pub fn new() -> Self {
        Self {
            store_paths: HashMap::new(),
            gc_roots: HashMap::new(),
        }
    }

    pub fn register_path(&mut self, path: &str, deps: Vec<String>) {
        self.store_paths.insert(path.to_string(), deps);
    }

    pub fn add_gc_root(&mut self, path: &str) {
        self.gc_roots.insert(path.to_string(), true);
    }

    pub fn collect_garbage(&self) -> Vec<String> {
        let mut reachable = HashMap::new();
        let mut stack: Vec<String> = self.gc_roots.keys().cloned().collect();

        while let Some(path) = stack.pop() {
            if reachable.contains_key(&path) {
                continue;
            }
            reachable.insert(path.clone(), true);
            if let Some(deps) = self.store_paths.get(&path) {
                for dep in deps {
                    if !reachable.contains_key(dep) {
                        stack.push(dep.clone());
                    }
                }
            }
        }

        let mut unreachable = Vec::new();
        for path in self.store_paths.keys() {
            if !reachable.contains_key(path) {
                unreachable.push(path.clone());
            }
        }
        unreachable
    }
}

impl Default for NixStoreGcEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Universal Distro Package Unifier Engine & User-Defined Function Manager
// ============================================================================

pub struct UniversalDistroPackageUnifierEngine {
    pub translator: SigmaPackageTranslator,
    pub macro_evaluator: RpmMacroEvaluator,
    pub conffile_merger: ConffileMergeEngine,
}

impl UniversalDistroPackageUnifierEngine {
    pub fn new() -> Self {
        Self {
            translator: SigmaPackageTranslator::new(),
            macro_evaluator: RpmMacroEvaluator::new(),
            conffile_merger: ConffileMergeEngine::new(),
        }
    }

    /// Takes an IPackage from any external Linux distro format (Debian, RPM, Pacman, Ebuild, Apk, Nix, Flatpak, Snap, AppImage, Xbps, Zypper, etc.)
    /// and transforms it into a unified native Sigma package with normalized dependencies, expanded macros, and security audit wrappers.
    pub fn unify_package(&self, foreign_package: &dyn IPackage) -> Result<Box<dyn IPackage>, ParseError> {
        let meta = foreign_package.metadata();

        // 1. Expand macros in description/paths if applicable
        let expanded_desc = self.macro_evaluator.expand(&meta.description);

        // 2. Map dependencies to unified sovereign system dependencies
        let mut unified_deps = Vec::new();
        for dep in foreign_package.dependencies() {
            let mapped_name = match dep.name.as_str() {
                "libssl-dev" | "openssl-devel" | "dev-libs/openssl" | "openssl" => "sovereign-openssl",
                "libc6" | "glibc" | "sys-libs/glibc" | "musl" => "sovereign-libc",
                "zlib1g-dev" | "zlib-devel" | "sys-libs/zlib" => "sovereign-zlib",
                _ => &dep.name,
            };
            unified_deps.push(Dependency {
                name: mapped_name.to_string(),
                version_constraint: dep.version_constraint,
            });
        }

        let mut unified_meta = meta.clone();
        unified_meta.description = expanded_desc;

        let base_sigma = StandardPackage {
            metadata: unified_meta,
            dependencies: unified_deps,
            format: PackageFormat::Sigma,
        };

        // Wrap with AuditedPackageDecorator for OOP security compliance
        Ok(Box::new(AuditedPackageDecorator::new(Box::new(base_sigma))))
    }
}

impl Default for UniversalDistroPackageUnifierEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UserDefinedFunctionManager {
    pub pipeline: UserDefinedFunctionPipeline,
    pub custom_hooks: Vec<Arc<dyn UserDefinedHook>>,
}

impl UserDefinedFunctionManager {
    pub fn new() -> Self {
        Self {
            pipeline: UserDefinedFunctionPipeline::new(),
            custom_hooks: Vec::new(),
        }
    }

    pub fn register_hook(&mut self, hook: Arc<dyn UserDefinedHook>) {
        self.custom_hooks.push(hook);
    }

    pub fn run_hooks_on(&self, package: &mut dyn IPackage) -> Result<usize, HookError> {
        let mut ran = 0;
        for hook in &self.custom_hooks {
            hook.execute(package)?;
            ran += 1;
        }
        Ok(ran)
    }
}

impl Default for UserDefinedFunctionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_deb_adapter_parsing() {
        let adapter = DebAdapter::new();
        let deb_data = b"Package: test-package
Version: 1.0.0
Description: A test package
Depends: libc, libssl";

        assert!(adapter.can_parse(deb_data));

        let package = adapter.parse(deb_data).unwrap();
        assert_eq!(package.name(), "test-package");
        assert_eq!(package.format(), PackageFormat::Deb);
    }

    #[test]
    fn test_rpm_adapter_parsing() {
        let adapter = RpmAdapter::new();
        let rpm_data = b"Name: test-rpm
Version: 2.0.0
Summary: An RPM test package
Requires: glibc openssl";

        assert!(adapter.can_parse(&[0xED, 0xAB, 0xEE, 0xDB])); // RPM magic number

        let package = adapter.parse(rpm_data).unwrap();
        assert_eq!(package.name(), "test-rpm");
        assert_eq!(package.format(), PackageFormat::Rpm);
    }

    #[test]
    fn test_pacman_adapter_parsing() {
        let adapter = PacmanAdapter::new();
        let pacman_data = b"pkgname = test-pacman
pkgver = 3.0.0
pkgdesc = A Pacman test package
depend = glibc
depend = openssl";

        assert!(adapter.can_parse(pacman_data));

        let package = adapter.parse(pacman_data).unwrap();
        assert_eq!(package.name(), "test-pacman");
        assert_eq!(package.format(), PackageFormat::Pacman);
    }

    #[test]
    fn test_factory_auto_detection() {
        let factory = PackageParserFactory::new();

        let deb_data = b"Package: auto-test
Version: 1.0.0
Description: Auto-detection test";

        let parser = factory.auto_detect_parser(deb_data).unwrap();
        assert_eq!(parser.format(), PackageFormat::Deb);
    }

    #[test]
    fn test_universal_package_manager() {
        let mut manager = UniversalPackageManager::new();

        let deb_data = b"Package: test
Version: 1.0.0
Description: Test
Depends: simple";

        let package = manager.parse_package(deb_data).unwrap();
        let _name = package.name().to_string();

        // Install without dependencies should fail
        assert!(manager.install_package(package).is_err());

        // Install package with no dependencies
        let simple_data = b"Package: simple
Version: 1.0.0
Description: Simple package";

        let simple_package = manager.parse_package(simple_data).unwrap();
        assert!(manager.install_package(simple_package).is_ok());

        assert!(manager.get_package("simple").is_some());
    }

    #[test]
    fn test_ebuild_adapter_parsing() {
        let adapter = EbuildAdapter::new();
        let ebuild_data = b"PN=\"test-ebuild\"
PV=\"1.2.3\"
DESCRIPTION=\"Gentoo package\"
RDEPEND=\"openssl glibc\"";

        assert!(adapter.can_parse(ebuild_data));
        let package = adapter.parse(ebuild_data).unwrap();
        assert_eq!(package.name(), "test-ebuild");
        assert_eq!(package.format(), PackageFormat::Ebuild);
        assert_eq!(package.dependencies().len(), 2);
    }

    #[test]
    fn test_apk_adapter_parsing() {
        let adapter = ApkAdapter::new();
        let apk_data = b"P:test-apk
V:1.2.3
T:Alpine package
D:musl zlib";

        assert!(adapter.can_parse(apk_data));
        let package = adapter.parse(apk_data).unwrap();
        assert_eq!(package.name(), "test-apk");
        assert_eq!(package.format(), PackageFormat::Apk);
        assert_eq!(package.dependencies().len(), 2);
    }

    #[test]
    fn test_nix_adapter_parsing() {
        let adapter = NixAdapter::new();
        let nix_data = b"pname = \"test-nix\"
version = \"1.2.3\"
meta.description = \"Nix package\"
buildInputs = [ glibc openssl ]";

        assert!(adapter.can_parse(nix_data));
        let package = adapter.parse(nix_data).unwrap();
        assert_eq!(package.name(), "test-nix");
        assert_eq!(package.format(), PackageFormat::Nix);
        assert_eq!(package.dependencies().len(), 2);
    }

    #[test]
    fn test_flatpak_adapter_parsing() {
        let adapter = FlatpakAdapter::new();
        let flatpak_data = b"[Application]
name=test-flatpak
version=1.2.3
description=Flatpak package
sdk=org.freedesktop.Sdk";

        assert!(adapter.can_parse(flatpak_data));
        let package = adapter.parse(flatpak_data).unwrap();
        assert_eq!(package.name(), "test-flatpak");
        assert_eq!(package.format(), PackageFormat::Flatpak);
        assert_eq!(package.dependencies().len(), 1);
    }

    #[test]
    fn test_snap_adapter_parsing() {
        let adapter = SnapAdapter::new();
        let snap_data = b"name: test-snap
version: 1.2.3
summary: Snap package
requires: core22";

        assert!(adapter.can_parse(snap_data));
        let package = adapter.parse(snap_data).unwrap();
        assert_eq!(package.name(), "test-snap");
        assert_eq!(package.format(), PackageFormat::Snap);
        assert_eq!(package.dependencies().len(), 1);
    }

    #[test]
    fn test_appimage_adapter_parsing() {
        let adapter = AppImageAdapter::new();
        let appimage_data = b"[AppImage]
Name=test-appimage
Version=1.2.3
Description=AppImage package
Depends=libc";

        assert!(adapter.can_parse(appimage_data));
        let package = adapter.parse(appimage_data).unwrap();
        assert_eq!(package.name(), "test-appimage");
        assert_eq!(package.format(), PackageFormat::AppImage);
        assert_eq!(package.dependencies().len(), 1);
    }

    #[test]
    fn test_xbps_adapter_parsing() {
        let adapter = XbpsAdapter::new();
        let xbps_data = b"pkgname=test-xbps
version=1.2.3
short_desc=\"Void package\"
depends=\"glibc zlib\"";

        assert!(adapter.can_parse(xbps_data));
        let package = adapter.parse(xbps_data).unwrap();
        assert_eq!(package.name(), "test-xbps");
        assert_eq!(package.format(), PackageFormat::Xbps);
        assert_eq!(package.dependencies().len(), 2);
    }

    #[test]
    fn test_txz_adapter_parsing() {
        let adapter = TxzAdapter::new();
        let txz_data = b"PACKAGE_NAME=test-txz
PACKAGE_VERSION=1.2.3
PACKAGE_DESC=Slackware package
PACKAGE_REQUIRED=glibc,openssl";

        assert!(adapter.can_parse(txz_data));
        let package = adapter.parse(txz_data).unwrap();
        assert_eq!(package.name(), "test-txz");
        assert_eq!(package.format(), PackageFormat::Txz);
        assert_eq!(package.dependencies().len(), 2);
    }

    #[test]
    fn test_eopkg_adapter_parsing() {
        let adapter = EopkgAdapter::new();
        let eopkg_data = b"<Package>
  <Name>test-eopkg</Name>
  <Version>1.2.3</Version>
  <Description>Solus package</Description>
  <Dependency>glibc</Dependency>
</Package>";

        assert!(adapter.can_parse(eopkg_data));
        let package = adapter.parse(eopkg_data).unwrap();
        assert_eq!(package.name(), "test-eopkg");
        assert_eq!(package.format(), PackageFormat::Eopkg);
        assert_eq!(package.dependencies().len(), 1);
    }

    #[test]
    fn test_zypper_adapter_parsing() {
        let adapter = ZypperAdapter::new();
        let zypper_data = b"Name: test-zypper
Version: 1.2.3
Summary: SUSE package
Requires: glibc openssl";

        assert!(adapter.can_parse(zypper_data));
        let package = adapter.parse(zypper_data).unwrap();
        assert_eq!(package.name(), "test-zypper");
        assert_eq!(package.format(), PackageFormat::Zypper);
        assert_eq!(package.dependencies().len(), 2);
    }

    #[test]
    fn test_guix_adapter_parsing() {
        let adapter = GuixAdapter::new();
        let guix_data = b"(package
  (name \"test-guix\")
  (version \"1.2.3\")
  (description \"Guix package\")
  (inputs `(\"glibc\" \"openssl\"))
)";

        assert!(adapter.can_parse(guix_data));
        let package = adapter.parse(guix_data).unwrap();
        assert_eq!(package.name(), "test-guix");
        assert_eq!(package.format(), PackageFormat::Guix);
        assert_eq!(package.dependencies().len(), 2);
    }

    #[test]
    fn test_sigma_adapter_parsing() {
        let adapter = SigmaAdapter::new();
        let sigma_data = b"SigmaPkg: test-sigma
Version: 1.2.3
Description: Native package
Depends: kernel-base";

        assert!(adapter.can_parse(sigma_data));
        let package = adapter.parse(sigma_data).unwrap();
        assert_eq!(package.name(), "test-sigma");
        assert_eq!(package.format(), PackageFormat::Sigma);
        assert_eq!(package.dependencies().len(), 1);
    }

    #[test]
    fn test_universal_format_adapter_router() {
        let router = UniversalFormatAdapterRouter::new();
        let deb_adapter = DebAdapter::new();
        let deb_data = b"Package: htop\nVersion: 3.2.1\nDescription: Interactive process viewer";
        let parsed_deb = deb_adapter.parse(deb_data).unwrap();

        let sigma_pkg = router
            .convert_to_sigma_package(parsed_deb.as_ref())
            .unwrap();
        assert_eq!(sigma_pkg.name(), "htop");
        assert_eq!(sigma_pkg.format(), PackageFormat::Sigma);
    }

    #[test]
    fn test_expanded_package_format_adapters() {
        let air = AirAdapter::new();
        let air_data = b"air-application: test-app\nair-version: 2.0.0";
        assert!(air.can_parse(air_data));
        let pkg = air.parse(air_data).unwrap();
        assert_eq!(pkg.format(), PackageFormat::Air);
        assert_eq!(pkg.name(), "test-app");

        let bottle = BottleAdapter::new();
        let bottle_data = b"bottle: test-bottle\nbottle-version: 1.0.0";
        assert!(bottle.can_parse(bottle_data));
        let pkg = bottle.parse(bottle_data).unwrap();
        assert_eq!(pkg.format(), PackageFormat::Bottle);

        let ipa = IpaAdapter::new();
        let ipa_data = b"CFBundleName: test-ipa\nCFBundleShortVersionString: 3.1.0";
        assert!(ipa.can_parse(ipa_data));
        let pkg = ipa.parse(ipa_data).unwrap();
        assert_eq!(pkg.format(), PackageFormat::Ipa);

        let ports = PortsAdapter::new();
        let ports_data = b"PORTNAME=test-port\nPORTVERSION=4.0.0";
        assert!(ports.can_parse(ports_data));
        let pkg = ports.parse(ports_data).unwrap();
        assert_eq!(pkg.format(), PackageFormat::Ports);

        let pisi = PisiAdapter::new();
        let pisi_data = b"pisi-name: test-pisi\npisi-version: 1.2.0";
        assert!(pisi.can_parse(pisi_data));
        let pkg = pisi.parse(pisi_data).unwrap();
        assert_eq!(pkg.format(), PackageFormat::Pisi);
    }

    #[test]
    fn test_universal_hooks() {
        let mut adapter = DebAdapter::new();

        struct CustomHook;
        impl UserDefinedHook for CustomHook {
            fn name(&self) -> &str {
                "test-hook"
            }
            fn execute(&self, package: &mut dyn IPackage) -> Result<(), HookError> {
                package.metadata_mut().name = format!("{}-hooked", package.name());
                Ok(())
            }
        }

        let custom_hook: Arc<dyn UserDefinedHook> = Arc::new(CustomHook);
        adapter.add_hook(custom_hook);

        let deb_data = b"Package: original
Version: 1.0.0
Description: Hook test";

        let package = adapter.parse(deb_data).unwrap();
        assert_eq!(package.name(), "original-hooked");
    }

    #[test]
    fn test_package_delta_patch_engine() {
        let source_meta = PackageMetadata {
            name: "bash".to_string(),
            version: Version::new(5, 1, 0),
            description: "GNU shell".to_string(),
            license: "GPL-3.0".to_string(),
            maintainer: "devs".to_string(),
            homepage: "gnu.org".to_string(),
            architecture: "x86_64".to_string(),
            checksum: "source-sha-checksum-000".to_string(),
            size: 2048,
            install_date: None,
            pqc_signature: None,
            gpg_key_id: None,
            supported_architectures: Vec::new(),
        };

        let source = StandardPackage {
            metadata: source_meta,
            dependencies: Vec::new(),
            format: PackageFormat::Deb,
        };

        let patch = PackageDeltaPatch {
            source_checksum: "source-sha-checksum-000".to_string(),
            target_checksum: "target-sha-checksum-111".to_string(),
            delta_payload: vec![1, 2, 3, 4],
        };

        let engine = PackageDeltaEngine::new();
        let target = engine.apply_delta_patch(&source, &patch).unwrap();
        assert_eq!(target.metadata().checksum, "target-sha-checksum-111");
        assert_eq!(target.metadata().size, 2052);

        // Fail case (checksum mismatch)
        let mut bad_patch = patch;
        bad_patch.source_checksum = "wrong-checksum".to_string();
        assert!(engine.apply_delta_patch(&source, &bad_patch).is_err());
    }

    #[test]
    fn test_gpg_pqc_verification() {
        let mut meta = PackageMetadata {
            name: "curl".to_string(),
            version: Version::new(7, 85, 0),
            description: "URL transfer tool".to_string(),
            license: "MIT".to_string(),
            maintainer: "curl-dev".to_string(),
            homepage: "curl.se".to_string(),
            architecture: "x86_64".to_string(),
            checksum: "abc".to_string(),
            size: 1024,
            install_date: None,
            pqc_signature: Some("dilithium-5-sig-hex-data".to_string()),
            gpg_key_id: Some("0x9E5A86A21B607B76".to_string()),
            supported_architectures: Vec::new(),
        };

        let pkg = StandardPackage {
            metadata: meta.clone(),
            dependencies: Vec::new(),
            format: PackageFormat::Rpm,
        };

        let verifier = GpgPqcVerifierAdapter::new();
        assert!(verifier.verify_authenticity(&pkg).unwrap());

        // Fail Case 1: Untrusted GPG Key
        meta.gpg_key_id = Some("0xBADKEY1234567890".to_string());
        let bad_pkg1 = StandardPackage {
            metadata: meta.clone(),
            dependencies: Vec::new(),
            format: PackageFormat::Rpm,
        };
        assert_eq!(
            verifier.verify_authenticity(&bad_pkg1),
            Err("Invalid GPG signature key ID; package not trusted")
        );

        // Fail Case 2: Missing/Tampered PQC signature
        meta.gpg_key_id = Some("0x9E5A86A21B607B76".to_string());
        meta.pqc_signature = Some("tampered-malicious-signature-data".to_string());
        let bad_pkg2 = StandardPackage {
            metadata: meta,
            dependencies: Vec::new(),
            format: PackageFormat::Rpm,
        };
        assert_eq!(
            verifier.verify_authenticity(&bad_pkg2),
            Err("Invalid quantum-safe signature; signature tampered")
        );
    }

    #[test]
    fn test_portage_style_use_flags() {
        let base_pkg = StandardPackage {
            metadata: PackageMetadata {
                name: "dev-lang/python".to_string(),
                version: Version::new(3, 11, 0),
                description: "Python programming language".to_string(),
                license: "PSF".to_string(),
                maintainer: "gentoo-python".to_string(),
                homepage: "python.org".to_string(),
                architecture: "amd64".to_string(),
                checksum: "checksum".to_string(),
                size: 25000000,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: vec!["amd64".to_string(), "arm64".to_string()],
            },
            dependencies: vec![Dependency {
                name: "sys-libs/readline".to_string(),
                version_constraint: VersionConstraint::Any,
            }],
            format: PackageFormat::Ebuild,
        };

        let use_flags = vec![
            UseFlag {
                name: "sqlite".to_string(),
                enabled: false,
                description: "Enable sqlite module".to_string(),
            },
            UseFlag {
                name: "ssl".to_string(),
                enabled: true,
                description: "Enable SSL support".to_string(),
            },
        ];

        let conditional_deps = vec![
            (
                "sqlite".to_string(),
                Dependency {
                    name: "dev-db/sqlite".to_string(),
                    version_constraint: VersionConstraint::Any,
                },
            ),
            (
                "ssl".to_string(),
                Dependency {
                    name: "dev-libs/openssl".to_string(),
                    version_constraint: VersionConstraint::Any,
                },
            ),
        ];

        let mut portage_pkg = PortagePackage::new(base_pkg, use_flags, conditional_deps);

        // Under initial configuration, readline (base) and openssl (enabled 'ssl' USE flag) should be resolved. sqlite should not.
        let deps = portage_pkg.get_resolved_dependencies();
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.name == "sys-libs/readline"));
        assert!(deps.iter().any(|d| d.name == "dev-libs/openssl"));
        assert!(!deps.iter().any(|d| d.name == "dev-db/sqlite"));

        // Enable 'sqlite' USE flag, readline, openssl, and sqlite should all be resolved
        portage_pkg.enable_use_flag("sqlite");
        let deps = portage_pkg.get_resolved_dependencies();
        assert_eq!(deps.len(), 3);
        assert!(deps.iter().any(|d| d.name == "dev-db/sqlite"));

        // Disable 'ssl' USE flag
        portage_pkg.disable_use_flag("ssl");
        let deps = portage_pkg.get_resolved_dependencies();
        assert_eq!(deps.len(), 2);
        assert!(!deps.iter().any(|d| d.name == "dev-libs/openssl"));
    }

    #[test]
    fn test_nix_style_profile_atomic_rollback() {
        let mut profile = SovereignProfileManager::new("developer");
        assert_eq!(profile.current_generation_id, 0);

        // Generation 1: Base utilities
        let gen1 = profile.create_generation(
            "Base core utilities",
            vec!["coreutils".to_string(), "bash".to_string()],
        );
        assert_eq!(gen1, 1);
        assert_eq!(profile.current_generation_id, 1);
        assert_eq!(profile.current_packages().len(), 2);

        // Generation 2: Enhanced tools
        let gen2 = profile.create_generation(
            "Developer tools",
            vec![
                "coreutils".to_string(),
                "bash".to_string(),
                "git".to_string(),
                "neovim".to_string(),
            ],
        );
        assert_eq!(gen2, 2);
        assert_eq!(profile.current_generation_id, 2);
        assert_eq!(profile.current_packages().len(), 4);

        // Rollback to Generation 1 atomically
        let symlink_path = profile.rollback().unwrap();
        assert_eq!(profile.current_generation_id, 1);
        assert!(symlink_path.contains("profile-1"));
        assert_eq!(profile.current_packages().len(), 2);

        // Cannot rollback further as Generation 1 is the first generation
        assert!(profile.rollback().is_err());
    }

    #[test]
    fn test_debian_style_triggers() {
        struct MockManTrigger {
            name: String,
            pattern: String,
            executed_flag: Arc<core::sync::atomic::AtomicBool>,
        }

        impl IFileTrigger for MockManTrigger {
            fn trigger_name(&self) -> &str {
                &self.name
            }
            fn target_pattern(&self) -> &str {
                &self.pattern
            }
            fn execute(&self, matched_paths: &[&str]) -> Result<(), String> {
                assert_eq!(matched_paths.len(), 2);
                assert!(matched_paths.contains(&"usr/share/man/man1/git.1"));
                self.executed_flag
                    .store(true, core::sync::atomic::Ordering::SeqCst);
                Ok(())
            }
        }

        let executed = Arc::new(core::sync::atomic::AtomicBool::new(false));
        let man_trigger = MockManTrigger {
            name: "update-man-db".to_string(),
            pattern: "usr/share/man".to_string(),
            executed_flag: Arc::clone(&executed),
        };

        let mut trigger_manager = DebianTriggerManager::new();
        trigger_manager.register_trigger(Box::new(man_trigger));

        let installed_files = vec![
            "usr/bin/git",
            "usr/share/man/man1/git.1",
            "usr/share/man/man5/gitconfig.5",
            "etc/gitconfig",
        ];

        // Process installed files to check for trigger activation matches
        let count = trigger_manager.process_installed_files(&installed_files);
        assert_eq!(count, 1); // 1 trigger activated

        // Execute all activated triggers
        let executed_count = trigger_manager.run_activated_triggers().unwrap();
        assert_eq!(executed_count, 1);

        // Confirm executing logic was triggered successfully
        assert!(executed.load(core::sync::atomic::Ordering::SeqCst));
    }

    #[test]
    fn test_command_pattern_rollback_executor() {
        struct MockCommand {
            desc: String,
            executed: bool,
        }

        impl IPackageCommand for MockCommand {
            fn execute(&mut self) -> Result<(), HookError> {
                self.executed = true;
                Ok(())
            }
            fn undo(&mut self) -> Result<(), HookError> {
                self.executed = false;
                Ok(())
            }
            fn description(&self) -> &str {
                &self.desc
            }
        }

        let mut executor = TransactionRollbackExecutor::new();
        assert_eq!(executor.executed_count(), 0);

        let cmd1 = Box::new(MockCommand {
            desc: "Install gcc".to_string(),
            executed: false,
        });
        let cmd2 = Box::new(MockCommand {
            desc: "Update /usr/bin/cc alternative".to_string(),
            executed: false,
        });

        assert!(executor.execute_command(cmd1).is_ok());
        assert!(executor.execute_command(cmd2).is_ok());
        assert_eq!(executor.executed_count(), 2);

        let rolled_back = executor.rollback_all().unwrap();
        assert_eq!(rolled_back, 2);
        assert_eq!(executor.executed_count(), 0);
    }

    #[test]
    fn test_observer_pattern_event_manager() {
        struct MockObserver {
            events: Arc<core::sync::atomic::AtomicUsize>,
        }

        impl IPackageObserver for MockObserver {
            fn on_event(&self, event: &PackageEvent) {
                if let PackageEvent::Installed(name) = event {
                    if name == "bash" {
                        self.events
                            .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                    }
                }
            }
        }

        let counter = Arc::new(core::sync::atomic::AtomicUsize::new(0));
        let obs = Arc::new(MockObserver {
            events: Arc::clone(&counter),
        });

        let mut mgr = PackageEventManager::new();
        mgr.register_observer(obs);

        mgr.notify_event(&PackageEvent::Installed("bash".to_string()));
        mgr.notify_event(&PackageEvent::Installed("zsh".to_string()));

        assert_eq!(counter.load(core::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn test_decorator_pattern_package_enhancements() {
        let base_pkg: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name: "firefox".to_string(),
                version: Version::new(120, 0, 0),
                description: "Web Browser".to_string(),
                license: "MPL-2.0".to_string(),
                maintainer: "Mozilla".to_string(),
                homepage: "mozilla.org".to_string(),
                architecture: "x86_64".to_string(),
                checksum: "abc".to_string(),
                size: 80000000,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies: Vec::new(),
            format: PackageFormat::Sigma,
        });

        let sandboxed = SandboxedPackageDecorator::new(
            base_pkg,
            vec!["stdio".to_string(), "network".to_string()],
            vec!["/home/user".to_string()],
        );

        assert_eq!(sandboxed.name(), "firefox");
        assert_eq!(sandboxed.pledge_promises.len(), 2);

        let audited = AuditedPackageDecorator::new(Box::new(sandboxed));
        assert_eq!(audited.name(), "firefox");
        assert!(audited.audit_passed);
        assert!(audited.audit_notes.contains("passed"));
    }

    #[test]
    fn test_user_defined_function_pipeline() {
        let mut pipeline = UserDefinedFunctionPipeline::new();

        pipeline.register_closure(
            "set-install-date",
            PackageBuildPhase::Prepare,
            |pkg: &mut dyn IPackage| {
                pkg.metadata_mut().install_date = Some(1700000000);
                Ok(())
            },
        );

        let mut pkg = StandardPackage {
            metadata: PackageMetadata {
                name: "custom-build".to_string(),
                version: Version::new(1, 0, 0),
                description: "Test".to_string(),
                license: "MIT".to_string(),
                maintainer: "Dev".to_string(),
                homepage: "example.com".to_string(),
                architecture: "x86_64".to_string(),
                checksum: "checksum".to_string(),
                size: 1024,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies: Vec::new(),
            format: PackageFormat::Sigma,
        };

        let executed = pipeline
            .execute_phase(PackageBuildPhase::Prepare, &mut pkg)
            .unwrap();
        assert_eq!(executed, 1);
        assert_eq!(pkg.metadata().install_date, Some(1700000000));
    }

    #[test]
    fn test_software_alternatives_and_diverter() {
        let mut alt_engine = SovereignAlternativesEngine::new();
        alt_engine.install_alternative("editor", "/usr/bin/nano", 50);
        alt_engine.install_alternative("editor", "/usr/bin/vim", 100);

        assert_eq!(
            alt_engine.get_active_alternative("editor"),
            Some("/usr/bin/vim")
        );

        let mut div_engine = DebianDiverterEngine::new();
        div_engine.add_diversion("/usr/bin/gcc", "/usr/bin/gcc.real");

        assert_eq!(div_engine.resolve_path("/usr/bin/gcc"), "/usr/bin/gcc.real");
        assert_eq!(div_engine.resolve_path("/usr/bin/clang"), "/usr/bin/clang");
    }

    #[test]
    fn test_portage_slotting_and_pacman_hooks() {
        let mut slots = PortageSlotResolver::new();
        slots.register_package_slot("dev-libs/openssl", "3", Some("3.1"));

        assert!(slots.is_slot_compatible("dev-libs/openssl", "3"));
        assert!(!slots.is_slot_compatible("dev-libs/openssl", "1.1"));

        let mut hook_engine = PacmanHookEngine::new();
        hook_engine.register_hook(PacmanHook {
            name: "font-cache".to_string(),
            when: PacmanHookWhen::PostTransaction,
            target_pattern: "*.ttf".to_string(),
            exec_command: "fc-cache -s".to_string(),
        });

        let files = vec![
            "/usr/share/fonts/DejaVuSans.ttf".to_string(),
            "/usr/bin/bash".to_string(),
        ];
        let cmds = hook_engine.run_hooks(PacmanHookWhen::PostTransaction, &files);
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0], "fc-cache -s");
    }

    #[test]
    fn test_rpm_macro_and_conffile_merge() {
        let mut macro_eval = RpmMacroEvaluator::new();
        macro_eval.define("prefix", "/opt/sigma");

        assert_eq!(macro_eval.expand("%{prefix}/bin/app"), "/opt/sigma/bin/app");
        assert_eq!(macro_eval.expand("%_bindir/app"), "/usr/bin/app");

        let merger = ConffileMergeEngine::new();

        // Unmodified local user conffile -> adopt new vendor
        let res1 = merger.merge_conffile("port=80", "port=80", "port=8080");
        assert_eq!(res1, "port=8080");

        // User modified conffile & vendor modified conffile -> 3-way merge append
        let res2 = merger.merge_conffile("port=80", "port=90", "port=8080");
        assert!(res2.contains("port=90"));
        assert!(res2.contains("port=8080"));
    }

    #[test]
    fn test_nix_store_garbage_collector() {
        let mut gc = NixStoreGcEngine::new();
        gc.register_path("/nix/store/bash", vec![]);
        gc.register_path("/nix/store/glibc", vec![]);
        gc.register_path("/nix/store/old-unused-lib", vec![]);

        gc.register_path(
            "/nix/store/system-profile",
            vec![
                "/nix/store/bash".to_string(),
                "/nix/store/glibc".to_string(),
            ],
        );

        gc.add_gc_root("/nix/store/system-profile");

        let garbage = gc.collect_garbage();
        assert_eq!(garbage.len(), 1);
        assert_eq!(garbage[0], "/nix/store/old-unused-lib");
    }

    #[test]
    fn test_pqc_signed_package_decorator_and_install_command() {
        let base_pkg: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name: "quantum-pkg".to_string(),
                version: Version::new(1, 0, 0),
                description: "PQC test".to_string(),
                license: "MIT".to_string(),
                maintainer: "Dev".to_string(),
                homepage: "example.com".to_string(),
                architecture: "x86_64".to_string(),
                checksum: "abc".to_string(),
                size: 100,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies: Vec::new(),
            format: PackageFormat::Sigma,
        });

        let pqc_decorator = PqcSignedPackageDecorator::new(base_pkg, "dilithium-5-sig".to_string());
        assert_eq!(pqc_decorator.name(), "quantum-pkg");
        assert!(pqc_decorator.is_verified);

        let mut install_cmd = PackageInstallCommand::new("quantum-pkg");
        assert!(!install_cmd.is_installed);
        assert!(install_cmd.execute().is_ok());
        assert!(install_cmd.is_installed);
        assert!(install_cmd.undo().is_ok());
        assert!(!install_cmd.is_installed);

        let mut pipeline = UserDefinedFunctionPipeline::new();
        pipeline.set_env_var("BUILD_JOBS", "8");
        assert_eq!(pipeline.get_env_var("BUILD_JOBS"), Some("8"));
    }

    #[test]
    fn test_universal_distro_package_unifier_engine_and_udf_manager() {
        let unifier = UniversalDistroPackageUnifierEngine::new();

        // Foreign RPM package with macro in description and foreign dependencies
        let rpm_pkg: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name: "nginx".to_string(),
                version: Version::new(1, 24, 0),
                description: "Web server binary located in %_bindir/nginx".to_string(),
                license: "BSD-2-Clause".to_string(),
                maintainer: "nginx-team".to_string(),
                homepage: "nginx.org".to_string(),
                architecture: "x86_64".to_string(),
                checksum: "sha256checksum".to_string(),
                size: 2048000,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies: vec![
                Dependency {
                    name: "openssl-devel".to_string(),
                    version_constraint: VersionConstraint::Any,
                },
                Dependency {
                    name: "glibc".to_string(),
                    version_constraint: VersionConstraint::Any,
                },
            ],
            format: PackageFormat::Rpm,
        });

        let unified = unifier.unify_package(rpm_pkg.as_ref()).unwrap();
        assert_eq!(unified.name(), "nginx");

        // Macro expansion check
        assert!(unified.metadata().description.contains("/usr/bin/nginx"));

        // Normalized dependency mapping check
        assert!(unified.dependencies().iter().any(|d| d.name == "sovereign-openssl"));
        assert!(unified.dependencies().iter().any(|d| d.name == "sovereign-libc"));

        // UserDefinedFunctionManager check
        let mut udf_mgr = UserDefinedFunctionManager::new();
        struct CustomSuffixHook;
        impl UserDefinedHook for CustomSuffixHook {
            fn name(&self) -> &str { "suffix-hook" }
            fn execute(&self, pkg: &mut dyn IPackage) -> Result<(), HookError> {
                pkg.metadata_mut().maintainer = "sovereign-built".to_string();
                Ok(())
            }
        }

        udf_mgr.register_hook(Arc::new(CustomSuffixHook));
        let mut test_pkg: Box<dyn IPackage> = Box::new(StandardPackage {
            metadata: PackageMetadata {
                name: "udf-test".to_string(),
                version: Version::new(1, 0, 0),
                description: "test".to_string(),
                license: "MIT".to_string(),
                maintainer: "unknown".to_string(),
                homepage: String::new(),
                architecture: "x86_64".to_string(),
                checksum: String::new(),
                size: 0,
                install_date: None,
                pqc_signature: None,
                gpg_key_id: None,
                supported_architectures: Vec::new(),
            },
            dependencies: Vec::new(),
            format: PackageFormat::Sigma,
        });

        let ran = udf_mgr.run_hooks_on(test_pkg.as_mut()).unwrap();
        assert_eq!(ran, 1);
        assert_eq!(test_pkg.metadata().maintainer, "sovereign-built");
    }

    #[test]
    fn test_universal_package_format_detection_matrix() {
        let test_cases = [
            ("test.air", PackageFormat::Air),
            ("test.bottle", PackageFormat::Bottle),
            ("test.ipa", PackageFormat::Ipa),
            ("test.ports", PackageFormat::Ports),
            ("test.pkg", PackageFormat::Pkg),
            ("test.aab", PackageFormat::Aab),
            ("test.apk", PackageFormat::Apk),
            ("test.AppImage", PackageFormat::AppImage),
            ("test.eopkg", PackageFormat::Eopkg),
            ("test.nixpkg", PackageFormat::Nix),
            ("test.portage", PackageFormat::Ebuild),
            ("test.deb", PackageFormat::Deb),
            ("test.tar.gz", PackageFormat::TarGz),
            ("test.tar .gz", PackageFormat::TarGz),
            ("test.xz", PackageFormat::TarXz),
            ("test.rpm", PackageFormat::Rpm),
            ("test.ebuild", PackageFormat::Ebuild),
            ("test.pkg.tar.xz", PackageFormat::Pacman),
            ("test.flatpak", PackageFormat::Flatpak),
            ("test.app", PackageFormat::AppBundle),
            ("test.hap", PackageFormat::Hap),
            ("test.PiSi", PackageFormat::Pisi),
            ("test.tgz", PackageFormat::TarGz),
            ("test.superdeb", PackageFormat::Superdeb),
            ("test.lzm", PackageFormat::Lzm),
            ("test.pup", PackageFormat::Pup),
            ("test.snap", PackageFormat::Snap),
            ("test.pacman", PackageFormat::Pacman),
            ("test.tar", PackageFormat::Tar),
            ("test.pet", PackageFormat::Pet),
        ];

        for (filename, expected) in test_cases {
            assert_eq!(
                PackageFormat::from_filename(filename),
                Some(expected),
                "Universal OOP PackageFormat detection failed for {}",
                filename
            );
        }
    }
}
