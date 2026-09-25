// SPDX-License-Identifier: MIT
// Sovereign Universal Package Manager Pull Request Gateway Engine
//
// Bridges Linux & BSD distro package managers (apt, pacman, dnf, zypper, apk, xbps, ebuild, pkg, nix, flatpak, snap, appimage)
// into SigmaPkg via Pull Request workflow submission format.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::pull_request_workflow::{
    ConsolidatedSovereignPackage, PullRequestPackageFormat,
    PullRequestStatus, SovereignPackagePullRequestEngine,
};
use super::universal::{
    ForeignDistroManifest, PackageFormat, UnifiedPackage, UniversalPackageManager,
};

/// Universal PR Gateway Entry mapping multi-distro package format metadata
#[derive(Debug, Clone)]
pub struct DistroPrGatewayEntry {
    pub pr_id: u64,
    pub submitter: String,
    pub distro_format: PullRequestPackageFormat,
    pub package_name: String,
    pub package_version: String,
    pub dependencies: Vec<String>,
    pub status: PullRequestStatus,
    pub translated_sigpkg_name: String,
}

/// Sovereign Universal PR Gateway Engine
/// Auto-converts incoming foreign distro PR submissions into sandboxed SigmaPkg packages
pub struct SovereignUniversalPrGatewayEngine {
    pub pr_engine: SovereignPackagePullRequestEngine,
    pub package_manager: UniversalPackageManager,
    pub pr_gateway_registry: BTreeMap<u64, DistroPrGatewayEntry>,
}

impl core::fmt::Debug for SovereignUniversalPrGatewayEngine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SovereignUniversalPrGatewayEngine")
            .field("pr_engine", &self.pr_engine)
            .field("pr_gateway_registry", &self.pr_gateway_registry)
            .finish()
    }
}

impl Default for SovereignUniversalPrGatewayEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignUniversalPrGatewayEngine {
    pub fn new() -> Self {
        Self {
            pr_engine: SovereignPackagePullRequestEngine::new(),
            package_manager: UniversalPackageManager::new(),
            pr_gateway_registry: BTreeMap::new(),
        }
    }

    /// Submits an incoming Linux or BSD distro package as a Pull Request to SigmaPkg
    pub fn submit_distro_package_pr(
        &mut self,
        author: &str,
        name: &str,
        version: &str,
        format: PullRequestPackageFormat,
        manifest_data: &str,
        dependencies: &[&str],
        pqc_signature: &[u8],
    ) -> u64 {
        let pr_id = self.pr_engine.submit_package_pr(
            author,
            name,
            version,
            format,
            manifest_data,
            dependencies,
            pqc_signature,
        );

        let entry = DistroPrGatewayEntry {
            pr_id,
            submitter: author.to_string(),
            distro_format: format,
            package_name: name.to_string(),
            package_version: version.to_string(),
            dependencies: dependencies.iter().map(|s| s.to_string()).collect(),
            status: PullRequestStatus::Open,
            translated_sigpkg_name: format!("sigpkg-{}", name),
        };

        self.pr_gateway_registry.insert(pr_id, entry);
        pr_id
    }

    /// Validates PR dependencies using SAT solver and translates foreign manifest into SigmaPkg
    pub fn validate_and_translate_pr(
        &mut self,
        pr_id: u64,
    ) -> Result<ConsolidatedSovereignPackage, &'static str> {
        if let Err(e) = self.pr_engine.validate_pr(pr_id) {
            println!("validate_pr failed for pr_id {}: {}", pr_id, e);
            return Err(e);
        }
        let translated = self.pr_engine.translate_pr(pr_id)?;

        if let Some(entry) = self.pr_gateway_registry.get_mut(&pr_id) {
            entry.status = PullRequestStatus::Translated;
        }

