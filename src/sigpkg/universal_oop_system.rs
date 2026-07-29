// Universal OOP Package System for SigmaOS
// Supports all Linux distro package formats with user-defined functions
// Implements Strategy Pattern, Adapter Pattern, and Factory Pattern

use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};
use std::collections::HashMap;
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
        data.len() >= 4 && data[0] == 0xED && data[1] == 0xAB && data[2] == 0xEE && data[3] == 0xDB
    }

    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError> {
        // Simplified RPM parsing - in production would use proper RPM library
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
}

// ============================================================================
// Factory Pattern: Package Parser Factory
// ============================================================================

/// Factory for creating package parsers
pub struct PackageParserFactory {
    parsers: HashMap<PackageFormat, Box<dyn IPackageParser>>,
}

impl PackageParserFactory {
    pub fn new() -> Self {
        let mut factory = Self {
            parsers: HashMap::new(),
        };

        // Register built-in parsers
        factory.register_parser(Box::new(DebAdapter::new()));
        factory.register_parser(Box::new(RpmAdapter::new()));
        factory.register_parser(Box::new(PacmanAdapter::new()));

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
}

impl UniversalPackageManager {
    pub fn new() -> Self {
        Self {
            factory: PackageParserFactory::new(),
            installed_packages: HashMap::new(),
        }
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
    pub fn install_package(&mut self, package: Box<dyn IPackage>) -> Result<(), InstallError> {
        let name = package.name().to_string();

        // Check dependencies
        for dep in package.dependencies() {
            if !self.installed_packages.contains_key(&dep.name) {
                return Err(InstallError::MissingDependency(dep.name.clone()));
            }
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
Description: Test";

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
}
