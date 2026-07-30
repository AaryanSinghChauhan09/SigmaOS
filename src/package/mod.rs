// SigmaOS Package Module
<<<<<<< HEAD
pub mod linux_translation;
pub mod universal;
pub mod store;

pub use linux_translation::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat, PackageFormatAdapter,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use store::{
    StoreError, StoreApp, SigmaSoftwareStore,
};
=======
pub mod universal;
pub mod spac;

pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};

pub use spac::{PackageState, SovereignPackage, SpacPackageManager, AURRecipe};
>>>>>>> origin/jules-18101178622594638830-97dc43c6
