// SPDX-License-Identifier: MIT
// SigmaOS - VoiceCraft V82 Zero-Shot Speech Synthesis & Audio DSP Advancements Integration Test
// (`tests/test_voicecraft_advancements_v82_pr.rs`)

#[path = "../src/creative/voicecraft_advancements_v82_pr.rs"]
mod voicecraft_advancements_v82_pr;

use voicecraft_advancements_v82_pr::*;

#[test]
fn test_voicecraft_v82_master_suite_integration() {
    let master_suite = get_voicecraft_v82_master_suite();
    assert!(
        master_suite.evaluate_voicecraft_v82_parity(),
        "VoiceCraft V82 parity evaluation should succeed"
    );

    let manifests = master_suite.generate_all_manifests();
    assert_eq!(manifests.len(), 5, "Expected 5 PR manifests in VoiceCraft V82 suite");

    let pr_titles = [
        "feat(voicecraft): Optimal Transport Continuous Flow Matching Acoustic Mel Synthesizer V82",
        "feat(voicecraft): Autoregressive Multi-Codebook RVQ Neural Codec Language Model Engine V82",
        "feat(voicecraft): Differentiable DSP (DDSP) Harmonic-Plus-Noise Neural Vocoder V82",
        "feat(voicecraft): Spatial Multi-Mic Array Beamforming & HRTF Binaural Renderer Engine V82",
        "feat(voicecraft): End-to-End Wayland Voice Assistant & Neural Accessibility Pipeline Engine V82",
    ];

    for (manifest, expected_title) in manifests.iter().zip(pr_titles.iter()) {
        assert_eq!(&manifest.pr_title, expected_title);
        assert_eq!(manifest.status, "Ready for Merge");
    }

    let markdown = master_suite.export_pr_manifest_markdown();
    assert!(markdown.contains("# VoiceCraft V82 Zero-Shot Speech Synthesis & Audio DSP Advancements PR Suite"));
    assert!(markdown.contains("PR #8201"));
    assert!(markdown.contains("PR #8205"));

    let parity_suite = VoicecraftAdvancementsV82ParitySuite::new();
    assert!(
        parity_suite.verify_complete_parity(),
        "VoicecraftAdvancementsV82ParitySuite should verify complete parity"
    );

    let digest = fnv1a_voicecraft_v82_pr_digest(8201, "Continuous Flow Matching V82");
    assert_ne!(digest, 0, "FNV-1a digest should produce non-zero hash");
}

#[test]
fn test_voicecraft_v82_individual_engines() {
    let flow = VoicecraftContinuousFlowMatchingMelSynthesizerPrEngineV82::new();
    let mel = flow.generate_flow_mel_v82(&[101, 102], 50);
    assert_eq!(mel.len(), 80);

    let rvq = VoicecraftAutoregressiveRvqCodecLanguageModelPrEngineV82::new();
    let tokens = rvq.sample_zero_shot_tokens_v82(&[vec![10, 20]], 15);
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].len(), 15);

    let vocoder = VoicecraftHarmonicPlusNoiseDdspVocoderPrEngineV82::new();
    let pcm = vocoder.synthesize_ddsp_pcm_v82(&[220.0; 5], &[-6.0; 5]);
    assert_eq!(pcm.len(), 5 * 320);

    let beamformer = VoicecraftSpatialArrayDelayAndSumBeamformerPrEngineV82::new();
    let mic_data = vec![vec![0.3f32; 160]; 4];
    let focused = beamformer.beamform_mic_array_v82(&mic_data, 0.1);
    assert_eq!(focused.len(), 160);
    let (l, r) = beamformer.render_binaural_hrtf_v82(&focused, 0.1);
    assert_eq!(l.len(), 160);
    assert_eq!(r.len(), 160);

    let assistant = VoicecraftWaylandVoiceAssistantAccessibilityPipelinePrEngineV82::new();
    let pcm_active = vec![0.5f32; 320];
    let intent = assistant.process_voice_pipeline_v82(&pcm_active, "hey sigma open editor");
    assert!(intent.is_some());
    let unwrapped = intent.unwrap();
    assert_eq!(unwrapped.intent_name, "open editor");
    assert_eq!(unwrapped.target_app, "helix");
}
