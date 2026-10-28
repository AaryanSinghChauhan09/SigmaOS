// SPDX-License-Identifier: MIT
// Test Suite for Sovereign AniCraft Advancements PR Gateway & Integration

#[path = "../src/creative/mod.rs"]
mod creative;

use creative::anicraft_advancements_pr::*;

#[test]
fn test_anicraft_advancements_pr_integration() {
    let mut gateway = SovereignAnicraftAdvancementsMasterSuite::new();
    assert!(gateway.evaluate_anicraft_parity());

    let manifests = gateway.collect_all_pr_manifests();
    assert_eq!(manifests.len(), 5);

    let md = gateway.export_pr_manifest_markdown();
    assert!(md.contains("AniCraft 2D/3D Animation & Rigging PR Manifest"));

    let mut suite = AnicraftAdvancementsParitySuite::new();
    assert!(suite.verify_complete_parity());
}
