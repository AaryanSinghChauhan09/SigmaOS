// SPDX-License-Identifier: MIT
// Test Suite for Repeatable Agent Operating Loop PR Engine

#[path = "../src/distro/repeatable_agent_operating_loop_pr.rs"]
mod repeatable_agent_operating_loop_pr;

use repeatable_agent_operating_loop_pr::*;

#[test]
fn test_agent_roles_and_metadata() {
    let engine = RepeatableAgentOperatingLoopPrEngine::new();
    let roles = engine.get_roles();
    assert_eq!(roles.len(), 7);

    assert_eq!(roles[0].code(), "steward");
    assert_eq!(roles[0].name(), "Repository Steward");
    assert_eq!(roles[0].owner_path(), ".github/");
    assert!(!roles[0].primary_responsibility().is_empty());

    assert_eq!(roles[1].code(), "sentinel");
    assert_eq!(roles[1].name(), "Build and Test Sentinel");

    assert_eq!(roles[2].code(), "boot-validator");
    assert_eq!(roles[2].name(), "Boot and Runtime Validator");

    assert_eq!(roles[3].code(), "capability-auditor");
    assert_eq!(roles[4].code(), "kernel-subsystem");
    assert_eq!(roles[5].code(), "package-compat");
    assert_eq!(roles[6].code(), "docs-onboarding");
}

#[test]
fn test_operating_loop_stages() {
    let engine = RepeatableAgentOperatingLoopPrEngine::new();
    let stages = engine.get_stages();
    assert_eq!(stages.len(), 9);

    for (idx, stage) in stages.iter().enumerate() {
        assert_eq!(stage.step_number(), (idx + 1) as u8);
        assert!(!stage.title().is_empty());
    }

    assert_eq!(stages[0].title(), "Select Task");
    assert_eq!(stages[8].title(), "Submit PR With Rollback Note");
}

#[test]
fn test_status_levels_and_permission_matrix() {
    let engine = RepeatableAgentOperatingLoopPrEngine::new();
    let levels = engine.get_status_levels();
    let matrix = engine.get_permission_matrix();

    assert_eq!(levels.len(), 6);
    assert_eq!(levels[0].as_str(), "unimplemented");
    assert_eq!(levels[3].as_str(), "qemu-verified");
    assert!(!levels[3].definition().is_empty());

    assert_eq!(matrix.len(), 7);
    for entry in &matrix {
        assert!(!entry.fs_permission.is_empty());
        assert!(!entry.git_write_permission.is_empty());
    }
}

#[test]
fn test_90_day_plan_and_proposal_diff() {
    let engine = RepeatableAgentOperatingLoopPrEngine::new();
    let plan = engine.get_90_day_plan();
    assert_eq!(plan.len(), 5);
    assert_eq!(plan[0].timeframe, "Days 1–14");

    let spec = engine.generate_proposal_spec();
    assert_eq!(spec.pr_branch, "feat/agent-operating-loop");
    assert_eq!(spec.roles.len(), 7);
    assert_eq!(spec.stages.len(), 9);
    assert_ne!(spec.patch_digest, 0);

    let patch = engine.generate_patch_diff();
    assert!(patch.contains("feat/agent-operating-loop"));
    assert!(patch.contains("Patch Digest:"));
}

#[test]
fn test_definition_of_done_verification() {
    let engine = RepeatableAgentOperatingLoopPrEngine::new();
    assert!(engine.verify_definition_of_done(10));
    assert!(engine.verify_definition_of_done(12));
    assert!(!engine.verify_definition_of_done(9));
}
