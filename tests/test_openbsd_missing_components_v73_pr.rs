// SPDX-License-Identifier: MIT
// Integration tests for OpenBSD Missing Components V73 PR Suite
// (`tests/test_openbsd_missing_components_v73_pr.rs`)

extern crate alloc;

#[path = "../src/distro/openbsd_missing_components_v73_pr.rs"]
mod openbsd_missing_components_v73_pr;

use alloc::string::String;
use alloc::vec::Vec;
use openbsd_missing_components_v73_pr::*;

#[test]
fn test_integration_openbsd_v73_master_suite() {
    let mut suite = OpenBsdMissingComponentsV73PrSuite::new();
    let summaries: Vec<String> = suite.run_all_checks();
    assert_eq!(summaries.len(), 10);
    for s in &summaries {
        assert!(!s.is_empty());
        assert!(s.contains("V73") || s.contains("v73") || s.contains("Dilithium"));
    }
}

#[test]
fn test_integration_openbsd_v73_engines() {
    let mut pinsys = OpenBsdPinsyscallShadowStackKernelEnforcerV73PrEngine::new();
    pinsys.register_pinned_region(PinsyscallShadowStackRegionV73 {
        process_id: 101,
        executable_path: String::from("/bin/sh"),
        text_segment_start: 0x1000,
        text_segment_end: 0x8000,
        shadow_stack_pointer: 0x7fff_0000,
        is_pinned: true,
        pte_read_only: true,
    });
    let v_ok = pinsys.verify_syscall(101, 1, 0x2000, 0x7fff_0000);
    assert!(v_ok.is_valid);

    let mut unveil = OpenBsdUnveilTreeInheritanceMountIsolationV73PrEngine::new();
    assert!(unveil
        .apply_unveil_tree(100, "/usr/bin", "rx", true)
        .is_ok());
    assert!(unveil.check_access(100, "/usr/bin/cat", 'r'));

    let mut pledge = OpenBsdPledgeSyscallCategoryGuardV73PrEngine::new();
    pledge.apply_pledge(200, "stdio rpath inet");
    assert!(pledge.is_permitted(200, "stdio"));
    assert!(!pledge.is_permitted(200, "wpath"));

    let signify = OpenBsdSignifyPqcDilithium5VerifierV73PrEngine::new();
    let res = signify.verify_file_signature("/bsd", "sig", "openbsd-v73-dilithium5.pub");
    assert!(res.signature_valid);
}
