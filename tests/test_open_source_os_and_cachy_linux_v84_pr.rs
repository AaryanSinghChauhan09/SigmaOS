// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! Integration Test Suite for V84 Open Source OS Unimplemented Ideas & Cachy Linux Missing Components PR Engine
//! (`tests/test_open_source_os_and_cachy_linux_v84_pr.rs`)

#[path = "../src/distro/cachy_linux_missing_components_v84_pr.rs"]
mod cachy_linux_missing_components_v84_pr;
#[path = "../src/distro/open_source_os_and_cachy_linux_v84_pr.rs"]
mod open_source_os_and_cachy_linux_v84_pr;
#[path = "../src/distro/open_source_os_unimplemented_ideas_v84_pr.rs"]
mod open_source_os_unimplemented_ideas_v84_pr;

use cachy_linux_missing_components_v84_pr::*;
use open_source_os_and_cachy_linux_v84_pr::*;
use open_source_os_unimplemented_ideas_v84_pr::*;

#[test]
fn test_open_source_os_v84_engines_and_manifests() {
    let suite = SovereignUnimplementedIdeasSuiteV84::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 9);

    assert_eq!(manifests[0].pr_id, "PR-SIGMA-V84-01-LINUX-638");
    assert!(manifests[0].title.contains("Linux 6.38+"));
    assert_eq!(manifests[0].status, OsPrStatusV84::Validated);
}

#[test]
fn test_cachy_linux_v84_engines_and_manifests() {
    let suite = CachyLinuxMissingComponentsV84PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 10);
    assert_eq!(manifests[0].pr_number, 8401);
}

#[test]
fn test_v84_gateway_execution() {
    let gateway = SovereignOpenSourceOsAndCachyLinuxV84Gateway::new();
    assert!(gateway.execute_all_v84_validations());
}
