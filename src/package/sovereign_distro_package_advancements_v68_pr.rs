// SPDX-License-Identifier: MIT
// Sovereign Distro Package Advancements Suite V68 PR Bridge Gateway
// (`src/package/sovereign_distro_package_advancements_v68_pr.rs`)

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
use crate::package::universal::PackageFormat;
#[cfg(not(feature = "standalone_test"))]
use crate::package::sovereign_distro_package_advancements_v68::*;

#[cfg(feature = "standalone_test")]
#[path = "sovereign_distro_package_advancements_v68.rs"]
pub mod sovereign_distro_package_advancements_v68;

#[cfg(feature = "standalone_test")]
pub use sovereign_distro_package_advancements_v68::*;

/// PR Proposal Gateway V68 Bridge
pub struct SovereignDistroPackagePrBridgeGatewayV68 {
    pub inner_suite: SovereignDistroPackageAdvancementsSuiteV68,
    pub published_proposals: BTreeMap<u64, PackagePrProposalManifestV68>,
}

impl SovereignDistroPackagePrBridgeGatewayV68 {
    pub fn new() -> Self {
        Self {
            inner_suite: SovereignDistroPackageAdvancementsSuiteV68::new(),
            published_proposals: BTreeMap::new(),
        }
    }

    pub fn process_package_and_generate_pr(
        &mut self,
        filename: &str,
        payload: &[u8],
    ) -> Result<PackagePrProposalManifestV68, String> {
        let pkg = self.inner_suite.process_and_install(filename, payload)?;
        let proposal_id = (self.published_proposals.len() + 1) as u64 + 6800;

        let format_kind = pkg.formats.first().cloned().unwrap_or(PackageFormat::SigmaPkg);

        let proposal = PackagePrProposalManifestV68 {
            proposal_id,
            title: alloc::format!("PR Bridge: Universal Adaptation for {}", pkg.name),
            source_format: format_kind,
            target_package: pkg.name.clone(),
            changes_summary: alloc::format!(
                "Successfully absorbed package '{}' (version: {}) into SigmaOS canonical primitive.",
                pkg.name, pkg.version
            ),
        };

        self.published_proposals.insert(proposal_id, proposal.clone());
        Ok(proposal)
    }
}

// ============================================================================
// Standalone Unit Tests V68 PR Bridge
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v68_pr_bridge_gateway() {
        let mut bridge = SovereignDistroPackagePrBridgeGatewayV68::new();
        let proposal = bridge
            .process_package_and_generate_pr("nginx-1.26.0.superdeb", b"SUPERDEB_PAYLOAD_V68")
            .unwrap();

        assert_eq!(proposal.proposal_id, 6801);
        assert_eq!(proposal.target_package, "nginx");
        assert!(proposal.title.contains("PR Bridge"));
        assert_eq!(bridge.published_proposals.len(), 1);
    }
}
