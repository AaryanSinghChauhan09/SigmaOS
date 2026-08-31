extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use crate::klib::collections::HashMap;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::security::Permission;

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
    pub confinement: String,
    pub plugs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FlatpakManifest {
    pub id: String,
    pub app_id: String,
    pub command: String,
    pub finish_args: Vec<String>,
}

pub type UniversalPackageAdapter = UniversalPackageManager;

/// Universal Package Format Adapter for SigmaOS (Sovereign Packaging)
/// Natively absorbs, parses, and translates package metadata formats from Apt (.deb),
/// Yum/Rpm (.rpm/.spec), Pacman (PKGBUILD), Snap (snapcraft.yaml), Flatpak (.json),
/// FreeBSD pkg (+MANIFEST), OpenBSD pkg_add (+CONTENTS), and Solus eopkg (pspec.xml).
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

#[derive(Debug, Clone)]
pub struct AptDebManifest {
    pub package: String,
    pub version: String,
    pub depends: Vec<String>,
    pub description: String,
    pub priority: PackagePriority,
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

impl PackageFormatAdapter for DebAdapter {
    fn format_name(&self) -> &str {
        "deb"
    }
    
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
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
        
        let version = Version::parse(&version_str)
            .unwrap_or_else(|_| Version::new(0, 0, 0));
        
        Ok(Package::new(name, version, description, dependencies, String::new()))
    }
    
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("Package: {}\n", package.name));
        output.push_str(&format!("Version: {}.{}.{}\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("Description: {}\n", package.description));
        
        if !package.dependencies.is_empty() {
            output.push_str("Depends: ");
            let dep_names: Vec<&str> = package.dependencies.iter().map(|d| d.name.as_str()).collect();
            output.push_str(&dep_names.join(", "));
            output.push('\n');
        }
        
        Ok(output.into_bytes())
    }
    
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        
        Ok(content.contains("Package:") && content.contains("Version:"))
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
    
    pub fn add_hook<F>(&mut self, hook: F) where F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for RpmAdapter {
    fn format_name(&self) -> &str {
        "rpm"
    }
    
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
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
        
        let version = Version::parse(&version_str)
            .unwrap_or_else(|_| Version::new(0, 0, 0));
        
        Ok(Package::new(name, version, description, dependencies, String::new()))
    }
    
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("Name: {}\n", package.name));
        output.push_str(&format!("Version: {}.{}.{}\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("Summary: {}\n", package.description));
        
        if !package.dependencies.is_empty() {
            output.push_str("Requires: ");
            let dep_names: Vec<&str> = package.dependencies.iter().map(|d| d.name.as_str()).collect();
            output.push_str(&dep_names.join(" "));
            output.push('\n');
        }
        
        Ok(output.into_bytes())
    }
    
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        
        Ok(content.contains("Name:") && content.contains("Version:"))
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
    
    pub fn add_hook<F>(&mut self, hook: F) where F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for PacmanAdapter {
    fn format_name(&self) -> &str {
        "pacman"
    }
    
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
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
        
        let version = Version::parse(&version_str)
            .unwrap_or_else(|_| Version::new(0, 0, 0));
        
        Ok(Package::new(name, version, description, dependencies, String::new()))
    }
    
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("pkgname = {}\n", package.name));
        output.push_str(&format!("pkgver = {}.{}.{}\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("pkgdesc = {}\n", package.description));
        
        for dep in &package.dependencies {
            output.push_str(&format!("depend = {}\n", dep.name));
        }
        
        Ok(output.into_bytes())
    }
    
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        
        Ok(content.contains("pkgname") && content.contains("pkgver"))
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

/// FreeBSD `pkg` (+MANIFEST) package adapter
pub struct FreeBsdPkgAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl FreeBsdPkgAdapter {
    pub fn new() -> Self { Self { user_hooks: Vec::new() } }
}

impl PackageFormatAdapter for FreeBsdPkgAdapter {
    fn format_name(&self) -> &str { "freebsd_pkg" }
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|e| AdapterError::ParseError(e.to_string()))?;
        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("name: ") {
                name = line[6..].replace('"', "").replace(',', "");
            } else if line.starts_with("version: ") {
                version_str = line[9..].replace('"', "").replace(',', "");
            } else if line.starts_with("comment: ") {
                description = line[9..].replace('"', "").replace(',', "");
            } else if line.starts_with("deps: ") || line.contains("origin:") {
                let dep_name = line.split(':').nth(0).unwrap_or("").trim().replace('"', "");
                if !dep_name.is_empty() && dep_name != "deps" && dep_name != "name" {
                    dependencies.push(Dependency { name: dep_name, version_constraint: VersionConstraint::Any });
                }
            }
        }
        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));
        Ok(Package::new(name, version, description, dependencies, String::new()))
    }
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("name: \"{}\"\n", package.name));
        output.push_str(&format!("version: \"{}.{}.{}\"\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("comment: \"{}\"\n", package.description));
        Ok(output.into_bytes())
    }
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("name:") && content.contains("version:"))
    }
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks { hook(package)?; }
        Ok(())
    }
}

