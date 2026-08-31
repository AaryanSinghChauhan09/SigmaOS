#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system

pub mod access;
pub mod accessibility;
pub mod ai;
pub mod boot;
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
pub mod logging;
pub mod network;
pub mod orchestration;
pub mod distro;
pub mod package;
pub mod performance;
pub mod plugin;
pub mod productivity;
pub mod resilience;
pub mod resource;
pub mod security;
pub mod shell;
pub mod sigpkg;
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
    ApplicationBinary, BinaryFormat, BodhiUpdateTriage, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ContainerRuntime, DnfPackageResolver,
    FedoraAlu, FedoraAluFlags, KojiBuildServer, MockChrootBuilder, SigmaChangeProposal,
    SigmaChangeProcessEngine, SigmaNextChannel, TargetPlatform, TranslationLayer,
    ZorinAppearanceSwitcher, ZorinLayoutPreset, ZorinConnectHub, ZorinWineLayer,
    ZorinLiteOptimizer, SigmaEcosystemInit, FhsRunlevel, SigmaEcosystemProfiler,
    GraphicPresetMode, SigmaOnboardingWelcome, SigmaOnboardingLog,
    SigmaSupportSubtitleSync, SigmaSupportSubtitleEdit, SubtitleFormat,
    SigmaSupportResourceOptimizer, SigmaSupportPriorityOptimizer,
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
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use kernel::{
    AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, ArchitectureEngine,
    AuditBlock, BuddyAllocator, Channel, CircularDoublyLinkedList,
    CpuRegisters, EdfTask, HardwareException,
    IpcError, IpcManager, Irql,
    LcgRandom, LookasideList, LotteryTask, MemoryBlock,
    MemoryDescriptorList, Message, Pcb, PoolType, Priority, Process,
    ProcessState, ProcessorInitState, RoundRobinConfig, RoundRobinScheduler,
    SchedulerError, SequencedSinglyLinkedList, SinglyLinkedList, SystemThread,
    Tcb, ThreadState, WorkItem, PAGE_SIZE,
};
pub use logging::*;
pub use network::{
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
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
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise,
    PrivacyFirstSandbox, SandboxRule, secure_zeroize, AuditLogEntry, HardenedAuditTrail,
    IntrusionMonitor, IntrusionSeverity,
};
pub use plugin::{
    ExtensionType, ManagerCapability, MarketplaceItem, Plugin, PluginCapability, PluginError,
    PluginID, PluginInfo, PluginManager, PluginMarketplace, PluginState, PluginStats, SimplePlugin,
    SimplePluginManager,
};
pub use shell::{ShellCommand, ShellRepl};
pub use tools::{
    SigmaBuildAttestation, SigmaCliCommand, SigmaMasterCli,
    AlmeidaCmosRtc, AlmeidaCoreDump, ClusterNode, NodeState, SigmaAccess,
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
    MemoryLeakDetector, MeshNetworking, MultiMonitorManager, MusicLibraryManager,
    NaturalLanguageShell, OfflinePackageInstaller, PacketSniffer, PartitionManager, PdfEditor,
    PerformanceOptimizer, PodcastRecorder, PredictiveMaintenance, PrivacyDashboard,
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
