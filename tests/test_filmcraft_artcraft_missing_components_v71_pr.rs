// SPDX-License-Identifier: MIT
// Integration Test Suite for Filmcraft ArtCraft Missing Components V71 PR Engine

#[path = "../src/creative/filmcraft_artcraft_missing_components_v71_pr.rs"]
mod filmcraft_artcraft_missing_components_v71_pr;

use filmcraft_artcraft_missing_components_v71_pr::*;

#[test]
fn test_v71_filmcraft_individual_engines() {
    // 1. AI Neural Super-Resolution Temporal Upscaler V71
    let sr_engine = FilmcraftNeuralSuperResolutionTemporalUpscalerV71PrEngine::new();
    let lr_data = [0.1f32, 0.2, 0.3, 0.4];
    let mvs = [(0.5f32, -0.2f32)];
    let mut hr_out = Vec::new();
    let res = sr_engine.upscale_frame(&lr_data, &mvs, &mut hr_out);
    assert!(res.is_ok());
    assert_eq!(hr_out.len(), 4 * (2 * 2));
    let p1 = sr_engine.generate_proposal();
    assert_eq!(p1.pr_id, 7151);
    assert_eq!(p1.status, "Proposed");

    // 2. Hardware-Accelerated ProRes RAW & CinemaDNG Bayer Demosaic V71
    let demosaic_engine = FilmcraftHardwareAcceleratedProResRawBayerDemosaicV71PrEngine::new();
    let (r, g, b) = demosaic_engine.demosaic_bayer_quad(12000, 15000, 15200, 18000);
    assert!(r > 0.0 && g > 0.0 && b > 0.0);
    let p2 = demosaic_engine.generate_proposal();
    assert_eq!(p2.pr_id, 7152);

    // 3. Acoustic Room Impulse Response (RIR) 3D Convolution V71
    let rir_engine = FilmcraftAcousticRoomImpulseResponseConvolutionV71PrEngine::new();
    let rt60 = rir_engine.calculate_rt60();
    assert!(rt60 > 0.0 && rt60 < 10.0);
    let dry_samples = [0.1f32, 0.5, -0.3, 0.0];
    let mut wet_samples = Vec::new();
    rir_engine.convolve_reverb(&dry_samples, &mut wet_samples);
    assert_eq!(wet_samples.len(), dry_samples.len());
    let p3 = rir_engine.generate_proposal();
    assert_eq!(p3.pr_id, 7153);

    // 4. AI Neural Object Segmentation & Video Inpainting V71
    let inpaint_engine = FilmcraftAiNeuralObjectSegmentationInpaintingV71PrEngine::new();
    let frame_rgb = [10u8, 20, 30, 40, 50, 60, 200, 210, 220];
    let mask = [0u8, 0, 255];
    let mut out_rgb = Vec::new();
    let inpaint_res = inpaint_engine.inpaint_frame(&frame_rgb, &mask, &mut out_rgb);
    assert!(inpaint_res.is_ok());
    assert_eq!(out_rgb.len(), frame_rgb.len());
    let p4 = inpaint_engine.generate_proposal();
    assert_eq!(p4.pr_id, 7154);

    // 5. Subframe Audio Multiband Transient Shaper V71
    let transient_engine = FilmcraftSubframeAudioMultibandTransientShaperV71PrEngine::new();
    let audio_in = [0.0f32, 0.8, 0.2, 0.1, -0.5, 0.0];
    let mut audio_out = Vec::new();
    transient_engine.shape_transients(&audio_in, &mut audio_out);
    assert_eq!(audio_out.len(), audio_in.len());
    let p5 = transient_engine.generate_proposal();
    assert_eq!(p5.pr_id, 7155);

    // 6. CRDT Peer-to-Peer Timeline Gossip Sync V71
    let mut gossip_local = FilmcraftCrdtPeerToPeerTimelineGossipSyncV71PrEngine::new("host_node");
    let (tx_seq, tx_hash) = gossip_local.record_mutation("split_clip_head");
    assert_eq!(tx_seq, 2);
    assert!(tx_hash.contains("split_clip_head"));
    let mut remote_vc = std::collections::BTreeMap::new();
    remote_vc.insert("remote_node".to_string(), 5u64);
    let updated = gossip_local.merge_remote_gossip(&remote_vc);
    assert_eq!(updated, 1);
    assert_eq!(gossip_local.vector_clock.get("remote_node"), Some(&5));
    let p6 = gossip_local.generate_proposal();
    assert_eq!(p6.pr_id, 7156);

    // 7. OCIOv2 Spectral Curve Mapper & Color Grading V71
    let curve_engine = FilmcraftColorGradingOcioV2SpectralCurveMapperV71PrEngine::new();
    let comp_val = curve_engine.compress_gamut_coordinate(1.2);
    assert!(comp_val < 1.2 && comp_val >= 0.85);
    let p7 = curve_engine.generate_proposal();
    assert_eq!(p7.pr_id, 7157);

    // 8. OpenFX Sandboxed GPU Kernel Host V71
    let ofx_host = FilmcraftOpenFxSandboxedGpuKernelHostV71PrEngine::new();
    assert!(ofx_host.validate_plugin_memory_request(512 * 1024 * 1024));
    assert!(!ofx_host.validate_plugin_memory_request(8 * 1024 * 1024 * 1024));
    let p8 = ofx_host.generate_proposal();
    assert_eq!(p8.pr_id, 7158);

    // 9. Kinetic Typography Physics Spline Renderer V71
    let typography_engine = FilmcraftKineticTypographyPhysicsSplineRendererV71PrEngine::new();
    let (next_pos, next_vel) = typography_engine.step_spring_physics(0.0, 0.0, 50.0, 0.016);
    assert!(next_pos > 0.0);
    assert!(next_vel > 0.0);
    let p9 = typography_engine.generate_proposal();
    assert_eq!(p9.pr_id, 7159);

    // 10. Lossless Multi-Codec Smart Render Exporter V71
    let exporter = FilmcraftLosslessMultiCodecSmartRenderExporterV71PrEngine::new();
    assert!(exporter.evaluate_smart_render_eligibility("DNxHR", false, false));
    assert!(!exporter.evaluate_smart_render_eligibility("DNxHR", false, true));
    let p10 = exporter.generate_proposal();
    assert_eq!(p10.pr_id, 7160);
}

#[test]
fn test_v71_sovereign_gateway_and_master_suite() {
    let gateway = SovereignFilmcraftArtcraftMissingComponentsV71PrGateway::new();
    assert!(gateway.evaluate_filmcraft_v71_parity());
    assert_eq!(gateway.compute_parity_score(), 100.0);

    let proposals = gateway.collect_all_proposals();
    assert_eq!(proposals.len(), 10);

    for prop in &proposals {
        assert!(prop.pr_id >= 7151 && prop.pr_id <= 7160);
        assert!(!prop.pr_title.is_empty());
        assert!(!prop.target_subsystem.is_empty());
        assert!(!prop.components_added.is_empty());
        assert!(!prop.performance_multiplier.is_empty());
        assert_eq!(prop.status, "Proposed");
    }

    let mut suite = FilmcraftArtcraftMissingComponentsV71PrSuite::new();
    assert!(suite.verify_complete_parity());
}
