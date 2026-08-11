// SigmaInit - Modern Init System Module
// Inspired by OpenRC, runit, s6 (systemd alternatives)

pub mod sigmainit;

pub use sigmainit::{
    SigmaInit, Service, ServiceState, RestartPolicy, SystemTarget,
    Supervisor, DependencyGraph, ServiceError, DependencyError
};