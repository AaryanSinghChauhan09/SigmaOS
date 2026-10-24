// SPDX-License-Identifier: MIT
// Comprehensive Integration Test Suite for Open-Source OS Unimplemented Ideas V72 & CachyOS Missing Components V71 PR Engines

#[path = "../src/distro/open_source_os_unimplemented_ideas_v72_pr.rs"]
mod open_source_os_unimplemented_ideas_v72_pr;

#[path = "../src/distro/cachy_linux_missing_components_v71_pr.rs"]
mod cachy_linux_missing_components_v71_pr;

use cachy_linux_missing_components_v71_pr::*;
use open_source_os_unimplemented_ideas_v72_pr::*;

#[test]
fn test_combined_open_source_os_v72_and_cachy_v71_pr_suites() {
    let os_suite = OpenSourceOsUnimplementedIdeasV72PrSuite::new();
    let os_specs = os_suite.collect_all_pr_specs();
    assert_eq!(os_specs.len(), 9);

    let cachy_suite = CachyLinuxMissingComponentsV71PrSuite::new();
    let cachy_manifests = cachy_suite.collect_all_manifests();
    assert_eq!(cachy_manifests.len(), 10);

    // Verify cross-paradigm compatibility
    for spec in &os_specs {
        assert_eq!(spec.status, OsPrStatusV72::Validated);
        assert!(!spec.title.is_empty());
    }

    for manifest in &cachy_manifests {
        assert_eq!(manifest.status, CachyOsPrStatusV71::Proposed);
        assert!(!manifest.title.is_empty());
    }
}
