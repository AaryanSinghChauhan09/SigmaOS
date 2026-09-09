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

pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting, BrailleDisplay, ColorFilter, KeyID, KeyType,
    Magnifier, MagnifierID, MagnifierManager, OnScreenKeyboard, ScreenReader, SimpleBrailleDisplay,
    SimpleColorFilter, SimpleMagnifier, SimpleMagnifierManager, SimpleOnScreenKeyboard,
    SimpleScreenReader, SimpleStickyKeys, SimpleVirtualKey, SimpleVoice, StickyKeys, VirtualKey,
    Voice, VoiceGender, VoiceID,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, ScriptArgumentRouter,
    SystemAction, SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction,
    SystemState,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use container::{
    ContainerError, ContainerID, ContainerInfo, ContainerRuntime as CoreContainerRuntime,
    ContainerState, RuntimeCapability, RuntimeStats, SimpleContainer, SimpleContainerRuntime,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::statutory_compliance::{
    ComplianceRuleStatus, DisputeAuditRollbackEngine, PenaltyBreachNotifier, StatutoryBreachAlert,
    StatutoryFramework, StatutoryGovernanceLayer, StatutoryGovernanceRule,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use distro::{
    AdminAction, AiSysAdmin, AppBundleRuntime, AppManifest, AppsAuditTool, AptCacheSimulator,
    ArchBuildSystem, ArchMirror, ArchPacmanHooksManager, ArchRepoType, AuditResult, AuditRule,
    AurHelper, AurPackage, BackupSnapshot, BackupSystem, BoreSchedulerGovernor, BountyStatus,
    BsdSecureNtpConstraintSync, BsdStatefulPacketFilter, BugBountyProgram, BugBountyReport,
    BuildJob, BuildStatus, BundleError, CachyKernelVariant, CachyPackageRepo, CanFrame,
    CertificationStatus, ChannelManager, CloudInitBootstrapEngine, CommunityConference,
    ComplianceAuditor, ComponentType, ConferenceTalk, ConfigHook, CpuArchitecture, CpuCapabilities,
    CrossBuildPipeline, CrossbowVnic, DaxMemoryRegion, DebianPolicyEnforcer, DebianSocialContract,
    DevTool, DeveloperToolkit, DirectoryService, DirectoryUser, DllLoader, DllModule,
    DpkgMultiArch, DragonFlyHammerFs, EcuController, EduChallenge, EduPlayground, FlakeInput,
    ForumChannel, ForumPost, FreezeBasedStabilization, GNUGuixShepherdSupervisor, GdiObjectType,
    GentooPortageUseFlagsEngine, GuixDerivation, GuixFunctionalStore, HalError,
    Hammer2MultiMasterPfsReplication, Hammer2Snapshot, Hammer2TxgRecord, HardwareAbstractionLayer,
    HardwareCertificate, HardwareCertificationProgram, HardwareProfile, HardwareRegressionSuite,
    HelpSystem, HookAction, HookWhen, HowToGuide, HpcClusterJob, HpcJobState, ImeCandidate,
    InputMethodEngine, InstallationTarget, InstallerError, InstallerStep, IntegrityState,
    KernelTrace, LanguagePack, LinuxSyscall, LiveDebugger, LiveInstaller, LivepatchManager,
    LivepatchPatch, LocaleManager, ManPage, MicroArchLevel, MpiCommunicator, NetBsdRumpKernel,
    NetplanConfig, NetplanInterface, NetplanManager, NetplanYamlRenderer, NixOSFlakeEngine,
    OstreeDeployment, OstreeDeploymentEngine, P2pNode, PackageBuildService, PacmanHook,
    PacmanSyncManager, PacmanSyncPackage, PfRuleAction, PfStateEntry, PfStateSynchronizationEngine,
    PfSyncMessage, PfSyncMsgType, PfsClusterNode, PortagePackage, PosixTranslation, PqcSelfHealing,
    QAStagedRelease, RegionalSettings, RegistryType, RegistryValue, ReleaseStage, RescueISO,
    RescueISOManager, RumpKernelServer, RunitService, RunitServiceState, ServiceState,
    ShepherdService, ShepherdServiceState, SigmaAppBundle, SlackBuildCompiler, SlackPackage,
    SlackwarePkgTools, SnapperBtrfsEngine, SnapperSnapshot, SnapperType,
    SoftwareCertificationProgram, SolarisCrossbowVnicEngine, SovereignAnonScrubber,
    SovereignBundleRuntime, SovereignChannelManager, SovereignDeltaPackageSigner,
    SovereignDeltaPatch, SovereignHal, SovereignInstaller, SovereignP2PSync, SystemClosure,
    SystemStateStatus, TargetArch, TczExtensionManager, ThreeTierReleaseModel,
    TimeTravelCheckpoint, TimeTravelEngine, TinyCoreMode, TinyCoreRAMEngine, TlsConstraint,
    UpdateChannel, UpdateError, VirtioFsZeroCopyBridge, VoidRunitManager, VoidRunitSupervisor,
    WikiPage, Win32Gdi, WindowsRegistry, Yast2ControlCenter, YastSetting,
};
pub use driver::pci_bus::{
    PciAddress, PciBarInfo, PciBarType, PciBusManager, PciDeviceNode, PciDriverMatchRule,
    PciHardwareAccess, PciHeaderType, PciInterruptMode, PcieAerLog, PcieAerSeverity, PcieAspmState,
    SimulatedPciHardwareAccess,
};
pub use drivers::*;
pub use filesystem::{
    FileMode, FileType, FsError, Inode, VirtualFilesystem,
};
pub use governance::{
    FoundationModel, FoundationMember, ReleaseType, RoadmapMilestone, TransparentRoadmap,
    DemocraticProposal, DemocraticVoting,
};
// pub use ipc::{
//     StandardStreamController, StandardStreamHandle, StreamBufferMode, StreamTeeSpliceRouter,
//     STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO,
// };
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, Message, MemoryBlock, PAGE_SIZE,
    Priority, Process, ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler,
    VirtualCpu,
    IoUringEngine, IoUringOpcode, SubmissionQueueEntry, CompletionQueueEntry,
    BoundedBufferProducerConsumer, SoftIrqType, BottomHalfKernelThread, BroadcastReceiver,
    AndroidBroadcastReceiverRegistry,
    KernelFastPacketEngine, FastPacketFrame, XdpAction,
    KernelAccessController, LandlockPathRule, LandlockAccessRight,
    InteractiveHybridScheduler, HybridTask,
    CowStorageEngine, CowBlock,
    MemoryCompactionSuperpagesAllocator, PhysicalFrameBlock, SovereignCgroupGovernor, CgroupResourceLimits,
    LinuxRcuSynchronizationEngine, LinuxKernelWorkqueueEngine, LinuxKernelTimerWheel, LinuxCmaAllocatorEngine,
};
pub use kernel::roundrobin::SchedulerError as RoundRobinSchedulerError;
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack,
};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use remote::{
    FileTransfer, RemoteDesktop, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, LayoutPreset as TmuxLayoutPreset,
    PomodoroState, PomodoroTimer, ProductivityScore, SplitDirection as TmuxSplitDirection,
    TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::hardening;
pub use security::{
    AnonSurfShunt, AppSandboxEngine, ArithmeticSubstitutionDeobfuscator, CapabilityGate,
    CapabilityToken, ForensicStorageFilter, Permission, PledgeManager, PledgePromise, RoutingMode,
    SandboxPolicy,
};
pub use userland::shell::{
    Parser as UserlandShellParser, RedirectSpec, RedirectionEngine, Shell as UserlandShell,
    StreamTarget,
};
pub use shell::{
    ContextualCompleter, HistoryExpansionEngine, JobControlManager, ParameterExpansionEngine,
    PipelineExecutor, ShellCommand, ShellPledgeUnveilGuard, ShellSyntaxHighlighter,
    SimpleShellSession as ShellRepl, ZshPromptFormatter,
};
pub use sigpkg::{
    AptDebManifest, BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError,
    RecipeManager, SatSolver, Transaction,
};
pub use unimplemented_tools::{
    AdaptiveUxAgent, AiAnomalyFirewall, AiCodeAssistant, AiDependencyResolver,
    AiDifficultyDirector, AiFileOrganizer, AiScheduler, AiSearchAssistant, AiTaskbar,
    AppSandboxing, AudioEditor, CloudBackupUtility, CloudGaming, CodeProfiler, ControllerMapper,
    CrossDeviceSync, CrossLanguageBuildTool, DeclarativeBuildSystem, DocumentScanner,
    EmulatorManager, FlatpakSnapLayer, GameHubLauncher, GameModManager, GamePerformanceBooster,
    GameRecorder, GamifiedTodo, GanttChartPlanner, GestureControl, GuiAppStore, IotDeviceManager,
    MemoryLeakDetector, MeshNetworking, MindMapCreator, MultiMonitorManager, MusicLibraryManager,
    NaturalLanguageShell, OfflinePackageInstaller, PackagePublishingHub, PdfEditor,
    PluginMarketplace, PodcastRecorder, PrivacyDashboard, SecureContainer, SecureFileSharing,
    SmartNotificationManager, StaticAnalyzer, SubtitleEditor, VoiceControl, VrArRuntime,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
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
