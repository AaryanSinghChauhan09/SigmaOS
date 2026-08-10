// SigmaOS Package Module
pub mod store;
pub mod universal;
pub mod debian;

pub use store::SigmaSoftwareStore;
pub use universal::{
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use debian::{
    DebControl, DebPackage, AptSource, DpkgStatusEntry, parse_sources_list, parse_dpkg_status,
};
