// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod historic_linux;
pub mod mint_linux;
pub mod chimera_linux;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat as CrossPlatformBinaryFormat,
    CompatibilityError as CrossPlatformError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, FhsConventionStatus, HtmlRendererCapability, LsbProfile,
    MediaDecoderCapability, PosixComplianceLevel, StandardsComplianceManager,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};
