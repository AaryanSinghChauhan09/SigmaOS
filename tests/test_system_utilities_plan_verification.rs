// Comprehensive Unit Tests for SigmaOS System Utilities Plan Verification

use std::fs;
use std::path::Path;

#[path = "../src/userland/system_utilities.rs"]
mod system_utilities;

use system_utilities::{
    CatEngine, CpEngine, EnvEngine, LsEngine, MkdirEngine, MutationBehavior, PeriodicRunnerEngine,
    PeriodicTaskGroup, PrivilegeLevel, PwdEngine, RmEngine, SigmaDiagEngine, SystemUtilityArgParser,
    SystemUtilityRegistry, TouchEngine, UnameEngine, UtilityStatus, WcEngine, EXIT_PERMISSION_DENIED,
    EXIT_SUCCESS,
};

#[test]
fn test_system_utilities_documentation_plan_verification() {
    let docs_path = "docs/SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md";
    let wiki_path_1 = "wiki/SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md";
    let wiki_path_2 = "wiki/26-System-Utilities-Development-Plan.md";

    assert!(Path::new(docs_path).exists(), "docs/SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md must exist");
    assert!(Path::new(wiki_path_1).exists(), "wiki/SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md must exist");
    assert!(Path::new(wiki_path_2).exists(), "wiki/26-System-Utilities-Development-Plan.md must exist");

    let docs_content = fs::read_to_string(docs_path).expect("Read docs plan");
    let wiki_content_1 = fs::read_to_string(wiki_path_1).expect("Read wiki plan 1");
    let wiki_content_2 = fs::read_to_string(wiki_path_2).expect("Read wiki plan 2");

    assert_eq!(docs_content, wiki_content_1, "Wiki plan 1 must match docs plan");
    assert_eq!(docs_content, wiki_content_2, "Wiki plan 2 must match docs plan");

    // Verify key sections and contracts exist in documentation
    assert!(docs_content.contains("SigmaOS System Utilities Development Plan"));
    assert!(docs_content.contains("Baseline Rules & Operating Principles"));
    assert!(docs_content.contains("Proposed 18-PR Roadmap Sequence"));
    assert!(docs_content.contains("Initial High-Value Sequence Focus"));
    assert!(docs_content.contains("Periodic Maintenance PR Cadences"));
    assert!(docs_content.contains("PR Review Template & Merge Gates"));
    assert!(docs_content.contains("Completion & Feature Status Gates"));
}

#[test]
fn test_system_utilities_registry_and_parser_contract_verification() {
    let registry = SystemUtilityRegistry::new();
    assert!(registry.total_registered_count() >= 35);
    assert!(registry.integrated_count() >= 12);

    let cat_cmd = registry.get_command("cat").expect("cat command registered");
    assert_eq!(cat_cmd.privilege, PrivilegeLevel::User);
    assert_eq!(cat_cmd.mutation, MutationBehavior::ReadOnly);

    let rm_cmd = registry.get_command("rm").expect("rm command registered");
    assert_eq!(rm_cmd.mutation, MutationBehavior::MutatingDestructive);

    let diag_cmd = registry.get_command("sigma-diag").expect("sigma-diag command registered");
    assert_eq!(diag_cmd.privilege, PrivilegeLevel::Admin);

    // Test Argument Parser POSIX compliance
    let args = vec!["-l", "-a", "--json", "--", "-not-a-flag", "file.txt"];
    let parser = SystemUtilityArgParser::parse(args);
    assert!(parser.has_flag("l"));
    assert!(parser.has_flag("a"));
    assert!(parser.format_json);
    assert!(parser.saw_delimiter);
    assert_eq!(parser.operands, vec!["-not-a-flag", "file.txt"]);

    // Test File Engines
    let pwd_output = PwdEngine::run("/root/workspace", &parser);
    assert_eq!(pwd_output.exit_status, EXIT_SUCCESS);
    assert!(pwd_output.stdout.contains("/root/workspace"));

    let cat_output = CatEngine::run(&[("/etc/version", Some("SigmaOS 1.0\n"))], &parser);
    assert_eq!(cat_output.exit_status, EXIT_SUCCESS);
    assert!(cat_output.stdout.contains("SigmaOS 1.0"));

    let ls_output = LsEngine::run("/tmp", &[("a.txt", false, 100)], &parser);
    assert_eq!(ls_output.exit_status, EXIT_SUCCESS);
    assert!(ls_output.stdout.contains("a.txt"));

    let rm_root = RmEngine::run(&["/"], &parser);
    assert_eq!(rm_root.exit_status, EXIT_PERMISSION_DENIED);

    // Test Diagnostics Engine
    let diag_ok = SigmaDiagEngine::run(true, "qemu-target", 7200);
    assert_eq!(diag_ok.exit_status, EXIT_SUCCESS);
    assert!(diag_ok.stdout.contains("qemu-target"));

    // Test Periodic Maintenance Engine
    let periodic_daily = PeriodicRunnerEngine::run(Some(PeriodicTaskGroup::Daily), true);
    assert_eq!(periodic_daily.exit_status, EXIT_SUCCESS);
    assert!(periodic_daily.stdout.contains("failed_services_check"));
}
