// Universal Package Adapter System
// OOPS-based design to support all Linux distro package formats in SigmaOS

use crate::sigpkg::{Dependency, Package, ParseError, Version, VersionConstraint};
use std::collections::HashMap;

/// Abstract trait for package format adapters (OOPS principle)
pub trait PackageFormatAdapter: Send + Sync {
    /// Get the format name (e.g., "deb", "rpm", "pacman")
    fn format_name(&self) -> &str;

    /// Parse package metadata from distro-specific format
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError>;

    /// Convert SigmaOS package to distro-specific format
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError>;

    /// Validate package integrity
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError>;

    /// Extract dependencies from package
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError>;

    /// User-defined hook for custom package processing
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError>;
}

/// Error types for package adapters
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    InvalidFormat,
    ParseError(String),
    SerializationError(String),
    ValidationError(String),
    UnsupportedFeature(String),
    HookError(String),
}

/// Debian/Ubuntu .deb package adapter
pub struct DebAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl DebAdapter {
    pub fn new() -> Self {
        Self {
            user_hooks: Vec::new(),
        }
    }

    /// Add user-defined processing hook
    pub fn add_hook<F>(&mut self, hook: F)
    where
        F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static,
    {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for DebAdapter {
    fn format_name(&self) -> &str {
        "deb"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        // Parse debian control file format
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("Package: ") {
                name = line[9..].to_string();
            } else if line.starts_with("Version: ") {
                version_str = line[9..].to_string();
            } else if line.starts_with("Description: ") {
                description = line[13..].to_string();
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

        Ok(Package::new(
            name,
            version,
            description,
            dependencies,
            String::new(),
        ))
    }

    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("Package: {}\n", package.name));
        output.push_str(&format!(
            "Version: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("Description: {}\n", package.description));

        if !package.dependencies.is_empty() {
            output.push_str("Depends: ");
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&dep_names.join(", "));
            output.push('\n');
        }

        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        // Basic validation: check if it looks like a debian control file
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;

        Ok(content.contains("Package:") || content.contains("Version:"))
    }

    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }

    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks {
            hook(package)?;
        }
        Ok(())
    }
}

impl Default for DebAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Fedora/RHEL .rpm package adapter
pub struct RpmAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl RpmAdapter {
    pub fn new() -> Self {
        Self {
            user_hooks: Vec::new(),
        }
    }

    pub fn add_hook<F>(&mut self, hook: F)
    where
        F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static,
    {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for RpmAdapter {
    fn format_name(&self) -> &str {
        "rpm"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        // Parse RPM header format (simplified)
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.contains("Name") && line.contains(':') {
                name = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.contains("Version") && line.contains(':') {
                version_str = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.contains("Summary") && line.contains(':') {
                description = line.split(':').nth(1).unwrap_or("").trim().to_string();
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

        Ok(Package::new(
            name,
            version,
            description,
            dependencies,
            String::new(),
        ))
    }

    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("Name: {}\n", package.name));
        output.push_str(&format!(
            "Version: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("Summary: {}\n", package.description));

        if !package.dependencies.is_empty() {
            output.push_str("Requires: ");
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&dep_names.join(" "));
            output.push('\n');
        }

        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;

        Ok(content.contains("Name") || content.contains("Version"))
    }

    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }

    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks {
            hook(package)?;
        }
        Ok(())
    }
}

impl Default for RpmAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Arch Linux pacman package adapter
pub struct PacmanAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl PacmanAdapter {
    pub fn new() -> Self {
        Self {
            user_hooks: Vec::new(),
        }
    }

    pub fn add_hook<F>(&mut self, hook: F)
    where
        F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static,
    {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for PacmanAdapter {
    fn format_name(&self) -> &str {
        "pacman"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        // Parse .PKGINFO format
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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

        Ok(Package::new(
            name,
            version,
            description,
            dependencies,
            String::new(),
        ))
    }

    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("pkgname = {}\n", package.name));
        output.push_str(&format!(
            "pkgver = {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("pkgdesc = {}\n", package.description));

        for dep in &package.dependencies {
            output.push_str(&format!("depend = {}\n", dep.name));
        }

        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;

        Ok(content.contains("pkgname") || content.contains("pkgver"))
    }

    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }

    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks {
            hook(package)?;
        }
        Ok(())
    }
}

impl Default for PacmanAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Universal Package Manager (OOPS Facade Pattern)
pub struct UniversalPackageManager {
    adapters: HashMap<String, Box<dyn PackageFormatAdapter>>,
    default_adapter: Option<String>,
}

impl UniversalPackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            adapters: HashMap::new(),
            default_adapter: None,
        };

        // Register built-in adapters
        manager.register_adapter(Box::new(DebAdapter::new()));
        manager.register_adapter(Box::new(RpmAdapter::new()));
        manager.register_adapter(Box::new(PacmanAdapter::new()));

