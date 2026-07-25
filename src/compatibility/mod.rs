// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod historic_linux;
pub mod mint_linux;
pub mod chimera_linux;
pub mod relay_nexus;
pub mod solid_kernel;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};

pub use historic_linux::{
    LinuxEra, HistoricalCpuState, HistoricSyscallEmulator, Era0_11SyscallEmulator,
    Era1_0SyscallEmulator, Era2_4SyscallEmulator, VintageVirtualizationSandbox,
    VintageDriverTranslator, VintagePackageConverter, HistoricError,
};

pub use mint_linux::{
    MintUpdateLevel, MintUpdatePackage, MintUpdateManager, MintBackupTool,
    MintAppMetadata, MintSoftwareManager, MintReportAlertSeverity, MintReportAlert,
    MintReportSystem,
};

pub use chimera_linux::{
    DinitServiceState, DinitService, DinitServiceManager, BsdUserlandCompat,
    ApkPackageMetadata, ApkPackageStore,
};

pub use relay_nexus::{
    PersonaType, KernelRelay, SyscallEntry, SyscallEncyclopediaEntry, FileEntry,
    NetworkEntry, ProcessEntry, SyscallEncyclopedia, LegacyDriver, DriverVaultV2,
    StorageVaultV2, NetworkVaultV2, GraphicsVaultV2, DriverVaultV2Manager, FirmwareType,
    FirmwareNexus, BIOSNexus, UEFINexus, CorebootNexus, FirmwareNexusManager,
    BuildChronicle, LegacyCChronicle, LegacyCppChronicle, LegacyAsmChronicle,
    BuildChronicleManager, SecurityModelType, SecurityNexus, DACNexus, SELinuxNexus,
    ZeroTrustNexus, SecurityNexusManager, PeripheralArchiveV2, FloppyArchiveV2,
    TapeArchiveV2, CRTArchiveV2, DotMatrixArchiveV2, PeripheralArchiveV2Manager,
};

pub use solid_kernel::{
    IScheduler, RoundRobinSchedulerPort, PrioritySchedulerPort, SolidKernelCore,
    ComplianceScheduler, AuditBlock, SigmaFSPlusPlus,
};
