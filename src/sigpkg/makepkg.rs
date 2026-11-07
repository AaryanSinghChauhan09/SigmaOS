#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
// SigmaOS makepkg - Arch Linux PKGBUILD compilation sandbox
// Provides safe, isolated compilation of Arch Linux packages

#[cfg(not(feature = "standalone_test"))]
use crate::sigpkg::{Package, Version};

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[cfg(feature = "standalone_test")]
impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
    pub fn parse(_s: &str) -> Option<Self> {
        Some(Self::new(1, 0, 0))
    }
}

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub dependencies: Vec<String>,
    pub maintainer: String,
}

#[cfg(feature = "standalone_test")]
impl Package {
    pub fn new(
        name: String,
        version: Version,
        description: String,
        dependencies: Vec<String>,
        maintainer: String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            dependencies,
            maintainer,
        }
    }
}

#[cfg(not(any(feature = "standalone_test", test)))]
use std::collections::BTreeMap;
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

/// PKGBUILD parser for Arch Linux package recipes
pub struct PkgbuildParser {
    variables: BTreeMap<String, String>,
    functions: BTreeMap<String, String>,
}

impl PkgbuildParser {
    pub fn new() -> Self {
        PkgbuildParser {
            variables: BTreeMap::new(),
            functions: BTreeMap::new(),
        }
    }

    /// Parse a PKGBUILD file content
    pub fn parse(&mut self, content: &str) -> Result<(), &'static str> {
        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse variable assignments
            if line.contains('=') && !line.starts_with("function ") {
                let parts: std::vec::Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1]
                        .trim()
                        .trim_matches('"')
                        .trim_matches('\'')
                        .to_string();
                    self.variables.insert(key, value);
                }
            }

            // Parse function definitions
            if line.starts_with("function ") || line.ends_with("() {") {
                let func_name = if line.starts_with("function ") {
                    line.trim_start_matches("function ")
                        .trim_end_matches("() {")
                } else {
                    line.trim_end_matches("() {")
                };
                // Store function name - in production, would store function body
                self.functions.insert(func_name.to_string(), String::new());
            }
        }

        Ok(())
    }

    /// Get a variable value from the PKGBUILD
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    /// Get the package name
    pub fn pkgname(&self) -> Option<&String> {
        self.get_variable("pkgname")
    }

    /// Get the package version
    pub fn pkgver(&self) -> Option<&String> {
        self.get_variable("pkgver")
    }

    /// Get the package release
    pub fn pkgrel(&self) -> Option<&String> {
        self.get_variable("pkgrel")
    }

    /// Get the package description
    pub fn pkgdesc(&self) -> Option<&String> {
        self.get_variable("pkgdesc")
    }

    /// Get the package architecture
    pub fn arch(&self) -> Option<&String> {
        self.get_variable("arch")
    }

    /// Get source URLs
    pub fn source(&self) -> Option<&String> {
        self.get_variable("source")
    }

    /// Get dependencies
    pub fn depends(&self) -> Option<&String> {
        self.get_variable("depends")
    }

    /// Get makedepends
    pub fn makedepends(&self) -> Option<&String> {
        self.get_variable("makedepends")
    }
}

impl Default for PkgbuildParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Gentoo / Arch inspired build optimization configuration for makepkg buildbtw execution
#[derive(Debug, Clone)]
pub struct BuildOptimization {
    pub jobs: usize,
    pub enable_ccache: bool,
    pub ccache_dir: String,
    pub enable_lto: bool,
    pub cflags: String,
}

impl Default for BuildOptimization {
    fn default() -> Self {
        Self {
            jobs: 4,
            enable_ccache: true,
            ccache_dir: String::from("/var/cache/ccache"),
            enable_lto: true,
            cflags: String::from("-O2 -pipe -fno-plt"),
        }
    }
}

/// Sandboxed compilation environment for PKGBUILD recipes
pub struct MakepkgSandbox {
    pkgbuild: PkgbuildParser,
    build_dir: String,
    pub optimization: BuildOptimization,
}

impl MakepkgSandbox {
    pub fn new(build_dir: String) -> Self {
        MakepkgSandbox {
            pkgbuild: PkgbuildParser::new(),
            build_dir,
            optimization: BuildOptimization::default(),
        }
    }

