// SPDX-License-Identifier: MIT
// Integration test for Sovereign ArtCraft Creative Suite Advancements V85 PR Proposal Suite

#[path = "../src/creative/artcraft_advancements_v85_pr.rs"]
mod artcraft_advancements_v85_pr;

use artcraft_advancements_v85_pr::{
    ArtcraftAdvancementsV85ParitySuite, ArtcraftComputeShaderBrushStampSimulationPrEngineV85,
    ArtcraftHighDynamicRangeHdr10GainmapCompositorPrEngineV85,
    ArtcraftLosslessVectorAssetExchangeFormatPrEngineV85,
    ArtcraftNonDestructiveLiveFilterAdjustmentGraphPrEngineV85,
    ArtcraftWideGamutOklabLutColorGradingPrEngineV85, BrushDabPoint, LinearRgb, LiveFilterKind,
    SovereignArtcraftAdvancementsV85MasterSuite, VectorRecordV85,
};

#[test]
fn test_artcraft_advancements_v85_integration_suite() {
    let mut master_suite = SovereignArtcraftAdvancementsV85MasterSuite::new();

    // Verify complete parity evaluation
    assert!(
        master_suite.evaluate_artcraft_advancements_v85_parity(),
        "ArtCraft V85 parity evaluation failed!"
    );

    // Verify parity score calculation
    let score = master_suite.compute_parity_score();
    assert_eq!(score, 100.0, "Parity score should be 100%");

    // Verify proposal collection
    let proposals = master_suite.collect_all_proposals();
    assert_eq!(proposals.len(), 5, "Expected 5 PR proposal manifests");

    let expected_pr_ids = [8501, 8502, 8503, 8504, 8505];
    for (i, pr) in proposals.iter().enumerate() {
        assert_eq!(pr.pr_id, expected_pr_ids[i]);
        assert_eq!(pr.status, "Ready for Merge");
        assert!(!pr.pr_title.is_empty());
        assert!(!pr.components_added.is_empty());
    }

    // Verify markdown export
    let markdown = master_suite.export_all_pr_manifests_markdown();
    assert!(markdown.contains("# ArtCraft Creative Suite Advancements V85 PR Manifest"));
    assert!(markdown.contains("PR #8501"));
    assert!(markdown.contains("PR #8505"));

    // Verify ParitySuite wrapper
    let mut parity_suite = ArtcraftAdvancementsV85ParitySuite::new();
    assert!(
        parity_suite.verify_complete_parity(),
        "ParitySuite wrapper verification failed!"
    );
}

#[test]
fn test_individual_v85_engines_integration() {
    // 1. Oklab 3D LUT Color Grading
    let color_engine = ArtcraftWideGamutOklabLutColorGradingPrEngineV85::new(17, 0.2);
    let srgb = LinearRgb { r: 0.7, g: 0.4, b: 0.2 };
    let oklab = color_engine.srgb_to_oklab(srgb);
    let recon = color_engine.oklab_to_srgb(oklab);
    assert!((srgb.r - recon.r).abs() < 0.05);

    // 2. Ultra HDR Gainmap
    let hdr_engine = ArtcraftHighDynamicRangeHdr10GainmapCompositorPrEngineV85::new(1200.0);
    let sdr = LinearRgb { r: 0.4, g: 0.4, b: 0.4 };
    let boosted = hdr_engine.blend_gainmap_pixel(sdr, 0.9);
    assert!(boosted.r >= sdr.r);

    // 3. Compute Shader Dynamic Dab Stamping
    let brush_engine = ArtcraftComputeShaderBrushStampSimulationPrEngineV85::new(0.25, 0.15);
    let dab = BrushDabPoint {
        x: 16.0,
        y: 16.0,
        pressure: 0.85,
        tilt_angle: 0.1,
        radius: 10.0,
        opacity: 0.9,
    };
    let mut tile_buf = vec![0u8; 32 * 32 * 4];
    let modified = brush_engine.stamp_dab_to_tile(dab, (0, 0), 32, &mut tile_buf);
    assert!(modified > 0);

    // 4. DAG Live Filter Graph
    let mut filter_engine = ArtcraftNonDestructiveLiveFilterAdjustmentGraphPrEngineV85::new();
    filter_engine.add_filter(1, LiveFilterKind::GaussianBlur { radius: 3.0 });
    filter_engine.add_filter(2, LiveFilterKind::ChromaticAberration { shift_px: 2.0 });
    let (hash1, recomputed1) = filter_engine.evaluate_graph_pipeline(999);
    assert_eq!(recomputed1, 2);
    let (hash2, recomputed2) = filter_engine.evaluate_graph_pipeline(999);
    assert_eq!(recomputed2, 0);
    assert_eq!(hash1, hash2);

    filter_engine.mark_node_dirty(1);
    let (_hash3, recomputed3) = filter_engine.evaluate_graph_pipeline(999);
    assert_eq!(recomputed3, 1);

    // 5. Binary Vector Asset Stream
    let mut vector_engine = ArtcraftLosslessVectorAssetExchangeFormatPrEngineV85::new();
    vector_engine.append_record(VectorRecordV85::PathBezier {
        points_count: 8,
        closed: false,
    });
    vector_engine.append_record(VectorRecordV85::TextSpan {
        glyphs_count: 15,
        font_size_pt: 12.0,
    });
    let stream = vector_engine.serialize_to_binary_stream();
    assert!(stream.len() >= 12);
    let count = vector_engine.validate_and_count_stream(&stream).unwrap();
    assert_eq!(count, 2);
}
