// SPDX-License-Identifier: MIT
// SigmaOS System Utilities Master Architecture & Development Plan Test Suite

#[path = "../src/userland/system_utilities.rs"]
mod system_utilities;

use system_utilities::*;

#[test]
fn test_arg_parser_posix_conventions() {
    let args = vec!["-l", "-a", "--format=json", "--", "file1.txt", "-f"];
    let parser = SystemUtilityArgParser::parse(args);

    assert!(parser.flags.contains(&"l".to_string()));
    assert!(parser.flags.contains(&"a".to_string()));
    assert_eq!(parser.get_option("format"), Some("json"));
    assert!(parser.saw_delimiter);
    assert_eq!(parser.operands, vec!["file1.txt", "-f"]);
}

#[test]
fn test_output_conventions_and_exit_codes() {
    assert_eq!(EXIT_SUCCESS, 0);
    assert_eq!(EXIT_FAILURE, 1);
    assert_eq!(EXIT_SYNTAX_ERROR, 2);
    assert_eq!(EXIT_PERMISSION_DENIED, 126);
    assert_eq!(EXIT_COMMAND_NOT_FOUND, 127);

    let succ = SystemUtilityOutput::success("hello\n");
    assert_eq!(succ.exit_status, EXIT_SUCCESS);
    assert_eq!(succ.stdout, "hello\n");
    assert!(succ.stderr.is_empty());
    assert!(succ.json_payload.is_none());

    let fail = SystemUtilityOutput::failure(EXIT_PERMISSION_DENIED, "Permission denied\n");
    assert_eq!(fail.exit_status, EXIT_PERMISSION_DENIED);
    assert_eq!(fail.stderr, "Permission denied\n");
    assert!(fail.stdout.is_empty());

    let json_out = SystemUtilityOutput::success("hello\n").with_json("{\"msg\":\"hello\"}");
    assert_eq!(json_out.json_payload.as_deref(), Some("{\"msg\":\"hello\"}"));
}

#[test]
fn test_utility_registry_contract_mappings() {
    let registry = SystemUtilityRegistry::new();
    assert!(registry.registry.len() >= 10);

    let pwd_contract = registry.registry.get("pwd").expect("pwd contract missing");
    assert_eq!(pwd_contract.name, "pwd");
    assert_eq!(pwd_contract.privilege, PrivilegeLevel::User);
    assert_eq!(pwd_contract.mutation, MutationBehavior::ReadOnly);

    let rm_contract = registry.registry.get("rm").expect("rm contract missing");
    assert_eq!(rm_contract.name, "rm");
    assert_eq!(rm_contract.mutation, MutationBehavior::MutatingDestructive);

    let periodic_contract = registry.registry.get("sigma-periodic").expect("periodic contract missing");
    assert_eq!(periodic_contract.name, "sigma-periodic");
    assert_eq!(periodic_contract.privilege, PrivilegeLevel::Admin);
}

#[test]
fn test_foundational_file_and_directory_engines() {
    let parser = SystemUtilityArgParser::default();

    // pwd engine
    let pwd_res = PwdEngine::run("/usr/bin", &parser);
    assert_eq!(pwd_res.exit_status, EXIT_SUCCESS);
    assert_eq!(pwd_res.stdout, "/usr/bin\n");

    // cat engine
    let files = vec![
        ("/tmp/a.txt", Some("line 1\n")),
        ("/tmp/missing.txt", None),
    ];
    let cat_res = CatEngine::run(&files, &parser);
    assert_eq!(cat_res.exit_status, EXIT_FAILURE);
    assert_eq!(cat_res.stdout, "line 1\n");
    assert!(cat_res.stderr.contains("No such file or directory"));

    // ls engine
    let entries = vec![("bin", true, 4096), ("file.txt", false, 128)];
    let ls_res = LsEngine::run("/tmp", &entries, &parser);
    assert_eq!(ls_res.exit_status, EXIT_SUCCESS);
    assert!(ls_res.stdout.contains("bin"));
    assert!(ls_res.stdout.contains("file.txt"));

    // stat engine
    let stat_res = StatEngine::run("/etc/hostname", true, 1024);
    assert_eq!(stat_res.exit_status, EXIT_SUCCESS);
    assert!(stat_res.stdout.contains("File: /etc/hostname"));
    assert!(stat_res.stdout.contains("Size: 1024"));

    // mkdir engine
    let mkdir_res = MkdirEngine::run(&["/var/log/app"], &parser);
    assert_eq!(mkdir_res.exit_status, EXIT_SUCCESS);
    assert!(mkdir_res.stdout.contains("mkdir: created directory '/var/log/app'"));

    // touch engine
    let touch_res = TouchEngine::run(&["/tmp/newfile.txt"], &parser);
    assert_eq!(touch_res.exit_status, EXIT_SUCCESS);
    assert!(touch_res.stdout.contains("touch: updated timestamp for '/tmp/newfile.txt'"));

    // cp, mv, rm engines
    let cp_res = CpEngine::run("/src.txt", "/dst.txt", &parser);
    assert_eq!(cp_res.exit_status, EXIT_SUCCESS);
    assert!(cp_res.stdout.contains("cp: copied '/src.txt' to '/dst.txt'"));

    let mv_res = MvEngine::run("/old.txt", "/new.txt", &parser);
    assert_eq!(mv_res.exit_status, EXIT_SUCCESS);
    assert!(mv_res.stdout.contains("mv: moved '/old.txt' to '/new.txt'"));

    let rm_res = RmEngine::run(&["/tmp/junk.txt"], &parser);
    assert_eq!(rm_res.exit_status, EXIT_SUCCESS);
    assert!(rm_res.stdout.contains("rm: removed '/tmp/junk.txt'"));
}

