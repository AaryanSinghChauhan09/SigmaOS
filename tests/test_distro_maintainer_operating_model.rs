// SPDX-License-Identifier: MIT
// Test Suite for Distro-Maintainer Operating Model PR Proposal Gateway Engine

#[path = "../src/distro/distro_maintainer_operating_model_pr.rs"]
mod distro_maintainer_operating_model_pr;

use distro_maintainer_operating_model_pr::*;

#[test]
fn test_distro_maintainer_operating_model_agent_roles() {
    let engine = DistroMaintainerPrEngine::new();
    let roles = engine.get_agent_roles();
    assert_eq!(roles.len(), 7);

    let steward = &roles[0];
    assert_eq!(steward.name(), "Repository Steward");
    assert!(steward.purpose().contains("hygiene"));
    assert_eq!(steward.primary_deliverable(), "reports/steward/daily-status.md");

    let sentinel = &roles[1];
    assert_eq!(sentinel.name(), "Build and Test Sentinel");
    assert_eq!(sentinel.primary_deliverable(), "reports/build/latest.json");

    let validator = &roles[2];
    assert_eq!(validator.name(), "Boot and Runtime Validator");
    assert_eq!(validator.primary_deliverable(), "reports/boot/qemu_x86_64.json");

    let auditor = &roles[3];
    assert_eq!(auditor.name(), "Capability Auditor");
    assert_eq!(auditor.primary_deliverable(), "reports/capability/truth_table.json");
}

#[test]
fn test_task_lifecycle_stages_sequence() {
    let engine = DistroMaintainerPrEngine::new();
    let stages = engine.get_lifecycle_stages();
    assert_eq!(stages.len(), 9);

    let stage_names: Vec<&'static str> = stages.iter().map(|s| s.name()).collect();
    assert_eq!(
        stage_names,
        vec![
            "SELECT",
            "PLAN",
            "ISOLATE",
            "EXECUTE",
            "VERIFY",
            "RECORD",
            "UPDATE STATUS",
            "REVIEW",
            "FOLLOW UP"
        ]
    );

    for (i, stage) in stages.iter().enumerate() {
        assert_eq!(*stage as usize, i + 1);
        assert!(!stage.description().is_empty());
    }
}

#[test]
fn test_mandatory_operating_rules_count_and_titles() {
    let engine = DistroMaintainerPrEngine::new();
    let rules = engine.get_operating_rules();
    assert_eq!(rules.len(), 10);

    for (i, rule) in rules.iter().enumerate() {
        assert_eq!(rule.id() as usize, i + 1);
        assert!(!rule.title().is_empty());
    }

    assert_eq!(rules[0].title(), "Evidence before status");
    assert_eq!(rules[4].title(), "Human approval for high-impact actions");
    assert_eq!(rules[9].title(), "Prefer runtime proof over model proof");
}

#[test]
fn test_release_channels_and_requirements() {
    let engine = DistroMaintainerPrEngine::new();
    let channels = engine.get_release_channels();
    assert_eq!(channels.len(), 4);

    assert_eq!(channels[0].name(), "dev");
    assert_eq!(channels[1].name(), "nightly");
    assert_eq!(channels[2].name(), "validated");
    assert_eq!(channels[3].name(), "experimental");

    for channel in &channels {
        assert!(!channel.requirement().is_empty());
    }
}

#[test]
fn test_first_ten_backlog_tasks() {
    let engine = DistroMaintainerPrEngine::new();
    let tasks = engine.get_first_ten_tasks();
    assert_eq!(tasks.len(), 10);

    assert_eq!(tasks[0].id, "REG-001");
    assert_eq!(tasks[1].id, "POL-001");
    assert_eq!(tasks[2].id, "CLI-001");
    assert_eq!(tasks[3].id, "CI-001");
    assert_eq!(tasks[4].id, "BOOT-X86-001");
    assert_eq!(tasks[5].id, "AUDIT-001");
    assert_eq!(tasks[6].id, "DASH-001");
    assert_eq!(tasks[7].id, "TASK-001");
    assert_eq!(tasks[8].id, "PKG-001");
    assert_eq!(tasks[9].id, "NIGHTLY-001");

    for task in &tasks {
        assert!(!task.title.is_empty());
        assert!(!task.agent.is_empty());
        assert!(!task.priority.is_empty());
    }
}

#[test]
fn test_proposal_spec_generation_and_patch_diff() {
    let engine = DistroMaintainerPrEngine::new();
    let spec = engine.generate_proposal_spec();

    assert_eq!(
        spec.pr_title,
        "feat(distro): Adopt Distro-Maintainer Operating Model and Evidence-Gated Agent Framework"
    );
    assert_eq!(spec.pr_branch, "feature/distro-maintainer-operating-model");
    assert_eq!(spec.roles.len(), 7);
    assert_eq!(spec.stages.len(), 9);
    assert_eq!(spec.rules.len(), 10);
    assert_eq!(spec.channels.len(), 4);
    assert_eq!(spec.initial_tasks.len(), 10);
    assert_ne!(spec.patch_digest, 0);

    let diff = engine.generate_patch_diff();
    assert!(diff.contains("PR Title: feat(distro): Adopt Distro-Maintainer Operating Model"));
    assert!(diff.contains("PR Branch: feature/distro-maintainer-operating-model"));
    assert!(diff.contains("Agent Roles Count: 7"));
    assert!(diff.contains("Lifecycle Stages Count: 9"));
    assert!(diff.contains("Rules Count: 10"));
    assert!(diff.contains("Patch Digest:"));
}

#[test]
fn test_operating_rules_satisfaction_verifier() {
    let engine = DistroMaintainerPrEngine::new();
    assert!(engine.verify_operating_rules_satisfied(10));
    assert!(engine.verify_operating_rules_satisfied(12));
    assert!(!engine.verify_operating_rules_satisfied(9));
    assert!(!engine.verify_operating_rules_satisfied(0));
}
