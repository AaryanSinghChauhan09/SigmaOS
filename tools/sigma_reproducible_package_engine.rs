//! SigmaOS Reproducible Package Build & Verification Engine
//! Inspired by Debian reproducible-builds.org, Arch Linux arch-repro-status,
//! Nix deterministic store derivations, Void xbps-src chroot isolation, and FreeBSD Ports SOURCE_DATE_EPOCH.

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use alloc::collections::BTreeMap;

/// Reproducible build status verdict
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReproducibleVerdict {
    Reproducible,
    Unreproducible,
    BuildFailed,
    Untested,
}

/// Environment normalization settings for deterministic package builds
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibleEnvironmentConfig {
    pub source_date_epoch: u64,
    pub lang: String,
    pub tz: String,
    pub umask: u32,
    pub path: String,
    pub strip_build_paths: bool,
    pub hostname_override: String,
}

impl ReproducibleEnvironmentConfig {
    pub fn new(source_date_epoch: u64) -> Self {
        Self {
            source_date_epoch,
            lang: "C.UTF-8".to_string(),
            tz: "UTC".to_string(),
            umask: 0o022,
            path: "/usr/bin:/bin".to_string(),
            strip_build_paths: true,
            hostname_override: "reproducible-builder.sigmaos.org".to_string(),
        }
    }

    pub fn to_env_vars(&self) -> BTreeMap<String, String> {
        let mut vars = BTreeMap::new();
        vars.insert("SOURCE_DATE_EPOCH".to_string(), self.source_date_epoch.to_string());
        vars.insert("LANG".to_string(), self.lang.clone());
        vars.insert("LC_ALL".to_string(), self.lang.clone());
        vars.insert("TZ".to_string(), self.tz.clone());
        vars.insert("UMASK".to_string(), format!("{:04o}", self.umask));
        vars.insert("PATH".to_string(), self.path.clone());
        vars.insert("HOSTNAME".to_string(), self.hostname_override.clone());
        if self.strip_build_paths {
            vars.insert("SIGMA_BUILD_PATH".to_string(), "/sovereign/build".to_string());
            vars.insert("FFILE_PREFIX_MAP".to_string(), "/sovereign/build=/usr/src/sigma".to_string());
        }
        vars
    }
}

/// `.buildinfo` & `.buildmanifest` metadata record generator (Debian/Arch/Void parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildinfoManifest {
    pub package_name: String,
    pub version: String,
    pub architecture: String,
    pub source_date_epoch: u64,
    pub build_environment: BTreeMap<String, String>,
    pub file_checksums_sha256: BTreeMap<String, String>,
}

impl BuildinfoManifest {
    pub fn new(pkg_name: &str, version: &str, arch: &str, epoch: u64) -> Self {
        Self {
            package_name: pkg_name.to_string(),
            version: version.to_string(),
            architecture: arch.to_string(),
            source_date_epoch: epoch,
            build_environment: BTreeMap::new(),
            file_checksums_sha256: BTreeMap::new(),
        }
    }

    pub fn add_file_checksum(&mut self, filename: &str, sha256_hex: &str) {
        self.file_checksums_sha256.insert(filename.to_string(), sha256_hex.to_string());
    }

    pub fn render_debian_format(&self) -> String {
        let mut out = String::new();
        out.push_str("Format: 1.0\n");
        out.push_str(&format!("Build-Package-Name: {}\n", self.package_name));
        out.push_str(&format!("Version: {}\n", self.version));
        out.push_str(&format!("Architecture: {}\n", self.architecture));
        out.push_str(&format!("Build-Date: {}\n", self.source_date_epoch));
        out.push_str("Build-Environment:\n");
        for (k, v) in &self.build_environment {
            out.push_str(&format!(" {}={}\n", k, v));
        }
        out.push_str("Checksums-Sha256:\n");
        for (file, hash) in &self.file_checksums_sha256 {
            out.push_str(&format!(" {} {}\n", hash, file));
        }
        out
    }
}

/// Binary Artifact Diffoscope Engine comparing double-build outputs bitwise
pub struct BinaryArtifactDiffEngine;