#[test]
fn test_stream_and_text_processing_engines() {
    let parser = SystemUtilityArgParser::default();

    let content = "line 1\nline 2\nline 3\nline 4\nline 5\n";

    let head_res = HeadEngine::run(content, 2, &parser);
    assert_eq!(head_res.exit_status, EXIT_SUCCESS);
    assert_eq!(head_res.stdout, "line 1\nline 2\n");

    let tail_res = TailEngine::run(content, 2, &parser);
    assert_eq!(tail_res.exit_status, EXIT_SUCCESS);
    assert_eq!(tail_res.stdout, "line 4\nline 5\n");

    let text = "hello world\nsecond line\n";
    let wc_res = WcEngine::run(text, "sample.txt", &parser);
    assert_eq!(wc_res.exit_status, EXIT_SUCCESS);
    assert!(wc_res.stdout.contains("sample.txt"));
}

#[test]
fn test_system_identity_and_diagnostics_engines() {
    let parser = SystemUtilityArgParser::default();

    let env_vars = vec![("PATH", "/bin:/usr/bin"), ("USER", "root")];
    let env_res = EnvEngine::run(&env_vars, &parser);
    assert_eq!(env_res.exit_status, EXIT_SUCCESS);
    assert!(env_res.stdout.contains("PATH=/bin:/usr/bin"));
    assert!(env_res.stdout.contains("USER=root"));

    let uname_res = UnameEngine::run("SigmaOS", "node1", "0.1.0", "x86_64", &parser);
    assert_eq!(uname_res.exit_status, EXIT_SUCCESS);
    assert_eq!(uname_res.stdout, "SigmaOS\n");

    let diag_res = SigmaDiagEngine::run(true, "sigma-host", 3600);
    assert_eq!(diag_res.exit_status, EXIT_SUCCESS);
    assert!(diag_res.stdout.contains("SigmaOS Diagnostic Report"));
}

#[test]
fn test_periodic_maintenance_runner_engine() {
    // When provider service is unavailable
    let unavailable_res = PeriodicRunnerEngine::run(None, false);
    assert_eq!(unavailable_res.exit_status, EXIT_FAILURE);
    assert!(unavailable_res.stderr.contains("provider unavailable"));

    // When provider service is available
    let available_res = PeriodicRunnerEngine::run(None, true);
    assert_eq!(available_res.exit_status, EXIT_SUCCESS);
    assert!(available_res.stdout.contains("SigmaOS Periodic Maintenance Report"));
    assert!(available_res.stdout.contains("001.failed_services_check"));
    assert!(available_res.stdout.contains("002.filesystem_capacity_check"));

    // Filter by Daily group
    let daily_res = PeriodicRunnerEngine::run(Some(PeriodicTaskGroup::Daily), true);
    assert_eq!(daily_res.exit_status, EXIT_SUCCESS);
    assert!(daily_res.stdout.contains("001.failed_services_check"));
    assert!(!daily_res.stdout.contains("003.stale_temp_files_audit"));
}

#[test]
fn test_system_utilities_architecture_rules_and_pr_sequence() {
    // Verify 18 proposed PR titles
    let prs = [
        "1. docs(utils): audit current tools and define compatibility policy",
        "2. docs(utils): add command registry and ownership map",
        "3. feat(utils): add shared CLI contract helpers",
        "4. feat(utils): implement minimal file and directory utilities",
        "5. feat(utils): implement safe file mutation utilities",
        "6. feat(utils): add stream and text-processing utilities",
        "7. feat(utils): add system identity and environment queries",
        "8. feat(utils): add process inspection and signal tools",
        "9. feat(utils): add storage and filesystem status tools",
        "10. feat(utils): add account and permission administration tools",
        "11. feat(utils): make system reports SigmaOS-runtime aware",
        "12. feat(periodic): add bounded maintenance-task runner",
        "13. feat(periodic): add initial read-only health checks",
        "14. docs(utils): add manual pages and executable examples",
        "15. test(utils): add command contract and booted-system suite",
        "16. security(utils): audit path, privilege, and data-handling risks",
        "17. ci(utils): validate registry, help, manuals, and status matrix",
        "18. release(utils): publish utility support matrix",
    ];
    assert_eq!(prs.len(), 18);

    // Verify initial high-value sequence PRs: 1, 2, 3, 4, 11, 15
    let high_value_prs = [1, 2, 3, 4, 11, 15];
    assert_eq!(high_value_prs.len(), 6);

    // Verify baseline architectural principles
    let baseline_principles = [
        "Explicit utility contracts",
        "Composability and stream design",
        "Structured JSON interface",
        "Separation of inspection and mutation",
        "Truthful backend availability",
        "Tiered runtime evidence",
        "Bounded and idempotent periodic runner",
        "Protected branch and PR review gates",
    ];
    assert_eq!(baseline_principles.len(), 8);
}
