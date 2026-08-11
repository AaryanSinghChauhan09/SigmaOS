// SigmaOS Package Management Module
// Enhanced AUR integration and package management

pub mod aur_integration;

pub use aur_integration::{
    AurClient, AurPackage, PkgBuildRecipe, BuildSandboxConfig, BuiltPackage,
    PkgBuildParser, AurError, BuildError, InstallError, ParseError
};