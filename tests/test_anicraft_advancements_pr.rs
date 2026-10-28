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
    assert_eq!(manifests.len(), 9);

    let md = gateway.export_pr_manifest_markdown();
    assert!(md.contains("AniCraft 2D/3D Animation & Rigging PR Manifest"));

    let mut suite = AnicraftAdvancementsParitySuite::new();
    assert!(suite.verify_complete_parity());
}

#[test]
fn test_individual_anicraft_advancements_engines() {
    let bezier = AnicraftBezierEasingCurveOptimizationPrEngine::new(256);
    let h1 = BezierControlHandle2D { x: 0.42, y: 0.0 };
    let h2 = BezierControlHandle2D { x: 0.58, y: 1.0 };
    let eased = bezier.sample_optimized_easing(h1, h2, 0.5);
    assert!(eased > 0.0 && eased < 1.0);

    let ik = AnicraftSkeletalIkRiggingSolverPrEngine::new();
    let chain = ik.solve_fabrik_chain(&[100.0, 100.0], (150.0, 50.0));
    assert_eq!(chain.len(), 3);

    let atlas = AnicraftSpriteSheetAtlasExporterPrEngine::new(2048, 2048);
    let regions = atlas.pack_frame_grid(64, 64, 16);
    assert_eq!(regions.len(), 16);

    let mocap = AnicraftMocapBvhImporterPrEngine::new();
    let retargeted = mocap.retarget_bvh_channel_rotations(&[10.0, 20.0, 0.5, -0.2]);
    assert!(!retargeted.is_empty());
}
