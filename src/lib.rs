extern crate alloc;
// SigmaOS Library
// Core library for SigmaOS operating system

// Core working modules
pub mod accessibility;
pub mod ai;
pub mod audio;
pub mod ipc;
pub mod init;
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
pub mod drivers;
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
pub mod wiki_unimplemented_ideas;
pub use wiki_unimplemented_ideas::*;

pub use distro::{
    distro_inspiration_engine::{
        AlpineLbuApkOverlayEngine, ApkovlCommit, ArcCacheBlock, ArcState, ClearLinuxIsaSelectorEngine,
        DragonFlyHammer2ClusterEngine, FreeBsdZfsArcGeomEngine, GenerationRecord, Hammer2DedupEntry,
        IsaLevel, MuslLightweightInitEngine, MuslStaticService,
        NixOsDeclarativeStateReconciliationEngine, NixOsPureStoreDerivationEngine,
        OpenBsdStatefulPacketFilterEngine, OpenWrtUciSqmRouterEngine, PaxSecurityLevel, PfProtocol,
        PfStateEntry, PortageUseFlag, PortageUseFlagGovernor, QubeDomainType,
        QubesHardenedBsdSecurityGuard, RunitStage, ServiceRunState, SqmAlgorithm, StoreDerivationPath,
        UciSection, UseFlagState, VoidRunitStageController,
    },
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
    SovereignStatefulNatEngine, SovereignAheadOfDistrosSuite, SovereignPredictiveSchedExtEngine,
    SovereignOmniCasStoreEngine, SovereignCrossPlatformCapabilityEngine,
    SovereignResilientHammer2Engine, SovereignUniversalMicroarchEngine, SovereignXdpCarpMeshEngine,
    StoreClosurePackage, SystemGenerationRecord, ZfsPoolState,
    ZramCompressionAlgorithm, DebianMultiarchAptEngine, EndeavourReflectorMirrorRanker,
    FreeBsdZfsBootEnvManager, GarudaDracutBtrfsSnapper, GarudaPerformanceTweakEngine,
    HardenedBsdPaxCfiEngine, NetBsdRumpUserlandEngine, NixOsFlakeProfileManager,
    OpenBsdDoasPrivilegeManager, SolusEopkgBudgieEngine, BodhiUpdateRecord, BodhiUpdateStatus,
    AlpmHook, AlpmHookWhen, ArchGpgKey, ArchKeyringEngine, ArchPacmanHookManager,
    ArchCommunitySigRepoManager, ArchPacmanContribEngine, ArchSignstarSignerEngine,
    PacDiffCandidate, PacLogEntry, SigRepositoryBranch, SignstarAttestation,
    ArchReflectorEngine, ArchinstallEngine, ArchFilesystemType, ArchInstallerProfile,
    KeyTrustLevel, ReflectorMirror, ReflectorSortKey,
    DesktopLayoutPreset, HardwareQuirkRule, ManjaroBranch, ManjaroBranchManager,
    ManjaroHelloSetupEngine, ManjaroTimeshiftAutoSnap, MhwdHardwareQuirkDatabase,
    MhwdKernelDriverAutobuilder, PackageSearchResult, PamacTransactionEntry,
    PamacTransactionJournalEngine, PamacUnifiedSearchEngine, PrimeOffloadMode,
    SearchResultBackend, SetupWizardTask, ManjaroSnapshotMode, TimeshiftSnapshot,
    PamacTransactionType, VendorHardwareType,
    FedoraBodhiUpdateEngine, FedoraIgnitionProvisionEngine, FedoraKojiDistGitBuilder,
    FedoraRpmOstreeEngine, IgnitionFile, IgnitionUnit, KojiBuildTask, OstreeDeploymentPin,
    FedoraTargetArchitecture,
    OmarchyAudioPipewireConfig, OmarchyModernDesktopEngine, OmarchyNerdFont,
    OmarchyNeovimPresetEngine, OmarchyTerminalFontConfig,
};
pub use driver::{
    DkmsAbiRebuildEngine, DkmsModuleSpec, DriverHardwareCategory, DriverLicense,
    UbuntuAdditionalDriversRegistry, UbuntuCommonDriverEngine, UbuntuDriverPackage,
    UbuntuLivepatchDriverHook,
};
pub use package::bsd_linux_package_innovations::{
    AlpineApkCachePeerSyncEngine, AlpineApkEdgeOverlayEngine, AlpineApkWorldAndVirtualPkgEngine,
    AlternativeProvider, ApkIndexMetadata, ApkPeerNode, ApkRepositoryOverlay, ApkSignatureKey,
    ApkV3SignatureEngine, AptBugReport, AptListChangesNewsAuditorEngine, AptMarkRecord, AptMarkState,
    AptNewsEntry, AptPinRule, ArchCachyosMicroarchOptimizationEngine, ArchCachyOsMicroarchBuildProfileEngine,
    ArchPacmanParallelDownloadEngine, ArchSplitPackageHookRunnerEngine, CasStorePath, CachedPackageFile,
    CommunityPackageBuildSource, CommunityRepoBackend, CoprAurBuildRepositoryGatewayEngine,
    DebconfPreseedEntry, DebconfQuestionType, DebianAptMarkPackageStateGovernor, DebianDebconfStatoverrideEngine,
    DebianDpkgTriggersAptListbugsGuardEngine, DeltaRpmSpec, DnfActionKind, DnfActionRecord, DnfTransactionItem,
    DpkgDivertEngine, DpkgDivertRule, DpkgStatoverrideRule, DpkgTrigger, DpkgTriggerKind,
    DragonFlyDportsHammer2SnapshotEngine, EbuildSlotRecord, EtcUpdateOverlayCommit, FedoraDnf5AdvisoryAndDeltaRpmEngine,
    FedoraDnf5AdvisorySecurityEngine, FedoraDnfHistoryRollbackJournalEngine, FedoraModularityModulemdEngine,
    FlakeInputLock, FreeBsdPkgAuditEngine, FreeBsdPkgMessageNotifierEngine, FreeBsdPortsFlavoursAndVuxmlEngine,
    FreeBsdPoudriereMatrixEngine, GentooPortageEapiSlotOperatorEngine, GentooPortageSubslotAndUseExpandEngine,
    HaikuHpkgPackageFsEngine, Hammer2PfsSnapshot, MicroarchCompilerFlags, MicroarchRepoRoute, MicroarchitectureLevel,
    ModulemdStreamSpec, NetBsdPkginBinaryDatabaseEngine, NetBsdPkgsrcOptionsFrameworkEngine, NixCasStoreGcGovernor,
    NixFlakesDevshellResolverEngine, NixGuixCasGcProfileEngine, NixGuixStoreDeduplicatorEngine,
    OpenBsdPkgAddSignifyEngine, OpenBsdPledgeUnveilSandboxScriptletEngine, OpenBsdSignifyBinaryIntegrityEngine,
    OpenSuseZypperVendorStickinessEngine, OstreeLayeredDeployment, PacdiffCandidate, PacdiffConfigMergeGovernorEngine,
    PacdiffMergeDecision, PacmanGpgKey, PacmanKeyTrust, PacmanKeyringEngine, PackageBuildAttestation,
    PackageBuildEnvironment, ParallelDownloadTask, PkgAuditAdvisory, PkgMessageNotice, PkgSummaryRecord,
    PkgsrcOptionSpec, PortageEnvProfile, PortageEapiLevel, PortageEtcUpdateGitOverlayEngine, PortagePackageEnvEngine,
    PoudriereBuildTask, PoudriereJailSpec, PpaRepository, RestrictedPackageSpec, RpmDeltaReconstitutionEngine,
    RpmOstreeLayeredImageGovernorEngine, ScriptletSandboxPolicy, SecurityAdvisoryDetail, SignifyPqcSignatureHeader,
    SlackBuildInfo, SlackPackageRecord, SlackwarePkgtoolSlackBuildEngine, SlotOperator,
    SovereignPackageBuildProvenanceEngine, StoreFileMetadata, UbuntuPpaAptPinningEngine, UnveilPathRule,
    XbpsCachedPkg, XbpsDebianAlternativesGovernorEngine, XbpsDowngradeRepoEngine, XbpsRestrictedNonFreeLicenseEngine,
    XbpsSonameAndOrphanEngine, XbpsSrcTemplate, XbpsSrcTemplateSandboxEngine, ZypperPackageOffer, ZypperRepository,
};

