// Comprehensive Unit Tests for SigmaOS System Utilities Plan Verification

use std::fs;
use std::path::Path;

#[path = "../src/userland/system_utilities.rs"]
mod system_utilities;

use system_utilities::{
    BackendDependency, CatEngine, CpEngine, EnvEngine, LsEngine, MkdirEngine, MutationBehavior,
    PeriodicRunnerEngine, PeriodicTaskGroup, PeriodicTaskRecord, PeriodicTaskResult,
    PrivilegeLevel, PwdEngine, RmEngine, SigmaDiagEngine, SystemUtilityArgParser,
    SystemUtilityContract, SystemUtilityRegistry, TouchEngine, UnameEngine, UtilityStatus,
    WcEngine, EXIT_COMMAND_NOT_FOUND, EXIT_FAILURE, EXIT_PERMISSION_DENIED, EXIT_SUCCESS,
    EXIT_SYNTAX_ERROR,
};

#[test]
fn test_system_utilities_documentation_plan_verification() {
    let docs_path = "docs/SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md";
    let wiki_path_1 = "wiki/SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md";
    let wiki_path_2 = "wiki/26-System-Utilities-Development-Plan.md";

    assert!(
        Path::new(docs_path).exists(),
        "docs/SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md must exist"
    );
    assert!(
        Path::new(wiki_path_1).exists(),
        "wiki/SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md must exist"
    );
    assert!(
        Path::new(wiki_path_2).exists(),
        "wiki/26-System-Utilities-Development-Plan.md must exist"
    );

    let docs_content = fs::read_to_string(docs_path).expect("Read docs plan");
    let wiki_content_1 = fs::read_to_string(wiki_path_1).expect("Read wiki plan 1");
    let wiki_content_2 = fs::read_to_string(wiki_path_2).expect("Read wiki plan 2");

    assert_eq!(
        docs_content, wiki_content_1,
        "Wiki plan 1 must match docs plan"
    );
    assert_eq!(
        docs_content, wiki_content_2,
        "Wiki plan 2 must match docs plan"
    );

    // Verify key sections and contracts exist in documentation
    assert!(docs_content.contains("SigmaOS System Utilities Development Plan"));
    assert!(docs_content.contains("Baseline Rules & Operating Principles"));
    assert!(docs_content.contains("Define each command’s contract"));
    assert!(docs_content.contains("Make tools composable"));
    assert!(docs_content.contains("Treat JSON as a separate interface"));
    assert!(docs_content.contains("Separate inspection from mutation"));
    assert!(docs_content.contains("Never claim success for an unavailable backend"));
    assert!(docs_content.contains("Test runtime behavior"));
    assert!(docs_content.contains("Keep periodic checks safe and bounded"));
    assert!(docs_content.contains("Use focused PRs and protected branches"));

    assert!(docs_content.contains("Initial High-Value Sequence Focus"));
    assert!(docs_content.contains("highest-value initial sequence is PRs 1–4, 11, and 15"));

    assert!(docs_content.contains("Proposed 18-PR Roadmap Sequence"));
    assert!(docs_content.contains("docs(utils): audit current tools and define compatibility policy"));
    assert!(docs_content.contains("docs(utils): add command registry and ownership map"));
    assert!(docs_content.contains("feat(utils): add shared CLI contract helpers"));
    assert!(docs_content.contains("feat(utils): implement minimal file and directory utilities"));
    assert!(docs_content.contains("feat(utils): implement safe file mutation utilities"));
    assert!(docs_content.contains("feat(utils): add stream and text-processing utilities"));
    assert!(docs_content.contains("feat(utils): add system identity and environment queries"));
    assert!(docs_content.contains("feat(utils): add process inspection and signal tools"));
    assert!(docs_content.contains("feat(utils): add storage and filesystem status tools"));
    assert!(docs_content.contains("feat(utils): add account and permission administration tools"));
    assert!(docs_content.contains("feat(utils): make system reports SigmaOS-runtime aware"));
    assert!(docs_content.contains("feat(periodic): add bounded maintenance-task runner"));
    assert!(docs_content.contains("feat(periodic): add initial read-only health checks"));
    assert!(docs_content.contains("docs(utils): add manual pages and executable examples"));
    assert!(docs_content.contains("test(utils): add command contract and booted-system suite"));
    assert!(docs_content.contains("security(utils): audit path, privilege, and data-handling risks"));
    assert!(docs_content.contains("ci(utils): validate registry, help, manuals, and status matrix"));
    assert!(docs_content.contains("release(utils): publish utility support matrix"));

    assert!(docs_content.contains("Periodic Maintenance PR Cadences"));
    assert!(docs_content.contains("ci(utils): run utility contract checks"));
    assert!(docs_content.contains("maint(utils): triage failures and stale status claims"));
    assert!(docs_content.contains("docs(utils): reconcile registry, help, and manuals"));
    assert!(docs_content.contains("test(utils): run core command behavior matrix"));
    assert!(docs_content.contains("security(utils): review privileged and destructive utilities"));
    assert!(docs_content.contains("maint(periodic): review maintenance-task results and retention"));
    assert!(docs_content.contains("docs(utils): review POSIX and Linux/BSD deviations"));
    assert!(docs_content.contains("test(utils): run clean-boot and upgrade smoke tests"));
    assert!(docs_content.contains("release(utils): certify shipped tools and maintenance checks"));
    assert!(docs_content.contains("fix(utils): prevent recurrence of <failure>"));

    assert!(docs_content.contains("Component Rules"));
    assert!(docs_content.contains("File utilities:"));
    assert!(docs_content.contains("Text utilities:"));
    assert!(docs_content.contains("Process utilities:"));
    assert!(docs_content.contains("Storage utilities:"));
    assert!(docs_content.contains("System reporting:"));
    assert!(docs_content.contains("Maintenance runner:"));
    assert!(docs_content.contains("Documentation:"));
    assert!(docs_content.contains("Testing:"));

    assert!(docs_content.contains("PR Review Template & Merge Gates"));
    assert!(docs_content.contains("Completion & Feature Status Gates"));
    assert!(docs_content.contains("Prototype:"));
    assert!(docs_content.contains("Integrated:"));
    assert!(docs_content.contains("Supported:"));

    // Verify FEATURE_STATUS.toml registration
    let feature_status = fs::read_to_string("FEATURE_STATUS.toml").expect("Read FEATURE_STATUS.toml");
    assert!(feature_status.contains("[userspace.system_utilities]"));
}

