use sigmaos::distro::distro_maintainer_operating_model_pr::{
    DistroMaintainerOperatingModelPrEngine, MandatoryOperatingRule, ReleaseChannel, AgentRole,
};

#[test]
fn test_distro_maintainer_operating_model_pr_proposal_generation() {
    let engine = DistroMaintainerOperatingModelPrEngine::new();
    let proposal = engine.generate_pr_proposal();

    assert!(proposal.contains("# PR Proposal: Distro-Maintainer Operating Model for SigmaOS"));
    assert!(proposal.contains("Open-Source OS Practice to Agent Mapping"));
    assert!(proposal.contains("Mandatory Rules"));
    assert!(proposal.contains("Release Channel Governance"));
    assert!(proposal.contains("First Ten Tasks"));
}

#[test]
fn test_mandatory_operating_rules() {
    let rule1 = MandatoryOperatingRule::Rule1EvidenceBeforeStatus;
    assert_eq!(rule1.number(), 1);
    assert_eq!(rule1.title(), "Evidence before status");
    assert!(!rule1.description().is_empty());

    let rule10 = MandatoryOperatingRule::Rule10PreferRuntimeProofOverModelProof;
    assert_eq!(rule10.number(), 10);
    assert_eq!(rule10.title(), "Prefer runtime proof over model proof");
}

#[test]
fn test_release_channels() {
    let dev = ReleaseChannel::Dev;
    assert_eq!(dev.name(), "dev");
    assert!(dev.requirements().contains("Build and unit tests pass"));

    let nightly = ReleaseChannel::Nightly;
    assert_eq!(nightly.name(), "nightly");

    let validated = ReleaseChannel::Validated;
    assert_eq!(validated.name(), "validated");

    let experimental = ReleaseChannel::Experimental;
    assert_eq!(experimental.name(), "experimental");
}

#[test]
fn test_agent_roles() {
    let steward = AgentRole::RepositorySteward;
    assert_eq!(steward.title(), "Repository Steward");
    assert!(steward.scope().contains("Repository cleanliness"));

    let validator = AgentRole::BootAndRuntimeValidator;
    assert_eq!(validator.title(), "Boot and Runtime Validator");
    assert!(validator.scope().contains("QEMU boot matrix"));
}

#[test]
fn test_first_ten_tasks_generation() {
    let engine = DistroMaintainerOperatingModelPrEngine::new();
    let tasks = engine.get_first_ten_tasks();

    assert_eq!(tasks.len(), 10);

    let agent_task = &tasks[0];
    assert_eq!(agent_task.id, "AGENT-001");
    assert_eq!(agent_task.agent, "repository-steward");

    let toml = agent_task.to_toml();
    assert!(toml.contains("id = \"AGENT-001\""));
    assert!(toml.contains("title = \"Define agent registry and ownership boundaries\""));
    assert!(toml.contains("scope = ["));
    assert!(toml.contains("steps = ["));
    assert!(toml.contains("evidence = ["));
    assert!(toml.contains("success = ["));
    assert!(toml.contains("on_failure = \"Create blocked task with registry syntax error\""));
}

#[test]
fn test_human_approval_requirement() {
    let engine = DistroMaintainerOperatingModelPrEngine::new();

    assert!(engine.requires_human_approval("modify_kernel_security_boundary"));
    assert!(engine.requires_human_approval("update_bootloader_stage"));
    assert!(engine.requires_human_approval("execute_package_script"));
    assert!(engine.requires_human_approval("change_release_channel"));
    assert!(engine.requires_human_approval("force_push_branch"));

    assert!(!engine.requires_human_approval("run_unit_tests"));
    assert!(!engine.requires_human_approval("generate_report"));
    assert!(!engine.requires_human_approval("fix_formatting_typo"));
}
