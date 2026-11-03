// SPDX-License-Identifier: MIT
// SigmaOS - ArtCraft Advancements V86 PR Integration Test Suite
// (`tests/test_artcraft_advancements_v86_pr.rs`)

#[path = "../src/creative/artcraft_advancements_v86_pr.rs"]
mod artcraft_advancements_v86_pr;

use artcraft_advancements_v86_pr::{
    ArtcraftAdvancementsV86ParitySuite, ArtcraftGpuBezierClothCurvaturePrEngineV86,
    ArtcraftMultiLayerDagCompositorCachePrEngineV86,
    ArtcraftNeuralDiffusionInpaintingPrEngineV86,
    ArtcraftSpectralCmykHalftoneScreeningPrEngineV86,
    ArtcraftUniversalOpenRasterExchangePrEngineV86, OraBlendMode, RectTile,
    SovereignArtcraftAdvancementsV86MasterSuite, Vector3D,
};

#[test]
fn test_artcraft_advancements_v86_integration_suite() {
    let mut master_suite = SovereignArtcraftAdvancementsV86MasterSuite::new();

    // Verify complete parity evaluation
    assert!(
        master_suite.evaluate_artcraft_advancements_v86_parity(),
        "ArtCraft V86 parity evaluation failed!"
    );

    // Verify parity score calculation
    let score = master_suite.compute_parity_score();
    assert_eq!(score, 100.0, "Parity score should be 100%");

    // Verify proposal collection
    let proposals = master_suite.collect_all_proposals();
    assert_eq!(proposals.len(), 5, "Expected 5 PR proposal manifests");

    let expected_pr_ids = [8601, 8602, 8603, 8604, 8605];
    for (i, pr) in proposals.iter().enumerate() {
        assert_eq!(pr.pr_id, expected_pr_ids[i]);
        assert_eq!(pr.status, "Ready for Merge");
        assert!(!pr.pr_title.is_empty());
        assert!(!pr.components_added.is_empty());
    }

    // Verify markdown export
    let markdown = master_suite.export_all_pr_manifests_markdown();
    assert!(markdown.contains("# ArtCraft Creative Suite Advancements V86 PR Manifest"));
    assert!(markdown.contains("PR #8601"));
    assert!(markdown.contains("PR #8605"));

    // Verify ParitySuite wrapper
    let mut parity_suite = ArtcraftAdvancementsV86ParitySuite::new();
    assert!(
        parity_suite.verify_complete_parity(),
        "ParitySuite wrapper verification failed!"
    );
}

#[test]
fn test_individual_v86_engines_integration() {
    // 1. Neural Diffusion Inpainting Engine
    let inpaint_engine = ArtcraftNeuralDiffusionInpaintingPrEngineV86::new(64, 15);
    let ref_spec = [1.0, 0.8, 0.6, 0.4];
    let synth_spec = [0.98, 0.79, 0.61, 0.39];
    let freq_loss = inpaint_engine.compute_perceptual_frequency_loss(&ref_spec, &synth_spec);
    assert!(freq_loss >= 0.0);
    let mut latents = [0.1, 0.2, 0.3, 0.4];
    let mask = [true, false, true, false];
    let mag = inpaint_engine.diffuse_patch_infill(&mut latents, &mask, 5);
    assert!(mag > 0.0);

    // 2. GPU Bézier Cloth Curvature Engine
    let cloth_engine = ArtcraftGpuBezierClothCurvaturePrEngineV86::new(6, 1.5);
    let mut cps = [Vector3D::new(0.0, 0.0, 0.0); 16];
    for row in 0..4 {
        for col in 0..4 {
            cps[row * 4 + col] = Vector3D::new(col as f32, 0.2 * ((row + col) as f32), row as f32);
        }
    }
    let norm = cloth_engine.compute_surface_normal(&cps, 0.5, 0.5);
    assert!(norm.length() > 0.8);
    let strain = cloth_engine.compute_cloth_strain_energy(&cps);
    assert!(strain >= 0.0);

    // 3. Spectral CMYK Halftone Screening Engine
    let halftone_engine = ArtcraftSpectralCmykHalftoneScreeningPrEngineV86::new(150.0);
    let cmyk = halftone_engine.rgb_to_cmyk_spectral([0.1, 0.5, 0.9]);
    assert!(cmyk.c >= 0.0 && cmyk.c <= 1.0);
    assert!(cmyk.m >= 0.0 && cmyk.m <= 1.0);
    let dot = halftone_engine.evaluate_halftone_dot(5.0, 5.0, 0, cmyk.c);
    let _ = dot; // Valid boolean output
    let mut dithered = [0u8; 3];
    halftone_engine.error_diffuse_scanline(&[0.85, 0.15, 0.70], &mut dithered);
    assert_eq!(dithered[0], 255);

    // 4. DAG Multi-Layer Compositor Cache Engine
    let mut cache_engine = ArtcraftMultiLayerDagCompositorCachePrEngineV86::new(32);
    cache_engine.register_layer_node(201);
    cache_engine.register_layer_node(202);
    cache_engine.invalidate_node_region(201, RectTile::new(10, 10, 50, 50));
    assert_eq!(cache_engine.count_valid_cached_nodes(), 0);
    cache_engine.commit_node_cache(201, 0xFEEDBEEF);
    assert_eq!(cache_engine.count_valid_cached_nodes(), 1);

    // 5. Universal OpenRaster Exchange Engine
    let mut ora_engine = ArtcraftUniversalOpenRasterExchangePrEngineV86::new(1920, 1080);
    ora_engine.add_layer("Artwork Layer", 0.95, OraBlendMode::Multiply, true);
    let xml = ora_engine.generate_stack_xml();
    assert!(xml.contains("Artwork Layer"));
    assert!(xml.contains("svg:multiply"));
    assert!(ora_engine.validate_mimetype_stream(b"image/openraster"));
}
