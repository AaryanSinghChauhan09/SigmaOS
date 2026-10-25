// SPDX-License-Identifier: MIT
// Integration Test for OpenBSD Missing Components V67 PR Gateway Engine

use sigmaos::distro::openbsd_missing_components_v67_pr::*;

#[test]
fn test_openbsd_missing_components_v67_pr_integration() {
    let mut suite = OpenBsdMissingComponentsV67PrSuite::new();
    let summaries = suite.run_all_checks();
    assert_eq!(summaries.len(), 11);
    for summary in summaries {
        assert!(!summary.is_empty());
    }
}
