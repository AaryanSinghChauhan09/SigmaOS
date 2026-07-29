// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod india_stack_localization;
pub mod scosmos;
pub mod standards;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat as CrossPlatformBinaryFormat,
    CompatibilityError as CrossPlatformError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability, MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};
pub use standards::{FhsConventionStatus, LsbProfile, PosixComplianceLevel, StandardsComplianceManager};
pub use india_stack_localization::{IndianLanguage, LocalizationManager, LocalizationProvider};
pub use scosmos::{
    ApkLoader, BinaryFormat as ScosmosBinaryFormat, BinderCallType,
    CompatibilityError as ScosmosError, MachoLoader, PeBinaryLoader, ScosmosManager,
};
