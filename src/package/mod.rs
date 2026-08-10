// SigmaOS Package Module
pub mod universal;
pub mod store;

pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use store::{
    StoreError, StoreApp, SigmaSoftwareStore,
};
