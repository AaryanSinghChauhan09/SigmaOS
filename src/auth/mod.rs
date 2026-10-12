// SigmaOS Authentication and Security Subsystem Mod

pub mod sigmapam;

pub use sigmapam::{
    SigmaPAM, PamConfig, PamModule, PamService, PamFlag, AuthResult,
    UserAccount, AuthRequest, AuthResponse, PasswordAuthModule,
    init_sigmapam, init_sigmapam_default, get_sigmapam,
    authenticate_user, check_account,
};
