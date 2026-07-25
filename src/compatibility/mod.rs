// SigmaOS Compatibility Module
pub mod constellation;
pub mod cross_platform;
pub mod gap_closure;
pub mod historic_linux;
pub mod personality;
pub mod sigmawin;
pub mod superiority;

pub use superiority::{
    SovereignRegistry, SovereignObjectBus, SovereignCloudFS, SovereignSigLoader,
    SigSection, SigSectionType, SovereignTimeMachine, ShardCheckpoint, NumaTask,
    NumaCfsScheduler, LockFreeQueue, SovereignThemeEngine, SovereignForensics,
    SovereignRecoverUtility, ShardIgnitor,
};

pub use gap_closure::{
    AiTaskOrchestrator, BootInterface, BuildLedgerSystem, DriverClass, DriverRepositoryManager,
    EmulatedPeripheral, FirmwareBridgeManager, GapSandboxPolicy, HardwareDriver, HidGraphicsDriver,
    JobClass, KernelModule, KernelModuleManager, LedgerSnapshot, MemoryProtection, ModuleState,
    NetworkStackGateway, PeripheralEmulationLibrary, SecurityPolicyManager,
    SyscallCompatibilityRegistry, VirtualMemoryManager,
};

pub use constellation::{
    ArchiveProfile, BuildArchive, ChronicleType, ConstellationNode, DriverMuseum, ExhibitType,
    FirmwarePavilion, KernelConstellation, ObsoletePeripheral, PavilionType, PeripheralMuseum,
    SecurityModel as ConstellationSecurityModel, SecurityPavilion, SyscallChronicle,
};

pub use personality::{
    BuildCapsule, BuildProfile, CapsuleVersion, DriverEmulator, EmulatorProfile, FirmwarePersona,
    FirmwareType, KernelShard, ObsoleteDevice, PeripheralPod, SecurityGrid, SecurityModel,
    ShardType, SyscallCapsule,
};

pub use sigmawin::{
    D3dToVulkanTranslator, PeFormat, PeLoader, RegistryManager, User32MessageQueue, Win32Error,
    Win32Message, WinSockAdapter,
};

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

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox, LfsToolchainBuilder,
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
