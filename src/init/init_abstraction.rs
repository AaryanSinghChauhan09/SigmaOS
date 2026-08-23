// Init System Abstraction Layer for SigmaOS
// Location: src/init/init_abstraction.rs
extern crate alloc;
use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Failed,
    Disabled,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitError {
    ServiceNotFound(String),
    PermissionDenied,
    ExecutionFailed(String),
    AlreadyRunning,
    AlreadyStopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitSystemType {
    SigmaInit,
    Runit,
    S6,
    Dinit,
    Sysvinit,
    OpenRC,
}

pub trait InitSystem {
    fn init_type(&self) -> InitSystemType;
    fn start_service(&mut self, name: &str) -> Result<(), InitError>;
    fn stop_service(&mut self, name: &str) -> Result<(), InitError>;
    fn restart_service(&mut self, name: &str) -> Result<(), InitError>;
    fn service_status(&self, name: &str) -> ServiceStatus;
    fn enable_service(&mut self, name: &str) -> Result<(), InitError>;
    fn disable_service(&mut self, name: &str) -> Result<(), InitError>;
}
