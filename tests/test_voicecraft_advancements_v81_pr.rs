// SPDX-License-Identifier: MIT
// SigmaOS - VoiceCraft V81 Zero-Shot Speech Synthesis & Audio DSP Advancements Integration Test
// (`tests/test_voicecraft_advancements_v81_pr.rs`)

#[path = "../src/creative/voicecraft_advancements_v81_pr.rs"]
mod voicecraft_advancements_v81_pr;

use voicecraft_advancements_v81_pr::*;

#[test]
fn test_voicecraft_v81_master_suite_integration() {
    let master_suite = get_voicecraft_v81_master_suite();
    assert!(
        master_suite.evaluate_voicecraft_v81_parity(),
        "VoiceCraft V81 parity evaluation should succeed"
    );

    let manifests = master_suite.generate_all_manifests();
    assert_eq!(manifests.len(), 5, "Expected 5 PR manifests in VoiceCraft V81 suite");

    let pr_titles = [
        "feat(voicecraft): Optimal Transport Continuous Flow Matching Acoustic Mel Synthesizer V81",
        "feat(voicecraft): Autoregressive Multi-Codebook RVQ Neural Codec Language Model Engine V81",
        "feat(voicecraft): Differentiable DSP (DDSP) Harmonic-Plus-Noise Neural Vocoder V81",
        "feat(voicecraft): Spatial Multi-Mic Array Beamforming & HRTF Binaural Renderer Engine V81",
        "feat(voicecraft): End-to-End Wayland Voice Assistant & Neural Accessibility Pipeline Engine V81",
    ];

    for (manifest, expected_title) in manifests.iter().zip(pr_titles.iter()) {
        assert_eq!(&manifest.pr_title, expected_title);
        assert_eq!(manifest.status, "Ready for Merge");
    }

    let markdown = master_suite.export_pr_manifest_markdown();
    assert!(markdown.contains("# VoiceCraft V81 Zero-Shot Speech Synthesis & Audio DSP Advancements PR Suite"));
    assert!(markdown.contains("PR #8101"));
    assert!(markdown.contains("PR #8105"));

    let parity_suite = VoicecraftAdvancementsV81ParitySuite::new();
    assert!(
        parity_suite.verify_complete_parity(),
        "VoicecraftAdvancementsV81ParitySuite should verify complete parity"
    );

    let digest = fnv1a_voicecraft_v81_pr_digest(8101, "Continuous Flow Matching V81");
    assert_ne!(digest, 0, "FNV-1a digest should produce non-zero hash");
}

#[test]
fn test_voicecraft_v81_individual_engines() {
    let flow_engine = VoicecraftContinuousFlowMatchingMelSynthesizerPrEngineV81::new();
    let mel = flow_engine.generate_flow_mel_v81(&[101, 202], 400);
    assert_eq!(mel.len(), 80);
    assert!(!mel[0].is_empty());

    let codec_lm = VoicecraftAutoregressiveRvqCodecLanguageModelPrEngineV81::new();
    let tokens = codec_lm.sample_zero_shot_tokens_v81(&[vec![10, 20]], 8);
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].len(), 8);

    let vocoder = VoicecraftHarmonicPlusNoiseDdspVocoderPrEngineV81::new();
    let pcm = vocoder.synthesize_ddsp_pcm_v81(&[220.0; 5], &[-10.0; 5]);
    assert_eq!(pcm.len(), 5 * 320);

    let spatial = VoicecraftSpatialArrayDelayAndSumBeamformerPrEngineV81::new();
    let focused = spatial.beamform_mic_array_v81(&[vec![0.5; 320], vec![0.4; 320]], 0.1);
    assert_eq!(focused.len(), 320);

    let pipeline = VoicecraftWaylandVoiceAssistantAccessibilityPipelinePrEngineV81::new();
    let intent = pipeline.process_voice_pipeline_v81(&vec![0.5; 320], "hey sigma take screenshot");
    assert!(intent.is_some());
    let res = intent.unwrap();
    assert_eq!(res.intent_name, "take screenshot");
    assert_eq!(res.target_app, "grim");
}
