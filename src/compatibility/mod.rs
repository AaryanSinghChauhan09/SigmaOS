// SigmaOS Compatibility Module
pub mod absorb_tools;
pub mod apache_ossie;
pub mod chimera_linux;
pub mod cross_platform;
pub mod historic_linux;
pub mod mint_linux;
pub mod relay_nexus;
pub mod solid_kernel;
pub mod india_stack_localization;
pub mod legacy_adapters;
pub mod india_professional_tools;
pub mod canonical;
pub mod fedora;
pub mod arch_linux;
pub mod gap_closure;
pub mod superiority;
pub mod open_source_tier1;
pub mod interim;
pub mod lubuntu;
pub mod cross_platform_kernel;
pub mod wasm_sandbox;
pub mod tiny_core;
pub mod sovereign_suite;

pub use gap_closure::{
    KernelModuleManager, SyscallCompatibilityRegistry, DriverRepositoryManager,
    FirmwareBridgeManager, BuildLedgerSystem, SecurityPolicyManager,
    PeripheralEmulationLibrary, VirtualMemoryManager, NetworkStackGateway,
    HidGraphicsDriver, AiTaskOrchestrator,
};
pub use superiority::{
    SovereignRegistry, SovereignObjectBus, SovereignCloudFS, SovereignSigLoader,
    SovereignTimeMachine, NumaCfsScheduler, LockFreeQueue, SovereignThemeEngine,
    SovereignForensics, SovereignRecoverUtility, ShardIgnitor,
};

pub use arch_linux::{
    ProcFileType, ProcFile, DevFileType, DevFile, ArchPackage, PacmanError, PacmanEngine,
    RunlevelTarget, SystemdBootMetrics, ArchInitSystem, RuleAction, FirewallRule, ArchFirewall,
    LsmMode, LsmSentinel, PamGate, PaneLayout, TmuxMultiplexer, SovereignEnvRegistry,
    AurRepoStatus, YayParuAdapter, ArchMirror, ReflectorMirrorlist, SubvolumeConfig,
    ArchinstallConfig, ArchinstallParity, ArtixInitSystemType, ServiceState, ArtixInitBridge,
    KeyTrustLevel, PacmanKey, PacmanKeyring, AurPatch, AurPatchEngine,
    MkinitcpioGenerator, NewsItem, ArchNewsFeedParser, CachedPackage, PacmanDbCleaner,
    WikiPage, ArchWikiSearchEngine,
};

pub use open_source_tier1::{
    WasmerIntegration, SmolTcpIntegration, LibsodiumIntegration, SqliteIntegration,
};

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use interim::{InterimLispVM, LispVal, MntReformLpcDriver, ReformPowerStats};
pub use lubuntu::{CpuGovernor, LubuntuHealthReport, LubuntuSystemManager, SystemPressure};

pub use cross_platform_kernel::{
    PageAccessMode, MemoryArch, PageDirectory, DeferredProcedureCall,
    Kpcrb, Kpcr, Irql, IrqlController, IdtEntry, Idtr, SystemServiceTable,
    UmsThreadState, UmsContext, SovereignKernelInternals,
};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};

pub use mint_linux::{
    MintAppMetadata, MintBackupTool, MintReportAlert, MintReportAlertSeverity, MintReportSystem,
    MintSoftwareManager, MintUpdateLevel, MintUpdateManager, MintUpdatePackage,
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
    CasObject, Clause, ContentAddressedStorage, DpllSatSolver, Literal, PledgePermission,
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
pub use antix::*;
pub use zorin::*;
pub use legacy_adapters::*;
