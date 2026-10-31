// SPDX-License-Identifier: MIT
// SigmaOS - ArtCraft Advancements V85 PR Integration Test Suite
// (`tests/test_artcraft_advancements_v85_pr.rs`)

#[path = "../src/creative/artcraft_advancements_v85_pr.rs"]
mod artcraft_advancements_v85_pr;

use artcraft_advancements_v85_pr::{
    ArtcraftAdvancementsV85ParitySuite,
    ArtcraftComputeShaderBrushStampSimulationPrEngineV85,
    ArtcraftHighDynamicRangeHdr10GainmapCompositorPrEngineV85,
    ArtcraftLosslessVectorAssetExchangeFormatPrEngineV85,
    ArtcraftNonDestructiveLiveFilterAdjustmentGraphPrEngineV85,
    ArtcraftWideGamutOklabLutColorGradingPrEngineV85,
    LiveFilterNodeType,
    StylusInputSample,
    VectorCommand,
    SovereignArtcraftAdvancementsV85MasterSuite,
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
    // 1. Wide-Gamut Oklab/Oklch & 3D LUT Engine
    let wide_gamut = ArtcraftWideGamutOklabLutColorGradingPrEngineV85::new(17);
    let oklab = wide_gamut.linear_srgb_to_oklab([0.7, 0.3, 0.2]);
    assert!(oklab.l > 0.0);
    let oklch = wide_gamut.oklab_to_oklch(oklab);
    assert!(oklch.l > 0.0 && oklch.c >= 0.0);
    let lut_out = wide_gamut.sample_lut_trilinear([0.4, 0.4, 0.4]);
    assert_eq!(lut_out.len(), 3);

    // 2. HDR Gainmap Compositor Engine
    let hdr_engine = ArtcraftHighDynamicRangeHdr10GainmapCompositorPrEngineV85::new();
    let hdr_color = hdr_engine.reconstruct_hdr_pixel([0.6, 0.6, 0.6], 0.9, 4.0);
    assert!(hdr_color[0] >= 0.6);
    let eotf = hdr_engine.pq_eotf_curve(500.0);
    assert!(eotf > 0.0);

    // 3. Brush Stamp Simulation Engine
    let brush_sim = ArtcraftComputeShaderBrushStampSimulationPrEngineV85::new(40.0, 0.3, 0.1);
    let sample = StylusInputSample {
        x: 10.0,
        y: 20.0,
        pressure: 0.75,
        tilt_azimuth: 30.0,
        tilt_altitude: 45.0,
    };
    let radius = brush_sim.compute_stamp_radius(sample);
    assert!(radius > 0.0);
    let alpha = brush_sim.evaluate_stamp_alpha(radius * 0.2, radius);
    assert!(alpha > 0.0 && alpha <= 1.0);

    // 4. Live Filter Adjustment Graph Engine
    let mut filter_graph = ArtcraftNonDestructiveLiveFilterAdjustmentGraphPrEngineV85::new();
    filter_graph.add_filter_node(LiveFilterNodeType::LevelsAndCurves, 1.1, 1.0);
    filter_graph.add_filter_node(LiveFilterNodeType::VibranceSaturation, 1.2, 0.9);
    let out_ch = filter_graph.evaluate_channel_adjustment(0.5);
    assert!(out_ch > 0.0 && out_ch <= 1.0);

    // 5. Vector Asset Interchange Format Engine
    let vector_fmt = ArtcraftLosslessVectorAssetExchangeFormatPrEngineV85::new();
    let commands = vec![
        VectorCommand::MoveTo { x: 0.0, y: 0.0 },
        VectorCommand::LineTo { x: 50.0, y: 50.0 },
        VectorCommand::CubicCurveTo {
            cp1x: 60.0,
            cp1y: 60.0,
            cp2x: 80.0,
            cp2y: 70.0,
            end_x: 100.0,
            end_y: 100.0,
        },
        VectorCommand::Close,
    ];
    let bytes = vector_fmt.serialize_path_commands(&commands);
    let count = vector_fmt.validate_and_count_stream(&bytes).expect("Valid stream");
    assert_eq!(count, 4);
}
