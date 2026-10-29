// SPDX-License-Identifier: MIT
// Integration test for Sovereign ArtCraft Creative Suite Advancements V80 PR Proposal Suite

#[path = "../src/creative/artcraft_advancements_v80_pr.rs"]
mod artcraft_advancements_v80_pr;

use artcraft_advancements_v80_pr::{
    ArtcraftAdvancementsV80ParitySuite, ArtcraftGpuAcceleratedSubpixelVectorFilterPrEngineV80,
    ArtcraftMultiThreadedTileBasedTileBufferRenderPrEngineV80,
    ArtcraftProceduralTextureGraphGeneratorPrEngineV80,
    ArtcraftSpectralColorGradingIccProfilePrEngineV80,
    ArtcraftVectorPathNurbsSplineEvaluationPrEngineV80, NoiseType, NurbsControlPoint,
    SovereignArtcraftAdvancementsV80MasterSuite,
};

#[test]
fn test_artcraft_advancements_v80_integration_suite() {
    let mut master_suite = SovereignArtcraftAdvancementsV80MasterSuite::new();

    // Verify complete parity evaluation
    assert!(
        master_suite.evaluate_artcraft_advancements_v80_parity(),
        "ArtCraft V80 parity evaluation failed!"
    );

    // Verify parity score calculation
    let score = master_suite.compute_parity_score();
    assert_eq!(score, 100.0, "Parity score should be 100%");

    // Verify proposal collection
    let proposals = master_suite.collect_all_proposals();
    assert_eq!(proposals.len(), 5, "Expected 5 PR proposal manifests");

    let expected_pr_ids = [8001, 8002, 8003, 8004, 8005];
    for (i, pr) in proposals.iter().enumerate() {
        assert_eq!(pr.pr_id, expected_pr_ids[i]);
        assert_eq!(pr.status, "Ready for Merge");
        assert!(!pr.pr_title.is_empty());
        assert!(!pr.components_added.is_empty());
    }

    // Verify markdown export
    let markdown = master_suite.export_all_pr_manifests_markdown();
    assert!(markdown.contains("# ArtCraft Creative Suite Advancements V80 PR Manifest"));
    assert!(markdown.contains("PR #8001"));
    assert!(markdown.contains("PR #8005"));

    // Verify ParitySuite wrapper
    let mut parity_suite = ArtcraftAdvancementsV80ParitySuite::new();
    assert!(
        parity_suite.verify_complete_parity(),
        "ParitySuite wrapper verification failed!"
    );
}

#[test]
fn test_individual_v80_engines_integration() {
    // 1. NURBS Engine
    let mut nurbs = ArtcraftVectorPathNurbsSplineEvaluationPrEngineV80::new(3);
    nurbs.setup_curve(vec![
        NurbsControlPoint { x: 0.0, y: 0.0, weight: 1.0 },
        NurbsControlPoint { x: 50.0, y: 100.0, weight: 1.0 },
        NurbsControlPoint { x: 100.0, y: 100.0, weight: 1.0 },
        NurbsControlPoint { x: 150.0, y: 0.0, weight: 1.0 },
    ]);
    let (pt_x, pt_y) = nurbs.evaluate_point(0.5);
    assert!(pt_x > 0.0 && pt_y > 0.0);

    // 2. Tile Render Engine
    let tile_render = ArtcraftMultiThreadedTileBasedTileBufferRenderPrEngineV80::new(512, 512, 64);
    assert_eq!(tile_render.total_tiles_count(), 64);
    let buf = tile_render.render_tile_buffer(0).expect("Tile 0 failed to render");
    assert_eq!(buf.len(), 64 * 64 * 4);

    // 3. Spectral CMS Engine
    let spectral_cms = ArtcraftSpectralColorGradingIccProfilePrEngineV80::new("P3-Spectral-v4");
    let dist = spectral_cms.rgb_to_spectral([0.9, 0.2, 0.1]);
    let rgb = spectral_cms.spectral_to_rgb(dist);
    assert!(rgb[0] > 0.0);

    // 4. Procedural Texture Engine
    let proc_tex = ArtcraftProceduralTextureGraphGeneratorPrEngineV80::new(999, 5, 0.6);
    let sample_p = proc_tex.sample_noise(10.0, 20.0, NoiseType::Perlin);
    let sample_w = proc_tex.sample_noise(10.0, 20.0, NoiseType::Worley);
    assert!((0.0..=1.0).contains(&sample_p));
    assert!((0.0..=1.0).contains(&sample_w));

    // 5. Subpixel Filter Engine
    let subpixel = ArtcraftGpuAcceleratedSubpixelVectorFilterPrEngineV80::new();
    let row_in = vec![128u8; 64];
    let row_out = subpixel.apply_subpixel_filter_row(&row_in);
    assert_eq!(row_out.len(), 64);
}
