// SPDX-License-Identifier: MIT
// Integration tests for PhotoCraft ArtCraft Missing Components V68 PR Suite

#[path = "../src/creative/photocraft_artcraft_missing_components_v68_pr.rs"]
mod photocraft_artcraft_missing_components_v68_pr;

use photocraft_artcraft_missing_components_v68_pr::*;

#[test]
fn test_integration_photocraft_v68_suite() {
    let suite = PhotocraftArtcraftMissingComponentsV68PrSuite::new();
    assert_eq!(suite.total_pr_proposals(), 10);

    let gateway = SovereignPhotocraftArtcraftMissingComponentsV68PrGateway::new();
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
fn test_integration_photocraft_v68_all_engines() {
    // 1. RAW 32-Bit Floating Point Demosaic Engine
    let engine1 = PhotocraftRaw32BitFloatingPointDemosaicV68PrEngine::new();
    let bayer = vec![0.8f32; 2 * 2];
    let mut dst_rgb = vec![0.0f32; 2 * 2 * 3];
    assert_eq!(engine1.demosaic_float(&bayer, 2, 2, &mut dst_rgb).unwrap(), 4);
    assert!(dst_rgb[0] > 0.0);

    // 2. AI Diffusion Generative Inpainting Engine
    let engine2 = PhotocraftAiDiffusionGenerativeInpaintingV68PrEngine::new();
    let mut rgb = vec![0u8; 12];
    let mask = vec![1u8, 0u8, 1u8, 0u8];
    assert_eq!(engine2.inpaint_mask(&mut rgb, &mask, 2, 2).unwrap(), 2);
    assert_eq!(rgb[0], 128);

    // 3. CMYK Spectral Color Separation Engine
    let engine3 = PhotocraftCmykSpectralColorSeparationV68PrEngine::new();
    let src_rgb = vec![255u8, 0u8, 0u8];
    let mut dst_cmyk = vec![0u8; 4];
    assert_eq!(engine3.rgb_to_cmyk(&src_rgb, &mut dst_cmyk, 1).unwrap(), 1);
    assert_eq!(dst_cmyk[0], 0);

    // 4. Oklab Color Grading Wheel Matrix Engine
    let engine4 = PhotocraftOklabColorGradingWheelMatrixV68PrEngine::new();
    let mut rgb_wheel = vec![100u8; 6];
    assert_eq!(engine4.grade_rgb_oklab(&mut rgb_wheel, 2).unwrap(), 2);

    // 5. Subpixel Pyramid Focus Stacking Engine
    let engine5 = PhotocraftSubpixelPyramidFocusStackingV68PrEngine::new();
    let slice1 = vec![40u8; 6];
    let slice2 = vec![180u8; 6];
    let mut out_stack = vec![0u8; 6];
    assert_eq!(engine5.stack_focus_slices(&[slice1, slice2], 2, 1, &mut out_stack).unwrap(), 2);
    assert_eq!(out_stack[0], 180);

    // 6. Non-Destructive Smart Filter Graph Engine
    let mut engine6 = PhotocraftNonDestructiveSmartFilterGraphV68PrEngine::new();
    engine6.insert_filter_node(FilterGraphNodeV68 {
        node_id: 101,
        op: FilterGraphOpV68::GaussianConvolution { sigma: 1.0 },
        is_active: true,
        opacity: 1.0,
    });
    let mut rgb_filter = vec![200u8; 3];
    assert_eq!(engine6.evaluate_graph(&mut rgb_filter, 1, 1).unwrap(), 1);
    assert!(rgb_filter[0] < 200);

    // 7. Subpixel Vector Pen Clipping Path Engine
    let mut engine7 = PhotocraftSubpixelVectorPenClippingPathV68PrEngine::new();
    engine7.add_knot(BezierKnotSegmentV68 {
        anchor_start: VectorCoord2DV68 { x: 0.0, y: 0.0 },
        control_out: VectorCoord2DV68 { x: 10.0, y: 10.0 },
        control_in: VectorCoord2DV68 { x: 20.0, y: 20.0 },
        anchor_end: VectorCoord2DV68 { x: 30.0, y: 30.0 },
    });
    let mut mask_out = vec![0u8; 4];
    assert_eq!(engine7.rasterize_clipping_mask(&mut mask_out, 2, 2).unwrap(), 4);
    assert_eq!(mask_out[0], 255);

    // 8. Neural Semantic Hair Alpha Matting Engine
    let engine8 = PhotocraftNeuralSemanticHairAlphaMattingV68PrEngine::new();
    let mut trimap = vec![128u8; 4];
    assert_eq!(engine8.solve_matte_transparency(&mut trimap, 2, 2).unwrap(), 4);
    assert_eq!(trimap[0], 230);

    // 9. Gigapixel Multi-Band Spherical Stitcher Engine
    let engine9 = PhotocraftGigapixelMultiBandSphericalStitcherV68PrEngine::new();
    assert_eq!(engine9.stitch_overlapping_frames(5).unwrap(), 5);

    // 10. Spatial ARAP Mesh Puppet Warp Engine
    let mut engine10 = PhotocraftSpatialArapMeshPuppetWarpV68PrEngine::new();
    engine10.insert_pin(PuppetControlConstraintPinV68 {
        source_point: (5.0, 5.0),
        destination_point: (8.0, 10.0),
        pin_depth: 1,
        pin_stiffness: 0.9,
    });
    assert_eq!(engine10.deform_mesh_vertices(300).unwrap(), 300);
}
