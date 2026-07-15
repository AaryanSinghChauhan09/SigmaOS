// SigmaOS Package Module
pub mod universal;

pub use universal::{UniversalPackageManager, UnifiedPackage, PackageFormat, PackageSource, PackageAdapter, DependencyResolver, ConflictResolution, PackageError};
