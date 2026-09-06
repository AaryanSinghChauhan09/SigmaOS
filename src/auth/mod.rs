#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Authentication & Identity Module
pub mod access;
pub mod att_security;
pub mod authentication_pipeline;
pub mod identity;
pub mod user;
pub mod systemd_homed;

pub use user::{AuthError, AuthService, SimpleAuthService, SimpleUser, User, UserID, UserState};
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
