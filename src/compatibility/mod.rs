// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod standards;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability, MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};
pub use standards::{
    FhsConventionStatus, LsbProfile, PosixComplianceLevel, StandardsComplianceManager,
};
pub mod india_stack_localization;
pub use india_stack_localization::{IndianLanguage, LocalizationManager, LocalizationProvider};
