// SigmaOS Package Module
pub mod dependency_resolver;
pub mod universal;

pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};

pub use dependency_resolver::{
    PackageDependencyResolver, PackageRecipe as DeclarativePackageRecipe,
    Version as DependencyVersion,
};
