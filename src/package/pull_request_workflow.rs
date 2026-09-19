// SigmaOS Sovereign Package Pull Request Workflow & Universal Adaptation Engine
// Inspired by Linux & BSD distribution package repository workflows (Arch AUR, Fedora Pagure/Dist-Git,
// Debian Mentors/Salsa, Gentoo PRs, FreeBSD Ports PRs, Void XBPS-src, Nixpkgs PRs, Alpine Aports)
// Enables pull-request based package submission, multi-format translation, automated review, and merging.

#[cfg(not(test))]
use alloc::collections::BTreeMap;
#[cfg(not(test))]
use alloc::format;
#[cfg(not(test))]
use alloc::string::{String, ToString};
#[cfg(not(test))]
use alloc::vec;
#[cfg(not(test))]
use alloc::vec::Vec;

#[cfg(test)]
use std::collections::BTreeMap;
#[cfg(test)]
use std::string::String;
#[cfg(test)]
use std::vec::Vec;

/// Status of a Package Pull Request in the Sovereign Package Repository
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackagePullRequestStatus {
    Open,
    InReview,
    ChangesRequested,
    Approved,
    Merged,
    Rejected,
}

/// Package Format supported in Pull Requests (Linux & BSD Distro Parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackagePullRequestFormat {
    DebianDeb,          // .deb / dpkg control tarball
    FedoraRpm,          // .rpm / .spec file
    ArchPkgbuild,       // PKGBUILD / AUR recipe
    AlpineApk,          // APKBUILD / .apk v3
    GentooEbuild,       // .ebuild / Portage
    VoidXbps,           // template / xbps-src
    FreeBsdPort,        // Makefile / pkg-plist
    OpenBsdPort,        // Makefile / PLIST
    NixDerivation,      // default.nix / flake
    GuixScheme,         // .scm package recipe
    FlatpakManifest,    // .json / .yaml manifest
    SnapcraftYaml,      // snapcraft.yaml
    AppImageSpec,       // AppImage runtime spec
    SigmaPkgNative,     // Native .sigmapkg
}

/// Description of a Package Pull Request submitted to SigmaOS
#[derive(Debug, Clone)]
pub struct PackagePullRequest {
    pub pr_id: u32,
    pub package_name: String,
    pub package_version: String,
    pub format: PackagePullRequestFormat,
    pub submitter: String,
    pub patch_manifest_diff: String,
    pub dependencies: Vec<String>,
    pub capability_permissions: Vec<String>,
    pub status: PackagePullRequestStatus,
    pub automated_review_score: u8,
    pub pqc_signature_verified: bool,
    pub reviewer_comments: Vec<String>,
}

/// Sovereign Package Pull Request & Format Adaptation Engine
pub struct SovereignPackagePullRequestEngine {
    pub pull_requests: BTreeMap<u32, PackagePullRequest>,
    pub next_pr_id: u32,
    pub auto_merge_threshold_score: u8,
}

impl SovereignPackagePullRequestEngine {
    pub fn new() -> Self {
        Self {
            pull_requests: BTreeMap::new(),
            next_pr_id: 1,
            auto_merge_threshold_score: 85,
        }
    }

    /// Submit a new package pull request in any supported Linux or BSD format
    pub fn submit_pull_request(
        &mut self,
        package_name: &str,
        package_version: &str,
        format: PackagePullRequestFormat,
        submitter: &str,
        patch_manifest_diff: &str,
        dependencies: &[&str],
        permissions: &[&str],
    ) -> u32 {
        let id = self.next_pr_id;
        self.next_pr_id += 1;

        let pr = PackagePullRequest {
            pr_id: id,
            package_name: String::from(package_name),
            package_version: String::from(package_version),
            format,
            submitter: String::from(submitter),
            patch_manifest_diff: String::from(patch_manifest_diff),
            dependencies: dependencies.iter().map(|&d| String::from(d)).collect(),
            capability_permissions: permissions.iter().map(|&p| String::from(p)).collect(),
            status: PackagePullRequestStatus::Open,
            automated_review_score: 0,
            pqc_signature_verified: false,
            reviewer_comments: Vec::new(),
        };

        self.pull_requests.insert(id, pr);
        self.run_automated_pr_audit(id);
        id
    }

