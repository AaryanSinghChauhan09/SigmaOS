// SigmaOS Library
// Core library for SigmaOS operating system

extern crate alloc;

// Core working modules
pub mod kernel;
pub mod klib;
pub mod memory;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod process;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod storage;
pub mod thread;
pub mod process;
pub use process::{
    ProcessControlError, ProcessVmReadWriteEngine, JobState, CoreDumpMetadata, ProcessJobEntry,
    JobControlLifecycleEngine, WNOHANG, WUNTRACED, WCONTINUED, BsdRusage, WaitStatus,
    ProcessWaiterAndRusageCollector, CancellationType, ProcessCancelState,
    ProcessCancellationAndTerminationManager, PosixMessage, PosixMessageQueue, EventFd,
    SigQueuePayload, AdvancedIpcHub, SovereignProcessState, SovereignProcess, ZeroCopyIpcChannel,
    SovereignProcessManager,
};
pub mod community;
pub mod memory;
pub mod access;
pub mod tools;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod userland;

pub use unimplemented_features::{
    AntiXLowRamSysVInitGovernor, BareMetalPeripheralManager, BareMetalUnifiedPeripheral,
    GenerationManager, GentooPortageMaskEngine, HaikuMediaTranslator, HaikuTranslatorEngine, Jbd2TransactionLedger,
    LegacyController, ModernController, PciBusScanner, PowerState, SatSolverEngine,
    SerenityIpcEvent, SerenityOsAsyncIpcLoop, SovereignIpcBus, UdfVm, ZorinAppMapping,
    ZorinWinAppDbRegistry, AlpineApkPackageIndex, DragonFlyHammer2FsSnapshot, NixOsDeclarativeConfigEngine,
};
pub mod expanded_wiki_innovations;
pub mod virtualization;

pub mod interrupt;

