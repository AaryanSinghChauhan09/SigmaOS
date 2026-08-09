// SigmaOS Package Module
pub mod store;
pub mod universal;
pub mod debian;

pub use store::{SigmaSoftwareStore, StoreApp, StoreError};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageFormatAdapter as PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use debian::{
    DebControl, DebPackage, AptSource, DpkgStatusEntry, parse_sources_list, parse_dpkg_status,
};
