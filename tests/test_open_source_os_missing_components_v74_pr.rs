// SPDX-License-Identifier: MIT
// Integration Test for Sovereign Open Source OS Missing Components V74 PR Suite
// (`tests/test_open_source_os_missing_components_v74_pr.rs`)

#[path = "../src/distro/open_source_os_missing_components_v74_pr.rs"]
mod open_source_os_missing_components_v74_pr;

use open_source_os_missing_components_v74_pr::*;

#[test]
fn test_open_source_os_missing_components_v74_suite_integration() {
    let suite = SovereignOpenSourceOsMissingComponentsV74PrSuite::new();
    let proposals = suite.execute_full_v74_pr_pipeline();

    assert_eq!(proposals.len(), 11);
    assert_eq!(suite.calculate_subsystem_interop_score(), 100);

    for proposal in &proposals {
        assert_eq!(proposal.status, OsPrStatusV74::Validated);
        assert!(!proposal.pr_id.is_empty());
        assert!(!proposal.patch_diff.is_empty());
    }

    // Verify comparison matrix engine
    let matrix_report = suite.comparison_matrix.generate_matrix_report();
    assert!(matrix_report.contains("SigmaOS vs Open Source Operating Systems Feature Parity Matrix V74"));
    assert!(matrix_report.contains("Linux 6.29+"));
    assert!(matrix_report.contains("FreeBSD 16.3+"));
    assert!(matrix_report.contains("OpenBSD 8.9+"));

    // Verify sub-engines
    let freq = suite.linux_sched.predict_optimal_cpu_frequency(&[0.2, 0.4, 0.6]);
    assert_eq!(freq, 2000);

    assert!(suite.freebsd_casper.verify_jail_vnet_isolation(100));
    assert!(suite.openbsd_fineibt.validate_syscall_entry_point(0x0000_1234_5678_9000));

    let parity = suite.dragonfly_hammer2.compute_erasure_parity(&[&[1, 2, 3], &[4, 5, 6]]);
    assert_eq!(parity, vec![5, 7, 5]);

    assert!(suite.netbsd_rump.route_pci_hypercall(5));
    let query_res = suite.haiku_bfs.execute_live_query("TITLE");
    assert_eq!(query_res, vec!["META:TITLE"]);

    let mut plan9 = suite.plan9_9p;
    let ns = plan9.rfork_namespace(1);
    assert_eq!(ns, 257);

    assert!(suite.nix_guix.verify_hermetic_closure("sha256-a1b2c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0"));
    assert_eq!(suite.illumos_crossbow.probe_vnic_latency(10), 10);

    let mut omarchy = suite.omarchy_wallust;
    assert!(omarchy.reload_theme("Nord"));
    assert_eq!(omarchy.active_theme, "Nord");

    let wiki_doc = suite.tech_media.generate_wiki_documentation_page("V74 Harvester");
    assert!(wiki_doc.contains("Phoronix"));
    assert!(wiki_doc.contains("Ars Technica"));

    assert_eq!(suite.universal_adapter.supported_categories.len(), 44);
}
