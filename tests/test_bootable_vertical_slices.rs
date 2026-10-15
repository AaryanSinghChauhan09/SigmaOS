// SPDX-License-Identifier: MIT
// Test Suite for Bootable Vertical Slices Missing Components PR Engine
// (`tests/test_bootable_vertical_slices.rs`)

#[path = "../src/distro/bootable_vertical_slices_missing_components_pr.rs"]
mod bootable_vertical_slices_missing_components_pr;

use bootable_vertical_slices_missing_components_pr::*;

#[test]
fn test_milestones_and_rules_completeness() {
    let engine = BootableVerticalSlicesPrEngine::new();
    let milestones = engine.get_milestones();
    let rules = engine.get_rules();

    assert_eq!(milestones.len(), 9);
    assert_eq!(rules.len(), 12);

    assert_eq!(milestones[0].code(), "M0");
    assert_eq!(milestones[8].code(), "M8");
    assert_eq!(rules[0].id(), 1);
    assert_eq!(rules[11].id(), 12);
}

#[test]
fn test_gap_map_roadmap_and_spec_generation() {
    let engine = BootableVerticalSlicesPrEngine::new();
    let gap_map = engine.get_gap_map();
    let roadmap = engine.get_90_day_roadmap();
    let spec = engine.generate_proposal_spec();
    let patch = engine.generate_patch_diff();

    assert_eq!(gap_map.len(), 13);
    assert_eq!(roadmap.len(), 5);
    assert_eq!(spec.milestones.len(), 9);
    assert_eq!(spec.rules.len(), 12);
    assert!(patch.contains("feature/bootable-vertical-slices-roadmap"));
    assert!(engine.verify_definition_of_done(10));
    assert!(!engine.verify_definition_of_done(9));
}
