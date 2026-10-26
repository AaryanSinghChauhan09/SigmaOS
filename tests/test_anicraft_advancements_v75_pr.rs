// SPDX-License-Identifier: MIT
// Integration test for AniCraft V75 Animation Advancements PR Suite

#[path = "../src/creative/mod.rs"]
mod creative;

use creative::anicraft_advancements_v75_pr::*;

#[test]
fn test_anicraft_v75_master_suite_integration() {
    let mut master = SovereignAnicraftAdvancementsV75MasterSuite::new();
    assert!(master.evaluate_v75_anicraft_parity());

    let manifests = master.collect_all_pr_manifests();
    assert_eq!(manifests.len(), 5);

    let markdown = master.export_pr_manifest_markdown();
    assert!(markdown.contains("AniCraft 2D/3D Animation & Rigging V75 PR Manifest"));
    assert!(markdown.contains("Subpixel Vector Contour Scanline Rasterizer"));
    assert!(markdown.contains("Dual Quaternion Skinning"));

    let mut suite = AnicraftAdvancementsV75ParitySuite::new();
    assert!(suite.verify_complete_v75_parity());
}
