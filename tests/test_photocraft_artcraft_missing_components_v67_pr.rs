// SPDX-License-Identifier: MIT
// Integration tests for PhotoCraft ArtCraft Missing Components V67 PR Suite

#[path = "../src/creative/photocraft_artcraft_missing_components_v67_pr.rs"]
mod photocraft_artcraft_missing_components_v67_pr;

use photocraft_artcraft_missing_components_v67_pr::*;

#[test]
fn test_integration_photocraft_v67_suite() {
    let suite = PhotocraftArtcraftMissingComponentsV67PrSuite::new();
    assert_eq!(suite.total_pr_proposals(), 10);
    assert!(suite.verify_complete_v67_parity());

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
    // 1. Super-Resolution Upscaler
    let engine1 = PhotocraftAiNeuralSuperResolutionUpscalerV67PrEngine::new();
    let src = vec![80u8; 2 * 2 * 3];
    let mut dst = vec![0u8; 4 * 4 * 3];
    assert_eq!(engine1.upscale_rgb(&src, 2, 2, &mut dst).unwrap(), 16);
    assert_eq!(dst[0], 80);

    // 2. Frequency Separation
    let engine2 = PhotocraftSubpixelWaveletTextureFrequencySeparationV67PrEngine::new();
    let input = vec![120u8; 9];
    let mut low = vec![0u8; 9];
    let mut high = vec![0u8; 9];
    assert_eq!(engine2.separate_frequencies(&input, &mut low, &mut high).unwrap(), 9);

    // 3. ACEScc / ACEScg 3D LUT
    let engine3 = PhotocraftAcesCcColorGradingOcioLutV67PrEngine::new();
    let mut buf = vec![200u8; 3];
    assert_eq!(engine3.transform_colors(&mut buf).unwrap(), 1);

    // 4. Depth Map Relighting & Virtual Gobo
    let mut engine4 = PhotocraftDepthMapRelightingVirtualGoboV67PrEngine::new();
    engine4.add_light(VirtualLightSourceV67 {
        position_xyz: (0.1, 0.1, 0.9),
        color_rgb: (0.9, 0.9, 1.0),
        intensity: 1.1,
        falloff_radius: 2.0,
    });
    let mut rgb = vec![110u8; 4 * 3];
    let depth = vec![40u8; 4];
    assert_eq!(engine4.relight_scene(&mut rgb, &depth, 2, 2).unwrap(), 4);

    // 5. HDR Bracket Fusion & Tone Mapper
    let engine5 = PhotocraftHdrExposureBracketFusionToneMapperV67PrEngine::new();
    let e1 = vec![60u8; 3];
    let e2 = vec![140u8; 3];
    let mut out_rgb = vec![0u8; 3];
    assert_eq!(engine5.fuse_and_tonemap(&[&e1, &e2], 1, &mut out_rgb).unwrap(), 1);

    // 6. Non-Destructive DAG Smart Object Pipeline
    let mut engine6 = PhotocraftAiSmartObjectNonDestructiveDagPipelineV67PrEngine::new();
    engine6.add_filter_node(SmartDagFilterNodeV67 {
        node_id: 101,
        filter_name: "Contrast".to_string(),
        intensity: 1.2,
        is_enabled: true,
    });
    let mut test_buf = vec![100u8; 4];
    assert_eq!(engine6.evaluate_graph(&mut test_buf).unwrap(), 1);

    // 7. Sub-Pixel Precision Vector Mask & SVG Engine
    let mut engine7 = PhotocraftSubpixelPrecisionVectorMaskSvgEngineV67::new();
    engine7.push_node(VectorNode2DV67 { x: 5.0, y: 5.0 });
    let mut mask = vec![0u8; 4];
    assert_eq!(engine7.rasterize_vector_mask(&mut mask, 2, 2).unwrap(), 4);
    assert_eq!(mask[0], 255);

    // 8. Neural Object Segmentation Matting
    let engine8 = PhotocraftNeuralObjectSegmentationMattingV67PrEngine::new();
    let mut trimap = vec![127u8; 4];
    assert_eq!(engine8.refine_alpha_matting(&mut trimap, 2, 2).unwrap(), 4);
    assert_eq!(trimap[0], 220);

    // 9. Multi-Row Gigapixel Panorama Stitcher
    let engine9 = PhotocraftMultiRowSphericalGigaPixelStitcherV67PrEngine::new();
    assert_eq!(engine9.stitch_frames(5).unwrap(), 5);

    // 10. Spatial Perspective Defish & Puppet Mesh Deformer
    let mut engine10 = PhotocraftSpatialPerspectiveAnamorphicDefishMeshV67PrEngine::new();
    engine10.add_pin(PuppetMeshPinV67 {
        pin_id: 1,
        base_coord: (1.0, 1.0),
        target_coord: (2.0, 2.0),
    });
    assert_eq!(engine10.deform_mesh_vertices(150).unwrap(), 150);
}
