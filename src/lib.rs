// SigmaOS Library
// Core library for SigmaOS operating system

// Core working modules
pub mod accessibility;
pub mod ai;
pub mod app;
pub mod auth;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod driver;
pub mod filesystem;
pub mod futuristic_modules;
pub mod kernel;
pub mod klib;
pub use klib::ZeroDependencyPrimitiveHub;
pub mod memory;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod process;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod runtime;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod storage;
pub mod thread;
pub use process::{
    AdvancedIpcHub, BsdRusage, CancellationType, CoreDumpMetadata, EventFd,
    JobControlLifecycleEngine, JobState, PosixMessage, PosixMessageQueue, ProcessCancelState,
    ProcessCancellationAndTerminationManager, ProcessControlError, ProcessJobEntry,
    ProcessVmReadWriteEngine, ProcessWaiterAndRusageCollector, SigQueuePayload, SovereignProcess,
    SovereignProcessManager, SovereignProcessState, WaitStatus, ZeroCopyIpcChannel, WCONTINUED,
    WNOHANG, WUNTRACED,
};
pub mod access;
pub mod community;
pub mod open_source_os_gap_closure;
pub mod tools;
pub use open_source_os_gap_closure::*;
pub mod open_source_obsoletion;
pub mod open_source_os_gap_closure;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod userland;

pub use distro::{
    missing_distro_innovations::{
        CompletionQueueEntry, IoUringEngine, IoUringOp, LinuxBsdSysctlEngine, SubmissionQueueEntry,
    },
    ApkChrootBuildSandboxEngine, ClusterNodeRole, CpuGovernorMode, FreeBsdGeomVdevTopology,
    GarudaZenPerformanceEngine, GeomVdevNode, GuixShepherdServiceEngine, HaStateEntry,
    HermeticClosureRecord, HermeticStoreClosureEngine, LandlockAccessType, LandlockV5Rule,
    NomadBsdLivePersistenceEngine, NomadBsdZfsDataset, OpenBsdFdPledgeGate, SchedExtTask,
    ScxSchedulerKind, ScxTaskState, SovereignDistroLeapSuite, SovereignHermeticCasStoreEngine,
    SovereignHighAvailabilityMeshEngine, SovereignLandlockV5Guard, SovereignSchedExtEngine,
    StoreClosurePackage, SystemGenerationRecord, ZfsPoolState, ZramCompressionAlgorithm,
};
pub use driver::{
    DkmsAbiRebuildEngine, DkmsModuleSpec, DriverHardwareCategory, DriverLicense,
    UbuntuAdditionalDriversRegistry, UbuntuCommonDriverEngine, UbuntuDriverPackage,
    UbuntuLivepatchDriverHook,
};
pub use package::bsd_linux_package_innovations::{
    AlpineApkWorldAndVirtualPkgEngine, AptPinRule, ArchSplitPackageHookRunnerEngine,
    DebconfPreseedEntry, DebconfQuestionType, DebianDebconfStatoverrideEngine,
    DpkgStatoverrideRule, FedoraDnf5AdvisoryAndDeltaRpmEngine, FlakeInputLock,
    FreeBsdPortsFlavoursAndVuxmlEngine, GentooPortageSubslotAndUseExpandEngine,
    HaikuHpkgPackageFsEngine, NixFlakesDevshellResolverEngine, NixGuixCasGcProfileEngine,
    OpenBsdPkgAddSignifyEngine, OpenSuseZypperVendorStickinessEngine, PpaRepository,
    SlackBuildInfo, SlackPackageRecord, SlackwarePkgtoolSlackBuildEngine,
    UbuntuPpaAptPinningEngine, XbpsSonameAndOrphanEngine, ZypperPackageOffer, ZypperRepository,
};
pub use security::{
    GksuAuthBackend, GksuDisplayServer, GksuExecutionRequest, GksuExecutionResult,
    GksuSecurityGuard, HardenedSyscallDispatcher, HardenedSyscallError, KaliAirgeddonWifiAudit,
    KaliMetasploitPayloadFilter, KaliWiresharkPacketAnalyzer, LibGksuGraphicalSudoEngine,
    MemoryAccessError, PagePermissions, PcapPacketHeader, PiaDedicatedIpBinding, PiaMaceAdBlocker,
    PiaMultiHopShadowsocksBridge, PiaPortForwardingEngine, PiaServerRegion, PiaSplitTunnelGovernor,
    PiaStrictKillSwitch, PiaVpnManager, RetpolineKptiMitigationEngine, SmepSmapEnforcer,
    SovereignKaslrEngine, SplitTunnelRule, WifiFrameType,
};
pub use unimplemented_features::{
    AlpineApkPackageIndex, AndroidApexContainerModuleEngine, AndroidApexModule,
    AntiXLowRamSysVInitGovernor, BareMetalPeripheralManager, BareMetalUnifiedPeripheral,
    DeepinDdeControlCenterEngine, DistroWatchParityMetricsHub, DragonFlyHammer2DeduplicationEngine,
    DragonFlyHammer2FsSnapshot, GenerationManager, GentooPortageMaskResolver, HaikuMediaTranslator,
    HaikuTranslatorEngine, Hammer2Block, Jbd2TransactionLedger, LegacyController, MageiaMirror,
    MageiaSynthesisPackage, MageiaUrpmiMccResolver, ManjaroHardwareDetectionEngine,
    ModernController, NetBsdRumpComponentEngine, NixOsDeclarativeConfigEngine, PciBusScanner,
    PhoronixAutomatedBenchmarkEngine, PhoronixTestSuiteRunner, PowerState,
    PuppyLinuxOverlayRamdiskEngine, RavenWidgetState, RockyAlmaLinuxEnterpriseLifecycleGovernor,
    RosettaDynamicBinaryTranslator, RumpComponent, RumpComponentType, SatSolverEngine,
    SerenityIpcEvent, SerenityOsAsyncIpcLoop, SlackwarePackage, SlackwarePkgtoolEngine,
    SolusEopkgDeltaPackage, SolusEopkgRavenGovernor, SovereignIpcBus,
    SteamOsGamescopeCompositorEngine, TargetArch, TinyCoreModularTczLoader, UdfVm,
    VoidXbpsContainerEngine, ZorinAppMapping, ZorinWinAppDbRegistry,
};
pub mod expanded_wiki_innovations;
pub use expanded_wiki_innovations::{
    GrowthDomainItem, SigmaosGrowthArchitectureEngine, StrategicImportItem,
    StrategicImportPlanEngine,
};
pub mod virtualization;

