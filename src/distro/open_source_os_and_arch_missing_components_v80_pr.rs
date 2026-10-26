// SPDX-License-Identifier: MIT
// Master Sovereign Open Source OS Unimplemented Ideas & Arch Linux Missing Components V80 PR Gateway
// (`src/distro/open_source_os_and_arch_missing_components_v80_pr.rs`)

#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;

#[path = "open_source_os_unimplemented_ideas_v80_pr.rs"]
pub mod open_source_os_unimplemented_ideas_v80_pr;

#[path = "arch_linux_missing_components_v80_pr.rs"]
pub mod arch_linux_missing_components_v80_pr;

pub use arch_linux_missing_components_v80_pr::*;
pub use open_source_os_unimplemented_ideas_v80_pr::*;

/// Master Gateway unifying Open Source OS Unimplemented Ideas and Arch Linux Missing Components V80 PRs.
pub struct MasterOpenSourceOsAndArchMissingComponentsV80PrGateway {
    pub os_suite: OpenSourceOsUnimplementedIdeasV80PrSuite,
    pub arch_suite: ArchLinuxMissingComponentsV80PrSuite,
}

impl MasterOpenSourceOsAndArchMissingComponentsV80PrGateway {
    pub fn new() -> Self {
        Self {
            os_suite: OpenSourceOsUnimplementedIdeasV80PrSuite::new(),
            arch_suite: ArchLinuxMissingComponentsV80PrSuite::new(),
        }
    }

    pub fn execute_full_v80_pr_gateway(&mut self) -> (Vec<OpenSourceOsPrProposalV80>, Vec<ArchLinuxV80PrSpec>) {
        let os_prs = self.os_suite.collect_all_pr_proposals();
        let arch_prs = self.arch_suite.run_all_prs();
        (os_prs, arch_prs)
    }

    pub fn compute_unified_v80_parity_score(&mut self) -> u32 {
        let (os_prs, arch_prs) = self.execute_full_v80_pr_gateway();
        if !os_prs.is_empty() && !arch_prs.is_empty() && self.arch_suite.compute_gap_closure_score() == 100 {
            100
        } else {
            0
        }
    }
}
