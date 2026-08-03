// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod capability;
pub mod pledge;
pub mod qubes_isolation;
pub mod root_improvement;
pub mod selinux;

pub use root_improvement::{
    CapSplitter, PamContext, PamControlFlag, PamEngine, PamFaillockModule, PamGroup,
    PamLimitsModule, PamMfaAuthenticator, PamMfaPluggableModule, PamModule, PamResult, PamRule,
    PamTimeModule, PamUnixModule, PolkitEnforcer, RootlessNamespaceManager, SudoDoasElevator,
};

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use qubes_isolation::{
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain, IsolationError,
};
pub use selinux::{
    AppArmorManager, AppArmorMode, AppArmorProfile, SecurityPolicy, SecurityLabel, SecurityRule, ObjectType, SelinuxPermission,
};