pub mod graphics {
    pub mod compositor;
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
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
    ScriptArgumentRouter,
};
pub use compatibility::{
    AntiXInitSystem, AntiXServiceState, AntiXService, AntiXInitSwitcher,
    AntiXPersistenceMode, AntiXPersistenceManager, AntiXSystemRemasterEngine,
    AntiXControlCentre, ZorinLayout, ZorinLayoutMetrics, ZorinLayoutSwitcher,
    ZorinChameleonColor, ZorinChameleonEngine, ZorinConnectState, ZorinConnectManager,
    ZorinWindowsAppSupport,
    ApplicationBinary, BinaryFormat, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ContainerRuntime,
    LegacyKernelAdapter, LegacyPackageAdapter, LegacySecurityAdapter,
    LegacyUIAdapter, TargetPlatform, TranslationLayer,
};
pub use container::{
    ContainerCapability, ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability, RuntimeStats,
    SimpleContainer, SimpleContainerRuntime,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use dashboard::statutory_compliance::{
    ComplianceRuleStatus, DisputeAuditRollbackEngine, PenaltyBreachNotifier, StatutoryBreachAlert,
    StatutoryFramework, StatutoryGovernanceLayer, StatutoryGovernanceRule,
};
pub use driver::pci_bus::{
    PciAddress, PciBarInfo, PciBarType, PciBusManager, PciDeviceNode, PciDriverMatchRule,
    PciHardwareAccess, PciHeaderType, PciInterruptMode, PcieAerLog, PcieAerSeverity,
    PcieAspmState, SimulatedPciHardwareAccess,
};
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use governance::{
    MilestoneCategory, OkrError, OkrTracker, StrategicMilestone, StrategicOkrEvaluator,
};
pub mod distro;

pub use kernel::{
    Apc, ApcMode, ApcQueue, ArchitectureEngine,
    BoundedBufferProducerConsumer, SoftIrqType, BottomHalfKernelThread, BroadcastReceiver,
    AndroidBroadcastReceiverRegistry,
    AuditBlock, BuddyAllocator, Channel, CircularDoublyLinkedList, CpuArchitectureClass,
    CpuRegisters, EdfTask, HardwareException, InstructionCyclePhase as ArchInstructionCyclePhase,
    InstructionCyclePhase, InterruptClass, IpcError, IpcManager, Irql,
    LcgRandom, LookasideList, LotteryTask, MemoryBlock,
    MemoryDescriptorList, Message, Pcb, PolicyManager, PolicyError, FastPathIpc, InterruptMechanism,
    ProtectionDomain, ResourceBroker, PrivilegeLevel, PoolType, Priority, Process,
    ProcessState, ProcessorInitState, RoundRobinConfig, RoundRobinScheduler, Scheduler,
    RoundRobinSchedulerError, SequencedSinglyLinkedList, SinglyLinkedList, SystemThread,
    Tcb, ThreadState, WorkItem, PAGE_SIZE,
};
pub use kernel::io_uring::{IoUringEngine, IoUringOpcode, SubmissionQueueEntry, CompletionQueueEntry};
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack,
};
pub use distro::{
    AppManifest, CertificationStatus, ComponentType, HardwareCertificate,
    HardwareCertificationProgram, HardwareProfile, HardwareRegressionSuite, QAStagedRelease,
    ReleaseStage, SoftwareCertificationProgram,
    BountyStatus, BugBountyProgram, BugBountyReport, CommunityConference, ConferenceTalk,
    ForumChannel, ForumPost, HelpSystem, HowToGuide, ManPage, WikiPage,
    DllLoader, DllModule, GdiObjectType, LinuxSyscall, PosixTranslation, RegistryType,
    RegistryValue, Win32Gdi, WindowsRegistry,
    BuildJob, BuildStatus, CrossBuildPipeline, DevTool, DeveloperToolkit, PackageBuildService,
    TargetArch,
    AuditResult, AuditRule, ComplianceAuditor, ConfigHook, DirectoryService, DirectoryUser,
    ImeCandidate, InputMethodEngine, LanguagePack, LocaleManager, RegionalSettings,
    AdminAction, AiSysAdmin, IntegrityState, P2pNode, PqcSelfHealing, SovereignP2PSync,
    TimeTravelCheckpoint, TimeTravelEngine, NetplanConfig, NetplanManager,
    LivepatchPatch, LivepatchManager,
    BackupSnapshot, BackupSystem, KernelTrace, LiveDebugger, RescueISO, RescueISOManager,
    CanFrame, EcuController, EduChallenge, EduPlayground, HpcClusterJob, HpcJobState,
    MpiCommunicator,
    BsdSecureNtpConstraintSync, BsdStatefulPacketFilter, DaxMemoryRegion, DragonFlyHammerFs,
    Hammer2MultiMasterPfsReplication, Hammer2Snapshot, Hammer2TxgRecord, PfRuleAction,
    PfStateEntry, PfStateSynchronizationEngine, PfSyncMessage, PfSyncMsgType, PfsClusterNode,
    RunitService, RunitServiceState, SovereignAnonScrubber, SovereignDeltaPackageSigner,
    SovereignDeltaPatch, TlsConstraint, VirtioFsZeroCopyBridge, VoidRunitManager,
    SlackPackage, SlackwarePkgTools, SlackBuildCompiler, GuixDerivation, GuixFunctionalStore,
    ShepherdServiceState, ShepherdService, GNUGuixShepherdSupervisor, OstreeDeployment,
    OstreeDeploymentEngine, CrossbowVnic, SolarisCrossbowVnicEngine, RumpKernelServer,
    NetBsdRumpKernel, NetplanInterface, NetplanYamlRenderer, CloudInitBootstrapEngine,
    YastSetting, Yast2ControlCenter, SnapperType, SnapperSnapshot, SnapperBtrfsEngine,
    Generation, NixDeclarativeSystemState, SigpkgRecipe, ArchRecipeSandboxCompiler,
    SnapperTransactionGuard, SigmaZeroCopySpliceEngine,
    PolicyAction, EbpfSyscallPolicyVerifier, CapsicumCapability, FreeBsdCapsicumDescriptorDelegate,
    CAP_READ, CAP_WRITE, CAP_SEEK, CAP_FSTAT, SystemdUnitType, SystemdUnitActiveState, SystemdUnit,
    SovereignSystemdParityEngine, SchedulerClass, RealtimeTask, SovereignHybridSchedulerInnovations,
    ClearLinuxStatelessEngine, TailsAmnesicEngine, DinitServiceState, DinitService,
    ChimeraDinitSupervisor, SolusEopkgManager, MageiaUrpmiEngine, AlpineApkWorldEngine,
    VoidXbpsEngine, VnetStack, FreeBsdVnetStackEngine, UnveilAuditViolation, OpenBsdUnveilAuditor,
};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager, PinPriority, PackagePinEngine,
    MirrorSyncEngine, PackageTransactionJournal,
};
pub use remote::{
    FileTransfer, RemoteDesktop, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
    SplitDirection as TmuxSplitDirection, LayoutPreset as TmuxLayoutPreset,
    TmuxPane, TmuxWindow, TmuxSession, TmuxSessionManager,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::hardening;
pub use security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken,
    ArithmeticSubstitutionDeobfuscator,
    ForensicStorageFilter, Permission, PledgeManager,
    PledgePromise, RoutingMode, SandboxPolicy,
};
pub use shell::{
    ShellCommand, SimpleShellSession as ShellRepl, RedirectionType, ParsedPipelineCommand,
    SovereignBashZshParityShell,
};
pub use sigpkg::{
    AptDebManifest, BuildSystem, ContentAddressedStore, CryptoVerifier,
    PackageRecipe, RecipeError, RecipeManager, SatSolver,
    Transaction, UniversalPackageAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use unimplemented_tools::{
    AudioEditor, PodcastRecorder, SubtitleEditor, MemoryLeakDetector, GamifiedTodo, MindMapCreator,
    GameHubLauncher, EmulatorManager, GameRecorder, GamePerformanceBooster, CloudGaming, VrArRuntime,
    ControllerMapper, GameModManager, AiDifficultyDirector, GanttChartPlanner, PdfEditor,
    DocumentScanner, CodeProfiler, StaticAnalyzer, PackagePublishingHub, AdaptiveUxAgent,
    AiSearchAssistant, NaturalLanguageShell, AiCodeAssistant, AiFileOrganizer, SmartNotificationManager,
    MeshNetworking, IotDeviceManager, CloudBackupUtility, SecureFileSharing,
    GuiAppStore, MultiMonitorManager, GestureControl, VoiceControl, AiTaskbar,
    CrossDeviceSync, FlatpakSnapLayer, DeclarativeBuildSystem, AiDependencyResolver, AiAnomalyFirewall,
    SecureContainer, PrivacyDashboard, OfflinePackageInstaller, AppSandboxing, CrossLanguageBuildTool,
    PluginMarketplace, MusicLibraryManager,
};

pub mod init {
    pub mod systemd_init;
}
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
