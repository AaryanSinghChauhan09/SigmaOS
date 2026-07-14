use crate::capability::Capability;
use crate::profile::Profile;

pub enum Operation {
    FileOpen(String),
    NetworkBind(u16),
    HardwareAccess(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnforcementResult {
    Allowed,
    Denied(String),
}

pub struct EnforcementEngine {}

impl Default for EnforcementEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EnforcementEngine {
    pub fn new() -> Self {
        Self {}
    }

    /// Evaluates if an operation is permitted under a given profile's capabilities.
    pub fn evaluate_operation(&self, profile: &Profile, operation: &Operation) -> EnforcementResult {
        let required_cap = match operation {
            Operation::FileOpen(path) => Capability::FileRead(path.clone()),
            Operation::NetworkBind(port) => Capability::NetworkBind(*port),
            Operation::HardwareAccess(hw) => Capability::HardwareAccess(hw.clone()),
        };

        if profile.active_policy.allowed_capabilities.contains(&required_cap) {
            EnforcementResult::Allowed
        } else {
            EnforcementResult::Denied(format!("Missing capability: {:?}", required_cap))
        }
    }

    /// Dummy enforcement method for the top-level subsystem.
    pub fn enforce(&self, _operation: &Operation) -> EnforcementResult {
        // In real execution, this checks the active thread's bound profile.
        EnforcementResult::Denied("No active profile bound".to_string())
    }
}