#[test]
fn test_system_utilities_registry_and_parser_contract_verification() {
    let registry = SystemUtilityRegistry::new();
    assert!(registry.total_registered_count() >= 35);
    assert!(registry.integrated_count() >= 12);

    let cat_cmd = registry.get_command("cat").expect("cat command registered");
    assert_eq!(cat_cmd.privilege, PrivilegeLevel::User);
    assert_eq!(cat_cmd.mutation, MutationBehavior::ReadOnly);
    assert_eq!(cat_cmd.backend, BackendDependency::Vfs);
    assert_eq!(cat_cmd.status, UtilityStatus::Integrated);
    assert_eq!(cat_cmd.owner, "SentinelTeam");
    assert_eq!(cat_cmd.manual_page, "man/cat.1");
    assert!(!cat_cmd.synopsis.is_empty());

    let rm_cmd = registry.get_command("rm").expect("rm command registered");
    assert_eq!(rm_cmd.mutation, MutationBehavior::MutatingDestructive);

    let diag_cmd = registry
        .get_command("sigma-diag")
        .expect("sigma-diag command registered");
    assert_eq!(diag_cmd.privilege, PrivilegeLevel::Admin);

    // Verify enums and contract coverage
    let custom_contract = SystemUtilityContract {
        name: "net-admin".to_string(),
        synopsis: "Manage network stack".to_string(),
        privilege: PrivilegeLevel::Root,
        mutation: MutationBehavior::MutatingSafe,
        backend: BackendDependency::NetworkService,
        owner: "NetTeam".to_string(),
        status: UtilityStatus::Prototype,
        manual_page: "man/net-admin.8".to_string(),
    };
    assert_eq!(custom_contract.privilege, PrivilegeLevel::Root);
    assert_eq!(custom_contract.backend, BackendDependency::NetworkService);
    assert_eq!(custom_contract.status, UtilityStatus::Prototype);
    assert_eq!(custom_contract.synopsis, "Manage network stack");
    assert_eq!(custom_contract.owner, "NetTeam");
    assert_eq!(custom_contract.manual_page, "man/net-admin.8");

    assert_eq!(EXIT_SUCCESS, 0);
    assert_eq!(EXIT_FAILURE, 1);
    assert_eq!(EXIT_SYNTAX_ERROR, 2);
    assert_eq!(EXIT_PERMISSION_DENIED, 126);
    assert_eq!(EXIT_COMMAND_NOT_FOUND, 127);

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

    let mkdir_out = MkdirEngine::run(&["/tmp/test_dir"], &parser);
    assert_eq!(mkdir_out.exit_status, EXIT_SUCCESS);

    let touch_out = TouchEngine::run(&["/tmp/test_dir/file.txt"], &parser);
    assert_eq!(touch_out.exit_status, EXIT_SUCCESS);

    let cp_out = CpEngine::run("/tmp/a", "/tmp/b", &parser);
    assert_eq!(cp_out.exit_status, EXIT_SUCCESS);

    let wc_out = WcEngine::run("line1\nline2\n", "test.txt", &parser);
    assert_eq!(wc_out.exit_status, EXIT_SUCCESS);

    let env_out = EnvEngine::run(&[("PATH", "/bin:/usr/bin")], &parser);
    assert_eq!(env_out.exit_status, EXIT_SUCCESS);

    let uname_out = UnameEngine::run("SigmaOS", "sigma-node", "1.0", "x86_64", &parser);
    assert_eq!(uname_out.exit_status, EXIT_SUCCESS);

    // Test Diagnostics Engine
    let diag_ok = SigmaDiagEngine::run(true, "qemu-target", 7200);
    assert_eq!(diag_ok.exit_status, EXIT_SUCCESS);
    assert!(diag_ok.stdout.contains("qemu-target"));

    // Test Periodic Maintenance Engine
    let periodic_daily = PeriodicRunnerEngine::run(Some(PeriodicTaskGroup::Daily), true);
    assert_eq!(periodic_daily.exit_status, EXIT_SUCCESS);
    assert!(periodic_daily.stdout.contains("failed_services_check"));

    let periodic_monthly_record = PeriodicTaskRecord {
        name: "900.quarterly_log_rotation".to_string(),
        group: PeriodicTaskGroup::Monthly,
        result: PeriodicTaskResult::Warning,
        summary: "Log size exceeds rotation soft-limit".to_string(),
    };
    assert_eq!(periodic_monthly_record.group, PeriodicTaskGroup::Monthly);
    assert_eq!(periodic_monthly_record.result, PeriodicTaskResult::Warning);

    let bad_config_record = PeriodicTaskRecord {
        name: "901.cron_syntax_check".to_string(),
        group: PeriodicTaskGroup::Daily,
        result: PeriodicTaskResult::BadConfig,
        summary: "Invalid crontab syntax detected".to_string(),
    };
    assert_eq!(bad_config_record.result, PeriodicTaskResult::BadConfig);

    let exec_fail_record = PeriodicTaskRecord {
        name: "902.kernel_core_dump_scrub".to_string(),
        group: PeriodicTaskGroup::Weekly,
        result: PeriodicTaskResult::ExecutionFailure,
        summary: "Execution timed out after 30s".to_string(),
    };
    assert_eq!(exec_fail_record.result, PeriodicTaskResult::ExecutionFailure);
}
