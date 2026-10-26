// SPDX-License-Identifier: MIT
// Test Suite for Sovereign Artcraft Creative Suite Master PR Gateway & Benchmark Suite

#[path = "../src/creative/mod.rs"]
mod creative;

use creative::sovereign_artcraft_creative_suite_master_pr::*;

#[test]
fn test_sovereign_artcraft_creative_suite_master_gateway_integration() {
    let mut gateway = SovereignArtcraftCreativeSuiteMasterPrGateway::new();
    assert!(gateway.evaluate_complete_artcraft_suite_parity());

    let proposals = gateway.collect_all_artcraft_pr_proposals();
    assert!(proposals.len() >= 100);

    let score = gateway.compute_artcraft_creative_suite_parity_score();
    assert_eq!(score, 100.0);

    let md = gateway.generate_pr_proposal_manifest_markdown();
    assert!(md.contains("Sovereign ArtCraft Creative Suite Master PR Manifest"));
    assert!(md.contains("PhotoCraft"));
    assert!(md.contains("FilmCraft"));
    assert!(md.contains("PdfCraft"));
    assert!(md.contains("ArtCraft V75 Core"));
}

#[test]
fn test_sovereign_artcraft_creative_suite_master_suite_integration() {
    let mut suite = SovereignArtcraftCreativeSuiteMasterPrSuite::new();
    assert!(suite.verify_complete_suite_parity());
}
