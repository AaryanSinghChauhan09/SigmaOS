// Sovereign Service Manager v4

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceState {
    Inactive,
    Active,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub state: ServiceState,
}

#[derive(Debug, Default)]
pub struct SigmaServiceManager4 {
    pub services: Vec<Service>,
}

impl SigmaServiceManager4 {
    pub fn new() -> Self {
        Self::default()
    }
}
