//! Integration tests — service start / stop / restart lifecycle.
//!
//! These tests exercise `ServiceManager` directly (not via InitSystem)
//! to verify the service state machine in isolation.

use sigmaos_system::service::{
    RestartPolicy, ServiceConfig, ServiceError, ServiceManager, ServiceState,
};

// ── helpers ───────────────────────────────────────────────────────────────────

fn manager_with(services: &[&str]) -> ServiceManager {
    let mut mgr = ServiceManager::new();
    for name in services {
        mgr.register(ServiceConfig::new(*name, format!("/usr/bin/{}", name)))
            .unwrap();
    }
    mgr
}

// ── registration tests ────────────────────────────────────────────────────────

#[test]
fn register_adds_service() {
    let mgr = manager_with(&["ssh"]);
    assert!(mgr.get("ssh").is_some());
}

#[test]
fn duplicate_registration_returns_error() {
    let mut mgr = manager_with(&["ssh"]);
    let err = mgr
        .register(ServiceConfig::new("ssh", "/usr/bin/sshd"))
        .unwrap_err();
    assert!(matches!(err, ServiceError::AlreadyRegistered(_)));
}

#[test]
fn new_service_is_stopped() {
    let mgr = manager_with(&["ssh"]);
    assert_eq!(mgr.get("ssh").unwrap().state, ServiceState::Stopped);
}

// ── start tests ───────────────────────────────────────────────────────────────

#[test]
fn start_transitions_to_running() {
    let mut mgr = manager_with(&["ssh"]);
    mgr.start("ssh").unwrap();
    assert_eq!(mgr.get("ssh").unwrap().state, ServiceState::Running);
}

#[test]
fn start_assigns_pid() {
    let mut mgr = manager_with(&["ssh"]);
    mgr.start("ssh").unwrap();
    assert!(mgr.get("ssh").unwrap().pid.is_some());
}

#[test]
fn start_nonexistent_service_returns_error() {
    let mut mgr = ServiceManager::new();
    let err = mgr.start("ghost").unwrap_err();
    assert!(matches!(err, ServiceError::NotFound(_)));
}

#[test]
fn start_already_running_returns_error() {
    let mut mgr = manager_with(&["ssh"]);
    mgr.start("ssh").unwrap();
    let err = mgr.start("ssh").unwrap_err();
    assert!(matches!(err, ServiceError::AlreadyRunning(_)));
}

#[test]
fn start_blocked_by_unstarted_dependency() {
    let mut mgr = ServiceManager::new();
    mgr.register(ServiceConfig::new("network", "/bin/network"))
        .unwrap();
    mgr.register(
        ServiceConfig::new("ssh", "/usr/bin/sshd").depends_on("network"),
    )
    .unwrap();

    // network is stopped → ssh must not start
    let err = mgr.start("ssh").unwrap_err();
    assert!(matches!(err, ServiceError::DependencyNotRunning(_, _)));
}

#[test]
fn start_succeeds_after_dependency_is_running() {
    let mut mgr = ServiceManager::new();
    mgr.register(ServiceConfig::new("network", "/bin/network"))
        .unwrap();
    mgr.register(
        ServiceConfig::new("ssh", "/usr/bin/sshd").depends_on("network"),
    )
    .unwrap();

    mgr.start("network").unwrap();
    mgr.start("ssh").unwrap();

    assert_eq!(mgr.get("ssh").unwrap().state, ServiceState::Running);
}

// ── stop tests ────────────────────────────────────────────────────────────────

#[test]
fn stop_running_service_transitions_to_stopped() {
    let mut mgr = manager_with(&["ssh"]);
    mgr.start("ssh").unwrap();
    mgr.stop("ssh").unwrap();
    assert_eq!(mgr.get("ssh").unwrap().state, ServiceState::Stopped);
}

#[test]
fn stop_clears_pid() {
    let mut mgr = manager_with(&["ssh"]);
    mgr.start("ssh").unwrap();
    mgr.stop("ssh").unwrap();
    assert!(mgr.get("ssh").unwrap().pid.is_none());
}

#[test]
fn stop_already_stopped_returns_error() {
    let mut mgr = manager_with(&["ssh"]);
    let err = mgr.stop("ssh").unwrap_err();
    assert!(matches!(err, ServiceError::NotRunning(_)));
}

#[test]
fn stop_nonexistent_service_returns_error() {
    let mut mgr = ServiceManager::new();
    let err = mgr.stop("ghost").unwrap_err();
    assert!(matches!(err, ServiceError::NotFound(_)));
}

// ── restart tests ─────────────────────────────────────────────────────────────

#[test]
fn restart_running_service_increments_restart_count() {
    let mut mgr = manager_with(&["ssh"]);
    mgr.start("ssh").unwrap();
    mgr.restart("ssh").unwrap();
    assert_eq!(mgr.get("ssh").unwrap().restart_count, 1);
}

#[test]
fn restart_leaves_service_running() {
    let mut mgr = manager_with(&["ssh"]);
    mgr.start("ssh").unwrap();
    mgr.restart("ssh").unwrap();
    assert_eq!(mgr.get("ssh").unwrap().state, ServiceState::Running);
}

#[test]
fn restart_reassigns_pid() {
    let mut mgr = manager_with(&["ssh"]);
    mgr.start("ssh").unwrap();
    let old_pid = mgr.get("ssh").unwrap().pid;
    mgr.restart("ssh").unwrap();
    let new_pid = mgr.get("ssh").unwrap().pid;
    // PIDs are monotonically increasing stubs; new > old
    assert!(new_pid > old_pid);
}

#[test]
fn multiple_restarts_accumulate_count() {
    let mut mgr = manager_with(&["ssh"]);
    mgr.start("ssh").unwrap();
    for _ in 0..3 {
        mgr.restart("ssh").unwrap();
    }
    assert_eq!(mgr.get("ssh").unwrap().restart_count, 3);
}

// ── status / query tests ──────────────────────────────────────────────────────

#[test]
fn status_all_returns_all_services() {
    let mgr = manager_with(&["a", "b", "c"]);
    assert_eq!(mgr.status_all().len(), 3);
}

#[test]
fn status_all_sorted_by_name() {
    let mgr = manager_with(&["zebra", "alpha", "mango"]);
    let names: Vec<_> = mgr.status_all().iter().map(|s| s.name.clone()).collect();
    assert_eq!(names, vec!["alpha", "mango", "zebra"]);
}

#[test]
fn restart_policy_persists_after_start_stop() {
    let mut mgr = ServiceManager::new();
    mgr.register(
        ServiceConfig::new("daemon", "/usr/bin/daemon").restart(RestartPolicy::Always),
    )
    .unwrap();
    mgr.start("daemon").unwrap();
    mgr.stop("daemon").unwrap();
    assert_eq!(
        mgr.get("daemon").unwrap().config.restart_policy,
        RestartPolicy::Always
    );
}

#[test]
fn manager_len_reflects_registrations() {
    let mut mgr = ServiceManager::new();
    assert_eq!(mgr.len(), 0);
    mgr.register(ServiceConfig::new("a", "/bin/a")).unwrap();
    assert_eq!(mgr.len(), 1);
    mgr.register(ServiceConfig::new("b", "/bin/b")).unwrap();
    assert_eq!(mgr.len(), 2);
}