impl BinaryArtifactDiffEngine {
    pub fn compare_bytes(build1: &[u8], build2: &[u8]) -> (bool, usize) {
        if build1.len() != build2.len() {
            let min_len = core::cmp::min(build1.len(), build2.len());
            let diff_count = build1.iter().zip(build2.iter()).filter(|(a, b)| a != b).count();
            let extra = (build1.len() as isize - build2.len() as isize).abs() as usize;
            return (false, diff_count + extra);
        }

        let diff_count = build1.iter().zip(build2.iter()).filter(|(a, b)| a != b).count();
        (diff_count == 0, diff_count)
    }

    pub fn compute_sha256_dummy(data: &[u8]) -> String {
        let mut state: u64 = 0xcbf29ce484222325;
        for &b in data {
            state ^= b as u64;
            state = state.wrapping_mul(0x100000001b3);
        }
        format!("{:016x}", state)
    }
}

/// Sovereign Reproducible Package Verifier running double builds and evaluating reproducibility
pub struct SovereignReproduciblePackageVerifier {
    pub environment_config: ReproducibleEnvironmentConfig,
    pub records: BTreeMap<String, ReproducibleVerdict>,
}

impl SovereignReproduciblePackageVerifier {
    pub fn new(epoch: u64) -> Self {
        Self {
            environment_config: ReproducibleEnvironmentConfig::new(epoch),
            records: BTreeMap::new(),
        }
    }

    pub fn verify_double_build(&mut self, pkg_name: &str, build1_bytes: &[u8], build2_bytes: &[u8]) -> ReproducibleVerdict {
        let (is_identical, _diff_count) = BinaryArtifactDiffEngine::compare_bytes(build1_bytes, build2_bytes);
        let verdict = if is_identical {
            ReproducibleVerdict::Reproducible
        } else {
            ReproducibleVerdict::Unreproducible
        };
        self.records.insert(pkg_name.to_string(), verdict);
        verdict
    }

    pub fn get_reproducible_percentage(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        let reproducible_count = self.records.values().filter(|&&v| v == ReproducibleVerdict::Reproducible).count();
        (reproducible_count as f64 / self.records.len() as f64) * 100.0
    }
}

// =========================================================================
// Unit Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_normalization() {
        let config = ReproducibleEnvironmentConfig::new(1700000000);
        let vars = config.to_env_vars();
        assert_eq!(vars.get("SOURCE_DATE_EPOCH").unwrap(), "1700000000");
        assert_eq!(vars.get("LANG").unwrap(), "C.UTF-8");
        assert_eq!(vars.get("TZ").unwrap(), "UTC");
    }

    #[test]
    fn test_buildinfo_manifest() {
        let mut manifest = BuildinfoManifest::new("curl", "8.5.0", "x86_64", 1700000000);
        manifest.add_file_checksum("curl.sigpkg", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        let deb_fmt = manifest.render_debian_format();
        assert!(deb_fmt.contains("Build-Package-Name: curl"));
        assert!(deb_fmt.contains("curl.sigpkg"));
    }

    #[test]
    fn test_binary_artifact_diff_engine() {
        let b1 = b"BITWISE_IDENTICAL_BUILD_OUTPUT_BYTES";
        let b2 = b"BITWISE_IDENTICAL_BUILD_OUTPUT_BYTES";
        let b3 = b"BITWISE_DIFFERENT_BUILD_OUTPUT_BYTES";

        let (identical1, diff1) = BinaryArtifactDiffEngine::compare_bytes(b1, b2);
        assert!(identical1);
        assert_eq!(diff1, 0);

        let (identical2, diff2) = BinaryArtifactDiffEngine::compare_bytes(b1, b3);
        assert!(!identical2);
        assert!(diff2 > 0);
    }

    #[test]
    fn test_sovereign_reproducible_verifier() {
        let mut verifier = SovereignReproduciblePackageVerifier::new(1700000000);
        let build_a = b"BUILD_A_PAYLOAD";
        let build_b = b"BUILD_B_PAYLOAD_DIFF";

        let verdict1 = verifier.verify_double_build("nginx", build_a, build_a);
        assert_eq!(verdict1, ReproducibleVerdict::Reproducible);

        let verdict2 = verifier.verify_double_build("apache", build_a, build_b);
        assert_eq!(verdict2, ReproducibleVerdict::Unreproducible);

        assert_eq!(verifier.get_reproducible_percentage(), 50.0);
    }
}
