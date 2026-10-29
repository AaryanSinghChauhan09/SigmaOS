// SPDX-License-Identifier: MIT
// SigmaOS - FontCraft Advancements V81 PR Integration Test Suite
// (`tests/test_fontcraft_advancements_v81_pr.rs`)

#[path = "../src/creative/fontcraft_advancements_v81_pr.rs"]
mod fontcraft_advancements_v81_pr;

use fontcraft_advancements_v81_pr::*;

#[test]
fn test_fontcraft_v81_variable_font_design_space() {
    let engine = FontcraftVariableFontDesignSpaceGridPrEngineV81::new();
    let norm = engine.normalize_axis_value_v81(600.0, 100.0, 400.0, 900.0);
    assert!((norm - 0.4).abs() < 1e-3);

    let advance = engine.calculate_multi_axis_grid_deltas_v81(500.0, &[0.5, -0.2], &[100.0, 50.0]);
    assert_eq!(advance, 540.0);

    let manifest = engine.generate_pr_manifest();
    assert_eq!(manifest.pr_id, 8101);
    assert_eq!(manifest.status, "Ready for Merge");
}

#[test]
fn test_fontcraft_v81_msdf_texture_atlas() {
    let engine = FontcraftGlyphOutlineSdfTextureAtlasPrEngineV81::new();
    let (r, g, b) = engine.sample_msdf_outline_distance_v81(2.0, 2.0, (0.0, 0.0), (10.0, 0.0));
    assert!(r > 0.0 && g > 0.0 && b > 0.0);

    let mut pixels = vec![(10, 20, 30), (40, 50, 60), (70, 80, 90)];
    engine.apply_subpixel_cleartype_filtering_v81(&mut pixels);
    assert_eq!(pixels.len(), 3);

    let manifest = engine.generate_pr_manifest();
    assert_eq!(manifest.pr_id, 8102);
}

#[test]
fn test_fontcraft_v81_complex_shaping_and_bidi() {
    let engine = FontcraftOpentypeGsubGposComplexShapingPrEngineV81::new();
    let shaped = engine.shape_text_run_v81("affirmation flow");
    assert_eq!(shaped[0], ('a' as u32, 500));
    assert_eq!(shaped[1], (0xFB03, 750)); // 'ffi' ligature

    let spans = engine.reorder_bidi_spans_v81("Latin \u{0627}\u{0644}\u{0633}\u{0644}\u{0627}\u{0645}");
    assert_eq!(spans.len(), 2);
    assert!(!spans[0].1);
    assert!(spans[1].1);

    let manifest = engine.generate_pr_manifest();
    assert_eq!(manifest.pr_id, 8103);
}

#[test]
fn test_fontcraft_v81_color_emoji_and_woff2() {
    let emoji_engine = FontcraftColorEmojiColrV1BitmapCompositorPrEngineV81::new();
    let blend = emoji_engine.composite_colrv1_paint_layer_v81((100, 100, 100), (200, 200, 200), 0.5);
    assert_eq!(blend, (150, 150, 150));

    let zwj_key = emoji_engine.resolve_zwj_sequence_key_v81(&[0x1F468, 0x200D, 0x1F469]);
    assert_ne!(zwj_key, 0);

    let woff2_engine = FontcraftWoff2BrotliTranscoderSubsettingPrEngineV81::new();
    assert_eq!(woff2_engine.validate_brotli_stream_header_v81(128).unwrap(), 512);

    let subset = woff2_engine.subset_glyph_directory_v81(500, &[1, 2, 3, 999]);
    assert_eq!(subset, vec![1, 2, 3]);
}

#[test]
fn test_fontcraft_v81_master_suite_and_parity() {
    let master = get_fontcraft_v81_master_suite();
    assert!(master.evaluate_v81_typography_parity());

    let manifests = master.collect_all_pr_manifests();
    assert_eq!(manifests.len(), 5);

    let markdown = master.export_pr_manifest_markdown();
    assert!(markdown.contains("FontCraft Typography & Vector Font V81 Advancements PR Manifest"));
    assert!(markdown.contains("OpenType GSUB/GPOS Complex Script Text Shaping"));

    let digest = fnv1a_fontcraft_v81_pr_digest(8101, "Test V81 Title");
    assert_ne!(digest, 0);

    let parity_suite = FontcraftAdvancementsV81ParitySuite::new();
    assert!(parity_suite.verify_complete_v81_parity());
}
