#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system

pub mod access;
pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod graphics;
pub mod kernel;
pub mod klib;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod performance;
pub mod plugin;
pub mod power;
pub mod productivity;
pub mod resilience;
pub mod resource;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod thread;
pub mod process;
pub mod memory;
pub mod tools;
pub mod unimplemented_tools;
pub mod virtualization;

pub use access::{
    AccessError, AccessManager, AccessMode, AccessPattern, AccessResult, AccessRule,
    AccessTimeTracker, AnonymousAccessPolicy, DeviceAccessType, LdapAccessClient,
    LdapUserEntry, ProcessMigrationControl, ProtectionLevel, RemoteAccessController,
    RemoteAccessProtocol, RemoteAccessSession, RemoteFileHandle, SecurityAccessToken,
    WirelessAccessPoint, WirelessAccessPointManager,
};
pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    APITimelineManager, AiResourceScheduler, AkabeiBundle, AkabeiPackageEngine, AlternativeLink,
    AnanicyManager, AnacondaInstaller, AntixControlCenter, AntixDesktopProfiler, AntixInitManager,
    AppSuiteBundle, AppSuiteType, ApplicationBinary, AptRepositorySync, BinaryCompatMatrix,
    BinaryFormat, BodhiUpdateTriage, BoreSchedulerGovernor, BrailleMatrix, BsdJailSandbox,
    BundleType, CachyInitramfs, CloudOrchestrator, CloudProvider, CompatBinary, CompatBinaryFormat,
    CompatibilityError, CompatibilityLayer, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, ContinuityCoordinator, DebianAlternativesSystem, DebianChannel,
    DebootstrapEngine, DesktopMode, DesktopProfile, DesktopTheme, DiscontinuedFS,
    DistroReleaseChannel, DnfPackageResolver, DriverBridge, EcosystemSnapshot, FSRevival,
    FedoraAlu, FedoraAluFlags, FlatpakApp, GraphicsBridge, HandoffTask, InstallerStep,
    KapudanAssistant, KernelPersona, KernelPersonaVM, KojiBuildServer, LanguageTranslationCatalog,
    LegacyBus, LegacyDriver, LegacyMemoryTrimmer, LegacyPluginManager, LibcVersion, LocaleManager,
    MicroService, MicroServiceState, MockChrootBuilder, NetworkBridge, ReleaseGovernanceCouncil,
    ReproducibleBuildVerifier, SeLinuxContext, SeLinuxEngine, SchedPolicy, SigmaChangeProposal,
    SigmaChangeProcessEngine, SigmaContainer, SigmaNextChannel, SnapshotManager, StorageBridge,
    SuiteRegistry, SysVInitEngine, SysVRunlevel, SyscallAbi, SystemdPresetConfigurator,
    TargetPlatform, TranslationLayer, TribeInstaller, TtsSynthesizer, UnifiedAppStore,
    V4OptimizedPackageManager, WorkloadOptimizer, WorkloadProfile, ZorinAppearanceSwitcher,
    GLOBAL_AKABEI, GLOBAL_ANTIX_CONTROL, GLOBAL_ANTIX_DESKTOP, GLOBAL_ANTIX_INIT, GLOBAL_KAPUDAN,
    GLOBAL_MEMORY_TRIMMER, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_TRIBE,
    GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use distro::{
    ArchDependencyResolver, PackageNode, FreeBSDJail, OpenBSDPledge, NixStyleStore,
    PinRule, AptPinStore, OpenRCService,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    Bdle, Ch340Driver, DeviceGeneration, E1000Driver, GpuCommand, GpuCommandBuffer, GpuDriver,
    GpuError, GpuPipeline, GpuShader, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, IntelHdaDriver, NetworkCommand, NetworkDriver, NetworkError,
    NetworkType, NvmeCmd, NvmeCqe, NvmeDriver, PeripheralDevice, PeripheralManager, PowerState,
    RxDescriptor, ShaderStage, StorageCommand, StorageDriver, StorageError, StorageType,
    TxDescriptor, UsbHidDriver, VesaDriver, VesaError, VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, LegacyLinuxRule, LinuxPersonaRule,
    SmartSymlink, SymlinkResolverRule, VirtualFilesystem,
};
pub use graphics::paint::ColorRgba;
pub use kernel::{
    AdaptivePolicy, AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, ArchitectureEngine,
    AuditBlock, BuddyAllocator, Channel, CircularDoublyLinkedList, CpuArchitectureClass,
    CpuRegisters, EdfTask, HardwareException, InstructionCyclePhase as ArchInstructionCyclePhase,
    InstructionCyclePhase, InterruptClass, IoWaitProfile, IpcError, IpcManager, Irql,
    KernelMechanism, KernelPolicy, LcgRandom, LookasideList, LotteryTask, MemoryBlock,
    MemoryDescriptorList, Message, Pcb, PolicyMechanismCoordinator, PoolType, Priority, Process,
    ProcessState, ProcessorInitState, RoundRobinConfig, RoundRobinScheduler, Scheduler,
    SchedulerError, SequencedSinglyLinkedList, SinglyLinkedList, SovereignMechanism, SystemThread,
    Tcb, ThreadState, WorkItem, PAGE_SIZE,
};
pub use network::{
    CloudSyncManager, PeerInfo, TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
    TorrentClient, TorrentError, TorrentState,
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
pub use performance::{
    AdaptiveIOScheduler, BbrEngine, BbrState, CallGraph, EevdfScheduler, EevdfTask,
    IoOpcode, MultiGenLRU, PageInfo, PageState, Profile, ProfileType, Profiler, ProfilerError,
    SimpleCallGraph, SimpleProfile, SimpleProfiler, ZeroCopyQueue,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore, NoteTakingApp, ScreenRecorder, ScreenshotTool, SigmaOffice, TaskManager,
    IntegratedTerminal, TmuxSessionManager, MindMapCreator,
};
pub use resilience::{
    BackupError, BackupSnapshot, RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError,
    SelfHealingModule, SigmaTimeshift, SystemSnapshot,
};
pub use security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken, CronDaemon, CronJob,
    DefensiveAuditSystem, DmesgLog, FirewallRule, ForensicBlock, ForensicStorageFilter,
    IptablesFirewall, KaliError, MaliciousSignature, Permission, PluggableAuthenticationModule,
    PledgeManager, PledgePromise, RoutingMode, SandboxPolicy, SudoPrivilegeEscalation,
    SwapSpaceManager, TmuxMultiplexer, TmuxPane, GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
    MAX_AUDIT_BLOCKS, MAX_SIGNATURES, SIGNATURE_LEN,
    secure_zeroize, AuditLogEntry, CpuMitigationFlags, HardenedAuditTrail,
    HardenedSyscallDispatcher, IntrusionMonitor, IntrusionSeverity, KaslrConfig, KaslrError,
    KaslrManager, KaslrSlide, KernelSection, MemoryRegionPermission, SmepSmapEngine,
    SmepSmapViolation, SyscallHardeningConfig, SyscallHardeningError, SyscallRegisterState,
    UserAccessGuard, UserPtr,
    SecurityEnforcer, PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use plugin::{
    ExtensionType, ManagerCapability, MarketplaceItem, Plugin, PluginCapability, PluginError,
    PluginID, PluginInfo, PluginManager, PluginMarketplace, PluginState, PluginStats, SimplePlugin,
    SimplePluginManager,
};
pub use shell::{ShellCommand, ShellRepl};
pub use tools::{
    AccessibilityFeature, AlmeidaCmosRtc, AlmeidaCoreDump, ClusterNode, NodeState, SigmaAccess,
    SigmaCluster, SigmaDeploy, SigmaIdentity, SigmaMonitor, SigmaPatch, SigmaRescue, SigmaToolError,
    SovereignAptDuo, SovereignDpkgEtcher, SovereignIPCalculator, SovereignImeConvertCase,
    SovereignImageToDataUri, SovereignJsonPrettifier, SovereignKeyboardTester, SovereignIsWebsiteDown,
    SovereignPasswordGenerator, SovereignTableConverter, SovereignTextFixer, SovereignWordCounter,
    UserIdentity,
};
pub use unimplemented_tools::{
    AdaptiveUxAgent, AiAnomalyFirewall, AiCodeAssistant, AiComplianceDashboard, AiDependencyResolver,
    AiDifficultyDirector, AiFileOrganizer, AiScheduler, AiSearchAssistant, AiTaskbar, ApiTestingTool,
    AppSandboxing, AudioEditor, AudioTrack, BatterySaver, CameraFilter, CloudBackupUtility,
    CloudGaming, ControllerMapper, CrossDeviceSync, CrossLanguageBuildTool, DeclarativeBuildSystem,
    DiskDefragmenter, DocumentScanner, DuplicateFileFinder, EmailClient, EmulatorManager,
    FlatpakSnapLayer, ForensicSnapshot, GameHubLauncher, GameModManager, GamePerformanceBooster,
    GameRecorder, GamifiedDesktop, GamifiedTodo, GanttChartPlanner, GestureControl, GifConverter,
    GitGuiClient, GuiAppStore, IotDeviceManager, KanbanBoard, KanbanColumn, MarkdownNotebook,
    MemoryLeakDetector, MeshNetworking, MindMapCreator, MultiMonitorManager, MusicLibraryManager,
    NaturalLanguageShell, OfflinePackageInstaller, PacketSniffer, PartitionManager, PdfEditor,
    PerformanceOptimizer, PluginMarketplace, PodcastRecorder, PredictiveMaintenance, PrivacyDashboard,
    ProcessSandbox, ProfileSample, RemoteDesktop, SecureContainer, SecureFileSharing,
    SecureFileShredder, SmartCleanup, SmartNotificationManager, StartupOptimizer, StaticAnalyzer,
    StreamingOverlayManager, SubtitleEditor, SystemRestoreSnapshot, VectorDraftEngine,
    VirtualMachineGuest, VmGuestSupervisor, VoiceControl, VpnTunnelManager, VrArRuntime,
    WebcamEffects, ZeroKnowledgeVault, ZeroTrustTpmBoot,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};

pub use thread::management::{
    ThreadID, ThreadState as LibThreadState, ThreadAlertableState, Thread, ThreadError, SimpleThread, ThreadManager, SimpleThreadManager,
};

pub use process::spawn::{
    ProcessID, ProcessState as LibProcessState, ProcessError, Process, SimpleProcess, ProcessSpawner, SimpleProcessSpawner, ProcessWaiter, SimpleProcessWaiter, ProcessGroup, SimpleProcessGroup,
    CLONE_NEWNS, CLONE_NEWNET, CLONE_NEWPID,
};
pub use process::activity_manager::{
    ActivityManager, ActivityState, ProcessActivityRecord, RegisterSnapshot, AddressSpaceBinding,
};
pub use memory::segmentation_paging::{
    AddressBindingMode, AddressType, AslrEntropyConfig, CpuRing, ExecutableAddressBinding,
    RandomizedAddressSpace, SegmentDescriptor, SegmentSelector, SegmentationPagingEngine,
    SpaceProtectionFlags, SystemControlRegisters,
};

pub use tools::{
    AccessibilityFeature as LibAccessibilityFeature, ClusterNode as LibClusterNode, NodeState as LibNodeState,
    SigmaAccess as LibSigmaAccess, SigmaCluster as LibSigmaCluster, SigmaDeploy as LibSigmaDeploy,
    SigmaIdentity as LibSigmaIdentity, SigmaToolError as LibSigmaToolError, UserIdentity as LibUserIdentity,
    SovereignDpkgEtcher, SovereignAptDuo, SovereignImeConvertCase, SovereignTableConverter,
    SovereignWordCounter, SovereignTextFixer, SovereignImageToDataUri, SovereignKeyboardTester,
    SovereignIsWebsiteDown,
};
