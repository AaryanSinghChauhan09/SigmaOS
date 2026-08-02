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

// SigmaOS Package Importer Framework
// Facilitates importing and translating packages from Debian (.deb), RPM (Fedora/RHEL),
// and Pacman (Arch Linux) formats into native SigmaOS package recipes.

use crate::sigpkg::{BuildSystem, PackageRecipe, Version};

/// OOP Package Importer Trait
pub trait PackageImporter {
    fn importer_name(&self) -> &'static str;
    fn translate_metadata(&self, raw_metadata: &str) -> Result<PackageRecipe, &'static str>;
}

/// Debian .deb Control File Importer
pub struct DebPackageImporter;

impl DebPackageImporter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DebPackageImporter
    }
}

impl PackageImporter for DebPackageImporter {
    fn importer_name(&self) -> &'static str {
        "Debian Control Importer"
    }

    fn translate_metadata(&self, raw_metadata: &str) -> Result<PackageRecipe, &'static str> {
        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut depends = Vec::new();

        for line in raw_metadata.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').map(|s| s.trim()).collect();
            if parts.len() < 2 {
                continue;
            }

            match parts[0] {
                "Package" => name = parts[1].to_string(),
                "Version" => version_str = parts[1].to_string(),
                "Description" => description = parts[1].to_string(),
                "Depends" => {
                    let deps: Vec<&str> = parts[1].split(',').map(|d| d.trim()).collect();
                    for dep in deps {
                        // Split off versions like "libc6 (>= 2.15)"
                        let dep_name = dep.split_whitespace().next().unwrap_or(dep);
                        depends.push(dep_name.to_string());
                    }
                }
                _ => {}
            }
        }

        if name.is_empty() {
            return Err("Missing 'Package' name in DEB control file");
        }

        // Clean up Debian revision parts if any (e.g. 1.0.0-1 -> 1.0.0)
        let cleaned_ver = version_str.split('-').next().unwrap_or("1.0.0");
        let parsed_version = Version::parse(cleaned_ver).unwrap_or(Version::new(1, 0, 0));

        let mut recipe = PackageRecipe::new(name, parsed_version)
            .with_description(description)
            .with_build_system(BuildSystem::Make)
            .with_source(
                "https://deb.debian.org/pool/main/".to_string(),
                "deb-sha256-placeholder".to_string(),
            )
            .with_build_command("make".to_string());

        // We can add translated dependencies to build commands for verification
        for dep in depends {
            let cmd = format!("# depends: {}", dep);
            recipe = recipe.with_install_command(cmd);
        }

        Ok(recipe)
    }
}

/// RedHat .rpm SPEC File Importer
pub struct RpmPackageImporter;

impl RpmPackageImporter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        RpmPackageImporter
    }
}

impl PackageImporter for RpmPackageImporter {
    fn importer_name(&self) -> &'static str {
        "RPM SPEC Importer"
    }

    fn translate_metadata(&self, raw_metadata: &str) -> Result<PackageRecipe, &'static str> {
        let mut name = String::new();
        let mut version_str = String::new();
        let mut summary = String::new();
        let mut requires = Vec::new();

        for line in raw_metadata.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').map(|s| s.trim()).collect();
            if parts.len() < 2 {
                continue;
            }

            match parts[0] {
                "Name" => name = parts[1].to_string(),
                "Version" => version_str = parts[1].to_string(),
                "Summary" => summary = parts[1].to_string(),
                "Requires" => {
                    let reqs: Vec<&str> = parts[1].split_whitespace().collect();
                    for req in reqs {
                        requires.push(req.to_string());
                    }
                }
                _ => {}
            }
        }

        if name.is_empty() {
            return Err("Missing 'Name' tag in RPM SPEC file");
        }

        let parsed_version = Version::parse(&version_str).unwrap_or(Version::new(1, 0, 0));

        let mut recipe = PackageRecipe::new(name, parsed_version)
            .with_description(summary)
            .with_build_system(BuildSystem::CMake)
            .with_source(
                "https://mirrors.fedoraproject.org/".to_string(),
                "rpm-sha256-placeholder".to_string(),
            )
            .with_build_command("cmake . && make".to_string());

        for req in requires {
            let cmd = format!("# requires: {}", req);
            recipe = recipe.with_install_command(cmd);
        }

        Ok(recipe)
    }
}

/// Arch Linux Pacman PKGBUILD File Importer
pub struct PacmanPackageImporter;

impl PacmanPackageImporter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PacmanPackageImporter
    }
}

impl PackageImporter for PacmanPackageImporter {
    fn importer_name(&self) -> &'static str {
        "Pacman PKGBUILD Importer"
    }

    fn translate_metadata(&self, raw_metadata: &str) -> Result<PackageRecipe, &'static str> {
        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut _pkgrel = 1;

        for line in raw_metadata.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.splitn(2, '=').map(|s| s.trim()).collect();
            if parts.len() < 2 {
                continue;
            }

            // Remove double quotes or single quotes from values
            let val = parts[1].trim_matches(|c| c == '"' || c == '\'');

            match parts[0] {
                "pkgname" => name = val.to_string(),
                "pkgver" => version_str = val.to_string(),
                "pkgdesc" => description = val.to_string(),
                "pkgrel" => _pkgrel = val.parse::<u32>().unwrap_or(1),
                _ => {}
            }
        }

        if name.is_empty() {
            return Err("Missing 'pkgname' variable in PKGBUILD");
        }

        let parsed_version = Version::parse(&version_str).unwrap_or(Version::new(1, 0, 0));

        let recipe = PackageRecipe::new(name, parsed_version)
            .with_description(description)
            .with_build_system(BuildSystem::Cargo)
            .with_source(
                "https://sources.archlinux.org/".to_string(),
                "pacman-sha256-placeholder".to_string(),
            )
            .with_build_command("cargo build --release".to_string());

        Ok(recipe)
    }
}
