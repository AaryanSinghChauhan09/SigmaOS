// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod india_stack_localization;
pub mod scosmos;
pub mod standards;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, FhsConventionStatus, HtmlRendererCapability, LsbProfile,
    MediaDecoderCapability, PosixComplianceLevel, StandardsComplianceManager,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};
pub use india_stack_localization::{IndianLanguage, LocalizationManager, LocalizationProvider};
pub use scosmos::{
    ApkLoader, BinderCallType, BinaryFormat as ScosmosBinaryFormat, CompatibilityError as ScosmosError, MachoLoader, PeBinaryLoader, ScosmosManager,
};
