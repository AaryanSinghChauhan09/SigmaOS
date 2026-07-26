// SigmaOS Package Module
pub mod store;
pub mod universal;

pub use store::{SigmaSoftwareStore, StoreApp, StoreError};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat, PackageFormatAdapter,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
