extern crate alloc;
// SigmaOS Library
// Core library for SigmaOS operating system

extern crate alloc;

// Core working modules
pub mod ai;
pub mod app;
pub mod futuristic_modules;
pub mod auth;
pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod driver;
pub mod crypto;
pub mod filesystem;
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
    ProcessControlError, ProcessVmReadWriteEngine, JobState, CoreDumpMetadata, ProcessJobEntry,
    JobControlLifecycleEngine, WNOHANG, WUNTRACED, WCONTINUED, BsdRusage, WaitStatus,
    ProcessWaiterAndRusageCollector, CancellationType, ProcessCancelState,
    ProcessCancellationAndTerminationManager, PosixMessage, PosixMessageQueue, EventFd,
    SigQueuePayload, AdvancedIpcHub, SovereignProcessState, SovereignProcess, ZeroCopyIpcChannel,
    SovereignProcessManager,
};
pub mod community;
pub mod access;
pub mod tools;
pub mod open_source_os_gap_closure;
pub use open_source_os_gap_closure::*;
pub mod sovereign_wiki_master_engine;
pub use sovereign_wiki_master_engine::*;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod userland;
pub mod open_source_obsoletion;

pub use package::bsd_linux_package_innovations::{
    AlpineApkWorldAndVirtualPkgEngine, AptBugReport, AptPinRule,
    ArchCachyosMicroarchOptimizationEngine, ArchSplitPackageHookRunnerEngine,
    CommunityPackageBuildSource, CommunityRepoBackend, CoprAurBuildRepositoryGatewayEngine,
    DebconfPreseedEntry, DebconfQuestionType, DebianDebconfStatoverrideEngine,
    DebianDpkgTriggersAptListbugsGuardEngine, DpkgStatoverrideRule, DpkgTrigger, DpkgTriggerKind,
    DragonFlyDportsHammer2SnapshotEngine, EbuildSlotRecord, FedoraDnf5AdvisoryAndDeltaRpmEngine,
    FlakeInputLock, FreeBsdPortsFlavoursAndVuxmlEngine, GentooPortageEapiSlotOperatorEngine,
    GentooPortageSubslotAndUseExpandEngine, HaikuHpkgPackageFsEngine, Hammer2PfsSnapshot,
    MicroarchRepoRoute, MicroarchitectureLevel, NetBsdPkgsrcOptionsFrameworkEngine,
    NixFlakesDevshellResolverEngine, NixGuixCasGcProfileEngine, OpenBsdPkgAddSignifyEngine,
    OpenSuseZypperVendorStickinessEngine, PkgsrcOptionSpec, PortageEapiLevel, PpaRepository,
    SlackBuildInfo, SlackPackageRecord, SlackwarePkgtoolSlackBuildEngine, SlotOperator,
    UbuntuPpaAptPinningEngine, XbpsSonameAndOrphanEngine, ZypperPackageOffer, ZypperRepository,
};
pub use unimplemented_features::{
    Android15PrivateSpaceGovernor, AndroidApexContainerModuleEngine, AndroidApexModule,
    DeepinDdeControlCenterEngine, DistroWatchParityMetricsHub, FrappeFrameworkDocTypeEngine,
    HwbustersPowerSupplyMonitor, MacOsSequoiaWindowManager, ManjaroHardwareDetectionEngine,
    PhoronixAutomatedBenchmarkEngine, PhoronixTestSuiteRunner, PuppyLinuxOverlayRamdiskEngine,
    RockyAlmaLinuxEnterpriseLifecycleGovernor, RosettaDynamicBinaryTranslator,
    SteamOsGamescopeCompositorEngine, TargetArch, TinyCoreModularTczLoader, VoidXbpsContainerEngine,
    WindowsCopilotRecallAuditor,
    AntiXLowRamSysVInitGovernor, BareMetalPeripheralManager, BareMetalUnifiedPeripheral,
    GenerationManager, GentooPortageMaskResolver, HaikuMediaTranslator, HaikuTranslatorEngine, Jbd2TransactionLedger,
    LegacyController, ModernController, PciBusScanner, PowerState, SatSolverEngine,
    SerenityIpcEvent, SerenityOsAsyncIpcLoop, SovereignIpcBus, UdfVm, ZorinAppMapping,
    ZorinWinAppDbRegistry, AlpineApkPackageIndex, DragonFlyHammer2FsSnapshot, NixOsDeclarativeConfigEngine,
    SlackwarePkgtoolEngine, SlackwarePackage, SolusEopkgRavenGovernor, SolusEopkgDeltaPackage, RavenWidgetState,
    MageiaUrpmiMccResolver, MageiaSynthesisPackage, MageiaMirror, DragonFlyHammer2DeduplicationEngine, Hammer2Block,
    NetBsdRumpComponentEngine, RumpComponent, RumpComponentType,
};
pub use distro::{
    ApkChrootBuildSandboxEngine, OpenBsdFdPledgeGate, FreeBsdGeomVdevTopology, GeomVdevNode,
    HermeticStoreClosureEngine, StoreClosurePackage,
    NuttxRealtimeTaskGovernor, NuttxTask, OpenBsdVmmBhyveHypervisorBridge, MicroVmGuest, VmState,
    IllumosDTraceProbeProvider, DTraceProbe, GentooPortageEapi8SlotResolver, EbuildPackageRecord,
    missing_distro_innovations::{LinuxBsdSysctlEngine, IoUringEngine, IoUringOp, SubmissionQueueEntry, CompletionQueueEntry},
    CpuGovernorMode, GarudaZenPerformanceEngine, GuixShepherdServiceEngine,
    NomadBsdLivePersistenceEngine, NomadBsdZfsDataset, ZfsPoolState, ZramCompressionAlgorithm,
    SovereignSchedExtEngine, ScxSchedulerKind, ScxTaskState, SchedExtTask,
    SovereignLandlockV5Guard, LandlockAccessType, LandlockV5Rule,
    SovereignHermeticCasStoreEngine, HermeticClosureRecord, SystemGenerationRecord,
    SovereignHighAvailabilityMeshEngine, ClusterNodeRole, HaStateEntry,
    SovereignDistroLeapSuite, DragonFlyHammer2EmergencyCowEngine, SovereignFastInitramfsGenerator,
    GentooPortageSlotOperatorEngine, FedoraSelinuxMlsMcsGovernor,
    SovereignDnsTlsResolverEngine, SovereignDynamicDevfsEngine, SovereignStatefulNatEngine,
    SovereignJournaldBinaryStorageEngine,
};
pub use security::{
    HardenedSyscallDispatcher, HardenedSyscallError, MemoryAccessError,
    PagePermissions, RetpolineKptiMitigationEngine, SmepSmapEnforcer, SovereignKaslrEngine,
    KaliAirgeddonWifiAudit, KaliMetasploitPayloadFilter, KaliWiresharkPacketAnalyzer,
    PcapPacketHeader, WifiFrameType, PiaDedicatedIpBinding, PiaMaceAdBlocker,
    PiaMultiHopShadowsocksBridge, PiaPortForwardingEngine, PiaServerRegion,
    PiaSplitTunnelGovernor, PiaStrictKillSwitch, PiaVpnManager, SplitTunnelRule,
    GksuAuthBackend, GksuDisplayServer, GksuExecutionRequest, GksuExecutionResult,
    GksuSecurityGuard, LibGksuGraphicalSudoEngine,
    Dilithium5KernelSignatureVerifier, FedoraCryptoPolicyProfile, HybridPqcMeasurementEngine,
    SovereignFirmitasAttestationEngine, Tpm2PcrBank, Tpm2PcrRegister, TPM2_PCR_COUNT,
};
pub use driver::{
    DkmsAbiRebuildEngine, DkmsModuleSpec, DriverHardwareCategory, DriverLicense,
    UbuntuAdditionalDriversRegistry, UbuntuCommonDriverEngine, UbuntuDriverPackage,
    UbuntuLivepatchDriverHook,
};
pub mod expanded_wiki_innovations;
pub use expanded_wiki_innovations::{
    GrowthDomainItem, SigmaosGrowthArchitectureEngine, StrategicImportItem, StrategicImportPlanEngine,
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
pub mod performance;
pub mod ml;
pub mod iot;

pub mod distro;
