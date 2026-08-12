// SigmaOS Package Module
pub mod linux_translation;
pub mod store;
pub mod universal;
pub mod gentoo_opt;

pub use gentoo_opt::{
    UseFlagManager, SlottedPackage, PortageSlotResolver, EbuildSandbox,
    OptLevel, GccOptimizationTuner, GenkernelOrchestrator,
};

pub use linux_translation::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
pub use store::{SigmaSoftwareStore, SoftwareRegistryEntry, GLOBAL_SOFTWARE_STORE};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use apm::{
    SovereignApp, SovereignApm, IsolationLevel as SovereignIsolationLevel,
};
