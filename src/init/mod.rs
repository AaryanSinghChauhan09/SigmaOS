// Core Init and Service Supervision Modules for SigmaOS
pub mod sigma_init;
pub mod system;
pub mod systemd_init;
pub mod openrc_init;

pub use sigma_init::{InitError, Service, ServiceState, SigmaInit, SimpleService};
pub use systemd_init::{SystemdInit, SystemdUnit, UnitState, UnitType, RestartPolicy};
pub use openrc_init::{
    DependencyType, OpenRCManager, Runlevel as OpenRCRunlevel, ServiceConfig as OpenRCServiceConfig, ServiceDependency, 
    ServiceState as OpenRCServiceState, ServiceSupervisor, ServiceInstance as OpenRCServiceInstance,
};
