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

// Universal OOP Package System for SigmaOS
// Supports all Linux distro package formats with user-defined functions
// Implements Strategy Pattern, Adapter Pattern, and Factory Pattern

use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};
use crate::klib::HashMap;
use std::sync::Arc;

// ============================================================================
// Core Abstractions (OOP Interface Layer)
// ============================================================================

/// Core package trait - defines the contract for all package operations
pub trait IPackage: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &Version;
    fn dependencies(&self) -> &[Dependency];
    fn format(&self) -> PackageFormat;
    fn metadata(&self) -> &PackageMetadata;
    fn metadata_mut(&mut self) -> &mut PackageMetadata;
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
        if data.len() >= 4 && data[0] == 0xED && data[1] == 0xAB && data[2] == 0xEE && data[3] == 0xDB {
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
                    name = trimmed[4..4+end].to_string();
                }
            } else if trimmed.starts_with("PV=\"") {
                if let Some(end) = trimmed[4..].find('"') {
                    version_str = trimmed[4..4+end].to_string();
                }
            } else if trimmed.starts_with("DESCRIPTION=\"") {
                if let Some(end) = trimmed[13..].find('"') {
                    description = trimmed[13..13+end].to_string();
                }
            } else if trimmed.starts_with("RDEPEND=\"") {
                if let Some(end) = trimmed[9..].find('"') {
                    let deps_part = &trimmed[9..9+end];
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
                    name = trimmed[9..9+end].to_string();
                }
            } else if trimmed.starts_with("version = \"") {
                if let Some(end) = trimmed[11..].find('"') {
                    version_str = trimmed[11..11+end].to_string();
                }
            } else if trimmed.starts_with("meta.description = \"") {
                if let Some(end) = trimmed[20..].find('"') {
                    description = trimmed[20..20+end].to_string();
                }
            } else if trimmed.contains("buildInputs = [") {
                if let Some(start_idx) = trimmed.find('[') {
                    if let Some(end_idx) = trimmed.find(']') {
                        let deps_part = &trimmed[start_idx+1..end_idx];
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
                    name = trimmed[6..6+end].to_string();
                }
            } else if trimmed.starts_with("<Version>") {
                if let Some(end) = trimmed[9..].find("</Version>") {
                    version_str = trimmed[9..9+end].to_string();
                }
            } else if trimmed.starts_with("<Description>") {
                if let Some(end) = trimmed[13..].find("</Description>") {
                    description = trimmed[13..13+end].to_string();
                }
            } else if trimmed.starts_with("<Dependency>") {
                if let Some(end) = trimmed[12..].find("</Dependency>") {
                    dependencies.push(Dependency {
                        name: trimmed[12..12+end].to_string(),
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
        output.push_str(&format!("  <Description>{}</Description>\n", meta.description));
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
        content.contains("Vendor: openSUSE") || (content.contains("Name:") && content.contains("Requires:"))
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
                    name = trimmed[7..7+end].to_string();
                }
            } else if trimmed.starts_with("(version \"") {
                if let Some(end) = trimmed[10..].find('"') {
                    version_str = trimmed[10..10+end].to_string();
                }
            } else if trimmed.starts_with("(description \"") {
                if let Some(end) = trimmed[14..].find('"') {
                    description = trimmed[14..14+end].to_string();
                }
            } else if trimmed.starts_with("(inputs `(") {
                let inputs_part = trimmed.split('`').nth(1).unwrap_or("").trim_start_matches('(').trim_end_matches(')');
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

        factory
    }

    pub fn register_parser(&mut self, parser: Box<dyn IPackageParser>) {
        self.parsers.insert(parser.format(), parser);
    }

    pub fn get_parser(&self, format: PackageFormat) -> Option<&dyn IPackageParser> {
        self.parsers.get(&format).map(|p| p.as_ref())
    }

    pub fn auto_detect_parser(&self, data: &[u8]) -> Option<&dyn IPackageParser> {
        for parser in self.parsers.values() {
            if parser.can_parse(data) {
                return Some(parser.as_ref());
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

/// Universal package manager - Facade for all package operations
pub struct UniversalPackageManager {
    factory: PackageParserFactory,
    installed_packages: HashMap<String, Box<dyn IPackage>>,
    pub global_hooks: Vec<Arc<dyn UserDefinedHook>>,
}

impl UniversalPackageManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            factory: PackageParserFactory::new(),
            installed_packages: HashMap::new(),
            global_hooks: Vec::new(),
        }
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
            return Err(InstallError::InstallFailed(format!("Hook chain failed: {:?}", e)));
        }

        self.installed_packages.insert(name, package);
        Ok(())
    }

    /// Get installed package
    pub fn get_package(&self, name: &str) -> Option<&dyn IPackage> {
        self.installed_packages.get(name).map(|p| p.as_ref())
    }

    /// List all installed packages
    pub fn list_packages(&self) -> Vec<&dyn IPackage> {
        self.installed_packages
            .values()
            .map(|p| p.as_ref())
            .collect()
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

        adapter.add_hook(Arc::new(CustomHook));

        let deb_data = b"Package: original
Version: 1.0.0
Description: Hook test";

        let package = adapter.parse(deb_data).unwrap();
        assert_eq!(package.name(), "original-hooked");
    }
}
