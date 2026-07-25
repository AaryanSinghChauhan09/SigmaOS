// SigmaOS Compatibility Module
pub mod chimera_linux;
pub mod cross_platform;
pub mod historic_linux;
pub mod standards;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat as CrossPlatformBinaryFormat,
    CompatibilityError as CrossPlatformError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability,
    MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};

pub use standards::{
    FhsConventionStatus, LsbProfile, PosixComplianceLevel, StandardsComplianceManager,
};

pub mod india_stack_localization;

pub use india_stack_localization::{IndianLanguage, LocalizationManager, LocalizationProvider};