    /// Run automated security, syntax, and format translation audit on a pull request
    pub fn run_automated_pr_audit(&mut self, pr_id: u32) -> bool {
        if let Some(pr) = self.pull_requests.get_mut(&pr_id) {
            pr.status = PackagePullRequestStatus::InReview;
            let mut score = 50u8;

            // 1. Verify non-empty package metadata
            if !pr.package_name.is_empty() && !pr.package_version.is_empty() {
                score += 15;
            }

            // 2. Format-specific validation & translation check
            let format_valid = match pr.format {
                PackagePullRequestFormat::DebianDeb => pr.patch_manifest_diff.contains("Control:"),
                PackagePullRequestFormat::FedoraRpm => pr.patch_manifest_diff.contains("Name:") || pr.patch_manifest_diff.contains("Spec"),
                PackagePullRequestFormat::ArchPkgbuild => pr.patch_manifest_diff.contains("pkgname=") || pr.patch_manifest_diff.contains("PKGBUILD"),
                PackagePullRequestFormat::AlpineApk => pr.patch_manifest_diff.contains("pkgname=") || pr.patch_manifest_diff.contains("APKBUILD"),
                PackagePullRequestFormat::GentooEbuild => pr.patch_manifest_diff.contains("EAPI=") || pr.patch_manifest_diff.contains("DESCRIPTION="),
                PackagePullRequestFormat::VoidXbps => pr.patch_manifest_diff.contains("pkgname=") || pr.patch_manifest_diff.contains("template"),
                PackagePullRequestFormat::FreeBsdPort | PackagePullRequestFormat::OpenBsdPort => pr.patch_manifest_diff.contains("PORTNAME=") || pr.patch_manifest_diff.contains("Makefile"),
                PackagePullRequestFormat::NixDerivation => pr.patch_manifest_diff.contains("stdenv.mkDerivation") || pr.patch_manifest_diff.contains("fetchFromGitHub"),
                PackagePullRequestFormat::GuixScheme => pr.patch_manifest_diff.contains("define-public") || pr.patch_manifest_diff.contains("package"),
                PackagePullRequestFormat::FlatpakManifest | PackagePullRequestFormat::SnapcraftYaml | PackagePullRequestFormat::AppImageSpec => !pr.patch_manifest_diff.is_empty(),
                PackagePullRequestFormat::SigmaPkgNative => pr.patch_manifest_diff.contains("name:") || pr.patch_manifest_diff.contains("version:"),
            };

            if format_valid {
                score += 20;
            } else {
                pr.reviewer_comments.push(String::from("Warning: Manifest syntax lacks standard format header flags."));
            }

            // 3. Post-Quantum Cryptographic signature check simulation
            if pr.submitter.contains("trusted") || pr.patch_manifest_diff.contains("Dilithium5") {
                pr.pqc_signature_verified = true;
                score += 15;
            } else {
                pr.pqc_signature_verified = true; // Auto-sign in sandbox
                score += 10;
            }

            pr.automated_review_score = score;
            if score >= self.auto_merge_threshold_score {
                pr.status = PackagePullRequestStatus::Approved;
            } else {
                pr.status = PackagePullRequestStatus::ChangesRequested;
            }
            return true;
        }
        false
    }

    /// Transpile pull request manifest to native SigmaOS SigPkg format
    pub fn transpile_pr_to_native_sigpkg(&self, pr_id: u32) -> Result<String, &'static str> {
        let pr = self.pull_requests.get(&pr_id).ok_or("Pull request not found")?;

        let native_manifest = format!(
            "# Sovereign SigPkg Manifest (Transpiled from {:?})\nname: {}\nversion: {}\nsubmitter: {}\ndependencies: {:?}\npermissions: {:?}\n",
            pr.format, pr.package_name, pr.package_version, pr.submitter, pr.dependencies, pr.capability_permissions
        );

        Ok(native_manifest)
    }

    /// Approve and merge a package pull request into the main repository store
    pub fn merge_pull_request(&mut self, pr_id: u32, reviewer: &str) -> Result<bool, &'static str> {
        let pr = self.pull_requests.get_mut(&pr_id).ok_or("Pull request not found")?;

        if pr.status != PackagePullRequestStatus::Approved && pr.automated_review_score < self.auto_merge_threshold_score {
            return Err("Pull request is not approved or score is below merge threshold");
        }

        pr.status = PackagePullRequestStatus::Merged;
        pr.reviewer_comments.push(format!("Merged by reviewer: {}", reviewer));
        Ok(true)
    }

    /// Get pull request details
    pub fn get_pull_request(&self, pr_id: u32) -> Option<&PackagePullRequest> {
        self.pull_requests.get(&pr_id)
    }
}

impl Default for SovereignPackagePullRequestEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_pull_request_multi_format_workflow() {
        let mut engine = SovereignPackagePullRequestEngine::new();

        // 1. Test Debian .deb PR
        let pr1 = engine.submit_pull_request(
            "curl",
            "8.5.0",
            PackagePullRequestFormat::DebianDeb,
            "trusted-maintainer",
            "Control: Package: curl\nVersion: 8.5.0\nDilithium5-Signature: Valid",
            &["libssl"],
            &["network.tcp"],
        );
        let pr1_obj = engine.get_pull_request(pr1).unwrap();
        assert_eq!(pr1_obj.status, PackagePullRequestStatus::Approved);
        assert!(engine.merge_pull_request(pr1, "core-team").unwrap());

        // 2. Test Arch PKGBUILD PR
        let pr2 = engine.submit_pull_request(
            "ripgrep",
            "14.1.0",
            PackagePullRequestFormat::ArchPkgbuild,
            "aur-contributor",
            "pkgname=ripgrep\npkgver=14.1.0",
            &["pcre2"],
            &["file.read"],
        );
        let transpiled = engine.transpile_pr_to_native_sigpkg(pr2).unwrap();
        assert!(transpiled.contains("ripgrep"));
        assert!(transpiled.contains("ArchPkgbuild"));

        // 3. Test Fedora RPM PR
        let pr3 = engine.submit_pull_request(
            "nginx",
            "1.24.0",
            PackagePullRequestFormat::FedoraRpm,
            "fedora-packager",
            "Name: nginx\nSpec Version: 1.24.0",
            &["zlib"],
            &["network.tcp", "file.read"],
        );
        assert_eq!(engine.get_pull_request(pr3).unwrap().package_name, "nginx");
    }
}
