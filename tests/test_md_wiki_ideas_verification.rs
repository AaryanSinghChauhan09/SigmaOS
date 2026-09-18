// SPDX-License-Identifier: MIT
// Verification Test Suite for .MD Files & GitHub Wiki Unimplemented Ideas Parity

#[path = "../src/sovereign_wiki_master_engine.rs"]
mod sovereign_wiki_master_engine;

#[path = "../src/wiki_unimplemented_ideas.rs"]
mod wiki_unimplemented_ideas;

use sovereign_wiki_master_engine::*;
use wiki_unimplemented_ideas::*;

#[test]
fn test_sovereign_wiki_master_engine_full_parity() {
    let wiki_engine = SovereignWikiMasterEngine::new();
    assert!(!wiki_engine.suite_100_ideas.ideas.is_empty());
    assert_eq!(wiki_engine.shards_registry.shards.len(), 12);
    assert!(wiki_engine.evaluate_master_wiki_fulfillment());
}

#[test]
fn test_sigma_office_suite_engine_wiki_parity() {
    let mut office = SigmaOfficeSuiteEngine::new();
    office.edit_word_doc("SigmaOS Sovereign Office Doc");
    assert_eq!(office.word_document_content, "SigmaOS Sovereign Office Doc");

    office.set_spreadsheet_cell(0, 0, "=SUM(A1:A5)", 42.0);
    let cell = office.spreadsheet_grid.get(&(0, 0)).unwrap();
    assert_eq!(cell.evaluated_number, 42.0);
}
