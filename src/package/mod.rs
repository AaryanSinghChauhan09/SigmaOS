// SigmaOS Package Module
pub mod universal;
pub mod spac;

pub use universal::{
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};

pub use spac::{PackageState, SovereignPackage, SpacPackageManager, AURRecipe};
