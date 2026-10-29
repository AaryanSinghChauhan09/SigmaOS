// SPDX-License-Identifier: MIT
// SigmaOS - AniCraft V75 Animation Advancements Integration Test
// (`tests/test_anicraft_advancements_v75_pr.rs`)

#[path = "../src/creative/anicraft_advancements_v75_pr.rs"]
mod anicraft_advancements_v75_pr;

use anicraft_advancements_v75_pr::*;

#[test]
fn test_anicraft_v75_master_suite_integration() {
    let mut master_suite = SovereignAnicraftAdvancementsV75MasterSuite::new();
    assert!(
        master_suite.evaluate_v75_anicraft_parity(),
        "AniCraft V75 animation parity evaluation should succeed"
    );

    let manifests = master_suite.collect_all_pr_manifests();
    assert_eq!(manifests.len(), 5, "Expected 5 PR manifests in AniCraft V75 suite");

    let pr_titles = [
        "feat(anicraft): Implement Subpixel Vector Contour Scanline Rasterizer & Anti-Aliased Edge Engine V75",
        "feat(anicraft): Implement Dual Quaternion Skinning (DQS) Multi-Bone Volume-Preserving Mesh Engine V75",
        "feat(anicraft): Implement Neural Phoneme-to-Viseme Alignment & Formantic Mouth Geometry Engine V75",
        "feat(anicraft): Implement NLA Track Crossfade Curve & Smoothstep Blend Weighting Engine V75",
        "feat(anicraft): Implement Multi-Layer Onion Skinning Z-Depth Compositing & Directional Tint Engine V75",
    ];

    for (manifest, expected_title) in manifests.iter().zip(pr_titles.iter()) {
        assert_eq!(&manifest.pr_title, expected_title);
        assert_eq!(manifest.status, "Ready for Merge");
    }

    let markdown = master_suite.export_pr_manifest_markdown();
    assert!(markdown.contains("# AniCraft 2D/3D Animation & Rigging V75 PR Manifest"));
    assert!(markdown.contains("PR ID"));
    assert!(markdown.contains("#8751"));

    let mut parity_suite = AnicraftAdvancementsV75ParitySuite::new();
    assert!(
        parity_suite.verify_complete_v75_parity(),
        "AniCraft V75 parity suite should verify complete parity"
    );

    let digest = fnv1a_anicraft_v75_pr_digest(8751, "AniCraft Vector Scanline V75");
    assert_ne!(digest, 0, "FNV-1a digest should produce non-zero hash");
}

#[test]
fn test_anicraft_v75_individual_engines() {
    let rasterizer = AnicraftSubpixelVectorContourRasterizerPrEngineV75::new(4);
    let edge = ContourEdge2DV75 {
        x0: 0.0,
        y0: 0.0,
        x1: 10.0,
        y1: 10.0,
    };
    let coverage = rasterizer.compute_scanline_coverage(&edge, 5.0);
    assert!(coverage > 0.0 && coverage <= 1.0);

    let dqs_engine = AnicraftMultiBoneDqsSkinningPrEngineV75::new();
    let dq = DualQuaternionV75::IDENTITY;
    let blended = dqs_engine.blend_dual_quaternions(&[dq], &[1.0]);
    assert!(blended.real[0] > 0.9);

    let aligner = AnicraftPhonemeVisemeAlignmentNeuralPrEngineV75::new();
    let (h, w) = aligner.predict_mouth_geometry(150);
    assert!(h > 0.0 && w > 0.0);

    let nla_engine = AnicraftNlaTrackCrossfadeCurvePrEngineV75::new(30);
    let (wa, wb) = nla_engine.compute_crossfade_weights(15, 0);
    assert!((wa - 0.5).abs() < 1e-2 && (wb - 0.5).abs() < 1e-2);

    let onion_engine = AnicraftMultiLayerOnionSkinningDepthPrEngineV75::new();
    let layers = onion_engine.compute_depth_sorted_layers();
    assert_eq!(layers.len(), 4);
    assert!(layers[0].z_depth < layers[3].z_depth);
}
