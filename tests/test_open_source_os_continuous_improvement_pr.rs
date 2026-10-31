// SPDX-License-Identifier: MIT
// Integration Test Suite for Continuous Open Source OS Improvement PR Engine

#[path = "../src/distro/continuous_open_source_os_improvement_pr.rs"]
mod continuous_open_source_os_improvement_pr;

use continuous_open_source_os_improvement_pr::*;

#[test]
fn test_cadences_and_metadata() {
    let engine = ContinuousOpenSourceOsImprovementPrEngine::new();
    let cadences = engine.get_cadences();
    assert_eq!(cadences.len(), 6);

    assert_eq!(cadences[0].name(), "Daily / Per-PR");
    assert!(cadences[0].focus().contains("Fast CI"));

    assert_eq!(cadences[1].name(), "Weekly");
    assert!(cadences[1].focus().contains("Backlog triage"));

    assert_eq!(cadences[2].name(), "Bi-weekly");
    assert_eq!(cadences[3].name(), "Monthly");
    assert_eq!(cadences[4].name(), "Quarterly");
    assert_eq!(cadences[5].name(), "Annual");
}

#[test]
fn test_open_source_os_paradigms_matrix() {
    let engine = ContinuousOpenSourceOsImprovementPrEngine::new();
    let paradigms = engine.get_paradigms();
    assert_eq!(paradigms.len(), 7);

    let redox = paradigms[0];
    assert_eq!(redox.name(), "Redox OS");
    assert!(redox.borrowed_concept().contains("Microkernel userspace"));
    assert!(redox.sigma_adaptation().contains("Redox-style"));

    let sel4 = paradigms[1];
    assert_eq!(sel4.name(), "seL4 Microkernel");
    assert!(sel4.borrowed_concept().contains("capability token"));

    let tock = paradigms[2];
    assert_eq!(tock.name(), "Tock OS");
    assert!(tock.borrowed_concept().contains("Capsule-style"));

    let fuchsia = paradigms[3];
    assert_eq!(fuchsia.name(), "Fuchsia / Zircon");

    let linux = paradigms[4];
    assert_eq!(linux.name(), "Linux Minimal Kernel");

    let freebsd = paradigms[5];
    assert_eq!(freebsd.name(), "FreeBSD");

    let openbsd = paradigms[6];
    assert_eq!(openbsd.name(), "OpenBSD");
}

#[test]
fn test_continuous_tasks_and_roadmap() {
    let engine = ContinuousOpenSourceOsImprovementPrEngine::new();
    let tasks = engine.get_continuous_tasks();
    let roadmap = engine.get_90_day_roadmap();

    assert_eq!(tasks.len(), 6);
    assert_eq!(tasks[0].id, "TASK-CI-001");
    assert_eq!(tasks[0].owner_role, "Build/CI Sentinel");

    assert_eq!(roadmap.len(), 3);
    assert_eq!(roadmap[0].timeframe, "Days 0–14");
    assert_eq!(roadmap[0].title, "Foundations & Fast CI");
    assert_eq!(roadmap[1].timeframe, "Days 15–45");
    assert_eq!(roadmap[2].timeframe, "Days 46–90");
}

#[test]
fn test_operating_kpis() {
    let engine = ContinuousOpenSourceOsImprovementPrEngine::new();
    let kpis = engine.get_operating_kpis();

    assert_eq!(kpis.len(), 5);
    assert_eq!(kpis[0].metric_name, "CI Fast-Check Pass Rate");
    assert_eq!(kpis[1].metric_name, "Fuzzer Crash Triage Time");
    assert_eq!(kpis[2].metric_name, "Syscall / IPC Latency Delta");
    assert_eq!(kpis[3].metric_name, "Unsafe Block Ratio");
    assert_eq!(kpis[4].metric_name, "Nightly Reproducibility");
}

#[test]
fn test_proposal_spec_generation_and_patch_diff() {
    let engine = ContinuousOpenSourceOsImprovementPrEngine::new();
    let spec = engine.generate_proposal_spec();

    assert_eq!(
        spec.pr_title,
        "feat(distro): Continuous Open Source OS Improvement Engine and Operating Plan"
    );
    assert_eq!(spec.pr_branch, "feat/continuous-os-improvement-plan");
    assert_eq!(spec.cadences.len(), 6);
    assert_eq!(spec.paradigms.len(), 7);
    assert_eq!(spec.tasks.len(), 6);
    assert_eq!(spec.roadmap.len(), 3);
    assert_eq!(spec.kpis.len(), 5);
    assert_ne!(spec.patch_digest, 0);

    let diff = engine.generate_patch_diff();
    assert!(diff.contains("PR Title: feat(distro): Continuous Open Source OS Improvement Engine"));
    assert!(diff.contains("PR Branch: feat/continuous-os-improvement-plan"));
    assert!(diff.contains("Cadences Count: 6"));
    assert!(diff.contains("Paradigms Count: 7"));
    assert!(diff.contains("Patch Digest:"));
}

#[test]
fn test_definition_of_done_verification() {
    let engine = ContinuousOpenSourceOsImprovementPrEngine::new();
    assert!(engine.verify_definition_of_done(10));
    assert!(engine.verify_definition_of_done(12));
    assert!(!engine.verify_definition_of_done(9));
}
