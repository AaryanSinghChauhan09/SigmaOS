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
