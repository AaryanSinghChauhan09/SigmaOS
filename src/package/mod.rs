// SigmaOS Package Module
pub mod universal;
pub mod spac;

pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};

pub use spac::{PackageState, SovereignPackage, SpacPackageManager, AURRecipe};
