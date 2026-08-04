// SigmaOS Compatibility Module
pub mod constellation;
||||||| 43be3a7e8
pub mod antix;
pub mod chakra;
||||||| 43be3a7e8
pub mod chimera_linux;
pub mod cross_platform;
pub mod historic_linux;
pub mod mint_linux;
pub mod chimera_linux;
pub mod relay_nexus;
pub mod solid_kernel;
pub mod india_stack_localization;
pub mod legacy_adapters;
pub mod freedos;

pub use legacy_adapters::{
    KernelPersona, KernelPersonaVM, LibcVersion, SyscallAbi, BinaryCompatMatrix,
    APITimelineManager, LegacyBus, StorageBridge, GraphicsBridge, WorkloadProfile,
    WorkloadOptimizer, DiscontinuedFS, DriverBridge, FSRevival,
    LegacyPluginManager, NetworkBridge, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER,
    GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use freedos::{ConfigSysSetting, TsrProgram, FatDirectoryEntry, FreeDosEmulator};
||||||| 43be3a7e8
pub mod legacy_adapters;
||||||| 43be3a7e8
pub mod interim;
pub mod lubuntu;
pub mod mint_linux;
||||||| 0ddf2eac7
pub mod india_stack_localization;
pub mod legacy_adapters;
pub mod freedos;

pub use legacy_adapters::{
    KernelPersona, KernelPersonaVM, LibcVersion, SyscallAbi, BinaryCompatMatrix,
    APITimelineManager, LegacyBus, StorageBridge, GraphicsBridge, WorkloadProfile,
    WorkloadOptimizer, DiscontinuedFS, DriverBridge, FSRevival,
    LegacyPluginManager, NetworkBridge, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER,
    GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use freedos::{ConfigSysSetting, TsrProgram, FatDirectoryEntry, FreeDosEmulator};

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, BinaryFormat as CrossPlatformBinaryFormat,
    CompatibilityError, CompatibilityError as CrossPlatformError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability, MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};
pub use india_stack_localization::{IndianLanguage, LocalizationManager, LocalizationProvider};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
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

pub use legacy_adapters::{
    APITimelineManager, BinaryCompatMatrix, DiscontinuedFS, DriverBridge, FSRevival,
    GraphicsBridge, KernelPersona, KernelPersonaVM, LegacyBus, LegacyDriver, LegacyPluginManager,
    LibcVersion, NetworkBridge, StorageBridge, SyscallAbi, WorkloadOptimizer, WorkloadProfile,
    GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_WORKLOAD_OPTIMIZER,
};

pub use chakra::{
    AkabeiBundle, AkabeiPackageEngine, BundleType, DesktopTheme, InstallerStep, KapudanAssistant,
    TribeInstaller, GLOBAL_AKABEI, GLOBAL_KAPUDAN, GLOBAL_TRIBE,
};

pub use antix::{
    AntixControlCenter, AntixDesktopProfiler, AntixInitManager, DesktopProfile,
    LegacyMemoryTrimmer, MicroService, MicroServiceState, GLOBAL_ANTIX_CONTROL,
    GLOBAL_ANTIX_DESKTOP, GLOBAL_ANTIX_INIT, GLOBAL_MEMORY_TRIMMER,
};
pub use interim::{InterimLispVM, LispVal, MntReformLpcDriver, ReformPowerStats};
pub use lubuntu::{CpuGovernor, LubuntuHealthReport, LubuntuSystemManager, SystemPressure};
