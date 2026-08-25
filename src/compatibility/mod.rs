// SigmaOS Compatibility Module
pub mod abi_extended;
pub mod abi_translator;
pub mod absorb_tools;
pub mod advanced_ecosystem;
pub mod alpine_linux;
pub mod android_chromeos;
pub mod antix;
pub mod apache_ossie;
pub mod arch;
pub mod arch_aur;
pub mod arch_linux;
pub mod artix_linux;
pub mod atomic_distribution;
pub mod bodhi_moksha;
pub mod bsd;
pub mod cachy_os;
pub mod canonical;
pub mod chakra;
pub mod chimera_linux;
pub mod clear_linux;
pub mod constellation;
pub mod constellation_mesh;
pub mod cross_platform;
pub mod cross_platform_kernel;
pub mod debian;
pub mod distro_bridge;
pub mod dragonfly_bsd;
pub mod elf_execution;
pub mod endeavour;
pub mod federation;
pub mod fedora;
pub mod fedora_domination;
pub mod freebsd_jails;
pub mod freedos;
pub mod gap_closure;
pub mod garuda_zen;
pub mod gentoo;
pub mod gentoo_useflags;
pub mod historic_linux;
pub mod hopper_lab;
pub mod india_professional_tools;
pub mod india_stack;
pub mod india_stack_localization;
pub mod innovations;
pub mod installer;
pub mod interim;
pub mod jails;
pub mod jehanne;
pub mod kimi_code;
pub mod lattice;
pub mod lattice_grid;
pub mod legacy_adapters;
pub mod linux_adapter;
pub mod linux_compat;
pub mod linux_distro_parity;
pub mod linux_init;
pub mod linux_network;
pub mod linux_security;
pub mod linux_standards;
pub mod linuxulator;
pub mod localsend;
pub mod lubuntu;
pub mod macos_darwin;
pub mod mesh_hub;
pub mod mint_linux;
pub mod mobile_desktop_parity;
pub mod nixos;
pub mod nixos_reproducible;
pub mod oldlinux;
pub mod open_source_dominance;
pub mod open_source_tier1;
pub mod opensuse_slackware;
pub mod overtake;
pub mod penetration_assistant;
pub mod persona;
pub mod personality;
pub mod pop_os;
pub mod prism;
pub mod proxy;
pub mod reactos;
pub mod register_set;
pub mod relay_nexus;
pub mod scosmos;
pub mod sigmawin;
pub mod solid_kernel;
pub mod sovereign_suite;
pub mod superiority;
pub mod tiny_core;
pub mod wasm_sandbox;

pub use gap_closure::{
    KernelModuleManager, SyscallCompatibilityRegistry, DriverRepositoryManager,
    FirmwareBridgeManager, BuildLedgerSystem, SecurityPolicyManager,
    PeripheralEmulationLibrary, VirtualMemoryManager, NetworkStackGateway,
    HidGraphicsDriver, AiTaskOrchestrator, SovereignDistroAbsorptionEngine,
    OpenSourceCompetitorOrchestrator, TargetDistroFamily,
};
pub use superiority::{
    LockFreeQueue, NumaCfsScheduler, ShardIgnitor, SovereignCloudFS, SovereignForensics,
    SovereignObjectBus, SovereignRecoverUtility, SovereignRegistry, SovereignSigLoader,
    SovereignThemeEngine, SovereignTimeMachine,
};

pub use arch_linux::{
    ArchFirewall, ArchInitSystem, ArchMirror, ArchNewsFeedParser, ArchPackage,
    ArchWikiSearchEngine, ArchinstallConfig, ArchinstallParity, ArtixInitBridge,
    ArtixInitSystemType, AurPatch, AurPatchEngine, AurRepoStatus, CachedPackage, DevFile,
    DevFileType, FirewallRule, KeyTrustLevel, LsmMode, LsmSentinel, MkinitcpioGenerator, NewsItem,
    PacmanDbCleaner, PacmanEngine, PacmanError, PacmanKey, PacmanKeyring, PamGate, PaneLayout,
    ProcFile, ProcFileType, ReflectorMirrorlist, RuleAction, RunlevelTarget, ServiceState,
    SovereignEnvRegistry, SubvolumeConfig, SystemdBootMetrics, TmuxMultiplexer, WikiPage,
    YayParuAdapter,
};

pub use open_source_dominance::*;
pub use open_source_tier1::{
    LibsodiumIntegration, SmolTcpIntegration, SqliteIntegration, WasmerIntegration,
};

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use interim::{InterimLispVM, LispVal, MntReformLpcDriver, ReformPowerStats};
pub use lubuntu::{CpuGovernor, LubuntuHealthReport, LubuntuSystemManager, SystemPressure};

pub use cross_platform_kernel::{
    DeferredProcedureCall, IdtEntry, Idtr, Irql, IrqlController, Kpcr, Kpcrb, MemoryArch,
    PageAccessMode, PageDirectory as CrossPlatformPageDirectory, SovereignKernelInternals,
    SystemServiceTable, UmsContext, UmsThreadState,
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
pub use legacy_adapters::{
    KernelPersona, SyscallAbi, KernelPersonaVM, BinaryCompatMatrix, LibcVersion,
    LegacyDriverAdapter, LegacyFSAdapter, LegacyProtocolAdapter,
};

pub use chimera_linux::{
    ApkPackageMetadata, ApkPackageStore,
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

pub use antix::*;
pub use legacy_adapters::*;
pub use tiny_core::{FiletoolOverlay, FrugalLoader, TceLoader, TczExtension, TinyCoreBootConfig};

pub use apache_ossie::{
    MetricAggregation, OssieCatalog, OssieDimension, OssieInterpreter, OssieMetric, OssieOntology,
    OssieRelationship, SemanticRow,
};

pub use sovereign_suite::{
    CreativeMatrix, EverySearch, FancyZonesManager, ImageLayer, JoplinE2ee, LayoutZone,
    ProcMonitor, ProcessExplorerState, SpreadsheetCore, SysDiag,
};

pub use open_source_dominance::{
    InspirationFeatureMatrix, InspirationPackageIntegrator,
    InspirationSecurityGuard, OpenSourceDominanceEngine, OpenSourceInspirationTier,
};
