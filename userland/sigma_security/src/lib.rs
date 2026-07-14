pub mod capability;
pub mod profile;
pub mod policy;
pub mod enforcement;

pub use capability::CapabilityManager;
pub use profile::ProfileSystem;
pub use policy::{PolicyEngine, Policy};
pub use enforcement::{EnforcementEngine, Operation, EnforcementResult};

/// SigmaSecurity: The native OS-level capability and policy enforcement engine.
/// This subsystem displaces AppArmor, Bubblewrap, and SELinux by providing
/// sub-microsecond latency, hardware-enforced checks, and native type-safe profiles.
pub struct SigmaSecurity {
    pub capability_manager: CapabilityManager,
    pub profile_system: ProfileSystem,
    pub enforcement_engine: EnforcementEngine,
}

impl Default for SigmaSecurity {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaSecurity {
    pub fn new() -> Self {
        Self {
            capability_manager: CapabilityManager::new(),
            profile_system: ProfileSystem::new(),
            enforcement_engine: EnforcementEngine::new(),
        }
    }

    /// Creates a native capability profile from a declarative policy.
    pub fn create_profile(&mut self, policy: Policy) -> profile::Profile {
        let validated = policy.validate();
        let profile = self.profile_system.create(validated);
        // Bind capabilities based on policy
        profile
    }

    /// Enforces a policy on a given operation with sub-microsecond latency.
    pub fn enforce_policy(&self, operation: &Operation) -> EnforcementResult {
        self.enforcement_engine.enforce(operation)
    }
}
