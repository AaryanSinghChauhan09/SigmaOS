// SigmaOS Package Module
pub mod store;
pub mod universal;
pub mod store;

pub use store::{SigmaSoftwareStore, StoreApp, StoreError};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use store::{
    StoreError, StoreApp, SigmaSoftwareStore,
};
