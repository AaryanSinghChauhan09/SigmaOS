// SigmaOS Compatibility Module
pub mod chimera_linux;
pub mod cross_platform;
pub mod interim;
pub mod lubuntu;
pub mod mint_linux;
pub mod reactos;
pub mod sigmawin;
pub mod relay_nexus;
pub mod solid_kernel;
pub mod india_stack_localization;
pub mod legacy_adapters;
pub mod cross_platform_kernel;
pub mod linux_adapter;
pub mod persona;
pub mod abi_translator;
pub mod lattice;
pub mod prism;
pub mod canonical;
pub mod chakra;
pub mod cross_platform;
pub mod zorin;
pub mod antix;

pub use zorin::{
    ZorinLayout, ZorinLayoutMetrics, ZorinLayoutSwitcher,
    ZorinChameleonColor, ZorinChameleonEngine,
    ZorinConnectState, ZorinConnectManager,
    ZorinWindowsAppSupport,
};

pub use antix::{
    AntiXInitSystem, AntiXServiceState, AntiXService, AntiXInitSwitcher,
    AntiXPersistenceMode, AntiXPersistenceManager, AntiXSystemRemasterEngine,
    AntiXControlCentre,
};
pub mod historic_linux;
pub mod mint_linux;
pub mod chimera_linux;
pub mod relay_nexus;
pub mod solid_kernel;
pub mod wasm_sandbox;
pub mod absorb_tools;
pub mod tiny_core;
pub mod apache_ossie;
pub mod sovereign_suite;
pub mod gentoo;
pub mod legacy_adapters;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use interim::{InterimLispVM, LispVal, MntReformLpcDriver, ReformPowerStats};
pub use lubuntu::{CpuGovernor, LubuntuHealthReport, LubuntuSystemManager, SystemPressure};

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
pub use linux_adapter::{
    LinuxKernelVersion, LegacyKernelAdapter, LegacyPackageAdapter, LegacySecurityAdapter, LegacyUIAdapter,
};
pub use persona::{
    PersonaVersion, KernelPersonaContainer, SyscallCategory, SyscallNode, SyscallGraph,
};
pub use abi_translator::{
    CpuArchitecture, ABITranslator,
};
pub use lattice::{
    LatticeFeature, KernelLattice, SyscallLifecycle, SyscallHistory, SyscallTracker,
};
pub use prism::{
    PrismFacet, KernelPrism, LedgerEntry, SyscallLedgerbook,
};

pub use canonical::{
    AiResourceScheduler, AppSuiteBundle, AppSuiteType, BrailleMatrix, BsdJailSandbox,
    CloudOrchestrator, CloudProvider, CompatBinary, CompatBinaryFormat, CompatibilityLayer,
    ContinuityCoordinator, DesktopMode, DistroReleaseChannel, EcosystemSnapshot, FlatpakApp,
    HandoffTask, LanguageTranslationCatalog, LocaleManager, ReleaseGovernanceCouncil,
    ReproducibleBuildVerifier, SigmaContainer, SnapshotManager, SuiteRegistry, TtsSynthesizer,
    UnifiedAppStore, ZorinAppearanceSwitcher,
};

pub use solid_kernel::{
    IScheduler, RoundRobinSchedulerPort, PrioritySchedulerPort, SolidKernelCore,
    ComplianceScheduler, AuditBlock, SigmaFSPlusPlus,
};

pub use wasm_sandbox::{
    WasmState, WasmModule, WasmSandboxEngine,
};

pub use absorb_tools::{
    PledgePermission, PledgeUnveilSandbox, PqcSecureChannel, Literal, Clause,
    DpllSatSolver, CasObject, ContentAddressedStorage,
};

pub use tiny_core::{
    TinyCoreBootConfig, TczExtension, TceLoader, FiletoolOverlay, FrugalLoader,
};

pub use apache_ossie::{
    MetricAggregation, OssieMetric, OssieDimension, OssieRelationship, OssieCatalog,
    SemanticRow, OssieInterpreter, OssieOntology,
};

pub use sovereign_suite::{
    EverySearch, SysDiag, ProcessExplorerState, ProcMonitor, CreativeMatrix, ImageLayer,
    FancyZonesManager, LayoutZone, JoplinE2ee, SpreadsheetCore,
};

pub use gentoo::{
    UseFlagManager, OpenRcRunlevel, ServiceStatus, OpenRcService, OpenRcManager,
    EbuildPackage, PortageEngine,
};

pub use fedora::{
    DnfPackageResolver, MockChrootBuilder, KojiBuildServer, BodhiUpdateTriage,
    SigmaChangeProposal, SigmaChangeProcessEngine, SigmaNextChannel, FedoraAluFlags,
    FedoraAlu, SeLinuxContext, SeLinuxEngine, SystemdPresetConfigurator, AnacondaInstaller,
};

pub use debian::{
    DebianChannel, AptRepositorySync, SysVRunlevel, SysVInitEngine, AlternativeLink,
    DebianAlternativesSystem, DebootstrapEngine,
};

pub use cachy_os::{
    BoreSchedulerGovernor, AnanicyManager, SchedPolicy, V4OptimizedPackageManager,
    CachyInitramfs,
};
