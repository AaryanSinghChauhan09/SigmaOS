// SigmaOS Package Module
pub mod dependency_resolver;
pub mod universal;
pub mod store;

pub use store::{SigmaSoftwareStore, StoreApp, StoreError};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};

pub use dependency_resolver::{
    PackageDependencyResolver, PackageRecipe as DeclarativePackageRecipe,
    Version as DependencyVersion,
};
