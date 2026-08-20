// SigmaOS Package Module
pub mod universal;
pub mod store;
pub mod linux_translation;
pub mod debian;

pub use linux_translation::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
pub use store::{SigmaSoftwareStore, StoreApp, StoreError};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat, PackageAdapter,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use debian::{
    DebControl, DebPackage, AptSource, DpkgStatusEntry, parse_sources_list, parse_dpkg_status,
};
