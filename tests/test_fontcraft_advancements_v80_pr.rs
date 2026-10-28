// SPDX-License-Identifier: MIT
// SigmaOS - FontCraft V80 Typography Advancements Integration Test
// (`tests/test_fontcraft_advancements_v80_pr.rs`)

#[path = "../src/creative/fontcraft_advancements_v80_pr.rs"]
mod fontcraft_advancements_v80_pr;

use fontcraft_advancements_v80_pr::*;

#[test]
fn test_fontcraft_v80_master_suite_integration() {
    let master_suite = get_fontcraft_v80_master_suite();
    assert!(
        master_suite.evaluate_v80_typography_parity(),
        "FontCraft V80 typography parity evaluation should succeed"
    );

    let manifests = master_suite.collect_all_pr_manifests();
    assert_eq!(manifests.len(), 5, "Expected 5 PR manifests in FontCraft V80 suite");

    let pr_titles = [
        "feat(fontcraft): OpenType GSUB/GPOS Complex Script Text Shaping & Bidi Engine V80",
        "feat(fontcraft): Multi-Channel Signed Distance Field (MSDF) Vector Shader Rasterizer V80",
        "feat(fontcraft): OpenType Variable Font Design Axis Delta Interpolator V80",
        "feat(fontcraft): COLRv1 Color Vector Emoji Layer Tree & SVG OpenType Compositor V80",
        "feat(fontcraft): WOFF2 Brotli Stream Decompressor & Micro-Font Subsetting Engine V80",
    ];

    for (manifest, expected_title) in manifests.iter().zip(pr_titles.iter()) {
        assert_eq!(&manifest.pr_title, expected_title);
        assert_eq!(manifest.status, "Ready for Merge");
    }

    let markdown = master_suite.export_pr_manifest_markdown();
    assert!(markdown.contains("# FontCraft Typography & Vector Font V80 Advancements PR Manifest"));
    assert!(markdown.contains("PR #8001:"));
    assert!(markdown.contains("PR #8005:"));

    let parity_suite = FontcraftAdvancementsV80ParitySuite::new();
    assert!(
        parity_suite.verify_complete_v80_parity(),
        "FontCraft V80 parity suite should verify complete parity"
    );

    let digest = fnv1a_fontcraft_v80_pr_digest(8001, "OpenType Shaping V80");
    assert_ne!(digest, 0, "FNV-1a digest should produce non-zero hash");
}

#[test]
fn test_fontcraft_v80_individual_engines() {
    let shaping_engine = FontcraftOpentypeGsubGposShapingPrEngineV80::new();
    let shaped = shaping_engine.shape_text_run_v80("ffi ffl");
    assert_eq!(shaped[0], (0xFB03, 750)); // 'ffi'
    assert_eq!(shaped[1], (' ' as u32, 500)); // space
    assert_eq!(shaped[2], (0xFB04, 780)); // 'ffl'

    let msdf_engine = FontcraftMsdfGpuVectorShaderRasterizerPrEngineV80::new();
    let sample = msdf_engine.generate_msdf_sample_v80(2.0, 2.0, (0.0, 0.0, 5.0, 0.0));
    assert!(sample.0 >= 0.0 && sample.1 >= 0.0 && sample.2 >= 0.0);

    let var_engine = FontcraftVariableFontInterpolationPrEngineV80::new();
    let axis_delta = var_engine.interpolate_axis_delta_v80(400.0, 600.0, 100.0, 900.0);
    assert!((axis_delta - 0.25).abs() < 1e-3);

    let emoji_engine = FontcraftColorEmojiCompositorPrEngineV80::new();
    let blended = emoji_engine.composite_colrv1_layer_v80((100, 100, 100), (200, 200, 200), 0.5);
    assert_eq!(blended, (150, 150, 150));

    let woff2_engine = FontcraftWoff2BrotliTranscoderPrEngineV80::new();
    let decomp = woff2_engine.decompress_brotli_header_v80(256).unwrap();
    assert_eq!(decomp, 1024);
}