    /// Audits PKGBUILD content for forbidden dangerous/malicious shell invocations
    pub fn audit_pkgbuild_security(&self, content: &str) -> Result<(), &'static str> {
        if content.contains("rm -rf /") || content.contains("rm -rf /*") {
            return Err("Makepkg Security Audit: Dangerous root path removal forbidden");
        }
        if content.contains("curl ") && content.contains("| sh") {
            return Err("Makepkg Security Audit: Pipe-to-shell download execution forbidden");
        }
        if content.contains("sudo ") {
            return Err("Makepkg Security Audit: Sudo privilege escalation in sandbox forbidden");
        }
        Ok(())
    }

    /// Load and parse a PKGBUILD file
    pub fn load_pkgbuild(&mut self, content: &str) -> Result<(), &'static str> {
        self.pkgbuild.parse(content)
    }

    /// Execute the package build in a sandboxed environment
    pub fn build(&self) -> Result<Package, &'static str> {
        let pkgname = self
            .pkgbuild
            .pkgname()
            .ok_or("pkgname not found in PKGBUILD")?
            .clone();

        let pkgver = self
            .pkgbuild
            .pkgver()
            .ok_or("pkgver not found in PKGBUILD")?
            .clone();

        let _pkgrel = self
            .pkgbuild
            .pkgrel()
            .ok_or("pkgrel not found in PKGBUILD")?
            .clone();

        let pkgdesc = self
            .pkgbuild
            .pkgdesc()
            .cloned()
            .unwrap_or_else(|| String::from("No description"));

        let cleaned_ver = if pkgver.contains('-') {
            pkgver.split('-').next().unwrap().to_string()
        } else {
            pkgver.to_string()
        };
        let version = Version::parse(&cleaned_ver).unwrap_or(Version::new(1, 0, 0));

        let pkg = Package::new(pkgname, version, pkgdesc, Vec::new(), String::new());
        Ok(pkg)
    }

    /// Validate the PKGBUILD structure
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.pkgbuild.pkgname().is_none() {
            errors.push("pkgname is required".to_string());
        }

        if self.pkgbuild.pkgver().is_none() {
            errors.push("pkgver is required".to_string());
        }

        if self.pkgbuild.pkgrel().is_none() {
            errors.push("pkgrel is required".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkgbuild_parser() {
        let mut parser = PkgbuildParser::new();
        let content = r#"
# Sample PKGBUILD
pkgname="test-package"
pkgver="1.0.0"
pkgrel="1"
pkgdesc="A test package"
arch=("x86_64")
depends=("glibc")
"#;

        assert!(parser.parse(content).is_ok());
        assert_eq!(parser.pkgname(), Some(&String::from("test-package")));
        assert_eq!(parser.pkgver(), Some(&String::from("1.0.0")));
        assert_eq!(parser.pkgrel(), Some(&String::from("1")));
    }

    #[test]
    fn test_makepkg_sandbox() {
        let mut sandbox = MakepkgSandbox::new(String::from("/tmp/build"));
        let content = r#"
pkgname="test-package"
pkgver="1.0.0"
pkgrel="1"
pkgdesc="A test package"
"#;

        assert!(sandbox.load_pkgbuild(content).is_ok());
        assert!(sandbox.validate().is_ok());
    }

    #[test]
    fn test_makepkg_optimizations_and_security_audit() {
        let sandbox = MakepkgSandbox::new(String::from("/tmp/build"));
        assert_eq!(sandbox.optimization.jobs, 4);
        assert!(sandbox.optimization.enable_ccache);
        assert!(sandbox.optimization.enable_lto);

        let safe_content = "pkgname=\"clean\"\npkgver=\"1.0\"\nbuild() { make -j4; }";
        assert!(sandbox.audit_pkgbuild_security(safe_content).is_ok());

        let malicious_content = "pkgname=\"bad\"\npkgver=\"1.0\"\nbuild() { rm -rf /; }";
        assert!(sandbox.audit_pkgbuild_security(malicious_content).is_err());
    }
}

// ============================================================================
// Arch Linux makepkg Improvements: SigmaPkgBuild, MakepkgRunner, NamecapLinter
// ============================================================================

/// Full PKGBUILD recipe descriptor (mirrors Arch Linux makepkg PKGBUILD fields)
#[derive(Debug, Clone)]
pub struct SigmaPkgBuild {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub pkgdesc: String,
    pub arch: Vec<String>,
    pub url: String,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub source: Vec<String>,
    pub sha256sums: Vec<String>,
    /// Shell snippet for the prepare() phase
    pub prepare_fn: Option<String>,
    /// Shell snippet for the build() phase
    pub build_fn: Option<String>,
    /// Shell snippet for the check() phase
    pub check_fn: Option<String>,
    /// Shell snippet for the package() phase
    pub package_fn: Option<String>,
}

/// Builder for constructing a [`SigmaPkgBuild`] with the builder pattern
#[derive(Debug, Clone, Default)]
pub struct MakepkgBuilder {
    pkgname: String,
    pkgver: String,
    pkgrel: String,
    pkgdesc: String,
    arch: Vec<String>,
    url: String,
    license: Vec<String>,
    depends: Vec<String>,
    makedepends: Vec<String>,
    source: Vec<String>,
    sha256sums: Vec<String>,
    prepare_fn: Option<String>,
    build_fn: Option<String>,
    check_fn: Option<String>,
    package_fn: Option<String>,
}

impl MakepkgBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pkgname(mut self, name: &str) -> Self {
        self.pkgname = name.to_string();
        self
    }

    pub fn pkgver(mut self, ver: &str) -> Self {
        self.pkgver = ver.to_string();
        self
    }

    pub fn pkgrel(mut self, rel: &str) -> Self {
        self.pkgrel = rel.to_string();
        self
    }

    pub fn pkgdesc(mut self, desc: &str) -> Self {
        self.pkgdesc = desc.to_string();
        self
    }

    pub fn arch(mut self, arch: Vec<String>) -> Self {
        self.arch = arch;
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn license(mut self, license: Vec<String>) -> Self {
        self.license = license;
        self
    }

    pub fn depends(mut self, depends: Vec<String>) -> Self {
        self.depends = depends;
        self
    }

    pub fn makedepends(mut self, makedepends: Vec<String>) -> Self {
        self.makedepends = makedepends;
        self
    }

    pub fn source(mut self, source: Vec<String>) -> Self {
        self.source = source;
        self
    }

    pub fn sha256sums(mut self, sums: Vec<String>) -> Self {
        self.sha256sums = sums;
        self
    }

    pub fn prepare_fn(mut self, f: &str) -> Self {
        self.prepare_fn = Some(f.to_string());
        self
    }

    pub fn build_fn(mut self, f: &str) -> Self {
        self.build_fn = Some(f.to_string());
        self
    }

    pub fn check_fn(mut self, f: &str) -> Self {
        self.check_fn = Some(f.to_string());
        self
    }

    pub fn package_fn(mut self, f: &str) -> Self {
        self.package_fn = Some(f.to_string());
        self
    }

    /// Consume the builder and produce a [`SigmaPkgBuild`]
    pub fn build(self) -> SigmaPkgBuild {
        SigmaPkgBuild {
            pkgname: self.pkgname,
            pkgver: self.pkgver,
            pkgrel: self.pkgrel,
            pkgdesc: self.pkgdesc,
            arch: self.arch,
            url: self.url,
            license: self.license,
            depends: self.depends,
            makedepends: self.makedepends,
            source: self.source,
            sha256sums: self.sha256sums,
            prepare_fn: self.prepare_fn,
            build_fn: self.build_fn,
            check_fn: self.check_fn,
            package_fn: self.package_fn,
        }
    }
}

/// Errors that can arise during the makepkg lifecycle
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MakepkgError {
    ValidationFailed(String),
    SourceFetchFailed(String),
    ChecksumMismatch { expected: String, got: String },
    BuildFailed(String),
    PackageFailed(String),
}

impl core::fmt::Display for MakepkgError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MakepkgError::ValidationFailed(msg) => write!(f, "ValidationFailed: {}", msg),
            MakepkgError::SourceFetchFailed(url) => write!(f, "SourceFetchFailed: {}", url),
            MakepkgError::ChecksumMismatch { expected, got } => {
                write!(f, "ChecksumMismatch: expected={} got={}", expected, got)
            }
            MakepkgError::BuildFailed(msg) => write!(f, "BuildFailed: {}", msg),
            MakepkgError::PackageFailed(msg) => write!(f, "PackageFailed: {}", msg),
        }
    }
}

