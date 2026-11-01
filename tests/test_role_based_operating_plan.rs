#[path = "../src/governance/role_based_operating_plan_pr.rs"]
mod role_based_operating_plan_pr;

use role_based_operating_plan_pr::*;

#[test]
fn test_engine_initialization() {
    let engine = RoleBasedOperatingPlanEngine::new();
    assert_eq!(engine.roles.len(), 12);
    assert_eq!(engine.phases.len(), 4);
    assert_eq!(engine.execution_steps.len(), 8);
    assert_eq!(engine.calendar.len(), 5);
}

#[test]
fn test_role_metadata_and_leadership() {
    let engine = RoleBasedOperatingPlanEngine::new();

    let maintainer = engine
        .get_role_definition(MaintenanceRole::Maintainer)
        .expect("Maintainer role found");
    assert_eq!(maintainer.title, "Maintainer");
    assert_eq!(
        maintainer.leadership,
        LeadershipProfile::HumanLedAgentAssisted
    );

    let updater = engine
        .get_role_definition(MaintenanceRole::Updater)
        .expect("Updater role found");
    assert_eq!(updater.title, "Updater");
    assert_eq!(updater.leadership, LeadershipProfile::MostlyAgent);

    let reviewer = engine
        .get_role_definition(MaintenanceRole::Reviewer)
        .expect("Reviewer role found");
    assert_eq!(
        reviewer.leadership,
        LeadershipProfile::AgentFirstPassHumanApproval
    );

    let sec = engine
        .get_role_definition(MaintenanceRole::SecurityOfficer)
        .expect("SecurityOfficer role found");
    assert_eq!(sec.leadership, LeadershipProfile::HumanLed);
}

#[test]
fn test_execution_step_outputs() {
    let engine = RoleBasedOperatingPlanEngine::new();

    let step1 = &engine.execution_steps[0];
    assert_eq!(step1.step_number, 1);
    assert_eq!(step1.required_output, "VISION.md");

    let step2 = &engine.execution_steps[1];
    assert_eq!(step2.step_number, 2);
    assert_eq!(step2.required_output, "FEATURE_STATUS.toml");

    let step3 = &engine.execution_steps[2];
    assert_eq!(step3.step_number, 3);
    assert_eq!(step3.required_output, "GOVERNANCE.md");
}

#[test]
fn test_governance_guardrail_validation() {
    let engine = RoleBasedOperatingPlanEngine::new();

    // Kernel/Crypto PR without human approval -> Fail
    let verdicts_fail = engine.validate_pr_guardrails(true, false, true, true);
    assert!(!verdicts_fail[0].passed);

    // All requirements met -> Pass
    let verdicts_pass = engine.validate_pr_guardrails(true, true, true, true);
    assert!(verdicts_pass.iter().all(|v| v.passed));

    // Missing CI evidence -> Fail
    let verdicts_no_evidence = engine.validate_pr_guardrails(false, false, false, true);
    assert!(!verdicts_no_evidence[1].passed);
}

#[test]
fn test_weekly_calendar_dispatch() {
    let engine = RoleBasedOperatingPlanEngine::new();

    let monday = engine
        .get_day_focus(CalendarDay::Monday)
        .expect("Monday focus found");
    assert!(monday.assigned_roles.contains(&MaintenanceRole::Triager));
    assert!(monday.assigned_roles.contains(&MaintenanceRole::Updater));

    let friday = engine
        .get_day_focus(CalendarDay::Friday)
        .expect("Friday focus found");
    assert!(friday
        .assigned_roles
        .contains(&MaintenanceRole::ReleaseManager));
}

#[test]
fn test_gateway_pr_generation_and_export() {
    let gateway = RoleBasedOperatingPlanPrGateway::new();
    let spec = gateway.generate_proposal_spec();
    assert_eq!(spec.roles_count, 12);
    assert_eq!(spec.phases_count, 4);
    assert_eq!(spec.execution_steps_count, 8);
    assert_eq!(spec.calendar_days_count, 5);
    assert_eq!(spec.guardrails_count, 7);

    let pr_md = gateway.generate_pr_proposal_markdown();
    assert!(pr_md.contains("# feat(governance): Adopt Role-Based Operating Plan and Strategic Wedge Roadmap"));
    assert!(pr_md.contains("Reality Check & Strategic Wedge"));
    assert!(pr_md.contains("12 Operating Maintenance Roles"));
    assert!(pr_md.contains("8-Step Team Execution Plan"));
    assert!(pr_md.contains("7 Governance Guardrails"));

    let diff = gateway.generate_patch_diff();
    assert!(diff.contains("PR_SPEC_ROLE_BASED_GOVERNANCE_AND_OPERATING_PLAN.md"));
    assert!(diff.contains("Roles Count: 12"));
    assert!(diff.contains("Phases Count: 4"));
}
