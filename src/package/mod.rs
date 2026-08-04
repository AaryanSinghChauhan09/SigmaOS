// SigmaOS Package Module
pub mod spac;
pub mod universal;

pub use universal::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat, PackageFormatAdapter,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};

pub use spac::{AURRecipe, PackageState, SovereignPackage, SpacPackageManager};
