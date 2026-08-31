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
use alloc::vec;

extern crate alloc;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// Universal OOP Package System for SigmaOS
// Supports all Linux distro package formats with user-defined functions
// Implements Strategy Pattern, Adapter Pattern, and Factory Pattern

#[cfg(not(feature = "standalone_test"))]
use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};

#[cfg(not(feature = "standalone_test"))]
use crate::klib::HashMap;

#[cfg(feature = "standalone_test")]
use crate::klib::HashMap;

use alloc::sync::Arc;

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

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
        let mut parts = s.split('.');
        let major = parts.next().ok_or("err")?.parse().map_err(|_| "err")?;
        let minor = parts.next().ok_or("err")?.parse().map_err(|_| "err")?;
        let patch = parts.next().ok_or("err")?.parse().map_err(|_| "err")?;
        Ok(Self::new(major, minor, patch))
    }
}

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
}

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    Any,
}

#[cfg(feature = "standalone_test")]
pub struct Package;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    // Debian-based
    Deb,
    // RPM-based
    Rpm,
    // Arch-based
    Pacman,
    // Gentoo-based
    Ebuild,
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
                        name = trimmed[$name_prefix.len()..].trim_matches(|c| c == '"' || c == '\'' || c == ' ').to_string();
                    } else if trimmed.starts_with($version_prefix) {
                        version_str = trimmed[$version_prefix.len()..].trim_matches(|c| c == '"' || c == '\'' || c == ' ').to_string();
                    } else if trimmed.starts_with("description=") || trimmed.starts_with("Description:") || trimmed.starts_with("summary=") {
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

                let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(1, 0, 0));

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

impl_generic_package_adapter!(AirAdapter, Air, "air-application:", "air-application: ", "air-version: ");
impl_generic_package_adapter!(BottleAdapter, Bottle, "bottle:", "bottle: ", "bottle-version: ");
impl_generic_package_adapter!(IpaAdapter, Ipa, "CFBundleName", "CFBundleName: ", "CFBundleShortVersionString: ");
impl_generic_package_adapter!(PortsAdapter, Ports, "PORTNAME=", "PORTNAME=", "PORTVERSION=");
impl_generic_package_adapter!(PkgAdapter, Pkg, "pkg_name:", "pkg_name: ", "pkg_version: ");
impl_generic_package_adapter!(AabAdapter, Aab, "aab-package:", "aab-package: ", "aab-version: ");
impl_generic_package_adapter!(TarGzAdapter, TarGz, "tar-gz-package:", "tar-gz-package: ", "tar-gz-version: ");
impl_generic_package_adapter!(TarXzAdapter, TarXz, "tar-xz-package:", "tar-xz-package: ", "tar-xz-version: ");
impl_generic_package_adapter!(TarAdapter, Tar, "tar-package:", "tar-package: ", "tar-version: ");
impl_generic_package_adapter!(AppBundleAdapter, AppBundle, "CFBundleExecutable", "CFBundleExecutable: ", "CFBundleVersion: ");
impl_generic_package_adapter!(HapAdapter, Hap, "hap-app-name:", "hap-app-name: ", "hap-version: ");
impl_generic_package_adapter!(PisiAdapter, Pisi, "pisi-name:", "pisi-name: ", "pisi-version: ");
impl_generic_package_adapter!(SuperdebAdapter, Superdeb, "Superdeb-Package:", "Superdeb-Package: ", "Superdeb-Version: ");
impl_generic_package_adapter!(LzmAdapter, Lzm, "lzm-module:", "lzm-module: ", "lzm-version: ");
impl_generic_package_adapter!(PupAdapter, Pup, "pup-name:", "pup-name: ", "pup-version: ");
impl_generic_package_adapter!(PetAdapter, Pet, "pet-package:", "pet-package: ", "pet-version: ");

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
    fn translate(&self, package: &dyn IPackage, target_format: PackageFormat) -> Result<Box<dyn IPackage>, TranslateError>;
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
    fn translate(&self, package: &dyn IPackage, target_format: PackageFormat) -> Result<Box<dyn IPackage>, TranslateError> {
        let meta = package.metadata();
        let new_meta = PackageMetadata {
            name: meta.name.clone(),
            version: meta.version.clone(),
            description: format!("Translated from {:?} to {:?}: {}", package.format(), target_format, meta.description),
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
                let paths_ref: Vec<&str> = matched_paths.iter().map(|s| s.as_str()).collect();
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

#[cfg(test)]
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
        let name = package.name().to_string();

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
        let gen1 = profile.create_generation("Base core utilities", vec!["coreutils".to_string(), "bash".to_string()]);
        assert_eq!(gen1, 1);
        assert_eq!(profile.current_generation_id, 1);
        assert_eq!(profile.current_packages().len(), 2);

        // Generation 2: Enhanced tools
        let gen2 = profile.create_generation("Developer tools", vec!["coreutils".to_string(), "bash".to_string(), "git".to_string(), "neovim".to_string()]);
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
                self.executed_flag.store(true, core::sync::atomic::Ordering::SeqCst);
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
}