        manager
    }

    /// Register a custom package format adapter
    pub fn register_adapter(&mut self, adapter: Box<dyn PackageFormatAdapter>) {
        let format_name = adapter.format_name().to_string();
        self.adapters.insert(format_name.clone(), adapter);

        // Set as default if no default exists
        if self.default_adapter.is_none() {
            self.default_adapter = Some(format_name);
        }
    }

    /// Set default adapter for unknown formats
    pub fn set_default_adapter(&mut self, format_name: &str) {
        if self.adapters.contains_key(format_name) {
            self.default_adapter = Some(format_name.to_string());
        }
    }

    /// Auto-detect package format and parse
    pub fn auto_parse(&self, data: &[u8]) -> Result<Package, AdapterError> {
        for (format_name, adapter) in &self.adapters {
            if adapter.validate(data).unwrap_or(false) {
                let mut package = adapter.parse_package(data)?;
                adapter.process_hook(&mut package)?;
                return Ok(package);
            }
        }

        // Try default adapter as fallback
        if let Some(default_name) = &self.default_adapter {
            if let Some(adapter) = self.adapters.get(default_name) {
                let mut package = adapter.parse_package(data)?;
                adapter.process_hook(&mut package)?;
                return Ok(package);
            }
        }

        Err(AdapterError::InvalidFormat)
    }

    /// Parse package with specific format
    pub fn parse_with_format(
        &self,
        format_name: &str,
        data: &[u8],
    ) -> Result<Package, AdapterError> {
        let adapter = self
            .adapters
            .get(format_name)
            .ok_or_else(|| AdapterError::UnsupportedFeature(format_name.to_string()))?;

        let mut package = adapter.parse_package(data)?;
        adapter.process_hook(&mut package)?;
        Ok(package)
    }

    /// Convert package between formats
    pub fn convert_format(
        &self,
        package: &Package,
        target_format: &str,
    ) -> Result<Vec<u8>, AdapterError> {
        let adapter = self
            .adapters
            .get(target_format)
            .ok_or_else(|| AdapterError::UnsupportedFeature(target_format.to_string()))?;

        adapter.serialize_package(package)
    }

    /// Get list of supported formats
    pub fn supported_formats(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}

impl Default for UniversalPackageManager {
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

        let package = adapter.parse_package(deb_data).unwrap();
        assert_eq!(package.name, "test-package");
        assert_eq!(package.version.major, 1);
        assert_eq!(package.dependencies.len(), 2);
    }

    #[test]
    fn test_rpm_adapter_parsing() {
        let adapter = RpmAdapter::new();
        let rpm_data = b"Name: test-rpm
Version: 2.0.0
Summary: An RPM test package
Requires: glibc openssl";

        let package = adapter.parse_package(rpm_data).unwrap();
        assert_eq!(package.name, "test-rpm");
        assert_eq!(package.version.major, 2);
    }

    #[test]
    fn test_pacman_adapter_parsing() {
        let adapter = PacmanAdapter::new();
        let pacman_data = b"pkgname = test-pacman
pkgver = 3.0.0
pkgdesc = A Pacman test package
depend = glibc
depend = openssl";

        let package = adapter.parse_package(pacman_data).unwrap();
        assert_eq!(package.name, "test-pacman");
        assert_eq!(package.version.major, 3);
    }

    #[test]
    fn test_universal_manager_auto_parse() {
        let manager = UniversalPackageManager::new();
        let deb_data = b"Package: auto-test
Version: 1.0.0
Description: Auto-detection test";

        let package = manager.auto_parse(deb_data).unwrap();
        assert_eq!(package.name, "auto-test");
    }

    #[test]
    fn test_user_defined_hook() {
        let mut adapter = DebAdapter::new();

        // Add a user-defined hook that modifies the package
        adapter.add_hook(|package: &mut Package| -> Result<(), AdapterError> {
            package.name = format!("hooked-{}", package.name);
            Ok(())
        });

        let deb_data = b"Package: original
Version: 1.0.0
Description: Hook test";

        let mut package = adapter.parse_package(deb_data).unwrap();
        adapter.process_hook(&mut package).unwrap();

        assert_eq!(package.name, "hooked-original");
    }

    #[test]
    fn test_format_conversion() {
        let manager = UniversalPackageManager::new();
        let package = Package::new(
            "convert-test".to_string(),
            Version::new(1, 0, 0),
            "Conversion test".to_string(),
            vec![],
            String::new(),
        );

        let rpm_data = manager.convert_format(&package, "rpm").unwrap();
        let rpm_str = String::from_utf8(rpm_data).unwrap();

        assert!(rpm_str.contains("Name: convert-test"));
        assert!(rpm_str.contains("Version: 1.0.0"));
    }
}
