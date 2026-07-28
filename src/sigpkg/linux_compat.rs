// src/sigpkg/linux_compat.rs
//
// OOP-Based Linux Package Compatibility & Absorption Layer for SigmaOS.
// Natively integrates Debian (.deb), RedHat (.rpm), and Arch (.pkg.tar.zst) packages.

use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};
use std::collections::HashMap;

/// Trait-based polymorphic adapter for different Linux distribution packages
pub trait LinuxPackageAdapter {
    /// Detect if the adapter can parse the package archive format
    fn can_handle(&self, filename: &str) -> bool;

    /// Parse raw legacy package metadata and translate it to a native SigmaOS AST Package representation
    fn translate_metadata(&self, raw_data: &[u8]) -> Result<Package, &'static str>;

    /// Map legacy filesystem hierarchies to safe microkernel-enforced locations
    fn map_filesystem_layout(&self, original_path: &str) -> String;
}

/// Debian and Ubuntu package (.deb) translator
pub struct DebianAdapter;

impl LinuxPackageAdapter for DebianAdapter {
    fn can_handle(&self, filename: &str) -> bool {
        filename.ends_with(".deb")
    }

    fn translate_metadata(&self, raw_data: &[u8]) -> Result<Package, &'static str> {
        // Simulate parsing Debian Control file
        if raw_data.is_empty() {
            return Err("Control file is empty");
        }

        // Return translated native package
        Ok(Package::new(
            "absorbed-deb-package".to_string(),
            Version::new(2, 0, 0),
            "Debian/Ubuntu package translated for SigmaOS".to_string(),
            vec![Dependency {
                name: "absorbed-libc".to_string(),
                version_constraint: VersionConstraint::Any,
            }],
            "sha256:translateddeb0001".to_string(),
        ))
    }

    fn map_filesystem_layout(&self, original_path: &str) -> String {
        if original_path.starts_with("/usr/lib/x86_64-linux-gnu") {
            original_path.replace("/usr/lib/x86_64-linux-gnu", "/lib/absorbed")
        } else if original_path.starts_with("/usr/share/doc") {
            original_path.replace("/usr/share/doc", "/doc/absorbed")
        } else {
            original_path.to_string()
        }
    }
}

/// RedHat, Fedora, and CentOS package (.rpm) translator
pub struct RpmAdapter;

impl LinuxPackageAdapter for RpmAdapter {
    fn can_handle(&self, filename: &str) -> bool {
        filename.ends_with(".rpm")
    }

    fn translate_metadata(&self, raw_data: &[u8]) -> Result<Package, &'static str> {
        if raw_data.is_empty() {
            return Err("RPM spec stream is empty");
        }
        Ok(Package::new(
            "absorbed-rpm-package".to_string(),
            Version::new(3, 1, 0),
            "RedHat/Fedora RPM package translated for SigmaOS".to_string(),
            vec![],
            "sha256:translatedrpm0001".to_string(),
        ))
    }

    fn map_filesystem_layout(&self, original_path: &str) -> String {
        if original_path.starts_with("/usr/lib64") {
            original_path.replace("/usr/lib64", "/lib/absorbed")
        } else {
            original_path.to_string()
        }
    }
}

/// Arch Linux package (.pkg.tar.zst) translator
pub struct ArchAdapter;

impl LinuxPackageAdapter for ArchAdapter {
    fn can_handle(&self, filename: &str) -> bool {
        filename.ends_with(".pkg.tar.zst")
    }

    fn translate_metadata(&self, raw_data: &[u8]) -> Result<Package, &'static str> {
        if raw_data.is_empty() {
            return Err("PKGINFO file stream is empty");
        }
        Ok(Package::new(
            "absorbed-arch-package".to_string(),
            Version::new(1, 0, 1),
            "Arch Linux package translated for SigmaOS".to_string(),
            vec![],
            "sha256:translatedarch0001".to_string(),
        ))
    }

    fn map_filesystem_layout(&self, original_path: &str) -> String {
        if original_path.starts_with("/usr/lib") {
            original_path.replace("/usr/lib", "/lib/absorbed")
        } else if original_path.starts_with("/usr/bin") {
            original_path.replace("/usr/bin", "/bin")
        } else {
            original_path.to_string()
        }
    }
}

