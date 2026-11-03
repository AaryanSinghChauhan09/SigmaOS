// SPDX-License-Identifier: MIT
// Test Suite for Filmcraft ArtCraft Missing Components V71 PR Engine & Integration

#[path = "../src/creative/filmcraft_artcraft_missing_components_v71_pr.rs"]
mod filmcraft_artcraft_missing_components_v71_pr;

use filmcraft_artcraft_missing_components_v71_pr::*;

#[test]
fn test_v71_filmcraft_individual_engines() {
    // 1. AI Temporal Super-Resolution
    let mut sr_engine = FilmcraftNeuralSuperResolutionTemporalUpscalerV71PrEngine::new();
    let lr_buf = [12u8, 24, 36, 48];
    let mut sr_out = Vec::new();
    let (w, h) = sr_engine.upscale_frame(&lr_buf, 2, 2, &mut sr_out).unwrap();
    assert_eq!(w, 8);
    assert_eq!(h, 8);
    assert_eq!(sr_out.len(), 8 * 8 * 4);
    assert_eq!(sr_engine.processed_frames_count, 1);

    // 2. Hardware ProRes RAW Bayer Demosaic
    let mut demosaic_engine = FilmcraftHardwareAcceleratedProResRawBayerDemosaicV71PrEngine::new();
    let raw = [500u16, 1000, 1500, 2000];
    let mut rgb = Vec::new();
    let channels = demosaic_engine.demosaic_bayer(&raw, 2, 2, &mut rgb).unwrap();
    assert_eq!(channels, 12);
    assert_eq!(demosaic_engine.processed_raw_chunks, 1);

    // 3. Acoustic RIR Spatial Convolution
    let acoustic_engine = FilmcraftAcousticRoomImpulseResponseConvolutionV71PrEngine::new();
    let dry = [0.1f32, 0.4, -0.2, 0.7, 0.3];
    let mut wet = Vec::new();
    let conv_len = acoustic_engine.convolve_dry_audio(&dry, &mut wet);
    assert_eq!(conv_len, 5);
    assert_eq!(wet.len(), 5);

    // 4. Zero-Shot Object Segmentation & Inpainting
    let mut inpaint_engine = FilmcraftAiNeuralObjectSegmentationInpaintingV71PrEngine::new();
    let frame = [100u8, 120, 140, 160];
    let mask = [0u8, 255, 0, 255];
    let mut clean_plate = Vec::new();
    let inpainted_count = inpaint_engine.segment_and_inpaint(&frame, &mask, &mut clean_plate).unwrap();
    assert_eq!(inpainted_count, 2);
    assert_eq!(clean_plate.len(), 4);
    assert_eq!(inpaint_engine.segmented_objects_count, 1);

    // 5. Subframe Multiband Transient Shaper
    let shaper_engine = FilmcraftSubframeAudioMultibandTransientShaperV71PrEngine::new();
    let audio_buf = [0.0f32, 0.6, 0.05, -0.5, 0.02];
    let mut shaped = Vec::new();
    let processed_samples = shaper_engine.process_transient_punch(&audio_buf, &mut shaped);
    assert_eq!(processed_samples, 5);
    assert_eq!(shaped.len(), 5);

    // 6. CRDT P2P Gossip Sync
    let mut gossip_engine = FilmcraftCrdtPeerToPeerTimelineGossipSyncV71PrEngine::new();
    let (v_clock, digest) = gossip_engine.broadcast_edit_operation(2, "ripple_delete");
    assert_eq!(v_clock, 2);
    assert!(digest.contains("p2p-op"));
    let merged = gossip_engine.ingest_peer_gossip(3, 10);
    assert!(merged);
    assert_eq!(gossip_engine.state_vector_clock, 11);

    // 7. OCIOv2 Spectral Radiance Curve Color Grading
    let color_engine = FilmcraftColorGradingOcioV2SpectralCurveMapperV71PrEngine::new();
    let graded = color_engine.grade_pixel_spectral([0.5, 0.4, 0.2], 0.5);
    assert!(graded[0] > 0.0);
    assert!(graded[1] > 0.0);
    assert!(graded[2] > 0.0);

    // 8. Sandboxed OFX GPU Kernel Host
    let mut ofx_engine = FilmcraftOpenFxSandboxedGpuKernelHostV71PrEngine::new();
    let plugin_handle = ofx_engine.register_plugin("com.borisfx.mocha").unwrap();
    assert_eq!(plugin_handle, 1);
    let dispatch_msg = ofx_engine.dispatch_gpu_render(plugin_handle, 42).unwrap();
    assert!(dispatch_msg.contains("OFX_DISPATCH:plugin=1:frame=42"));

    // 9. Kinetic Typography Physics Spline Renderer
    let mut text_engine = FilmcraftKineticTypographyPhysicsSplineRendererV71PrEngine::new();
    let svg = text_engine.deform_text_along_spline("SigmaOS Pro Video", 0.75);
    assert!(svg.contains("SigmaOS Pro Video"));
    assert_eq!(text_engine.active_text_elements_count, 1);

    // 10. Smart Render Zero-Transcode Exporter
    let mut exporter_engine = FilmcraftLosslessMultiCodecSmartRenderExporterV71PrEngine::new();
    assert!(exporter_engine.can_smart_render("Apple ProRes", false, false));
    assert!(!exporter_engine.can_smart_render("Apple ProRes", true, false));
    assert!(!exporter_engine.can_smart_render("Apple ProRes", false, true));
    let mut sink = Vec::new();
    let spliced = exporter_engine.splice_bitstream_packet(&[0x11, 0x22, 0x33, 0x44, 0x55], &mut sink).unwrap();
    assert_eq!(spliced, 5);
    assert_eq!(sink.len(), 5);
    assert_eq!(exporter_engine.exported_byte_savings, 5);
}

#[test]
fn test_v71_gateway_evaluation_and_proposals() {
    let gateway = SovereignFilmcraftArtcraftMissingComponentsV71PrGateway::new();
    assert!(gateway.evaluate_filmcraft_v71_parity());
    assert_eq!(gateway.compute_parity_score(), 100.0);

    let proposals = gateway.collect_all_proposals();
    assert_eq!(proposals.len(), 10);

    for (idx, p) in proposals.iter().enumerate() {
        let expected_pr_id = 7151 + idx as u64;
        assert_eq!(p.pr_id, expected_pr_id);
        assert!(!p.pr_title.is_empty());
        assert_eq!(p.status, "Ready for Merge");
        assert_eq!(p.components_added.len(), 3);
        assert!(!p.performance_multiplier.is_empty());
    }
}

#[test]
fn test_v71_suite_verification() {
    let mut suite = FilmcraftArtcraftMissingComponentsV71PrSuite::new();
    assert_eq!(suite.total_proposals_count(), 10);
    assert!(suite.verify_complete_parity());
}
