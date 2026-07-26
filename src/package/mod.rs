// SigmaOS Package Module
pub mod store;
pub mod universal;

pub use store::{SigmaSoftwareStore, StoreApp, StoreError};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
