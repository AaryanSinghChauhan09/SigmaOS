// SigmaOS Compatibility Module
pub mod absorb_tools;
pub mod android_chromeos;
pub mod apache_ossie;
pub mod canonical;
pub mod chimera_linux;
pub mod constellation_mesh;
pub mod cross_platform;
pub mod fedora;
pub mod india_stack;
pub mod india_professional_tools;
pub mod alpine_linux;
pub mod interim;
pub mod jehanne;
pub mod legacy_adapters;
pub mod macos_darwin;
pub mod mint_linux;
pub mod reactos;
pub mod lubuntu;
pub mod antix;
pub mod bodhi_moksha;
pub mod cachy_os;
pub mod chakra;
pub mod endeavour;
pub mod garuda_zen;
pub mod gentoo;
pub mod tiny_core;
pub mod localsend;
pub mod arch_linux;
pub mod zorin;

pub use macos_darwin::{
    ApfsFileClone, ApfsSnapshot, ApfsSnapshotManager, AudioStreamDescription, CoreAudioHalRouter,
    CoreAudioNode, FatArch, LaunchdJobConfig, LaunchdServiceManager, LaunchdState, MachO64Header,
    MachOLoader, MachOSegment64, SpotlightMetadata, SpotlightMetadataIndex, FAT_MAGIC,
    LC_LOAD_DYLIB, LC_MAIN, LC_SEGMENT_64, MH_MAGIC_64,
};

pub use android_chromeos::{
    AndroidActivity, AndroidAppManifest, ApkManifestParser, ArtBytecodeSandbox,
    CrostiniContainerBridge, CrostiniState, DexClassDef, IntentRouter, IntentTarget, PartitionSlot,
    VerifiedBootSlotSwitcher,
};

pub use arch_linux::{
    ProcFile, ProcFileType, DevFile, DevFileType, ArchPackage, PacmanError as PacmanCompatError,
    PacmanEngine, RunlevelTarget, SystemdBootMetrics, ArchInitSystem, RuleAction, FirewallRule,
    ArchFirewall, LsmMode, LsmSentinel, PamGate, TmuxMultiplexer, SovereignEnvRegistry,
    YayParuAdapter, ReflectorMirrorlist, ArchinstallParity, ArtixInitBridge, PacmanKeyring,
    AurPatchEngine,
};

pub use zorin::{
    ZorinLayoutSwitcher,
    ZorinChameleonEngine,
    ZorinConnectManager,
    ZorinWindowsAppSupport,
};
pub mod historic_linux;
pub mod relay_nexus;
pub mod solid_kernel;
pub mod sovereign_suite;
pub mod wasm_sandbox;
pub mod open_source_tier1;

pub use open_source_tier1::{
    WasmerIntegration, SmolTcpIntegration, LibsodiumIntegration, SqliteIntegration,
};

pub use canonical::{SigmaSubiquity, SigmaNetplan, SigmaCloudInit, SigmaMultipass, SigmaCurtin};
pub use fedora::{
    DnfPackageResolver, MockChrootBuilder, KojiBuildServer, BodhiUpdateTriage,
    SovereignOstreeDeployer, SovereignSeLinuxContext, SovereignSeLinuxEngine,
    SovereignFirewalldManager, SovereignCockpitConsole,
};
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
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer, WindowCoordinates, ZenithDisplayCompositor,
};
pub use reactos::{
    NtHandle, NtObjectManager, NtObjectType, NtStatus, PortableExecutableLoader, RegistryHive,
};
pub use india_stack::{GstCalculator, IndiaStackError, MockUPIService, MultilingualSupport};
pub use india_professional_tools::{
    JudicialTimelinePlanner, MsmeComplianceEngine, AyushFormularyHelper,
    PMWaniHotspotController, DigiYatraPassScanner, IrctcPnrTracker,
    SpacSatResolver,
};
pub use alpine_linux::{
    ApkInstalledPackage, ApkDatabaseIndex, SyslogSeverity, SyslogMessage,
    AlpineSyslogManager, BusyBoxMulticall,
};
pub use interim::{InterimLispVM, LispVal, MntReformLpcDriver, ReformPowerStats};
pub use jehanne::{
    ComputeNode, DistributedComputeHandoff, JehanneError, JehanneNamespace, NamespaceBindEntry,
    Plan9pMessage, Plan9pMsgType,
};

pub use mint_linux::{
    MintBackupTool, MintSoftwareManager, MintUpdateLevel, MintUpdateManager,
    MintCinnamonStyling, MintDriverInfo, MintDriverManager, MintReportAlert,
    MintReportSystem, MintTimeshiftEngine, TimeshiftSnapshot,
};

pub use chimera_linux::{
    ApkPackageMetadata, ApkPackageStore, BsdUserlandCompat, DinitService, DinitServiceManager,
    DinitServiceState,
};

pub use relay_nexus::{
    BIOSNexus, BuildChronicle, BuildChronicleManager, CRTArchiveV2, CorebootNexus, DACNexus,
    DotMatrixArchiveV2, DriverVaultV2, DriverVaultV2Manager, FileEntry, FirmwareNexus,
    FirmwareNexusManager, FirmwareType, FloppyArchiveV2, GraphicsVaultV2, KernelRelay,
    LegacyAsmChronicle, LegacyCChronicle, LegacyCppChronicle, LegacyDriver, NetworkEntry,
    NetworkVaultV2, PeripheralArchiveV2, PeripheralArchiveV2Manager, PersonaType, ProcessEntry,
    SELinuxNexus, SecurityModelType, SecurityNexus, SecurityNexusManager, StorageVaultV2,
    SyscallEncyclopedia, SyscallEncyclopediaEntry, SyscallEntry, TapeArchiveV2, UEFINexus,
    ZeroTrustNexus,
};

pub use solid_kernel::{
    AuditBlock, ComplianceScheduler, IScheduler, PrioritySchedulerPort, RoundRobinSchedulerPort,
    SigmaFSPlusPlus, SolidKernelCore,
};

pub use wasm_sandbox::{WasmModule, WasmSandboxEngine, WasmState};

pub use absorb_tools::{
    CasObject, Clause, ContentAddressedStorage, DpllSatSolver, PledgePermission,
    PledgeUnveilSandbox, PqcSecureChannel,
};

pub use tiny_core::{FiletoolOverlay, FrugalLoader, TceLoader, TczExtension, TinyCoreBootConfig};

pub use apache_ossie::{
    MetricAggregation, OssieCatalog, OssieDimension, OssieInterpreter, OssieMetric, OssieOntology,
    OssieRelationship, SemanticRow,
};

pub use sovereign_suite::{
    CreativeMatrix, EverySearch, FancyZonesManager, ImageLayer, JoplinE2ee, LayoutZone,
    ProcMonitor, ProcessExplorerState, SpreadsheetCore, SysDiag,
};
pub use gentoo::{EbuildPackage, OpenRcManager, OpenRcRunlevel, OpenRcService, PortageEngine, ServiceStatus, UseFlagManager};
pub use localsend::{LocalSendBridgeManager, LocalSendDevice, LocalSendDeviceType, LocalSendFileMetadata, LocalSendSession};
