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

        Ok(content.contains("Package:"))
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

        Ok(content.contains("Name:") || content.contains("Summary:"))
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

/// Nix Package Adapter (OOPS Concrete Implementation)
pub struct NixAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl NixAdapter {
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

impl PackageFormatAdapter for NixAdapter {
    fn format_name(&self) -> &str {
        "nix"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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
        output.push_str("{\n");
        output.push_str(&format!("  pname = \"{}\";\n", package.name));
        output.push_str(&format!(
            "  version = \"{}.{}.{}\";\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("  meta.description = \"{}\";\n", package.description));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("  buildInputs = [ {} ];\n", dep_names.join(" ")));
        }
        output.push_str("}\n");
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("pname =") || content.contains("stdenv.mkDerivation"))
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

impl Default for NixAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Gentoo Ebuild Package Adapter (OOPS Concrete Implementation)
pub struct EbuildAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl EbuildAdapter {
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

impl PackageFormatAdapter for EbuildAdapter {
    fn format_name(&self) -> &str {
        "ebuild"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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
        output.push_str(&format!("PN=\"{}\"\n", package.name));
        output.push_str(&format!(
            "PV=\"{}.{}.{}\"\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("DESCRIPTION=\"{}\"\n", package.description));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("RDEPEND=\"{}\"\n", dep_names.join(" ")));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("inherit ") || content.contains("PN=") || content.contains("PV="))
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

impl Default for EbuildAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Alpine APK Package Adapter (OOPS Concrete Implementation)
pub struct ApkAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl ApkAdapter {
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

impl PackageFormatAdapter for ApkAdapter {
    fn format_name(&self) -> &str {
        "apk"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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
        output.push_str(&format!("P:{}\n", package.name));
        output.push_str(&format!(
            "V:{}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("T:{}\n", package.description));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("D:{}\n", dep_names.join(" ")));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("P:") && content.contains("V:"))
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

impl Default for ApkAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Slackware TXZ slack-desc Package Adapter (OOPS Concrete Implementation)
pub struct TxzAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl TxzAdapter {
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

impl PackageFormatAdapter for TxzAdapter {
    fn format_name(&self) -> &str {
        "txz"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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
        output.push_str(&format!("PACKAGE_NAME={}\n", package.name));
        output.push_str(&format!(
            "PACKAGE_VERSION={}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("PACKAGE_DESC={}\n", package.description));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("PACKAGE_REQUIRED={}\n", dep_names.join(",")));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("PACKAGE_NAME=") || content.contains("slack-desc"))
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

impl Default for TxzAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Void Linux XBPS Package Adapter (OOPS Concrete Implementation)
pub struct XbpsAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl XbpsAdapter {
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

impl PackageFormatAdapter for XbpsAdapter {
    fn format_name(&self) -> &str {
        "xbps"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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
                description = trimmed[11..].to_string();
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
        output.push_str(&format!("pkgname={}\n", package.name));
        output.push_str(&format!(
            "version={}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("short_desc=\"{}\"\n", package.description));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("depends=\"{}\"\n", dep_names.join(" ")));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("pkgname=") && content.contains("short_desc="))
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

impl Default for XbpsAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// CachyOS Microarchitecture Optimized Package Adapter (OOPS Concrete Implementation)
pub struct CachyosAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl CachyosAdapter {
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

impl PackageFormatAdapter for CachyosAdapter {
    fn format_name(&self) -> &str {
        "cachyos"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("pkgname = ") {
                name = trimmed[10..].to_string();
            } else if trimmed.starts_with("pkgver = ") {
                version_str = trimmed[9..].to_string();
            } else if trimmed.starts_with("pkgdesc = ") {
                description = trimmed[10..].to_string();
            } else if trimmed.starts_with("depend = ") {
                dependencies.push(Dependency {
                    name: trimmed[9..].to_string(),
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
        output.push_str("arch = x86-64-v3\n");
        for dep in &package.dependencies {
            output.push_str(&format!("depend = {}\n", dep.name));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("pkgname") && content.contains("x86-64-v"))
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

impl Default for CachyosAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Snap Package Adapter (OOPS Concrete Implementation)
pub struct SnapAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl SnapAdapter {
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

impl PackageFormatAdapter for SnapAdapter {
    fn format_name(&self) -> &str {
        "snap"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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
        output.push_str(&format!("name: {}\n", package.name));
        output.push_str(&format!(
            "version: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("summary: {}\n", package.description));
        output.push_str("confinement: strict\n");
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("requires: {}\n", dep_names.join(" ")));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("name:") && (content.contains("confinement:") || content.contains("grade:")))
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

impl Default for SnapAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Flatpak Package Adapter (OOPS Concrete Implementation)
pub struct FlatpakAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl FlatpakAdapter {
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

impl PackageFormatAdapter for FlatpakAdapter {
    fn format_name(&self) -> &str {
        "flatpak"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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
        output.push_str("[Application]\n");
        output.push_str(&format!("name={}\n", package.name));
        output.push_str(&format!(
            "version={}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("description={}\n", package.description));
        for dep in &package.dependencies {
            output.push_str(&format!("sdk={}\n", dep.name));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("[Application]") || content.contains("flatpak"))
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

impl Default for FlatpakAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Clear Linux swupd Package Adapter (OOPS Concrete Implementation)
pub struct SwupdAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl SwupdAdapter {
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

impl PackageFormatAdapter for SwupdAdapter {
    fn format_name(&self) -> &str {
        "swupd"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("bundle: ") {
                name = trimmed[8..].to_string();
            } else if trimmed.starts_with("version: ") {
                version_str = trimmed[9..].to_string();
            } else if trimmed.starts_with("desc: ") {
                description = trimmed[6..].to_string();
            } else if trimmed.starts_with("include: ") {
                let dep_name = trimmed[9..].to_string();
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
        output.push_str("MANIFEST 1\n");
        output.push_str(&format!("bundle: {}\n", package.name));
        output.push_str(&format!(
            "version: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("desc: {}\n", package.description));
        for dep in &package.dependencies {
            output.push_str(&format!("include: {}\n", dep.name));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("MANIFEST") || content.contains("bundle:"))
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

impl Default for SwupdAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Solus eopkg XML Package Adapter (OOPS Concrete Implementation)
pub struct EopkgAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl EopkgAdapter {
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

impl PackageFormatAdapter for EopkgAdapter {
    fn format_name(&self) -> &str {
        "eopkg"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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
        output.push_str("<Package>\n");
        output.push_str(&format!("  <Name>{}</Name>\n", package.name));
        output.push_str(&format!(
            "  <Version>{}.{}.{}</Version>\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("  <Description>{}</Description>\n", package.description));
        for dep in &package.dependencies {
            output.push_str(&format!("  <Dependency>{}</Dependency>\n", dep.name));
        }
        output.push_str("</Package>\n");
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("<Source>") || content.contains("<Package>") || content.contains("eopkg"))
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

impl Default for EopkgAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// GNU Guix Scheme Package Adapter (OOPS Concrete Implementation)
pub struct GuixAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl GuixAdapter {
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

impl PackageFormatAdapter for GuixAdapter {
    fn format_name(&self) -> &str {
        "guix"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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
                // simple guix scheme inputs parser
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
        output.push_str("(package\n");
        output.push_str(&format!("  (name \"{}\")\n", package.name));
        output.push_str(&format!(
            "  (version \"{}.{}.{}\")\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("  (description \"{}\")\n", package.description));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("  (inputs `({}))\n", dep_names.join(" ")));
        }
        output.push_str(")\n");
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("define-public") || content.contains("(package") || content.contains("(name \""))
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

impl Default for GuixAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// openSUSE Zypper Spec Package Adapter (OOPS Concrete Implementation)
pub struct ZypperAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl ZypperAdapter {
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

impl PackageFormatAdapter for ZypperAdapter {
    fn format_name(&self) -> &str {
        "zypper"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

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
        output.push_str("Vendor: openSUSE\n");
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("Requires: {}\n", dep_names.join(" ")));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("Vendor: openSUSE") || (content.contains("Name:") && content.contains("Requires:")))
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

impl Default for ZypperAdapter {
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
        manager.register_adapter(Box::new(NixAdapter::new()));
        manager.register_adapter(Box::new(EbuildAdapter::new()));
        manager.register_adapter(Box::new(ApkAdapter::new()));
        manager.register_adapter(Box::new(TxzAdapter::new()));
        manager.register_adapter(Box::new(XbpsAdapter::new()));
        manager.register_adapter(Box::new(CachyosAdapter::new()));
        manager.register_adapter(Box::new(SnapAdapter::new()));
        manager.register_adapter(Box::new(FlatpakAdapter::new()));
        manager.register_adapter(Box::new(SwupdAdapter::new()));
        manager.register_adapter(Box::new(EopkgAdapter::new()));
        manager.register_adapter(Box::new(GuixAdapter::new()));
        manager.register_adapter(Box::new(ZypperAdapter::new()));

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

    #[test]
    fn test_nix_adapter_parsing() {
        let adapter = NixAdapter::new();
        let nix_data = b"pname = \"test-nix\"
version = \"1.2.3\"
meta.description = \"Nix package\"
buildInputs = [ glibc openssl ]";

        let package = adapter.parse_package(nix_data).unwrap();
        assert_eq!(package.name, "test-nix");
        assert_eq!(package.version.major, 1);
        assert_eq!(package.version.minor, 2);
        assert_eq!(package.version.patch, 3);
        assert_eq!(package.dependencies.len(), 2);
    }

    #[test]
    fn test_ebuild_adapter_parsing() {
        let adapter = EbuildAdapter::new();
        let ebuild_data = b"PN=\"test-ebuild\"
PV=\"4.5.6\"
DESCRIPTION=\"Gentoo package\"
RDEPEND=\"dev-libs/openssl sys-libs/glibc\"";

        let package = adapter.parse_package(ebuild_data).unwrap();
        assert_eq!(package.name, "test-ebuild");
        assert_eq!(package.version.major, 4);
        assert_eq!(package.dependencies.len(), 2);
    }

    #[test]
    fn test_apk_adapter_parsing() {
        let adapter = ApkAdapter::new();
        let apk_data = b"P:test-apk
V:7.8.9
T:Alpine package
D:musl zlib";

        let package = adapter.parse_package(apk_data).unwrap();
        assert_eq!(package.name, "test-apk");
        assert_eq!(package.version.major, 7);
        assert_eq!(package.dependencies.len(), 2);
    }

    #[test]
    fn test_txz_adapter_parsing() {
        let adapter = TxzAdapter::new();
        let txz_data = b"PACKAGE_NAME=test-txz
PACKAGE_VERSION=1.0.1
PACKAGE_DESC=Slackware package
PACKAGE_REQUIRED=glibc,openssl";

        let package = adapter.parse_package(txz_data).unwrap();
        assert_eq!(package.name, "test-txz");
        assert_eq!(package.version.patch, 1);
        assert_eq!(package.dependencies.len(), 2);
    }

    #[test]
    fn test_xbps_adapter_parsing() {
        let adapter = XbpsAdapter::new();
        let xbps_data = b"pkgname=test-xbps
version=2024.1.1
short_desc=Void package
depends=\"glibc zlib\"";

        let package = adapter.parse_package(xbps_data).unwrap();
        assert_eq!(package.name, "test-xbps");
        assert_eq!(package.version.major, 2024);
        assert_eq!(package.dependencies.len(), 2);
    }

    #[test]
    fn test_cachyos_adapter_parsing() {
        let adapter = CachyosAdapter::new();
        let cachy_data = b"pkgname = test-cachy
pkgver = 1.0.0
pkgdesc = CachyOS package
depend = glibc";

        let package = adapter.parse_package(cachy_data).unwrap();
        assert_eq!(package.name, "test-cachy");
        assert_eq!(package.version.major, 1);
        assert_eq!(package.dependencies.len(), 1);
    }

    #[test]
    fn test_snap_adapter_parsing() {
        let adapter = SnapAdapter::new();
        let snap_data = b"name: test-snap
version: 2.1.0
summary: Snap package
requires: core22";

        let package = adapter.parse_package(snap_data).unwrap();
        assert_eq!(package.name, "test-snap");
        assert_eq!(package.version.major, 2);
        assert_eq!(package.dependencies.len(), 1);
    }

    #[test]
    fn test_flatpak_adapter_parsing() {
        let adapter = FlatpakAdapter::new();
        let flatpak_data = b"name=test-flatpak
version=1.4.2
description=Flatpak package
sdk=org.freedesktop.Sdk";

        let package = adapter.parse_package(flatpak_data).unwrap();
        assert_eq!(package.name, "test-flatpak");
        assert_eq!(package.version.major, 1);
        assert_eq!(package.dependencies.len(), 1);
    }

    #[test]
    fn test_swupd_adapter_parsing() {
        let adapter = SwupdAdapter::new();
        let swupd_data = b"bundle: test-swupd
version: 1.0.0
desc: Clear Linux package
include: os-core";

        let package = adapter.parse_package(swupd_data).unwrap();
        assert_eq!(package.name, "test-swupd");
        assert_eq!(package.version.major, 1);
        assert_eq!(package.dependencies.len(), 1);
    }

    #[test]
    fn test_eopkg_adapter_parsing() {
        let adapter = EopkgAdapter::new();
        let eopkg_data = b"<Package>
  <Name>test-eopkg</Name>
  <Version>3.1.2</Version>
  <Description>Solus package</Description>
  <Dependency>glibc</Dependency>
</Package>";

        let package = adapter.parse_package(eopkg_data).unwrap();
        assert_eq!(package.name, "test-eopkg");
        assert_eq!(package.version.major, 3);
        assert_eq!(package.dependencies.len(), 1);
    }

    #[test]
    fn test_guix_adapter_parsing() {
        let adapter = GuixAdapter::new();
        let guix_data = b"(package
  (name \"test-guix\")
  (version \"0.9.1\")
  (description \"Guix package\")
  (inputs `(\"glibc\" \"openssl\"))
)";

        let package = adapter.parse_package(guix_data).unwrap();
        assert_eq!(package.name, "test-guix");
        assert_eq!(package.version.major, 0);
        assert_eq!(package.dependencies.len(), 2);
    }

    #[test]
    fn test_zypper_adapter_parsing() {
        let adapter = ZypperAdapter::new();
        let zypper_data = b"Name: test-zypper
Version: 15.3.0
Summary: SUSE package
Requires: glibc openssl";

        let package = adapter.parse_package(zypper_data).unwrap();
        assert_eq!(package.name, "test-zypper");
        assert_eq!(package.version.major, 15);
        assert_eq!(package.dependencies.len(), 2);
    }
}
