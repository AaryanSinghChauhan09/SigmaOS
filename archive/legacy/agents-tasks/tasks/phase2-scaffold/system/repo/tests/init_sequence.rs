//! Integration tests for the SigmaOS init system startup sequence.
//!
//! Follows AAA (Arrange-Act-Assert) structure throughout.

use sigmaos_system::{
    init::{InitError, InitSystem, SystemState},
    service::{RestartPolicy, ServiceConfig},
};

// ── helpers ──────────────────────────────────────────────────────────────────

fn minimal_init() -> InitSystem {
    InitSystem::new()
}

fn make_svc(name: &str) -> ServiceConfig {
    ServiceConfig::new(name, format!("/usr/bin/{}", name))
        .enabled(true)
        .required(false)
}

fn make_required_svc(name: &str) -> ServiceConfig {
    ServiceConfig::new(name, format!("/usr/bin/{}", name))
        .enabled(true)
        .required(true)
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[test]
fn initial_state_is_booting() {
    let init = minimal_init();
    assert_eq!(init.state(), SystemState::Booting);
}

#[test]
fn boot_with_no_services_transitions_to_running() {
    let mut init = minimal_init();
    assert!(init.boot().is_ok());
    assert_eq!(init.state(), SystemState::Running);
}

#[test]
fn boot_starts_all_enabled_services() {
    // Arrange
    let mut init = minimal_init();
    init.services.register(make_svc("network")).unwrap();
    init.services.register(make_svc("syslog")).unwrap();

    // Act
    init.boot().unwrap();

    // Assert
    use sigmaos_system::service::ServiceState;
    assert_eq!(
        init.services.get("network").unwrap().state,
        ServiceState::Running
    );
    assert_eq!(
        init.services.get("syslog").unwrap().state,
        ServiceState::Running
    );
}

#[test]
fn boot_respects_dependency_order() {
    // Arrange: syslog depends on network; both must end up Running.
    let mut init = minimal_init();
    init.services.register(make_svc("network")).unwrap();
    init.services
        .register(make_svc("syslog").depends_on("network"))
        .unwrap();

    // Act
    init.boot().unwrap();

    // Assert
    use sigmaos_system::service::ServiceState;
    assert_eq!(
        init.services.get("network").unwrap().state,
        ServiceState::Running
    );
    assert_eq!(
        init.services.get("syslog").unwrap().state,
        ServiceState::Running
    );
}

#[test]
fn boot_fails_on_dependency_cycle() {
    // Arrange: a → b → a  (cycle)
    let mut init = minimal_init();
    init.services
        .register(make_svc("a").depends_on("b"))
        .unwrap();
    init.services
        .register(make_svc("b").depends_on("a"))
        .unwrap();

    // Act
    let err = init.boot().unwrap_err();

    // Assert
    assert!(matches!(err, InitError::DependencyCycle(_)));
}

#[test]
fn boot_cannot_be_called_twice() {
    let mut init = minimal_init();
    init.boot().unwrap();
    let err = init.boot().unwrap_err();
    assert!(matches!(err, InitError::InvalidTransition(_, _)));
}

#[test]
fn shutdown_stops_running_services() {
    // Arrange
    let mut init = minimal_init();
    init.services.register(make_svc("network")).unwrap();
    init.boot().unwrap();

    // Act
    init.shutdown().unwrap();

    // Assert
    use sigmaos_system::service::ServiceState;
    assert_eq!(init.state(), SystemState::ShuttingDown);
    assert_eq!(
        init.services.get("network").unwrap().state,
        ServiceState::Stopped
    );
}

#[test]
fn disabled_services_are_not_started_on_boot() {
    // Arrange
    let mut init = minimal_init();
    let disabled = ServiceConfig::new("optional", "/usr/bin/optional").enabled(false);
    init.services.register(disabled).unwrap();

    // Act
    init.boot().unwrap();

    // Assert — service exists but was never started
    use sigmaos_system::service::ServiceState;
    assert_eq!(
        init.services.get("optional").unwrap().state,
        ServiceState::Stopped
    );
}

#[test]
fn enter_maintenance_sets_correct_state() {
    let mut init = minimal_init();
    init.enter_maintenance();
    assert_eq!(init.state(), SystemState::Maintenance);
}

#[test]
fn system_state_display() {
    assert_eq!(format!("{}", SystemState::Booting), "booting");
    assert_eq!(format!("{}", SystemState::Running), "running");
    assert_eq!(format!("{}", SystemState::ShuttingDown), "shutting-down");
    assert_eq!(format!("{}", SystemState::Maintenance), "maintenance");
    assert_eq!(format!("{}", SystemState::Panic), "panic");
}

#[test]
fn boot_with_deep_dependency_chain() {
    // a → b → c  (three-level chain)
    let mut init = minimal_init();
    init.services.register(make_svc("a")).unwrap();
    init.services
        .register(make_svc("b").depends_on("a"))
        .unwrap();
    init.services
        .register(make_svc("c").depends_on("b"))
        .unwrap();

    init.boot().unwrap();

    use sigmaos_system::service::ServiceState;
    for name in &["a", "b", "c"] {
        assert_eq!(
            init.services.get(name).unwrap().state,
            ServiceState::Running,
            "service {name} should be running"
        );
    }
}

#[test]
fn restart_policy_field_persists_through_registration() {
    let mut init = minimal_init();
    let cfg = ServiceConfig::new("daemon", "/usr/bin/daemon")
        .enabled(true)
        .restart(RestartPolicy::OnFailure);
    init.services.register(cfg).unwrap();

    assert_eq!(
        init.services.get("daemon").unwrap().config.restart_policy,
        RestartPolicy::OnFailure
    );
}
