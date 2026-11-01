// SPDX-License-Identifier: MIT
// Integration tests for PhotoCraft ArtCraft Missing Components V66 PR Suite

#[path = "../src/creative/photocraft_artcraft_missing_components_v66_pr.rs"]
mod photocraft_artcraft_missing_components_v66_pr;

use photocraft_artcraft_missing_components_v66_pr::*;

#[test]
fn test_integration_photocraft_v66_suite() {
    let suite = PhotocraftArtcraftMissingComponentsV66PrSuite::new();
    assert_eq!(suite.total_pr_proposals(), 10);

    let gateway = SovereignPhotocraftArtcraftMissingComponentsV66PrGateway::new();
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
fn test_integration_photocraft_v66_all_engines() {
    // 1. Float Demosaic
    let engine1 = PhotocraftRaw32BitFloatHdrDemosaicV66PrEngine::new();
    let raw = vec![32768u16; 16];
    let mut float_rgb = vec![0.0f32; 16 * 3];
    assert_eq!(engine1.demosaic_to_float_rgb(&raw, 4, 4, &mut float_rgb).unwrap(), 16);

    // 2. Generative Inpainting
    let engine2 = PhotocraftAiNeuralGenerativeInpaintingV66PrEngine::new();
    let mut rgba = vec![0u8; 16 * 4];
    let mut mask = vec![0u8; 16];
    mask[0] = 255;
    assert_eq!(engine2.inpaint_selection(&mut rgba, &mask, 4, 4).unwrap(), 1);

    // 3. CMYK Separation
    let engine3 = PhotocraftCmykSpotColorSeparationV66PrEngine::new();
    let rgb = vec![255, 128, 0];
    let mut c = vec![0u8; 1];
    let mut m = vec![0u8; 1];
    let mut y = vec![0u8; 1];
    let mut k = vec![0u8; 1];
    assert_eq!(engine3.convert_rgb_to_cmyk_planes(&rgb, &mut c, &mut m, &mut y, &mut k).unwrap(), 1);

    // 4. Perceptual Wheel
    let engine4 = PhotocraftLchOklabColorGradingWheelV66PrEngine::new();
    let mut rgb_grad = vec![120u8; 3];
    assert_eq!(engine4.grade_rgb_buffer(&mut rgb_grad, 1).unwrap(), 1);

    // 5. Focus Stacking
    let engine5 = PhotocraftMultiSliceDepthFocusStackingV66PrEngine::new();
    let s1 = vec![50u8; 3];
    let s2 = vec![150u8; 3];
    let mut out_stack = vec![0u8; 3];
    assert_eq!(engine5.stack_focus_slices(&[s1, s2], 1, 1, &mut out_stack).unwrap(), 1);
    assert_eq!(out_stack[0], 150);

    // 6. Vector Clipping
    let mut engine6 = PhotocraftSubpixelVectorPenPathClippingV66PrEngine::new();
    engine6.add_segment(BezierPathSegment {
        p0: SubpixelPoint2D { x: 0.0, y: 0.0 },
        c1: SubpixelPoint2D { x: 1.0, y: 1.0 },
        c2: SubpixelPoint2D { x: 2.0, y: 2.0 },
        p1: SubpixelPoint2D { x: 3.0, y: 3.0 },
    });
    let mut mask_out = vec![0u8; 4];
    assert_eq!(engine6.rasterize_vector_clipping_mask(&mut mask_out, 2, 2).unwrap(), 4);

    // 7. Smart Filter
    let mut engine7 = PhotocraftNonDestructiveSmartFilterPipelineV66PrEngine::new();
    engine7.append_filter(SmartFilterStep {
        filter: SmartFilterType::GaussianBlur { radius_px: 1.5 },
        opacity_percent: 100,
        blend_mode: "Normal".to_string(),
        enabled: true,
    });
    let mut px = vec![5u8; 4];
    assert_eq!(engine7.execute_filter_pipeline(&mut px, 2, 2).unwrap(), 1);

    // 8. Portrait Matting
    let engine8 = PhotocraftAiNeuralPortraitMattingHairRefinerV66PrEngine::new();
    let mut trimap = vec![100u8; 4];
    assert_eq!(engine8.refine_portrait_trimap(&mut trimap, 2, 2).unwrap(), 4);

    // 9. Panorama Stitcher
    let engine9 = PhotocraftMultiBandPanoramaEquirectangularStitcherV66PrEngine::new();
    assert_eq!(engine9.stitch_multi_frame_panorama(3).unwrap(), 3);

    // 10. Puppet Warp
    let mut engine10 = PhotocraftPerspectiveMeshPuppetWarpV66PrEngine::new();
    engine10.add_control_pin(PuppetControlPin {
        original_pos: (0.0, 0.0),
        deformed_pos: (1.0, 1.0),
        pin_depth_order: 0,
    });
    assert_eq!(engine10.solve_mesh_deformation(100).unwrap(), 100);
}