pub mod interrupt;

pub mod graphics {
    pub mod compositor;
    pub mod gpu_driver;
    pub mod nvidia_prime;
    pub mod paint;
    pub mod video;
}
pub mod hardware {
    pub mod compatibility;
    pub mod win32;
}
pub mod power {
    pub mod governor;
}
pub mod boot;
pub use boot::*;
pub mod toolchain {
    pub mod adapter;
    pub mod bootstrap;
    pub mod capsule;
    pub mod codex;
}
pub mod scheduler;
pub mod crypto {
    pub mod vectorized_pqc;
}

pub mod logging;
pub mod system;
pub mod update {
    pub mod distro_update_parity;
}
pub use update::distro_update_parity::{
    SovereignSystemUpdateAndTestingEngine, SystemDiagnosticReport,
};
pub mod installer;
pub mod iot;
pub mod ml;
pub mod performance;

// Temporarily disabled problematic modules
// pub mod accessibility;
// pub mod automation;
// pub mod compatibility;
// pub mod container;
// pub mod customization;
// pub mod dashboard;
// pub mod desktop;
// pub mod device;
// pub mod driver;
// pub mod filesystem;
// pub mod ml;
// pub mod network;
// pub mod observability;
// pub mod orchestration;
pub mod distro;
// pub mod package;
// pub mod performance;
// pub mod productivity;
// pub mod remote;
// pub mod resilience;
// pub mod shell;
// pub mod sigpkg;
// pub mod virtualization;
// pub mod graphics {
//     pub mod compositor;
//     pub mod paint;
//     pub mod video;
// }
// pub mod power {
//     pub mod governor;
// }
// pub mod ai {
//     pub mod agent;
//     pub mod orchestrator;
// }
// pub mod boot;
// pub mod system;
// pub mod installer;