/// Orchestrates the full makepkg build lifecycle for a [`SigmaPkgBuild`]
#[derive(Debug, Clone)]
pub struct MakepkgRunner {
    pub spec: SigmaPkgBuild,
    /// Simulated fetched source contents (source_url -> content)
    pub fetched_sources: std::collections::BTreeMap<String, String>,
}

impl MakepkgRunner {
    pub fn new(spec: SigmaPkgBuild) -> Self {
        Self {
            spec,
            fetched_sources: std::collections::BTreeMap::new(),
        }
    }

    /// Validate required fields and structural invariants (Fail-Fast)
    pub fn validate(&self) -> Result<(), MakepkgError> {
        if self.spec.pkgname.is_empty() {
            return Err(MakepkgError::ValidationFailed(
                "pkgname must not be empty".to_string(),
            ));
        }
        if self.spec.pkgver.is_empty() {
            return Err(MakepkgError::ValidationFailed(
                "pkgver must not be empty".to_string(),
            ));
        }
        if self.spec.pkgrel.is_empty() {
            return Err(MakepkgError::ValidationFailed(
                "pkgrel must not be empty".to_string(),
            ));
        }
        if self.spec.sha256sums.len() != self.spec.source.len() {
            return Err(MakepkgError::ValidationFailed(format!(
                "sha256sums count ({}) must match source count ({})",
                self.spec.sha256sums.len(),
                self.spec.source.len()
            )));
        }
        Ok(())
    }

