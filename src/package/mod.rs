#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

// SigmaOS Package Module
pub mod universal;
pub mod store;
pub mod linux_translation;
pub mod debian;
pub mod declarative_app;

pub use declarative_app::{
    DeclarativeAppManifest, HardwareAccessPermissions, ImmutableAppLayer, ShardsMarketplace,
};
pub use linux_translation::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
    DebianPackageHeader, DebianPackageParser, AptSandboxedDeployment, DebianParityVerifier, SandboxCapability,
};
pub use store::{SigmaSoftwareStore, StoreApp, StoreError};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat, PackageAdapter,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use debian::{
    DebControl, DebPackage, AptSource, DpkgStatusEntry, parse_sources_list, parse_dpkg_status,
};
