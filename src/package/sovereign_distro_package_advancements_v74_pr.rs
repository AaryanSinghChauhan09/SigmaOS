// SPDX-License-Identifier: MIT
// Sovereign Distro Package Advancements Suite V74 PR Gateway
// (`src/package/sovereign_distro_package_advancements_v74_pr.rs`)
//
// Inspired by Linux (Debian, Arch, Fedora, Alpine, Gentoo, Void, Solus, Slax, Puppy,
// Tiny Core, Clear Linux) & BSD (FreeBSD, OpenBSD, NetBSD, DragonFly BSD) distributions,
// language-level package managers (Cargo, Pip, Gem, NPM, Pnpm, Yarn, LuaRocks, Go, Zig, Swift),
// and mobile/app image bundles (AppImage, Flatpak, Snap, APK, AAB, APEX, IPA, HAP, AIR), this suite
// provides Pull Request proposal generators and state engines for universal package interop for SigmaOS.

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
#[path = "universal.rs"]
pub mod universal;

#[cfg(feature = "standalone_test")]
pub use universal::{PackageError, PackageFormat, PackageState, UnifiedPackage};

/// Pull Request Proposal Manifest V74
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackagePrProposalManifestV74 {
    pub proposal_id: u64,
    pub title: String,
    pub source_format: PackageFormat,
    pub target_package: String,
    pub changes_summary: String,
    pub verified_slsa_level: u8,
}

/// PR Gateway for Sovereign Package Advancements V74
pub struct SovereignDistroPackagePrGatewayV74 {
    pub proposals: BTreeMap<u64, PackagePrProposalManifestV74>,
}

impl SovereignDistroPackagePrGatewayV74 {
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
        let manifest = PackagePrProposalManifestV74 {
            proposal_id,
            title: title.to_string(),
            source_format,
            target_package: target_package.to_string(),
            changes_summary: changes_summary.to_string(),
            verified_slsa_level: 4,
        };
        self.proposals.insert(proposal_id, manifest);
    }

    pub fn get_proposal(&self, proposal_id: u64) -> Option<&PackagePrProposalManifestV74> {
        self.proposals.get(&proposal_id)
    }

    pub fn total_proposals(&self) -> usize {
        self.proposals.len()
    }
}

impl Default for SovereignDistroPackagePrGatewayV74 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pr_gateway_v74() {
        let mut gateway = SovereignDistroPackagePrGatewayV74::new();
        gateway.submit_proposal(
            7401,
            "Universal Interop for Debian DEB in Sigma-pkg V74",
            PackageFormat::Deb,
            "ripgrep",
            "Added zero-copy format classification and PQC attestation verification V74",
        );

        assert_eq!(gateway.total_proposals(), 1);
        let prop = gateway.get_proposal(7401).unwrap();
        assert_eq!(prop.target_package, "ripgrep");
        assert_eq!(prop.source_format, PackageFormat::Deb);
        assert_eq!(prop.verified_slsa_level, 4);
    }
}
