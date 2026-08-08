// SigmaOS Package Module
pub mod linux_translation;
pub mod store;
pub mod universal;

pub use linux_translation::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
    DebianPackageHeader, DebianPackageParser, AptSandboxedDeployment, DebianParityVerifier, SandboxCapability,
};
pub use store::{SigmaSoftwareStore, SoftwareRegistryEntry, GLOBAL_SOFTWARE_STORE};
pub use universal::{
<<<<<<< HEAD
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError,
    PackageFormat, PackageSource,
    UnifiedPackage, UniversalPackageManager,
};
||||||| 23ef22a4a
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError,
    PackageFormat, PackageSource,
    UnifiedPackage, UniversalPackageManager,
};
pub use apm::{
    SovereignApp, SovereignApm, IsolationLevel as SovereignIsolationLevel,
};
pub use debian::{
    DebControl, DebPackage, AptSource, DpkgStatusEntry, parse_sources_list, parse_dpkg_status,
};
pub use debian::{
    DebControl, DebPackage, AptSource, DpkgStatusEntry, parse_sources_list, parse_dpkg_status,
};
=======
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use debian::{
    DebControl, DebPackage, AptSource, DpkgStatusEntry, parse_sources_list, parse_dpkg_status,
};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