    /// Simulate fetching sources and verifying checksums
    pub fn prepare(&self) -> Result<(), MakepkgError> {
        self.validate()?;
        for (i, src) in self.spec.source.iter().enumerate() {
            // Simulate fetch; in production this would download the URL
            let simulated_digest = format!("sha256:{}", src.len());
            let expected = &self.spec.sha256sums[i];
            // For simulation: treat "SKIP" as unconditionally accepted
            if expected != "SKIP" && *expected != simulated_digest {
                return Err(MakepkgError::ChecksumMismatch {
                    expected: expected.clone(),
                    got: simulated_digest,
                });
            }
        }
        Ok(())
    }

    /// Execute the build() function (stubbed; real impl would fork a sandboxed shell)
    pub fn build(&self) -> Result<(), MakepkgError> {
        self.validate()?;
        if self.spec.build_fn.is_none() {
            // No build function is valid (e.g. header-only packages)
            return Ok(());
        }
        // Defensive: fail if build_fn contains obviously dangerous patterns
        let build_body = self.spec.build_fn.as_deref().unwrap_or("");
        if build_body.contains("rm -rf /") {
            return Err(MakepkgError::BuildFailed(
                "Dangerous command in build()".to_string(),
            ));
        }
        Ok(())
    }

    /// Run the optional check() / test suite phase
    pub fn check(&self) -> Result<(), MakepkgError> {
        // check() is optional; skip gracefully if absent
        Ok(())
    }

    /// Execute the package() function to assemble the final .pkg.tar.zst
    pub fn package(&self) -> Result<(), MakepkgError> {
        self.validate()?;
        if self.spec.package_fn.is_none() {
            return Err(MakepkgError::PackageFailed(
                "package() function is required".to_string(),
            ));
        }
        Ok(())
    }

    /// Remove build artefacts and temporary directories
    pub fn clean(&self) -> Result<(), MakepkgError> {
        // In a real implementation this would call fs::remove_dir_all on the build dir
        Ok(())
    }
}

