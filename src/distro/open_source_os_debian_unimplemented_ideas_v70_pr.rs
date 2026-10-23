// SPDX-License-Identifier: MIT
// Sovereign Open Source OS & Debian Linux Unimplemented Ideas V70 PR Engine
// (`src/distro/open_source_os_debian_unimplemented_ideas_v70_pr.rs`)

#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::distro::debian_github_repo_missing_components_v70_pr::*;
#[cfg(not(any(feature = "standalone_test", test)))]
use crate::distro::open_source_os_unimplemented_ideas_v70_pr::*;

#[cfg(any(feature = "standalone_test", test))]
use crate::debian_github_repo_missing_components_v70_pr::*;
#[cfg(any(feature = "standalone_test", test))]
use crate::open_source_os_unimplemented_ideas_v70_pr::*;

#[derive(Debug, Clone)]
pub struct UnifiedOsDebianV70PrProposal {
    pub pr_id: String,
    pub title: String,
    pub target_component: String,
    pub status: String,
    pub description: String,
    pub git_diff_patch: String,
    pub pqc_signature: String,
}

pub struct OpenSourceOsDebianUnimplementedIdeasV70PrSuite {
    pub os_unimplemented_suite: OpenSourceOsUnimplementedIdeasV70PrSuite,
    pub debian_missing_suite: DebianGithubRepoMissingComponentsV70PrSuite,
}

impl OpenSourceOsDebianUnimplementedIdeasV70PrSuite {
    pub fn new() -> Self {
        Self {
            os_unimplemented_suite: OpenSourceOsUnimplementedIdeasV70PrSuite::new(),
            debian_missing_suite: DebianGithubRepoMissingComponentsV70PrSuite::new(),
        }
    }

    pub fn collect_all_proposals(&self) -> Vec<UnifiedOsDebianV70PrProposal> {
        let mut proposals = Vec::new();

        for spec in self.os_unimplemented_suite.collect_all_pr_specs() {
            proposals.push(UnifiedOsDebianV70PrProposal {
                pr_id: format!("PR-OS-V70-{}", spec.pr_id),
                title: spec.title,
                target_component: spec.subsystem,
                status: "Staged".to_string(),
                description: spec.description,
                git_diff_patch: spec.diff_patch,
                pqc_signature: format!("DILITHIUM5-SIG-OS-{}", spec.pr_id),
            });
        }

        for spec in self.debian_missing_suite.collect_all_pr_specs() {
            proposals.push(UnifiedOsDebianV70PrProposal {
                pr_id: format!("PR-DEBIAN-V70-{}", spec.pr_id),
                title: spec.title,
                target_component: spec.component_name,
                status: "Staged".to_string(),
                description: spec.description,
                git_diff_patch: spec.diff_patch,
                pqc_signature: spec.pqc_signature,
            });
        }

        proposals
    }
}
