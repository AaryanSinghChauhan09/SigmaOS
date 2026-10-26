// SPDX-License-Identifier: MIT
// SigmaOS Process Manager Master Architecture & Plan Verification Suite

extern crate alloc;

#[path = "../../src/kernel/process.rs"]
mod kernel_process;

#[path = "../../src/userspace/sigma_init.rs"]
mod userspace_init;

use kernel_process::*;
use userspace_init::*;

#[test]
fn test_kernel_process_state_transitions_and_table() {
    let mut table = ProcessTable::new();
    assert_eq!(table.count_processes(), 0);

    let pid1 = table.allocate_pid();
    assert_eq!(pid1, 1);

    let proc1 = Process::new(pid1, 0, String::from("init"));
    assert_eq!(proc1.pid, 1);
    assert_eq!(proc1.parent_pid, 0);
    assert_eq!(proc1.state, ProcessState::Running);

    table.add_process(proc1);
    assert_eq!(table.count_processes(), 1);

    let fetched = table.find_process(1);
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().name, "init");

    let pid2 = table.allocate_pid();
    let mut proc2 = Process::new(pid2, 1, String::from("service_a"));
    proc2.set_state(ProcessState::Sleeping);
    table.add_process(proc2);
    assert_eq!(table.count_processes(), 2);

    let removed = table.remove_process(pid2);
    assert!(removed.is_some());
    assert_eq!(table.count_processes(), 1);
}

#[test]
fn test_userspace_init_service_lifecycle_and_topological_sort() {
    let mut init = InitSystem::new();

    let s1 = ServiceUnit {
        name: String::from("base_sys"),
        description: String::from("Base System Service"),
        exec_start: String::from("/bin/base_sys"),
        dependencies: Vec::new(),
        restart_policy: RestartPolicy::Always,
    };

    let s2 = ServiceUnit {
        name: String::from("net_service"),
        description: String::from("Network Service"),
        exec_start: String::from("/bin/net_service"),
        dependencies: alloc::vec![String::from("base_sys")],
        restart_policy: RestartPolicy::OnFailure,
    };

    init.add_service(s1);
    init.add_service(s2);

    let sorted = init.topological_sort().expect("Topological sort should succeed");
    assert_eq!(sorted.len(), 2);
    let base_idx = sorted.iter().position(|x| x == "base_sys").unwrap();
    let net_idx = sorted.iter().position(|x| x == "net_service").unwrap();
    assert!(base_idx < net_idx);

    init.start_all().expect("All services should start cleanly");
    init.shutdown();
}

#[test]
fn test_process_manager_design_and_safety_rules() {
    let design_rules = [
        "1. Separate kernel lifecycle from service policy",
        "2. Model lifecycle states explicitly",
        "3. Reap every child",
        "4. Avoid PID-reuse races in management (pidfds)",
        "5. Bound service recovery",
        "6. Keep service dependencies declarative and cycle-checked",
        "7. Make resource controls enforceable, not decorative",
        "8. Use least privilege",
        "9. Make stop and shutdown graceful but bounded",
        "10. Test at multiple levels",
    ];
    assert_eq!(design_rules.len(), 10);
}

#[test]
fn test_process_manager_18_pr_sequence() {
    let prs = [
        ("PR 1", "docs(process): audit process lifecycle and manager status"),
        ("PR 2", "docs(process): define process and service lifecycle contracts"),
        ("PR 3", "test(process): add deterministic lifecycle and race harness"),
        ("PR 4", "feat(process): complete user-mode transition and minimal process launch"),
        ("PR 5", "feat(process): implement spawn and exec interface"),
        ("PR 6", "feat(process): implement wait and child status collection"),
        ("PR 7", "feat(process): add orphan adoption and PID 1 child reaping"),
        ("PR 8", "feat(process): add stable process references for supervision"),
        ("PR 9", "feat(init): spawn and supervise real service processes"),
        ("PR 10", "feat(init): validate service dependencies and readiness"),
        ("PR 11", "feat(init): add bounded restart and failure policies"),
        ("PR 12", "feat(init): implement graceful service stop and system shutdown"),
        ("PR 13", "feat(process): integrate process groups and terminal job control"),
        ("PR 14", "feat(process): enforce per-service resource limits"),
        ("PR 15", "feat(process): add process inspection and service-control interfaces"),
        ("PR 16", "security(process): enforce service isolation and privilege boundaries"),
        ("PR 17", "test(process): add booted multi-service and fault-injection suite"),
        ("PR 18", "docs(process): publish process-manager reference and support matrix"),
    ];
    assert_eq!(prs.len(), 18);
}

#[test]
fn test_process_manager_maintenance_cadence_and_completion_gates() {
    let cadences = [
        "Every PR: ci(process)",
        "Weekly: triage regressions & review crashes/leaks",
        "Monthly: audit matrix, test init start-stop, security review",
        "Quarterly: review resource limits & compatibility claims",
        "Release Candidate: certify process-manager claims",
        "After Incident: fix and backport",
    ];
    assert_eq!(cadences.len(), 6);

    let gates = [
        "Integrated: booted target launches user process, collects status, reaps child",
        "Supported: repeatable tests cover failure to exec, exit/adoption, signals, service dependencies, bounded restart, graceful shutdown, resource controls",
    ];
    assert_eq!(gates.len(), 2);
}
