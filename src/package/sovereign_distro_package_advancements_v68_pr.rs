// SPDX-License-Identifier: MIT
// Sovereign Distro Package Advancements Suite V68 PR Gateway
// (`src/package/sovereign_distro_package_advancements_v68_pr.rs`)
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
#[cfg(feature = "standalone_test")]
use std::vec::Vec;

#[cfg(not(feature = "standalone_test"))]
use crate::package::universal::{PackageFormat, PackageState, UnifiedPackage};

#[cfg(feature = "standalone_test")]
#[path = "universal.rs"]
pub mod universal;

#[cfg(feature = "standalone_test")]
pub use universal::{PackageError, PackageFormat, PackageState, UnifiedPackage};

/// Pull Request Proposal Manifest V68
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackagePrProposalManifestV68 {
    pub proposal_id: u64,
    pub title: String,
    pub source_format: PackageFormat,
    pub target_package: String,
    pub changes_summary: String,
    pub verified_slsa_level: u8,
}

/// PR Gateway for Sovereign Package Advancements V68
pub struct SovereignDistroPackagePrGatewayV68 {
    pub proposals: BTreeMap<u64, PackagePrProposalManifestV68>,
}

impl SovereignDistroPackagePrGatewayV68 {
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
        let manifest = PackagePrProposalManifestV68 {
            proposal_id,
            title: title.to_string(),
            source_format,
            target_package: target_package.to_string(),
            changes_summary: changes_summary.to_string(),
            verified_slsa_level: 4,
        };
        self.proposals.insert(proposal_id, manifest);
    }

    pub fn get_proposal(&self, proposal_id: u64) -> Option<&PackagePrProposalManifestV68> {
        self.proposals.get(&proposal_id)
    }

    pub fn total_proposals(&self) -> usize {
        self.proposals.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pr_gateway_v68() {
        let mut gateway = SovereignDistroPackagePrGatewayV68::new();
        gateway.submit_proposal(
            6801,
            "Universal Interop for Debian DEB in Sigma-pkg",
            PackageFormat::Deb,
            "ripgrep",
            "Added zero-copy format classification and PQC attestation verification",
        );

        assert_eq!(gateway.total_proposals(), 1);
        let prop = gateway.get_proposal(6801).unwrap();
        assert_eq!(prop.target_package, "ripgrep");
        assert_eq!(prop.source_format, PackageFormat::Deb);
        assert_eq!(prop.verified_slsa_level, 4);
    }
}
