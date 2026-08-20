// SigmaOS Package Module
pub mod universal;
pub mod store;
pub mod repository;

pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use crate::sigpkg::universal_adapter::PackageFormatAdapter;
pub use store::{
    StoreError, StoreApp, SigmaSoftwareStore,
};
pub use repository::{
    PackageRepository, RepositoryManager, RepositoryMetadata, RepoError,
};
