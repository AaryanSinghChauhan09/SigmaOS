// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod standards;
pub mod legacy_adapters;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use standards::{
    FhsConventionStatus, LsbProfile, PosixComplianceLevel, StandardsComplianceManager,
};
pub use legacy_adapters::{
    LegacyKernelAdapter, LegacyDriverAdapter, LegacyPackageAdapter, LegacyFSAdapter,
    LegacyProtocolAdapter, LegacySecurityAdapter, LegacyUIAdapter,
};
