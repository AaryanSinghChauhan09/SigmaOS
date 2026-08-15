// SigmaInit - Modern Init System Module
// Inspired by OpenRC, runit, s6 (systemd alternatives)

pub mod sigmainit;
pub mod runit_service_manager;

pub use sigmainit::{
    SigmaInit, Service, ServiceState, SystemTarget,
    Supervisor, DependencyGraph, ServiceError, DependencyError
};

pub use runit_service_manager::{
    RunitServiceManager, RunitService, RunitServiceState, RunitSignal,
    DependencyType, ServiceDependency, RunitRestartPolicy,
    ServiceEvent, EventType, WatchdogConfig
};