impl Default for FreeBsdPkgAdapter {
    fn default() -> Self { Self::new() }
}

/// OpenBSD `pkg_add` (+CONTENTS) package adapter
pub struct OpenBsdPkgAddAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl OpenBsdPkgAddAdapter {
    pub fn new() -> Self { Self { user_hooks: Vec::new() } }
}

impl PackageFormatAdapter for OpenBsdPkgAddAdapter {
    fn format_name(&self) -> &str { "openbsd_pkg" }
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|e| AdapterError::ParseError(e.to_string()))?;
        let mut name = String::new();
        let mut version_str = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("@name ") {
                let full = &line[6..];
                if let Some(dash) = full.rfind('-') {
                    name = full[..dash].to_string();
                    version_str = full[dash + 1..].to_string();
                } else {
                    name = full.to_string();
                }
            } else if line.starts_with("@depend ") {
                let dep_str = &line[8..];
                let dep_name = dep_str.split(':').next().unwrap_or("").to_string();
                dependencies.push(Dependency { name: dep_name, version_constraint: VersionConstraint::Any });
            }
        }
        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));
        Ok(Package::new(name, version, "OpenBSD package".to_string(), dependencies, String::new()))
    }
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("@name {}-{}.{}.{}\n", package.name, package.version.major, package.version.minor, package.version.patch));
        for dep in &package.dependencies {
            output.push_str(&format!("@depend {}:*:\n", dep.name));
        }
        Ok(output.into_bytes())
    }
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("@name "))
    }
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks { hook(package)?; }
        Ok(())
    }
}

impl Default for OpenBsdPkgAddAdapter {
    fn default() -> Self { Self::new() }
}

/// Solus `eopkg` (pspec.xml) package adapter
pub struct SolusEopkgAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl SolusEopkgAdapter {
    pub fn new() -> Self { Self { user_hooks: Vec::new() } }
}

impl PackageFormatAdapter for SolusEopkgAdapter {
    fn format_name(&self) -> &str { "solus_eopkg" }
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|e| AdapterError::ParseError(e.to_string()))?;
        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("<Name>") && line.ends_with("</Name>") {
                name = line[6..line.len() - 7].to_string();
            } else if line.starts_with("<Version>") && line.ends_with("</Version>") {
                version_str = line[9..line.len() - 10].to_string();
            } else if line.starts_with("<Summary>") && line.ends_with("</Summary>") {
                description = line[9..line.len() - 10].to_string();
            }
        }
        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));
        Ok(Package::new(name, version, description, Vec::new(), String::new()))
    }
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str("<Pspec>\n");
        output.push_str(&format!("  <Name>{}</Name>\n", package.name));
        output.push_str(&format!("  <Version>{}.{}.{}</Version>\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("  <Summary>{}</Summary>\n", package.description));
        output.push_str("</Pspec>\n");
        Ok(output.into_bytes())
    }
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("<Name>") && content.contains("<Version>"))
    }
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks { hook(package)?; }
        Ok(())
    }
}

impl Default for SolusEopkgAdapter {
    fn default() -> Self { Self::new() }
}