pub use security::{
    Dilithium5KernelSignatureVerifier, FedoraCryptoPolicyProfile, GksuAuthBackend,
    GksuDisplayServer, GksuExecutionRequest, GksuExecutionResult, GksuSecurityGuard,
    HardenedSyscallDispatcher, HardenedSyscallError, HybridPqcMeasurementEngine,
    HashMode, KaliAirgeddonWifiAudit, KaliBurpSuiteWebProxy, KaliHashcatGpuCracker,
    KaliHydraPasswordBruteforce, KaliJohnTheRipperCracker, KaliMetasploitPayloadFilter,
    KaliNiktoWebScanner, KaliNmapPortScanner, KaliSqlmapInjectionAuditor, KaliUndercoverThemeMode,
    KaliWiresharkPacketAnalyzer, ScanTechnique,
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
    RumpComponentType, SatSolverEngine, SerenityIpcEvent, SerenityOsAsyncIpcLoop, SlackwarePackage,
    SlackwarePkgtoolEngine, SolusEopkgDeltaPackage, SolusEopkgRavenGovernor, SovereignIpcBus,
    SteamOsGamescopeCompositorEngine, TargetArch, TinyCoreModularTczLoader, UdfVm,
    VoidXbpsContainerEngine, WindowsCopilotRecallAuditor, ZorinAppMapping, ZorinWinAppDbRegistry,
};
pub mod expanded_wiki_innovations;
pub use expanded_wiki_innovations::{
    GrowthDomainItem, SigmaosGrowthArchitectureEngine, StrategicImportItem,
    StrategicImportPlanEngine,
};
pub mod virtualization;

