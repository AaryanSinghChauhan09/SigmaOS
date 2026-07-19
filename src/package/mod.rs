// SigmaOS Package Module
pub mod universal;
pub mod updater;

pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use updater::{
    OfficialUpdateSource, RollbackSnapshot, SoftwareUpdater, UpdateChannel, UpdateError,
    UpdatePackage, UpdateProgress, UpdateSource, UpdateStatus, UpdateType,
};
