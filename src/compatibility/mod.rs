// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod historic_linux;
pub mod standards;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
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
pub mod scosmos;

pub use india_stack_localization::{IndianLanguage, LocalizationManager, LocalizationProvider};

pub use scosmos::{
    ApkLoader as ScosmosApkLoader, BinaryFormat as ScosmosBinaryFormat, BinderCallType,
    CompatibilityError as ScosmosCompatibilityError, MachoLoader as ScosmosMachoLoader,
    PeBinaryLoader as ScosmosPeBinaryLoader, ScosmosManager,
};