pub mod interrupt;


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
pub use system::{AutomationTaskKind, SovereignAutomationEngine, TaskStatus};
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
pub mod arch_kernel_inspirations;
pub mod distro_inspirations;
pub mod linuxmint_inspirations;
pub mod innovation;
pub use innovation::{
    BootStageKind, BootStageRecipe, ComposableBootSequencesEngine, DriverShard,
    FilesystemAsDatabaseEngine, HardwareAbstractionShardsEngine, ImmutableUserlandLayersEngine,
    KernelPersonality, LayeredKernelPersonalitiesEngine, LegacyAbiEnvironment,
    NetworkNativeOsStateEngine, OsSessionState, ProgrammableSchedulerEngine,
    RetroSandboxSession, RetrocompatibilitySandboxEngine, SchedulingPolicyRule,
    UserlandOverlayLayer, VfsObjectRecord,
};

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{SystemdEngine, SystemdUnit, UnitState, UnitType};

pub use ai::next_gen::{
    AIModel, AdaptiveKernelPersona, AiTask, DeviceTargetType, EnergyGovernorMode, ModelType,
    MultiModelOrchestrator, PredictiveSyscallTranslator, WorkloadType,
};
pub use ai::wandr::{
    ResearchResult, SigmaWandrAgent, WandrDocument, WandrEvaluator, WandrResearchAgent, WandrTask,
};

pub use community::toolkit::{
    ArticleCategory, CommunityHandbookCatalog, HandbookArticle, HybridFirewallTemplateStore,
    PackageRecipe as CommunityPackageRecipe, RecipeSourceFormat, ReproduciblePackageRecipeManager,
    SecurityModelType, SecurityProfileTemplateStore, SecurityTemplate,
    VirtualizationBlueprintStore,
};

pub use tools::{
    AccessibilityFeature as LibAccessibilityFeature, ClusterNode as LibClusterNode,
    NodeState as LibNodeState, SigmaAccess as LibSigmaAccess, SigmaCluster as LibSigmaCluster,
    SigmaDeploy as LibSigmaDeploy, SigmaIdentity as LibSigmaIdentity,
    SigmaToolError as LibSigmaToolError, SovereignAptDuo, SovereignDpkgEtcher,
    SovereignImageToDataUri, SovereignImeConvertCase, SovereignIsWebsiteDown,
    SovereignKeyboardTester, SovereignTableConverter, SovereignTextFixer, SovereignWordCounter,
    UserIdentity as LibUserIdentity,
};
pub use tools::native_userland_replacements::MasterNativeUserlandReplacements;