        // Index in UniversalPackageManager as a foreign manifest
        let manifest = ForeignDistroManifest {
            raw_format: match translated.source_format {
                PullRequestPackageFormat::DebianDeb => PackageFormat::Deb,
                PullRequestPackageFormat::FedoraRpm => PackageFormat::Rpm,
                PullRequestPackageFormat::ArchPkgbuild => PackageFormat::Pacman,
                PullRequestPackageFormat::AlpineApk => PackageFormat::Apk,
                PullRequestPackageFormat::GentooEbuild => PackageFormat::Ebuild,
                PullRequestPackageFormat::VoidXbps => PackageFormat::Xbps,
                PullRequestPackageFormat::FreeBsdPorts => PackageFormat::Ports,
                PullRequestPackageFormat::OpenBsdPorts => PackageFormat::OpenBsdPkg,
                PullRequestPackageFormat::NixFlake => PackageFormat::Nixpkg,
                PullRequestPackageFormat::FlatpakApp => PackageFormat::Flatpak,
                PullRequestPackageFormat::SnapPackage => PackageFormat::Snap,
                PullRequestPackageFormat::AppImage => PackageFormat::AppImage,
                PullRequestPackageFormat::SwupdBundle | PullRequestPackageFormat::ClearBundle => PackageFormat::Swupd,
                PullRequestPackageFormat::StarlingPackage => PackageFormat::Starling,
                PullRequestPackageFormat::MacOsHomebrewBottle => PackageFormat::Bottle,
                PullRequestPackageFormat::IosIpaBundle => PackageFormat::Ipa,
                PullRequestPackageFormat::AndroidAabPackage => PackageFormat::Aab,
                PullRequestPackageFormat::HarmonyHapModule => PackageFormat::Hap,
                PullRequestPackageFormat::ZypperSpec => PackageFormat::Zypper,
                PullRequestPackageFormat::EopkgSpec | PullRequestPackageFormat::SolusEopkg => PackageFormat::Eopkg,
                PullRequestPackageFormat::IpkPackage | PullRequestPackageFormat::OpkgPackage | PullRequestPackageFormat::OpenWrtIpk => PackageFormat::Ipk,
                PullRequestPackageFormat::TczPackage => PackageFormat::Tcz,
                PullRequestPackageFormat::CportsPackage => PackageFormat::Cports,
                _ => PackageFormat::SigmaPkg,
            },
            original_name: translated.name.clone(),
            version: translated.version.clone(),
            architecture: "x86_64".to_string(),
            raw_dependencies: translated.resolved_dependencies.clone(),
            raw_provides: alloc::vec![translated.name.clone()],
            raw_conflicts: Vec::new(),
            maintainer: "Sovereign PR Gateway".to_string(),
        };

        self.package_manager
            .distro_repo_sync
            .index_foreign_manifest(manifest);

