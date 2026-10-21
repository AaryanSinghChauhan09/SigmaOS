use alloc::string::String;

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceState {
    Active,
    Inactive,
}

pub struct SigmaServiceManager4;
