// Verification test suite for SigmaOS Thread Manager Master Plan,
// Governance, Cadences, and Component Rules

use std::fs;

#[test]
fn test_thread_manager_documentation_parity() {
    let docs_plan = fs::read_to_string("docs/thread-manager.md")
        .expect("docs/thread-manager.md must exist");
    let wiki_plan = fs::read_to_string("wiki/thread-manager.md")
        .expect("wiki/thread-manager.md must exist");

    assert!(docs_plan.contains("SigmaOS Thread Manager Master Architecture, Support Matrix, and Program Plan"));
    assert!(docs_plan.contains("Thread Lifecycle States"));
    assert!(docs_plan.contains("`New`"));
    assert!(docs_plan.contains("`Ready`"));
    assert!(docs_plan.contains("`Running`"));
    assert!(docs_plan.contains("`Blocked`"));
    assert!(docs_plan.contains("`Suspended`"));
    assert!(docs_plan.contains("`Zombie`"));
    assert!(docs_plan.contains("`Terminated`"));

    assert!(docs_plan.contains("Engineering Cadence"));
    assert!(docs_plan.contains("Development Phases"));
    assert!(docs_plan.contains("Phase 1: Define the Contract"));
    assert!(docs_plan.contains("Phase 2: Core Thread Object"));
    assert!(docs_plan.contains("Phase 3: Scheduler Integration"));
    assert!(docs_plan.contains("Phase 4: POSIX-Facing Behavior"));

    assert!(docs_plan.contains("Missing Components Deliverable Matrix"));
    assert!(docs_plan.contains("First 10 Suggested PR Sequence"));

    assert_eq!(
        docs_plan, wiki_plan,
        "docs/thread-manager.md and wiki/thread-manager.md must be byte-for-byte identical"
    );
}

#[test]
fn test_governance_and_component_rules_presence() {
    let contributing = fs::read_to_string("CONTRIBUTING.md")
        .expect("CONTRIBUTING.md must exist");
    let agents = fs::read_to_string("AGENTS.md")
        .expect("AGENTS.md must exist");

    for content in &[&contributing, &agents] {
        assert!(content.contains("Engineering Cadence"));
        assert!(content.contains("Daily"));
        assert!(content.contains("Weekly"));
        assert!(content.contains("Biweekly"));
        assert!(content.contains("Monthly"));
        assert!(content.contains("Quarterly"));

        assert!(content.contains("Code Rules"));
        assert!(content.contains("Testing Rules"));
        assert!(content.contains("Documentation Rules"));
        assert!(content.contains("Security Rules"));

        assert!(content.contains("cargo fmt --check"));
        assert!(content.contains("cargo clippy --all-targets -- -D warnings"));
        assert!(content.contains("cargo check --lib"));
        assert!(content.contains("cargo test --lib"));
        assert!(content.contains("./run_sigma_tests.sh"));
    }
}

#[test]
fn test_pull_request_template_structure() {
    let template = fs::read_to_string(".github/PULL_REQUEST_TEMPLATE.md")
        .expect(".github/PULL_REQUEST_TEMPLATE.md must exist");

    assert!(template.contains("## Summary"));
    assert!(template.contains("## Component"));
    assert!(template.contains("- [ ] Thread manager"));
    assert!(template.contains("- [ ] Scheduler"));
    assert!(template.contains("- [ ] Process management"));
    assert!(template.contains("## Motivation"));
    assert!(template.contains("## Implementation notes"));
    assert!(template.contains("## Tests"));
    assert!(template.contains("## Verification"));
    assert!(template.contains("cargo check --lib"));
    assert!(template.contains("cargo test --lib"));
    assert!(template.contains("./run_sigma_tests.sh"));
    assert!(template.contains("## Capability status"));
    assert!(template.contains("## Integration impact"));
    assert!(template.contains("## Reviewer checklist"));
    assert!(template.contains("- [ ] Scope is narrow"));
    assert!(template.contains("- [ ] `unsafe` justified"));
}

#[test]
fn test_feature_status_thread_manager_entry() {
    let status_toml = fs::read_to_string("FEATURE_STATUS.toml")
        .expect("FEATURE_STATUS.toml must exist");

    assert!(status_toml.contains("[kernel.thread_manager]"));
    assert!(status_toml.contains("status = \"unit-tested\""));
    assert!(status_toml.contains("docs/thread-manager.md"));
    assert!(status_toml.contains("CONTRIBUTING.md"));
    assert!(status_toml.contains("AGENTS.md"));
    assert!(status_toml.contains(".github/PULL_REQUEST_TEMPLATE.md"));
}
