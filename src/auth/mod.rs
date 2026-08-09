// SigmaOS Authentication & Identity Module
pub mod user;
pub mod access;
pub mod identity;

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
