// SPDX-License-Identifier: MIT
// SigmaOS - FontCraft Advancements V83 PR Integration Test Suite
// (`tests/test_fontcraft_advancements_v83_pr.rs`)

#[path = "../src/creative/fontcraft_advancements_v83_pr.rs"]
mod fontcraft_advancements_v83_pr;

use fontcraft_advancements_v83_pr::*;

#[test]
fn test_fontcraft_v83_variable_font_design_space() {
    let engine = FontcraftVariableFontDesignSpaceGridPrEngineV83::new();
    let norm = engine.normalize_axis_value_v83(600.0, 100.0, 400.0, 900.0);
    assert!((norm - 0.4).abs() < 1e-3);

    let advance = engine.calculate_multi_axis_grid_deltas_v83(500.0, &[0.5, -0.2], &[100.0, 50.0]);
    assert_eq!(advance, 540.0);

    let manifest = engine.generate_pr_manifest();
    assert_eq!(manifest.pr_id, 8301);
    assert_eq!(manifest.status, "Ready for Merge");
}

#[test]
fn test_fontcraft_v83_msdf_texture_atlas() {
    let engine = FontcraftGlyphOutlineSdfTextureAtlasPrEngineV83::new();
    let (r, g, b) = engine.sample_msdf_outline_distance_v83(2.0, 2.0, (0.0, 0.0), (10.0, 0.0));
    assert!(r > 0.0 && g > 0.0 && b > 0.0);

    let mut pixels = vec![(10, 20, 30), (40, 50, 60), (70, 80, 90)];
    engine.apply_subpixel_cleartype_filtering_v83(&mut pixels);
    assert_eq!(pixels.len(), 3);

    let manifest = engine.generate_pr_manifest();
    assert_eq!(manifest.pr_id, 8302);
}

#[test]
fn test_fontcraft_v83_complex_shaping_and_bidi() {
    let engine = FontcraftOpentypeGsubGposComplexShapingPrEngineV83::new();
    let shaped = engine.shape_text_run_v83("affirmation flow");
    assert_eq!(shaped[0], ('a' as u32, 500));
    assert_eq!(shaped[1], (0xFB03, 750)); // 'ffi' ligature

    let spans = engine.reorder_bidi_spans_v83("Latin \u{0627}\u{0644}\u{0633}\u{0644}\u{0627}\u{0645}");
    assert_eq!(spans.len(), 2);
    assert!(!spans[0].1);
    assert!(spans[1].1);

    let manifest = engine.generate_pr_manifest();
    assert_eq!(manifest.pr_id, 8303);
}

#[test]
fn test_fontcraft_v83_color_emoji_and_woff2() {
    let emoji_engine = FontcraftColorEmojiColrV1BitmapCompositorPrEngineV83::new();
    let blend = emoji_engine.composite_colrv1_paint_layer_v83((100, 100, 100), (200, 200, 200), 0.5);
    assert_eq!(blend, (150, 150, 150));

    let zwj_key = emoji_engine.resolve_zwj_sequence_key_v83(&[0x1F468, 0x200D, 0x1F469]);
    assert_ne!(zwj_key, 0);

    let woff2_engine = FontcraftWoff2BrotliTranscoderSubsettingPrEngineV83::new();
    assert_eq!(woff2_engine.validate_brotli_stream_header_v83(128).unwrap(), 512);

    let subset = woff2_engine.subset_glyph_directory_v83(500, &[1, 2, 3, 999]);
    assert_eq!(subset, vec![1, 2, 3]);
}

#[test]
fn test_fontcraft_v83_master_suite_and_parity() {
    let master = get_fontcraft_v83_master_suite();
    assert!(master.evaluate_v83_typography_parity());

    let manifests = master.collect_all_pr_manifests();
    assert_eq!(manifests.len(), 5);

    let markdown = master.export_pr_manifest_markdown();
    assert!(markdown.contains("FontCraft Typography & Vector Font V83 Advancements PR Manifest"));
    assert!(markdown.contains("OpenType GSUB/GPOS Complex Script Text Shaping"));

    let digest = fnv1a_fontcraft_v83_pr_digest(8301, "Test V83 Title");
    assert_ne!(digest, 0);

    let parity_suite = FontcraftAdvancementsV83ParitySuite::new();
    assert!(parity_suite.verify_complete_v83_parity());
}
