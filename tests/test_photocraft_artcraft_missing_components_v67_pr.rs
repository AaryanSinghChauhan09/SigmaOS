// SPDX-License-Identifier: MIT
// Integration tests for PhotoCraft ArtCraft Missing Components V67 PR Suite

#[path = "../src/creative/photocraft_artcraft_missing_components_v67_pr.rs"]
mod photocraft_artcraft_missing_components_v67_pr;

use photocraft_artcraft_missing_components_v67_pr::*;

#[test]
fn test_integration_photocraft_v67_suite() {
    let suite = PhotocraftArtcraftMissingComponentsV67PrSuite::new();
    assert_eq!(suite.total_pr_proposals(), 10);

    let gateway = SovereignPhotocraftArtcraftMissingComponentsV67PrGateway::new();
    let manifests = gateway.collect_all_pr_manifests();
    assert_eq!(manifests.len(), 10);

    for m in &manifests {
        assert_eq!(m.status, "READY_FOR_MERGE");
        assert!(!m.components_added.is_empty());
        assert!(!m.pr_title.is_empty());
        assert!(!m.performance_multiplier.is_empty());
    }
}

#[test]
fn test_integration_photocraft_v67_all_engines() {
    // 1. Super Resolution
    let engine1 = PhotocraftAiNeuralSuperResolutionUpscalerV67PrEngine::new();
    let in_rgb = vec![120u8; 4 * 3]; // 2x2 image
    let mut out_rgb = vec![0u8; 8 * 8 * 3]; // 8x8 image
    assert_eq!(engine1.upscale_rgb(&in_rgb, 2, 2, &mut out_rgb).unwrap(), 64);
    assert_eq!(out_rgb[0], 120);

    // 2. Frequency Separation
    let engine2 = PhotocraftSubpixelWaveletTextureFrequencySeparationV67PrEngine::new();
    let input = vec![180u8; 8];
    let mut low = vec![0u8; 8];
    let mut high = vec![0u8; 8];
    assert_eq!(engine2.separate_frequency_layers(&input, &mut low, &mut high).unwrap(), 8);

    // 3. ACEScc LUT
    let engine3 = PhotocraftAcesCcColorGradingOcioLutV67PrEngine::new();
    let mut rgb_lut = vec![150u8; 3];
    assert_eq!(engine3.apply_grading_lut(&mut rgb_lut, 1).unwrap(), 1);

    // 4. Depth Relighting
    let engine4 = PhotocraftDepthMapRelightingVolumetricNormalV67PrEngine::new();
    let mut rgb_relight = vec![100u8; 4 * 3];
    let depth = vec![0.3f32; 4];
    assert_eq!(engine4.compute_relighting(&mut rgb_relight, &depth, 2, 2).unwrap(), 4);

    // 5. HDR Fusion
    let engine5 = PhotocraftMultiExposureHdrToneMappingFusionV67PrEngine::new();
    let br1 = vec![60u8; 3];
    let br2 = vec![180u8; 3];
    let mut fused = vec![0u8; 3];
    assert_eq!(engine5.fuse_exposure_brackets(&[br1, br2], 1, &mut fused).unwrap(), 1);

    // 6. Smart Filter DAG
    let mut engine6 = PhotocraftNonDestructiveSmartFilterDagPipelineV67PrEngine::new();
    engine6.add_filter_node(SmartFilterDagNode {
        node_id: 1,
        filter_name: "GaussianSharpen".to_string(),
        intensity_factor: 1.1,
        enabled: true,
    });
    let mut dag_buf = vec![50u8; 4];
    assert_eq!(engine6.evaluate_filter_graph(&mut dag_buf, 4).unwrap(), 1);

    // 7. Vector Pen Clipping Mask
    let mut engine7 = PhotocraftSubpixelVectorPenClippingMaskV67PrEngine::new();
    engine7.add_spline(CubicBezierSpline {
        start: ContinuousPoint2D { x: 0.0, y: 0.0 },
        ctrl1: ContinuousPoint2D { x: 0.5, y: 0.5 },
        ctrl2: ContinuousPoint2D { x: 1.5, y: 1.5 },
        end: ContinuousPoint2D { x: 2.0, y: 2.0 },
    });
    let mut clip_mask = vec![0u8; 9];
    assert_eq!(engine7.rasterize_clipping_mask(&mut clip_mask, 3, 3).unwrap(), 9);

    // 8. Alpha Matting
    let engine8 = PhotocraftNeuralSemanticInstanceAlphaMattingV67PrEngine::new();
    let mut trimap = vec![128u8; 4];
    assert_eq!(engine8.refine_alpha_matte(&mut trimap, 2, 2).unwrap(), 4);
    assert_eq!(trimap[0], 215);

    // 9. Spherical Stitcher
    let engine9 = PhotocraftGigapixelPanoramaSphericalStitcherV67PrEngine::new();
    assert_eq!(engine9.stitch_tiles(5, (4096, 2048)).unwrap(), 5);

    // 10. Puppet Warp Deformation
    let mut engine10 = PhotocraftSpatialMeshPuppetWarpDeformationV67PrEngine::new();
    engine10.add_pin(PuppetWarpControlPin {
        pin_id: 1,
        source_coord: (5.0, 5.0),
        target_coord: (10.0, 12.0),
        influence_radius: 20.0,
    });
    assert_eq!(engine10.solve_arap_deformation(256).unwrap(), 256);
}
