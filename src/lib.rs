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
pub mod crypto;
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
pub use desktop::{
    Gnome46MutterEngine, KdePlasma6Engine, LuminaBsdDesktopEngine, SwayRegolithWmEngine, Xfce418Engine,
};
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
pub mod sovereign_wiki_master_engine;
pub use sovereign_wiki_master_engine::*;
pub mod open_source_obsoletion;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod userland;

pub use distro::{
    missing_distro_innovations::{
        CompletionQueueEntry, IoUringEngine, IoUringOp, LinuxBsdSysctlEngine, SubmissionQueueEntry,
    },
    ApkChrootBuildSandboxEngine, ClusterNodeRole, CpuGovernorMode,
    DragonFlyHammer2EmergencyCowEngine, FedoraSelinuxMlsMcsGovernor, FreeBsdGeomVdevTopology,
    GarudaZenPerformanceEngine, GentooPortageSlotOperatorEngine, GeomVdevNode,
    GuixShepherdServiceEngine, HaStateEntry, HermeticClosureRecord, HermeticStoreClosureEngine,
    LandlockAccessType, LandlockV5Rule, NomadBsdLivePersistenceEngine, NomadBsdZfsDataset,
    OpenBsdFdPledgeGate, SchedExtTask, ScxSchedulerKind, ScxTaskState, SovereignDistroLeapSuite,
    SovereignDnsTlsResolverEngine, SovereignDynamicDevfsEngine, SovereignFastInitramfsGenerator,
    SovereignHermeticCasStoreEngine, SovereignHighAvailabilityMeshEngine,
    SovereignJournaldBinaryStorageEngine, SovereignLandlockV5Guard, SovereignSchedExtEngine,
    SovereignStatefulNatEngine, StoreClosurePackage, SystemGenerationRecord, ZfsPoolState,
    ZramCompressionAlgorithm, DebianMultiarchAptEngine, GarudaPerformanceTweakEngine,
    HardenedBsdPaxCfiEngine, NetBsdRumpUserlandEngine, SolusEopkgBudgieEngine,
    OmarchyAudioPipewireConfig, OmarchyModernDesktopEngine, OmarchyNerdFont,
    OmarchyNeovimPresetEngine, OmarchyTerminalFontConfig,
};
pub use unimplemented_features::{
    AlpineApkPackageIndex, Android15PrivateSpaceGovernor, AndroidApexContainerModuleEngine,
    AndroidApexModule, AntiXLowRamSysVInitGovernor, BareMetalPeripheralManager,
    BareMetalUnifiedPeripheral, DeepinDdeControlCenterEngine, DistroWatchParityMetricsHub,
    DragonFlyHammer2DeduplicationEngine, DragonFlyHammer2FsSnapshot, FrappeFrameworkDocTypeEngine,
    GenerationManager, GentooPortageMaskResolver, HaikuMediaTranslator, HaikuTranslatorEngine,
    Hammer2Block, HwbustersPowerSupplyMonitor, Jbd2TransactionLedger, LegacyController,
    MacOsSequoiaWindowManager, MageiaMirror, MageiaSynthesisPackage, MageiaUrpmiMccResolver,
    ManjaroHardwareDetectionEngine, ModernController, NetBsdRumpComponentEngine,
    NixOsDeclarativeConfigEngine, PciBusScanner, PhoronixAutomatedBenchmarkEngine,
    PhoronixTestSuiteRunner, PowerState, PuppyLinuxOverlayRamdiskEngine, RavenWidgetState,
    RockyAlmaLinuxEnterpriseLifecycleGovernor, RosettaDynamicBinaryTranslator, RumpComponent,
    RumpComponentType, SatSolverEngine, SerenityIpcEvent, SerenityOsAsyncIpcLoop, SlackwarePackage,
    SlackwarePkgtoolEngine, SolusEopkgDeltaPackage, SolusEopkgRavenGovernor, SovereignIpcBus,
    SteamOsGamescopeCompositorEngine, TargetArch, TinyCoreModularTczLoader, UdfVm,
    VoidXbpsContainerEngine, WindowsCopilotRecallAuditor, ZorinAppMapping, ZorinWinAppDbRegistry,
};
pub use package::bsd_linux_package_innovations::{
    AlpineApkWorldAndVirtualPkgEngine, ApkIndexMetadata, ApkSignatureKey, ApkV3SignatureEngine,
    AptBugReport, AptMarkRecord, AptMarkState, AptPinRule, ArchCachyosMicroarchOptimizationEngine,
    ArchCachyOsMicroarchBuildProfileEngine, ArchSplitPackageHookRunnerEngine, CasStorePath,
    CachedPackageFile, CommunityPackageBuildSource, CommunityRepoBackend,
    CoprAurBuildRepositoryGatewayEngine, DebconfPreseedEntry, DebconfQuestionType,
    DebianAptMarkPackageStateGovernor, DebianDebconfStatoverrideEngine,
    DebianDpkgTriggersAptListbugsGuardEngine, DeltaRpmSpec, DnfActionKind, DnfActionRecord,
    DnfTransactionItem, DpkgDivertEngine, DpkgDivertRule, DpkgStatoverrideRule, DpkgTrigger,
    DpkgTriggerKind, DragonFlyDportsHammer2SnapshotEngine, EbuildSlotRecord,
    FedoraDnf5AdvisoryAndDeltaRpmEngine, FedoraDnf5AdvisorySecurityEngine,
    FedoraDnfHistoryRollbackJournalEngine, FlakeInputLock, FreeBsdPkgAuditEngine,
    FreeBsdPortsFlavoursAndVuxmlEngine, GentooPortageEapiSlotOperatorEngine,
    GentooPortageSubslotAndUseExpandEngine, HaikuHpkgPackageFsEngine, Hammer2PfsSnapshot,
    MicroarchCompilerFlags, MicroarchRepoRoute, MicroarchitectureLevel, NetBsdPkginBinaryDatabaseEngine,
    NetBsdPkgsrcOptionsFrameworkEngine, NixCasStoreGcGovernor, NixFlakesDevshellResolverEngine,
    NixGuixCasGcProfileEngine, OpenBsdPkgAddSignifyEngine, OpenBsdSignifyBinaryIntegrityEngine,
    OpenSuseZypperVendorStickinessEngine, PacmanGpgKey, PacmanKeyTrust, PacmanKeyringEngine,
    PackageBuildAttestation, PackageBuildEnvironment, PkgAuditAdvisory, PkgSummaryRecord,
    PkgsrcOptionSpec, PortageEnvProfile, PortageEapiLevel, PortagePackageEnvEngine, PpaRepository,
    RestrictedPackageSpec, RpmDeltaReconstitutionEngine, SecurityAdvisoryDetail,
    SignifyPqcSignatureHeader, SlackBuildInfo, SlackPackageRecord, SlackwarePkgtoolSlackBuildEngine,
    SlotOperator, SovereignPackageBuildProvenanceEngine, UbuntuPpaAptPinningEngine, XbpsCachedPkg,
    XbpsDowngradeRepoEngine, XbpsRestrictedNonFreeLicenseEngine, XbpsSonameAndOrphanEngine,
    ZypperPackageOffer, ZypperRepository,
};
pub use security::{
    Dilithium5KernelSignatureVerifier, FedoraCryptoPolicyProfile, GksuAuthBackend,
    GksuDisplayServer, GksuExecutionRequest, GksuExecutionResult, GksuSecurityGuard,
    HardenedSyscallDispatcher, HardenedSyscallError, HybridPqcMeasurementEngine,
    KaliAirgeddonWifiAudit, KaliMetasploitPayloadFilter, KaliWiresharkPacketAnalyzer,
    LibGksuGraphicalSudoEngine, MemoryAccessError, PagePermissions, PcapPacketHeader,
    PiaDedicatedIpBinding, PiaMaceAdBlocker, PiaMultiHopShadowsocksBridge, PiaPortForwardingEngine,
    PiaServerRegion, PiaSplitTunnelGovernor, PiaStrictKillSwitch, PiaVpnManager,
    RetpolineKptiMitigationEngine, SmepSmapEnforcer, SovereignFirmitasAttestationEngine,
    SovereignKaslrEngine, SplitTunnelRule, Tpm2PcrBank, Tpm2PcrRegister, WifiFrameType,
    TPM2_PCR_COUNT,
};
pub use unimplemented_features::{
    AlpineApkPackageIndex, Android15PrivateSpaceGovernor, AndroidApexContainerModuleEngine,
    AndroidApexModule, AntiXLowRamSysVInitGovernor, BareMetalPeripheralManager,
    BareMetalUnifiedPeripheral, DeepinDdeControlCenterEngine, DistroWatchParityMetricsHub,
    DragonFlyHammer2DeduplicationEngine, DragonFlyHammer2FsSnapshot, FrappeFrameworkDocTypeEngine,
    GenerationManager, GentooPortageMaskResolver, HaikuMediaTranslator, HaikuTranslatorEngine,
    Hammer2Block, HwbustersPowerSupplyMonitor, Jbd2TransactionLedger, LegacyController,
    MacOsSequoiaWindowManager, MageiaMirror, MageiaSynthesisPackage, MageiaUrpmiMccResolver,
    ManjaroHardwareDetectionEngine, ModernController, NetBsdRumpComponentEngine,
    NixOsDeclarativeConfigEngine, PciBusScanner, PhoronixAutomatedBenchmarkEngine,
    PhoronixTestSuiteRunner, PowerState, PuppyLinuxOverlayRamdiskEngine, RavenWidgetState,
    RockyAlmaLinuxEnterpriseLifecycleGovernor, RosettaDynamicBinaryTranslator, RumpComponent,
    RumpComponentType, S6ServiceInitSupervisor, SatSolverEngine, SerenityIpcEvent,
    SerenityOsAsyncIpcLoop, SlackwarePackage, SlackwarePkgtoolEngine, SolusEopkgDeltaPackage,
    SolusEopkgRavenGovernor, SovereignIpcBus, SteamOsGamescopeCompositorEngine, TargetArch,
    TinyCoreModularTczLoader, UdfVm, UutilsCoreutilsZeroCopyBuffer, VoidXbpsContainerEngine,
    WindowsCopilotRecallAuditor, ZorinAppMapping, ZorinWinAppDbRegistry,
};
pub use unimplemented_tools::{
    ChainedAuditTrailLedger, DiskImageSignatureCarver, DistroWatchTrendAnalyzerTool,
    MetadataExifAntiForensicScrubber, NetworkPcapForensicSniffer, NixGuixStoreGarbageCollectorTool,
    OpenBsdUnveilAuditTool, PhoronixSuiteAutomatedBenchmarkRunnerTool,
    VolatileMemoryDumpForensicEngine,
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

pub mod distro;
pub mod distro_innovations;
pub mod distro_inspirations;
pub mod innovation;
pub use innovation::{
    BootStageKind, BootStageRecipe, ComposableBootSequencesEngine, DriverShard,
    FilesystemAsDatabaseEngine, HardwareAbstractionShardsEngine, ImmutableUserlandLayersEngine,
    KernelPersonality, LayeredKernelPersonalitiesEngine, LegacyAbiEnvironment,
    NetworkNativeOsStateEngine, OsSessionState, ProgrammableSchedulerEngine,
    RetroSandboxSession, RetrocompatibilitySandboxEngine, SchedulingPolicyRule,
    UserlandOverlayLayer, VfsObjectRecord,
};

pub mod docs;
pub mod graphics;
pub mod net;