/// Alpine Linux APK package adapter
pub struct ApkAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl ApkAdapter {
    pub fn new() -> Self {
        Self { user_hooks: Vec::new() }
    }
    pub fn add_hook<F>(&mut self, hook: F) where F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for ApkAdapter {
    fn format_name(&self) -> &str { "apk" }
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|e| AdapterError::ParseError(e.to_string()))?;
        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("P:") {
                name = line[2..].to_string();
            } else if line.starts_with("V:") {
                version_str = line[2..].to_string();
            } else if line.starts_with("T:") {
                description = line[2..].to_string();
            } else if line.starts_with("D:") {
                let deps_str = line[2..].to_string();
                for dep in deps_str.split_whitespace() {
                    dependencies.push(Dependency { name: dep.to_string(), version_constraint: VersionConstraint::Any });
                }
            }
        }
        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));
        Ok(Package::new(name, version, description, dependencies, String::new()))
    }
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("P:{}\n", package.name));
        output.push_str(&format!("V:{}.{}.{}\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("T:{}\n", package.description));
        if !package.dependencies.is_empty() {
            output.push_str("D:");
            let dep_names: Vec<&str> = package.dependencies.iter().map(|d| d.name.as_str()).collect();
            output.push_str(&dep_names.join(" "));
            output.push('\n');
        }
        Ok(output.into_bytes())
    }
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("P:") || content.contains("V:"))
    }
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks { hook(package)?; }
        Ok(())
    }
}

impl Default for ApkAdapter {
    fn default() -> Self { Self::new() }
}

/// NixOS derivation package adapter
pub struct NixAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl NixAdapter {
    pub fn new() -> Self {
        Self { user_hooks: Vec::new() }
    }
    pub fn add_hook<F>(&mut self, hook: F) where F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for NixAdapter {
    fn format_name(&self) -> &str { "nix" }
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|e| AdapterError::ParseError(e.to_string()))?;
        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.contains("pname =") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 { name = parts[1].to_string(); }
            } else if line.contains("version =") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 { version_str = parts[1].to_string(); }
            } else if line.contains("description =") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 { description = parts[1].to_string(); }
            } else if line.contains("buildInputs =") {
                let parts: Vec<&str> = line.split('[').nth(1).unwrap_or("").split(']').next().unwrap_or("").split_whitespace().collect();
                for dep in parts {
                    if !dep.is_empty() {
                        dependencies.push(Dependency { name: dep.to_string(), version_constraint: VersionConstraint::Any });
                    }
                }
            }
        }
        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));
        Ok(Package::new(name, version, description, dependencies, String::new()))
    }
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str("{\n");
        output.push_str(&format!("  pname = \"{}\";\n", package.name));
        output.push_str(&format!("  version = \"{}.{}.{}\";\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("  meta.description = \"{}\";\n", package.description));
        if !package.dependencies.is_empty() {
            output.push_str("  buildInputs = [ ");
            let dep_names: Vec<&str> = package.dependencies.iter().map(|d| d.name.as_str()).collect();
            output.push_str(&dep_names.join(" "));
            output.push_str(" ];\n");
        }
        output.push_str("}\n");
        Ok(output.into_bytes())
    }
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("pname =") || content.contains("buildInputs"))
    }
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks { hook(package)?; }
        Ok(())
    }
}

impl Default for NixAdapter {
    fn default() -> Self { Self::new() }
}

/// Gentoo Portage Ebuild package adapter
pub struct EbuildAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl EbuildAdapter {
    pub fn new() -> Self {
        Self { user_hooks: Vec::new() }
    }
    pub fn add_hook<F>(&mut self, hook: F) where F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for EbuildAdapter {
    fn format_name(&self) -> &str { "ebuild" }
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|e| AdapterError::ParseError(e.to_string()))?;
        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("PN=") {
                name = line[3..].replace('"', "").replace('\'', "");
            } else if line.starts_with("PV=") {
                version_str = line[3..].replace('"', "").replace('\'', "");
            } else if line.starts_with("DESCRIPTION=") {
                description = line[12..].replace('"', "").replace('\'', "");
            } else if line.starts_with("DEPEND=") {
                let deps_str = line[7..].replace('"', "").replace('\'', "").replace('(', "").replace(')', "");
                for dep in deps_str.split_whitespace() {
                    if !dep.is_empty() {
                        dependencies.push(Dependency { name: dep.to_string(), version_constraint: VersionConstraint::Any });
                    }
                }
            }
        }
        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));
        Ok(Package::new(name, version, description, dependencies, String::new()))
    }
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("PN=\"{}\"\n", package.name));
        output.push_str(&format!("PV=\"{}.{}.{}\"\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("DESCRIPTION=\"{}\"\n", package.description));
        if !package.dependencies.is_empty() {
            output.push_str("DEPEND=\"");
            let dep_names: Vec<&str> = package.dependencies.iter().map(|d| d.name.as_str()).collect();
            output.push_str(&dep_names.join(" "));
            output.push_str("\"\n");
        }
        Ok(output.into_bytes())
    }
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("PN=") || content.contains("PV="))
    }
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks { hook(package)?; }
        Ok(())
    }
}