        Ok(translated)
    }

    /// Generates Git-style PR diff for manifest comparison
    pub fn generate_distro_pr_diff(
        &self,
        pr_id: u64,
        base_manifest: &str,
    ) -> Result<String, &'static str> {
        self.pr_engine.generate_pr_diff(pr_id, base_manifest)
    }

    /// Auto-merges an approved PR submission into the active SigmaPkg system registry
    pub fn auto_merge_package_pr(
        &mut self,
        pr_id: u64,
    ) -> Result<UnifiedPackage, &'static str> {
        let consolidated = self.pr_engine.merge_pr(pr_id)?;

        if let Some(entry) = self.pr_gateway_registry.get_mut(&pr_id) {
            entry.status = PullRequestStatus::Merged;
        }

        let mut sigpkg = UnifiedPackage::new(
            format!("sigpkg-{}", consolidated.name),
            consolidated.version.clone(),
        )
        .with_format(PackageFormat::SigmaPkg)
        .with_provides(consolidated.name.clone());

        for dep in &consolidated.resolved_dependencies {
            sigpkg = sigpkg.with_dependency(dep.clone());
        }

        self.package_manager.add_package(sigpkg.clone());
        Ok(sigpkg)
    }

    /// Search active and merged PRs by package name or author
    pub fn search_distro_prs(&self, query: &str) -> Vec<DistroPrGatewayEntry> {
        let q = query.to_lowercase();
        self.pr_gateway_registry
            .values()
            .filter(|entry| {
                entry.package_name.to_lowercase().contains(&q)
                    || entry.submitter.to_lowercase().contains(&q)
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_universal_pr_gateway_engine() {
        let mut gateway = SovereignUniversalPrGatewayEngine::new();

        // 1. Submit Debian package PR
        let pr_deb = gateway.submit_distro_package_pr(
            "alice",
            "nginx",
            "1.24.0",
            PullRequestPackageFormat::DebianDeb,
            "Package: nginx\nVersion: 1.24.0\nDepends: libc6, libssl3",
            &["libc6", "libssl3"],
            b"valid_pqc_signature_dilithium5",
        );

        assert_eq!(pr_deb, 1);
        let translated = match gateway.validate_and_translate_pr(pr_deb) {
            Ok(t) => t,
            Err(e) => panic!("validate_and_translate_pr failed with error: {}", e),
        };
        assert_eq!(translated.name, "nginx");

        // 2. Generate PR diff
        let diff = gateway
            .generate_distro_pr_diff(pr_deb, "Package: nginx\nVersion: 1.22.0")
            .unwrap();
        assert!(diff.contains("- Version: 1.22.0"));
        assert!(diff.contains("+ Version: 1.24.0"));

        // 3. Auto-merge PR into SigmaPkg
        let merged_sigpkg = gateway.auto_merge_package_pr(pr_deb).unwrap();
        assert_eq!(merged_sigpkg.name, "sigpkg-nginx");
        assert_eq!(merged_sigpkg.version, "1.24.0");

        // 4. Search PRs
        let search_res = gateway.search_distro_prs("nginx");
        assert_eq!(search_res.len(), 1);
        assert_eq!(search_res[0].status, PullRequestStatus::Merged);
    }

    #[test]
    fn test_sovereign_multi_distro_pr_formats() {
        let mut gateway = SovereignUniversalPrGatewayEngine::new();

        let submissions = [
            ("bob", "ripgrep", "14.1.0", PullRequestPackageFormat::ArchPkgbuild, "pkgname=ripgrep", &["pcre2"][..]),
            ("carol", "htop", "3.3.0", PullRequestPackageFormat::FedoraRpm, "Name: htop", &["ncurses"][..]),
            ("dave", "curl", "8.5.0", PullRequestPackageFormat::AlpineApk, "P:curl", &["sovereign-openssl"][..]),
            ("eve", "vlc", "3.0.20", PullRequestPackageFormat::VoidXbps, "pkgname=vlc", &["ffmpeg"][..]),
            ("frank", "ffmpeg", "6.1.0", PullRequestPackageFormat::FreeBsdPorts, "PORTNAME=ffmpeg", &["libx264"][..]),
            ("grace", "git", "2.43.0", PullRequestPackageFormat::NixFlake, "description = \"git\"", &["zlib"][..]),
            ("heidi", "gimp", "2.10.36", PullRequestPackageFormat::FlatpakApp, "app-id: org.gimp.GIMP", &["babl"][..]),
            ("ivan", "blender", "4.0.2", PullRequestPackageFormat::AppImage, "AppImage Blender", &["glibc"][..]),
            ("jack", "sys-utils", "1.0.0", PullRequestPackageFormat::SwupdBundle, "Clear Swupd Bundle", &["glibc"][..]),
            ("kate", "starling-app", "2.0.0", PullRequestPackageFormat::StarlingPackage, "Starling App", &["sovereign-core"][..]),
            ("leo", "homebrew-tool", "3.0.0", PullRequestPackageFormat::MacOsHomebrewBottle, "Homebrew Bottle", &["openssl"][..]),
        ];

        for (author, name, ver, fmt, manifest, deps) in submissions {
            let pr = gateway.submit_distro_package_pr(
                author,
                name,
                ver,
                fmt,
                manifest,
                deps,
                b"valid_pqc_sig",
            );

            let translated = gateway.validate_and_translate_pr(pr).unwrap();
            assert_eq!(translated.name, name);

            let merged = gateway.auto_merge_package_pr(pr).unwrap();
            assert_eq!(merged.name, format!("sigpkg-{}", name));
        }

        assert_eq!(gateway.pr_gateway_registry.len(), 8);
    }
}
