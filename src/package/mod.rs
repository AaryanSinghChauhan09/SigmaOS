// SigmaOS Package Module
pub mod linux_translation;
pub mod store;
pub mod universal;
pub mod spac;

pub use linux_translation::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
pub use store::{SigmaSoftwareStore, SoftwareRegistryEntry, GLOBAL_SOFTWARE_STORE};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager, PackageFormatAdapter,
};

pub use spac::{PackageState, SovereignPackage, SpacPackageManager, AURRecipe};
