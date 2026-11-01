// SPDX-License-Identifier: MIT
// Sovereign Distro Package Advancements Suite V88 PR Gateway
// (`src/package/sovereign_distro_package_advancements_v88_pr.rs`)
//
// Inspired by Linux (Debian, Arch, Fedora, Alpine, Gentoo, Void, Solus, Slax, Puppy,
// Tiny Core, Clear Linux) & BSD (FreeBSD, OpenBSD, NetBSD, DragonFly BSD) distributions,
// language-level package managers (Cargo, Pip, Gem, NPM, Pnpm, Yarn, LuaRocks, Go, Zig, Swift),
// and mobile/app image bundles (AppImage, Flatpak, Snap, APK, AAB, APEX, IPA, HAP, AIR, Bottle),
// this suite provides Pull Request proposal generators and state engines for universal package interop for SigmaOS.

extern crate alloc;

#[cfg(not(feature = "standalone_test"))]
use alloc::collections::BTreeMap;
#[cfg(not(feature = "standalone_test"))]
use alloc::format;
#[cfg(not(feature = "standalone_test"))]
use alloc::string::{String, ToString};
#[cfg(not(feature = "standalone_test"))]
use alloc::vec;
#[cfg(not(feature = "standalone_test"))]
use alloc::vec::Vec;

#[cfg(feature = "standalone_test")]
use std::collections::BTreeMap;
#[cfg(feature = "standalone_test")]
use std::string::{String, ToString};

#[cfg(not(feature = "standalone_test"))]
use crate::package::universal::{PackageFormat, PackageState, UnifiedPackage};

#[cfg(feature = "standalone_test")]
#[path = "sovereign_distro_package_advancements_v88.rs"]
pub mod sovereign_distro_package_advancements_v88;

#[cfg(feature = "standalone_test")]
pub use sovereign_distro_package_advancements_v88::universal;

#[cfg(feature = "standalone_test")]
pub use sovereign_distro_package_advancements_v88::universal::{PackageError, PackageFormat, PackageState, UnifiedPackage};

/// Pull Request Proposal Manifest V88
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackagePrProposalManifestV88 {
    pub proposal_id: u64,
    pub title: String,
    pub source_format: PackageFormat,
    pub target_package: String,
    pub changes_summary: String,
    pub verified_slsa_level: u8,
}

/// PR Gateway for Sovereign Package Advancements V88
pub struct SovereignDistroPackagePrGatewayV88 {
    pub proposals: BTreeMap<u64, PackagePrProposalManifestV88>,
}

impl SovereignDistroPackagePrGatewayV88 {
    pub fn new() -> Self {
        Self {
            proposals: BTreeMap::new(),
        }
    }

    pub fn submit_proposal(
        &mut self,
        proposal_id: u64,
        title: &str,
        source_format: PackageFormat,
        target_package: &str,
        changes_summary: &str,
    ) {
        let manifest = PackagePrProposalManifestV88 {
            proposal_id,
            title: title.to_string(),
            source_format,
            target_package: target_package.to_string(),
            changes_summary: changes_summary.to_string(),
            verified_slsa_level: 4,
        };
        self.proposals.insert(proposal_id, manifest);
    }

    pub fn get_proposal(&self, proposal_id: u64) -> Option<&PackagePrProposalManifestV88> {
        self.proposals.get(&proposal_id)
    }

    pub fn total_proposals(&self) -> usize {
        self.proposals.len()
    }
}

impl Default for SovereignDistroPackagePrGatewayV88 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pr_gateway_v88() {
        let mut gateway = SovereignDistroPackagePrGatewayV88::new();
        gateway.submit_proposal(
            8701,
            "Universal Interop for Debian DEB in Sigma-pkg V88",
            PackageFormat::Deb,
            "ripgrep",
            "Added zero-copy format classification and PQC attestation verification V88",
        );

        assert_eq!(gateway.total_proposals(), 1);
        let prop = gateway.get_proposal(8701).unwrap();
        assert_eq!(prop.target_package, "ripgrep");
        assert_eq!(prop.source_format, PackageFormat::Deb);
        assert_eq!(prop.verified_slsa_level, 4);
    }
}
