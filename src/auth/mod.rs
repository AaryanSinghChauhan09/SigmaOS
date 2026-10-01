// SigmaOS Authentication & Identity Module
pub mod access;
pub mod att_security;
pub mod authentication_pipeline;
pub mod identity;
pub mod user;
pub mod systemd_homed;
pub mod root_security_enhancements;

pub use user::{AuthError, AuthService, SimpleAuthService, SimpleUser, User, UserID, UserState};
pub use root_security_enhancements::{
    DoasRule, RootAuditEvent, RootCapability, SovereignImmutableRootfsGuard,
    SovereignRootAuditLogger, SovereignRootCapabilityGovernor, SovereignRootSecurityMasterSuite,
    SovereignSuDoasPolicyEngine,
};
pub use systemd_homed::{
    HomedUserRecord, HomeState, HomeStorageBackend, SovereignSystemdHomedEngine,
};

pub use access::{
    AccessControl, AccessError, AccessResult, Permission, PermissionID, PermissionType,
    SimpleAccessControl, SimplePermission,
};

pub use identity::{
    CredentialManager, DecentralizedAuth, DigitalIdentity, IdentityError, IdentityID,
    IdentityManager, IdentityType, SimpleCredentialManager, SimpleDecentralizedAuth,
    SimpleDigitalIdentity, SimpleIdentityManager,
};

pub use att_security::{
    AdtAttributeRecord, AllocatedUserSession, AttSecurityEngine, AttSecurityError,
    AttributesDefinitionTable, AuthenticityVerifier, AutomaticResourceManager, IdentificationStep,
    SensitivityLevel, UserIdentityClaim,
};
