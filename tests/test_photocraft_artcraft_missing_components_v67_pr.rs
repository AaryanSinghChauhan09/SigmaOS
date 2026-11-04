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
    // 1. Neural Super-Resolution Upscaler
    let engine1 = PhotocraftAiNeuralSuperResolutionUpscalerV67PrEngine::new();
    let src = vec![128u8; 2 * 2 * 3];
    let mut dst = vec![0u8; 8 * 8 * 3];
    assert_eq!(engine1.upscale_rgb(&src, 2, 2, &mut dst).unwrap(), 64);
    assert_eq!(dst[0], 128);

    // 2. Wavelet Frequency Separation
    let engine2 = PhotocraftSubpixelWaveletTextureFrequencySeparationV67PrEngine::new();
    let src_img = vec![200u8; 4 * 4 * 3];
    let mut low = vec![0u8; 4 * 4 * 3];
    let mut high = vec![0u8; 4 * 4 * 3];
    assert_eq!(engine2.separate_frequencies(&src_img, &mut low, &mut high, 4, 4).unwrap(), 16);
    assert_eq!(low[0], 200);
    assert_eq!(high[0], 128);

    // 3. ACEScc Color Grading
    let engine3 = PhotocraftAcesCcColorGradingOcioLutV67PrEngine::new();
    let mut rgb_grad = vec![100u8; 6];
    assert_eq!(engine3.grade_acescc(&mut rgb_grad, 2).unwrap(), 2);

    // 4. Depth-Map Relighting
    let engine4 = PhotocraftDepthMapRelightingVolumetricNormalV67PrEngine::new();
    let mut rgb_relight = vec![40u8; 3];
    let depth_map = vec![128u8; 1];
    assert_eq!(engine4.relight_with_depth_map(&mut rgb_relight, &depth_map, 1, 1).unwrap(), 1);

    // 5. Multi-Exposure HDR Fusion
    let engine5 = PhotocraftMultiExposureHdrToneMappingFusionV67PrEngine::new();
    let b1 = vec![60u8; 6];
    let b2 = vec![140u8; 6];
    let mut out_fused = vec![0u8; 6];
    assert_eq!(engine5.fuse_exposures(&[b1, b2], 2, 1, &mut out_fused).unwrap(), 2);
    assert_eq!(out_fused[0], 100);

    // 6. Non-Destructive Smart Filter DAG Pipeline
    let mut engine6 = PhotocraftNonDestructiveSmartFilterDagPipelineV67PrEngine::new();
    engine6.add_node(DagNodeEntryV67 {
        node_id: 1,
        filter: SmartFilterDagNodeV67::UnsharpMask { radius: 1.0, amount: 1.5 },
        enabled: true,
    });
    let mut dag_buf = vec![50u8; 3];
    assert_eq!(engine6.execute_dag_pipeline(&mut dag_buf, 1, 1).unwrap(), 1);

    // 7. Subpixel Vector Pen Clipping Mask
    let mut engine7 = PhotocraftSubpixelVectorPenClippingMaskV67PrEngine::new();
    engine7.add_segment(BezierSegmentV67 {
        start: SubpixelPoint2DV67 { x: 0.0, y: 0.0 },
        control1: SubpixelPoint2DV67 { x: 1.0, y: 1.0 },
        control2: SubpixelPoint2DV67 { x: 2.0, y: 2.0 },
        end: SubpixelPoint2DV67 { x: 3.0, y: 3.0 },
    });
    let mut mask_out = vec![0u8; 4];
    assert_eq!(engine7.rasterize_mask(&mut mask_out, 2, 2).unwrap(), 4);
    assert_eq!(mask_out[0], 255);

    // 8. Neural Semantic Instance Alpha Matting
    let engine8 = PhotocraftNeuralSemanticInstanceAlphaMattingV67PrEngine::new();
    let mut trimap = vec![128u8; 4];
    assert_eq!(engine8.extract_matte(&mut trimap, 2, 2).unwrap(), 4);
    assert_eq!(trimap[0], 220);

    // 9. Gigapixel Panorama Spherical Stitcher
    let engine9 = PhotocraftGigapixelPanoramaSphericalStitcherV67PrEngine::new();
    assert_eq!(engine9.stitch_frames(4).unwrap(), 4);

    // 10. Spatial Mesh Puppet Warp Deformation
    let mut engine10 = PhotocraftSpatialMeshPuppetWarpDeformationV67PrEngine::new();
    engine10.add_control_pin(PuppetControlPinV67 {
        origin_xy: (10.0, 10.0),
        target_xy: (12.0, 15.0),
        depth_index: 0,
    });
    assert_eq!(engine10.solve_deformation(250).unwrap(), 250);
}
