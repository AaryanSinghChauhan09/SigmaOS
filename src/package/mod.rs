// SigmaOS Package Module
pub mod universal;
pub mod spac;

pub use universal::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager, PackageFormatAdapter,
};

pub use spac::{PackageState, SovereignPackage, SpacPackageManager, AURRecipe};