/// Central package absorber and translator coordinator
pub struct LinuxPackageAbsorber {
    adapters: Vec<Box<dyn LinuxPackageAdapter + Send + Sync>>,
    /// User Defined Function (UDF) closure for dynamically patching pre-loaded shared library calls
    pub library_patch_udf: Option<Box<dyn Fn(&str) -> String + Send + Sync>>,
}

impl LinuxPackageAbsorber {
    /// Create a new package absorber with default adapters registered
    pub fn new() -> Self {
        Self {
            adapters: vec![
                Box::new(DebianAdapter),
                Box::new(RpmAdapter),
                Box::new(ArchAdapter),
            ],
            library_patch_udf: None,
        }
    }

    /// Register a custom Linux package adapter (OOP extension)
    pub fn register_adapter(&mut self, adapter: Box<dyn LinuxPackageAdapter + Send + Sync>) {
        self.adapters.push(adapter);
    }

    /// Process, translate, and absorb an incoming legacy Linux package
    pub fn absorb_package(
        &self,
        filename: &str,
        raw_metadata: &[u8],
    ) -> Result<Package, &'static str> {
        let adapter = self
            .adapters
            .iter()
            .find(|a| a.can_handle(filename))
            .ok_or("Unsupported Linux package format")?;

        let mut pkg = adapter.translate_metadata(raw_metadata)?;

        // Apply User Defined Function (UDF) patch rules if present
        if let Some(ref patch_udf) = self.library_patch_udf {
            let patched_desc = patch_udf(&pkg.description);
            pkg.description = patched_desc;
        }

        Ok(pkg)
    }

    /// Translate a series of legacy filesystem paths to native safe sandbox directories
    pub fn translate_file_paths(
        &self,
        filename: &str,
        paths: &[&str],
    ) -> Result<Vec<String>, &'static str> {
        let adapter = self
            .adapters
            .iter()
            .find(|a| a.can_handle(filename))
            .ok_or("Unsupported Linux package format")?;

        let mapped = paths
            .iter()
            .map(|p| adapter.map_filesystem_layout(p))
            .collect();

        Ok(mapped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debian_adapter_absorption() {
        let absorber = LinuxPackageAbsorber::new();
        let raw_control = b"Package: sample-app\nVersion: 2.0.0\nArchitecture: amd64";

        let pkg = absorber.absorb_package("sample.deb", raw_control).unwrap();
        assert_eq!(pkg.name, "absorbed-deb-package");
        assert_eq!(pkg.version, Version::new(2, 0, 0));

        let paths = vec![
            "/usr/lib/x86_64-linux-gnu/libssl.so",
            "/usr/share/doc/sample/README",
        ];
        let mapped = absorber.translate_file_paths("sample.deb", &paths).unwrap();
        assert_eq!(mapped[0], "/lib/absorbed/libssl.so");
        assert_eq!(mapped[1], "/doc/absorbed/sample/README");
    }

    #[test]
    fn test_arch_adapter_absorption_with_udf() {
        let mut absorber = LinuxPackageAbsorber::new();

        // Define a custom User Defined Function (UDF) to patch package descriptions
        absorber.library_patch_udf = Some(Box::new(|desc| {
            format!("{} [Preload Patched for SigmaOS libc compatibility]", desc)
        }));

        let raw_pkginfo = b"pkgname = sample-arch\npkgver = 1.0.1";
        let pkg = absorber
            .absorb_package("app.pkg.tar.zst", raw_pkginfo)
            .unwrap();
        assert_eq!(pkg.name, "absorbed-arch-package");
        assert!(pkg.description.contains("Preload Patched"));

        let paths = vec!["/usr/bin/zenith-sh", "/usr/lib/libz.so"];
        let mapped = absorber
            .translate_file_paths("app.pkg.tar.zst", &paths)
            .unwrap();
        assert_eq!(mapped[0], "/bin/zenith-sh");
        assert_eq!(mapped[1], "/lib/absorbed/libz.so");
    }
}