impl Default for EbuildAdapter {
    fn default() -> Self { Self::new() }
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
        manager.register_adapter(Box::new(ApkAdapter::new()));
        manager.register_adapter(Box::new(NixAdapter::new()));
        manager.register_adapter(Box::new(EbuildAdapter::new()));
        manager.register_adapter(Box::new(FreeBsdPkgAdapter::new()));
        manager.register_adapter(Box::new(OpenBsdPkgAddAdapter::new()));
        manager.register_adapter(Box::new(SolusEopkgAdapter::new()));
        
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
        for (_format_name, adapter) in &self.adapters {
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
    pub fn parse_with_format(&self, format_name: &str, data: &[u8]) -> Result<Package, AdapterError> {
        let adapter = self.adapters.get(format_name)
            .ok_or_else(|| AdapterError::UnsupportedFeature(format_name.to_string()))?;
        
        let mut package = adapter.parse_package(data)?;
        adapter.process_hook(&mut package)?;
        Ok(package)
    }
    
    /// Convert package between formats
    pub fn convert_format(&self, package: &Package, target_format: &str) -> Result<Vec<u8>, AdapterError> {
        let adapter = self.adapters.get(target_format)
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
    fn test_bsd_and_solus_package_adapters() {
        let freebsd = FreeBsdPkgAdapter::new();
        let freebsd_data = b"name: \"nginx\"\nversion: \"1.24.0\"\ncomment: \"High performance web server\"";
        let pkg_fb = freebsd.parse_package(freebsd_data).unwrap();
        assert_eq!(pkg_fb.name, "nginx");
        assert_eq!(pkg_fb.version.major, 1);

        let openbsd = OpenBsdPkgAddAdapter::new();
        let openbsd_data = b"@name bash-5.2.15\n@depend gettext-runtime:*:";
        let pkg_ob = openbsd.parse_package(openbsd_data).unwrap();
        assert_eq!(pkg_ob.name, "bash");
        assert_eq!(pkg_ob.version.major, 5);
        assert_eq!(pkg_ob.dependencies.len(), 1);

        let solus = SolusEopkgAdapter::new();
        let solus_data = b"<Pspec>\n  <Name>gnome-terminal</Name>\n  <Version>3.44.0</Version>\n  <Summary>Terminal emulator</Summary>\n</Pspec>";
        let pkg_solus = solus.parse_package(solus_data).unwrap();
        assert_eq!(pkg_solus.name, "gnome-terminal");
        assert_eq!(pkg_solus.version.major, 3);
    }

    #[test]
    fn test_apk_adapter_parsing() {
        let adapter = ApkAdapter::new();
        let apk_data = b"P:test-apk\nV:4.2.0\nT:Alpine test\nD:musl openssl";
        let pkg = adapter.parse_package(apk_data).unwrap();
        assert_eq!(pkg.name, "test-apk");
        assert_eq!(pkg.version.major, 4);
        assert_eq!(pkg.dependencies.len(), 2);
    }

    #[test]
    fn test_nix_adapter_parsing() {
        let adapter = NixAdapter::new();
        let nix_data = b"pname = \"test-nix\";\nversion = \"5.1.0\";\ndescription = \"Nix test\";\nbuildInputs = [ glibc ];";
        let pkg = adapter.parse_package(nix_data).unwrap();
        assert_eq!(pkg.name, "test-nix");
        assert_eq!(pkg.version.major, 5);
        assert_eq!(pkg.dependencies.len(), 1);
    }

    #[test]
    fn test_ebuild_adapter_parsing() {
        let adapter = EbuildAdapter::new();
        let ebuild_data = b"PN=\"test-ebuild\"\nPV=\"6.2.3\"\nDESCRIPTION=\"Gentoo test\"\nDEPEND=\"gcc clang\"";
        let pkg = adapter.parse_package(ebuild_data).unwrap();
        assert_eq!(pkg.name, "test-ebuild");
        assert_eq!(pkg.version.major, 6);
        assert_eq!(pkg.dependencies.len(), 2);
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
}
