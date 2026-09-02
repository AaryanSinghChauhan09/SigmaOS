#![allow(unused_variables)]
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
use alloc::format;
use alloc::vec;
// SigmaOS AUR Helper - Arch User Repository integration
// Provides high-speed CLI helpers for AUR metadata parsing and package management

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// AUR package metadata
#[derive(Debug, Clone, PartialEq)]
pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub keywords: Vec<String>,
    pub popularity: f32,
}

impl Eq for AurPackage {}

/// AUR metadata parser
pub struct AurParser {
    cache: BTreeMap<String, AurPackage>,
}

impl AurParser {
    pub fn new() -> Self {
        AurParser {
            cache: BTreeMap::new(),
        }
    }

    /// Parse AUR package metadata from JSON-like format
    pub fn parse_metadata(&mut self, metadata: &str) -> Result<AurPackage, &'static str> {
        let mut name = String::from("unknown");
        let mut version = String::from("1.0.0");

        if let Some(idx) = metadata.find("\"name\":\"") {
            let rest = &metadata[idx + 8..];
            if let Some(end) = rest.find('"') {
                name = rest[..end].to_string();
            }
        }
        if let Some(idx) = metadata.find("\"version\":\"") {
            let rest = &metadata[idx + 11..];
            if let Some(end) = rest.find('"') {
                version = rest[..end].to_string();
            }
        }

        let pkg = AurPackage {
            name,
            version,
            description: String::from("No description"),
            url: String::from("https://aur.archlinux.org"),
            depends: Vec::new(),
            makedepends: Vec::new(),
            keywords: Vec::new(),
            popularity: 1.0,
        };

        self.cache.insert(pkg.name.clone(), pkg.clone());
        Ok(pkg)
    }

    /// Search for packages in AUR
    pub fn search(&self, query: &str) -> Vec<&AurPackage> {
        let mut results = Vec::new();

        for pkg in self.cache.values() {
            if pkg.name.contains(query) || pkg.description.contains(query) {
                results.push(pkg);
            }
        }

        results
    }

    /// Parse standard Arch Linux .SRCINFO format metadata
    pub fn parse_srcinfo(&mut self, srcinfo_text: &str) -> Result<AurPackage, &'static str> {
        let mut pkgname = String::from("unknown");
        let mut pkgver = String::from("1.0.0");
        let mut pkgrel = String::from("1");
        let mut pkgdesc = String::from("No description");
        let mut url = String::from("https://aur.archlinux.org");
        let mut depends = Vec::new();
        let mut makedepends = Vec::new();

        for line in srcinfo_text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }
            if let Some(idx) = trimmed.find('=') {
                let key = trimmed[..idx].trim();
                let val = trimmed[idx + 1..].trim();

                match key {
                    "pkgname" => pkgname = val.to_string(),
                    "pkgver" => pkgver = val.to_string(),
                    "pkgrel" => pkgrel = val.to_string(),
                    "pkgdesc" => pkgdesc = val.to_string(),
                    "url" => url = val.to_string(),
                    "depends" => depends.push(val.to_string()),
                    "makedepends" => makedepends.push(val.to_string()),
                    _ => {}
                }
            }
        }

        let pkg = AurPackage {
            name: pkgname,
            version: format!("{}-{}", pkgver, pkgrel),
            description: pkgdesc,
            url,
            depends,
            makedepends,
            keywords: Vec::new(),
            popularity: 1.0,
        };

        self.cache.insert(pkg.name.clone(), pkg.clone());
        Ok(pkg)
    }

    /// Finds installed orphan packages (packages not required by any installed package)
    pub fn find_orphans(&self, installed: &[String]) -> Vec<String> {
        let mut required = BTreeMap::new();
        for pkg_name in installed {
            if let Some(pkg) = self.get_package(pkg_name) {
                for dep in &pkg.depends {
                    required.insert(dep.clone(), true);
                }
            }
        }

        let mut orphans = Vec::new();
        for pkg_name in installed {
            if !required.contains_key(pkg_name) {
                orphans.push(pkg_name.clone());
            }
        }
        orphans
    }

    /// Get package info by name
    pub fn get_package(&self, name: &str) -> Option<&AurPackage> {
        self.cache.get(name)
    }

    /// Get package dependencies
    pub fn get_dependencies(&self, name: &str) -> Vec<&String> {
        self.get_package(name)
            .map(|pkg| pkg.depends.iter().collect())
            .unwrap_or_default()
    }

    /// Calculate build order based on dependencies
    pub fn calculate_build_order(&self, packages: &[String]) -> Result<Vec<String>, &'static str> {
        let mut order = Vec::new();
        let mut visited = BTreeMap::new();

        for pkg_name in packages {
            if !visited.contains_key(pkg_name) {
                self.visit(pkg_name, &mut order, &mut visited)?;
            }
        }

        Ok(order)
    }

    fn visit(
        &self,
        pkg_name: &str,
        order: &mut Vec<String>,
        visited: &mut BTreeMap<String, bool>,
    ) -> Result<(), &'static str> {
        if visited.get(pkg_name).copied().unwrap_or(false) {
            return Ok(());
        }

        visited.insert(pkg_name.to_string(), true);

        if let Some(pkg) = self.get_package(pkg_name) {
            for dep in &pkg.depends {
                self.visit(dep, order, visited)?;
            }
        }

        order.push(pkg_name.to_string());
        Ok(())
    }
}