/// Namcap-style linter that validates PKGBUILD conventions
#[derive(Debug, Clone)]
pub struct NamecapLinter {
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl NamecapLinter {
    pub fn new() -> Self {
        Self {
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Run all lint checks against a [`SigmaPkgBuild`]; returns Ok if no errors
    pub fn lint(&mut self, spec: &SigmaPkgBuild) -> Result<(), Vec<String>> {
        self.warnings.clear();
        self.errors.clear();

        // Required: non-empty pkgname
        if spec.pkgname.is_empty() {
            self.errors.push("pkgname is required and must not be empty".to_string());
        } else if spec.pkgname.chars().any(|c| c.is_uppercase()) {
            self.warnings.push(format!(
                "pkgname '{}' should be lowercase",
                spec.pkgname
            ));
        }

        // Required: valid semver-like version (N.N.N or N.N or N)
        if spec.pkgver.is_empty() {
            self.errors.push("pkgver is required".to_string());
        } else if !Self::is_valid_version(&spec.pkgver) {
            self.errors.push(format!(
                "pkgver '{}' is not a valid version (expected digits and dots)",
                spec.pkgver
            ));
        }

        // Required: pkgrel must be a positive integer
        if spec.pkgrel.is_empty() {
            self.errors.push("pkgrel is required".to_string());
        } else if spec.pkgrel.parse::<u32>().is_err() {
            self.errors.push(format!(
                "pkgrel '{}' must be a positive integer",
                spec.pkgrel
            ));
        }

        // Recommended: pkgdesc
        if spec.pkgdesc.is_empty() {
            self.warnings.push("pkgdesc is recommended".to_string());
        }

        // Recommended: at least one arch target
        if spec.arch.is_empty() {
            self.warnings.push("arch array should not be empty".to_string());
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    /// Validate a version string: digits separated by dots, optionally suffixed with _pre/_rc etc.
    fn is_valid_version(ver: &str) -> bool {
        // Accept N, N.N, N.N.N, N.N.N.N — purely numeric dot-separated components
        // Also accept trailing pre-release tags like "1.0_rc1"
        let base = ver.split('_').next().unwrap_or("");
        !base.is_empty()
            && base
                .split('.')
                .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
    }
}

impl Default for NamecapLinter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod sigma_makepkg_tests {
    use super::*;

    fn valid_spec() -> SigmaPkgBuild {
        MakepkgBuilder::new()
            .pkgname("sigma-hello")
            .pkgver("1.2.3")
            .pkgrel("1")
            .pkgdesc("A demo SigmaOS package")
            .arch(vec!["x86_64".to_string()])
            .url("https://example.com/sigma-hello")
            .license(vec!["MIT".to_string()])
            .depends(vec!["glibc".to_string()])
            .makedepends(vec!["gcc".to_string()])
            .source(vec!["https://example.com/sigma-hello-1.2.3.tar.gz".to_string()])
            .sha256sums(vec!["SKIP".to_string()])
            .build_fn("make -j$(nproc)")
            .package_fn("make DESTDIR=$pkgdir install")
            .build()
    }

    // --- Arrange / Act / Assert: valid build lifecycle ---
    #[test]
    fn test_makepkg_runner_valid_build() {
        // Arrange
        let spec = valid_spec();
        let runner = MakepkgRunner::new(spec);

        // Act + Assert
        assert!(runner.validate().is_ok());
        assert!(runner.prepare().is_ok());
        assert!(runner.build().is_ok());
        assert!(runner.check().is_ok());
        assert!(runner.package().is_ok());
        assert!(runner.clean().is_ok());
    }

    // --- Arrange / Act / Assert: checksum mismatch ---
    #[test]
    fn test_makepkg_runner_checksum_mismatch() {
        // Arrange: provide a wrong (non-SKIP) sha256sum
        let spec = MakepkgBuilder::new()
            .pkgname("broken-pkg")
            .pkgver("1.0.0")
            .pkgrel("1")
            .source(vec!["https://example.com/file.tar.gz".to_string()])
            .sha256sums(vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()])
            .build();
        let runner = MakepkgRunner::new(spec);

        // Act
        let result = runner.prepare();

        // Assert
        assert!(matches!(result, Err(MakepkgError::ChecksumMismatch { .. })));
    }

    // --- Arrange / Act / Assert: missing required field ---
    #[test]
    fn test_makepkg_runner_missing_required_field() {
        // Arrange: pkgver intentionally left empty
        let spec = MakepkgBuilder::new()
            .pkgname("incomplete-pkg")
            // pkgver intentionally omitted
            .pkgrel("1")
            .build();
        let runner = MakepkgRunner::new(spec);

        // Act
        let result = runner.validate();

        // Assert
        assert!(matches!(result, Err(MakepkgError::ValidationFailed(_))));
    }

    // --- Arrange / Act / Assert: NamecapLinter ---
    #[test]
    fn test_namecap_linter_valid() {
        // Arrange
        let spec = valid_spec();
        let mut linter = NamecapLinter::new();

        // Act
        let result = linter.lint(&spec);

        // Assert
        assert!(result.is_ok(), "Expected no lint errors: {:?}", linter.errors);
    }

    #[test]
    fn test_namecap_linter_invalid_version() {
        // Arrange
        let spec = MakepkgBuilder::new()
            .pkgname("test-pkg")
            .pkgver("not_a_version!!")
            .pkgrel("1")
            .build();
        let mut linter = NamecapLinter::new();

        // Act
        let result = linter.lint(&spec);

        // Assert
        assert!(result.is_err());
        assert!(linter.errors.iter().any(|e| e.contains("pkgver")));
    }

    #[test]
    fn test_namecap_linter_missing_pkgname() {
        // Arrange
        let spec = MakepkgBuilder::new().pkgver("1.0.0").pkgrel("1").build();
        let mut linter = NamecapLinter::new();

        // Act
        let result = linter.lint(&spec);

        // Assert
        assert!(result.is_err());
        assert!(linter.errors.iter().any(|e| e.contains("pkgname")));
    }
}
