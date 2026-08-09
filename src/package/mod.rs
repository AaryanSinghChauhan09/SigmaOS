// SigmaOS Package Module
pub mod debian_translator;
pub mod universal;

pub use debian_translator::{
    DebianPackageMetadata, DebianPackageTranslator, DebianScriptTrigger, DebianTranslatorError,
    DebianTriggerType,
};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
