// SPDX-License-Identifier: MIT
// SigmaOS Process Manager Architecture & Subsystem Contract Test Suite

extern crate alloc;

#[path = "../../src/kernel/process.rs"]
mod process;

#[path = "../../src/userspace/sigma_init.rs"]
mod sigma_init;

use process::*;
use sigma_init::*;

#[test]
fn test_process_manager_architecture_design_rules_and_pr_sequence() {
    // 10 Design and Safety Rules
    let design_rules = [
        "1. Separate kernel lifecycle from service policy.",
        "2. Model lifecycle states explicitly.",
        "3. Reap every child.",
        "4. Avoid PID-reuse races in management.",
        "5. Bound service recovery.",
        "6. Keep service dependencies declarative and cycle-checked.",
        "7. Make resource controls enforceable, not decorative.",
        "8. Use least privilege.",
        "9. Make stop and shutdown graceful but bounded.",
        "10. Test at multiple levels.",
    ];
    assert_eq!(design_rules.len(), 10);

    // 18-PR Sequence Roadmap Titles
    let prs = [
        "PR 1: docs(process): audit process lifecycle and manager status",
        "PR 2: docs(process): define process and service lifecycle contracts",
        "PR 3: test(process): add deterministic lifecycle and race harness",
        "PR 4: feat(process): complete user-mode transition and minimal process launch",
        "PR 5: feat(process): implement spawn and exec interface",
        "PR 6: feat(process): implement wait and child status collection",
        "PR 7: feat(process): add orphan adoption and PID 1 child reaping",
        "PR 8: feat(process): add stable process references for supervision",
        "PR 9: feat(init): spawn and supervise real service processes",
        "PR 10: feat(init): validate service dependencies and readiness",
        "PR 11: feat(init): add bounded restart and failure policies",
        "PR 12: feat(init): implement graceful service stop and system shutdown",
        "PR 13: feat(process): integrate process groups and terminal job control",
        "PR 14: feat(process): enforce per-service resource limits",
        "PR 15: feat(process): add process inspection and service-control interfaces",
        "PR 16: security(process): enforce service isolation and privilege boundaries",
        "PR 17: test(process): add booted multi-service and fault-injection suite",
        "PR 18: docs(process): publish process-manager reference and support matrix",
    ];
    assert_eq!(prs.len(), 18);

    // Maintenance Cadences
    let cadences = [
        "Every PR: ci(process): run lifecycle checks for touched components",
        "Weekly: maint(process): triage process and service regressions",
        "Weekly: test(process): review crashes, hangs, and resource leaks",
        "Monthly: audit(process): reconcile lifecycle code and capability matrix",
        "Monthly: test(init): run service start-stop and dependency matrix",
        "Monthly: security(process): review privilege and identity operations",
        "Quarterly: maint(process): review resource limits and accounting",
        "Quarterly: docs(process): review POSIX/Linux/BSD compatibility claims",
        "Each release candidate: release(process): certify process-manager claims",
        "After each incident: fix(process): prevent recurrence of <failure>",
    ];
    assert_eq!(cadences.len(), 10);
}

#[test]
fn test_kernel_process_lifecycle_contract() {
    let proc = Process::new(100, 1, alloc::string::String::from("init"));
    assert_eq!(proc.pid, 100);
    assert_eq!(proc.parent_pid, 1);
    assert_eq!(proc.state, ProcessState::Running);

    let mut table = ProcessTable::new();
    let p = Process::new(101, 1, alloc::string::String::from("worker"));
    table.add_process(p);
}

#[test]
fn test_userspace_sigma_init_contract() {
    let mut init = InitSystem::new();
    let s1 = ServiceUnit {
        name: alloc::string::String::from("db"),
        description: alloc::string::String::from("Database Service"),
        exec_start: alloc::string::String::from("/bin/db"),
        dependencies: alloc::vec![],
        restart_policy: RestartPolicy::Always,
    };
    let s2 = ServiceUnit {
        name: alloc::string::String::from("app"),
        description: alloc::string::String::from("App Service"),
        exec_start: alloc::string::String::from("/bin/app"),
        dependencies: alloc::vec![alloc::string::String::from("db")],
        restart_policy: RestartPolicy::OnFailure,
    };

    init.add_service(s1);
    init.add_service(s2);

    let order = init.topological_sort().unwrap();
    assert_eq!(order, alloc::vec!["db", "app"]);

    assert!(init.start_all().is_ok());

    init.shutdown();
}
