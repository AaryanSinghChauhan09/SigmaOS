// SigmaOS Compatibility Module
pub mod constellation;
pub mod cross_platform;
pub mod legacy_adapters;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat as CrossPlatformBinaryFormat,
    CompatibilityError as CrossPlatformError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability, MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};

pub use historic_linux::{
    LinuxEra, HistoricalCpuState, HistoricSyscallEmulator, Era0_11SyscallEmulator,
    Era1_0SyscallEmulator, Era2_4SyscallEmulator, VintageVirtualizationSandbox,
    VintageDriverTranslator, VintagePackageConverter, HistoricError,
};

pub mod india_stack_localization;
pub mod scosmos;

pub use india_stack_localization::{IndianLanguage, LocalizationManager, LocalizationProvider};

pub use scosmos::{
    ApkLoader as ScosmosApkLoader, BinaryFormat as ScosmosBinaryFormat, BinderCallType,
    CompatibilityError as ScosmosCompatibilityError, MachoLoader as ScosmosMachoLoader,
    PeBinaryLoader as ScosmosPeBinaryLoader, ScosmosManager,
};
