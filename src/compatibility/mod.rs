// SigmaOS Compatibility Module
pub mod antix;
pub mod canonical;
pub mod chakra;
pub mod cross_platform;
pub mod legacy_adapters;

pub use legacy_adapters::{
    KernelPersona, KernelPersonaVM, LibcVersion, SyscallAbi, BinaryCompatMatrix,
    APITimelineManager, LegacyBus, StorageBridge, GraphicsBridge, WorkloadProfile,
    WorkloadOptimizer, DiscontinuedFS, DriverBridge, FSRevival,
    LegacyPluginManager, NetworkBridge, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER,
    GLOBAL_WORKLOAD_OPTIMIZER,
};

pub use constellation_mesh::{
    BIOSGatewayMesh, BuildCodexGrid, CRTMesh, ConstellationNode, CorebootGatewayMesh,
    DACConstellation, DotMatrixMesh, DriverArchiveGridV2, FileAlmanacHub, FirmwareGatewayMesh,
    FloppyMesh, GraphicsArchiveGridV2, KernelConstellationGrid, LegacyAsmCodexGrid,
    LegacyCCodexGrid, LegacyCppCodexGrid, NetworkAlmanacHub, NetworkArchiveGridV2,
    PeripheralArchiveMesh, ProcessAlmanacHub, SELinuxConstellation, SecurityConstellation,
    StorageArchiveGridV2, SyscallAlmanacHub, TapeMesh, UEFIGatewayMesh, ZeroTrustConstellation,
};
pub use cross_platform::{
    ApplicationBinary, BinaryFormat, BinaryFormat as CrossPlatformBinaryFormat,
    CompatibilityError, CompatibilityError as CrossPlatformError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability, MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};
pub use endeavour::{
    EosLogTool, EosMirrorReflector, EosUpdateNotifier, EosWelcomeEngine, Mirror, WelcomeTab,
    YayAurHelper,
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

pub use cross_platform_kernel::{
    PageAccessMode, MemoryArch, TranslationEntry, PageDirectory, DeferredProcedureCall,
    Kpcrb, Kpcr, Irql, IrqlController, IdtEntry, Idtr, SystemServiceTable,
    UmsThreadState, UmsContext, SovereignKernelInternals,
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

pub use canonical::{
    AiResourceScheduler, AppSuiteBundle, AppSuiteType, BrailleMatrix, BsdJailSandbox,
    CloudOrchestrator, CloudProvider, CompatBinary, CompatBinaryFormat, CompatibilityLayer,
    ContinuityCoordinator, DesktopMode, DistroReleaseChannel, EcosystemSnapshot, FlatpakApp,
    HandoffTask, LanguageTranslationCatalog, LocaleManager, ReleaseGovernanceCouncil,
    ReproducibleBuildVerifier, SigmaContainer, SnapshotManager, SuiteRegistry, TtsSynthesizer,
    UnifiedAppStore, ZorinAppearanceSwitcher,
};