impl Default for AurParser {
    fn default() -> Self {
        Self::new()
    }
}

/// AUR helper CLI interface
pub struct AurHelper {
    parser: AurParser,
}

impl AurHelper {
    pub fn new() -> Self {
        AurHelper {
            parser: AurParser::new(),
        }
    }

    /// Sync package from AUR
    pub fn sync(&mut self, pkg_name: &str) -> Result<(), &'static str> {
        // In production, would fetch from AUR RPC
        println!("Syncing package {} from AUR...", pkg_name);
        Ok(())
    }

    /// Update AUR package database
    pub fn update(&mut self) -> Result<(), &'static str> {
        println!("Updating AUR package database...");
        Ok(())
    }

    /// Install package from AUR
    pub fn install(&mut self, pkg_name: &str) -> Result<(), &'static str> {
        self.sync(pkg_name)?;
        println!("Installing package {} from AUR...", pkg_name);
        Ok(())
    }

    /// Search AUR for packages
    pub fn search(&self, query: &str) -> Vec<&AurPackage> {
        self.parser.search(query)
    }

    /// Show package information
    pub fn info(&self, pkg_name: &str) -> Option<&AurPackage> {
        self.parser.get_package(pkg_name)
    }

    /// Clean build cache (equivalent to yay -Sc / pacman -Sc)
    pub fn clean_cache(&mut self) -> usize {
        let count = self.parser.cache.len();
        self.parser.cache.clear();
        count
    }

    /// Inspect PKGBUILD diff safety before execution
    pub fn inspect_pkgbuild(&self, pkgbuild_content: &str) -> bool {
        // Simple safety heuristic: check for suspicious commands
        !pkgbuild_content.contains("rm -rf /") && !pkgbuild_content.contains(":(){ :|:& };:")
    }
}

impl Default for AurHelper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aur_parser() {
        let mut parser = AurParser::new();
        let metadata = r#"{"name":"test","version":"1.0.0"}"#;

        assert!(parser.parse_metadata(metadata).is_ok());
        assert!(parser.get_package("test").is_some());
    }

    #[test]
    fn test_srcinfo_parsing_and_orphans() {
        let mut parser = AurParser::new();
        let srcinfo = r#"
pkgbase = neovim-git
	pkgname = neovim-git
	pkgver = 0.10.0
	pkgrel = 1
	pkgdesc = Vim-fork focused on extensibility and usability
	url = https://neovim.io
	depends = libunwind
	depends = libuv
	makedepends = cmake
"#;
        let pkg = parser.parse_srcinfo(srcinfo).unwrap();
        assert_eq!(pkg.name, "neovim-git");
        assert_eq!(pkg.version, "0.10.0-1");
        assert_eq!(pkg.depends.len(), 2);

        let installed = vec![String::from("neovim-git"), String::from("libunwind")];
        let orphans = parser.find_orphans(&installed);
        assert_eq!(orphans.len(), 1);
        assert_eq!(orphans[0], "neovim-git");
    }

    #[test]
    fn test_aur_helper_extended_operations() {
        let mut helper = AurHelper::new();
        let safe_pkgbuild = "pkgname=foo\nbuild() { cmake . ; make ; }";
        let unsafe_pkgbuild = "pkgname=foo\nbuild() { rm -rf / ; }";

        assert!(helper.inspect_pkgbuild(safe_pkgbuild));
        assert!(!helper.inspect_pkgbuild(unsafe_pkgbuild));

        helper.install("test-app").unwrap();
        let cleaned = helper.clean_cache();
        assert_eq!(cleaned, 0);
    }

    #[test]
    fn test_aur_helper() {
        let helper = AurHelper::new();
        let results = helper.search("test");

        // Should return empty results since cache is empty
        assert!(results.is_empty());
    }

    #[test]
    fn test_build_order() {
        let mut parser = AurParser::new();

        // Add a package with dependencies
        let pkg = AurPackage {
            name: String::from("dep"),
            version: String::from("1.0.0"),
            description: String::from("Dependency"),
            url: String::from("https://aur.archlinux.org"),
            depends: Vec::new(),
            makedepends: Vec::new(),
            keywords: Vec::new(),
            popularity: 0.0,
        };

        parser.cache.insert(pkg.name.clone(), pkg);

        let order = parser.calculate_build_order(&[String::from("dep")]);
        assert!(order.is_ok());
    }
}
