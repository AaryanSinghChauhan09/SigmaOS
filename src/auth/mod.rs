// SigmaOS Authentication & Identity Module
pub mod user;
pub mod access;
pub mod identity;
pub mod att_security;
pub mod authentication_pipeline;

pub use user::{
    UserID, UserState, User, AuthError, SimpleUser, AuthService, SimpleAuthService,
};

pub use access::{
    PermissionID, PermissionType, AccessResult, Permission, SimplePermission, AccessControl,
    AccessError, SimpleAccessControl,
};

pub use identity::{
    IdentityID, IdentityType, IdentityError, DigitalIdentity, SimpleDigitalIdentity,
    IdentityManager, SimpleIdentityManager, CredentialManager, SimpleCredentialManager,
    DecentralizedAuth, SimpleDecentralizedAuth,
};

pub use att_security::{
    SensitivityLevel, IdentificationStep, AttSecurityError, UserIdentityClaim,
    AdtAttributeRecord, AttributesDefinitionTable, AuthenticityVerifier,
    AllocatedUserSession, AutomaticResourceManager, AttSecurityEngine,
};
