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
    DrmAtomicPlaneState, WaylandDmaBuf, OpenBsdWsdisplayVt,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, LegacyLinuxRule, LinuxPersonaRule,
    SmartSymlink, SymlinkResolverRule, VirtualFilesystem,
};
pub use graphics::paint::ColorRgba;
pub use logging::{
    ConsoleLogTarget, FileLogTarget, LogCompressor, LogError, LogFacility, LogField, LogFile,
    LogLevel, LogRotateConfig, LogRotator, LogSeverity, LogTarget, LoggerCapability,
    MemoryLogTarget, NetworkFramingFormat, NetworkLogTarget, NetworkProtocol, RotationPolicy,
    SimpleLogCompressor, SimpleLogFile, SimpleLogRotator, SimpleUnifiedLogger, SyslogFacility,
    TargetCapability, TargetInfo, TargetType, UnifiedLogEntry, UnifiedLogStats, UnifiedLogger,
};
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
    FirewallAction, FirewallCommand, FirewallFilterRule, IpRoute2Command, LinkState, PingCommand,
    SocketStatsCommand, SocketStatsEntry, TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
    UfwDefaultRule, GLOBAL_FIREWALL, GLOBAL_IP_COMMAND, GLOBAL_UFW_RULE,
};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use performance::{
    AnanicyCppDaemon, AnanicyRule, BoreScheduler, CachyKernelManager, CallGraph,
    CpuInstructionExtension, CpuPriorityOptimizer, GlarySmartRule, IPCError, IpcError,
    IoPriorityOptimizer, IoSchedClass, IoTaskPriority, PerformanceProfileRule, PhysicalPageFrame,
    ProcessProfile, Profile, ProfileType, Profiler, ProfilerError, RamDefragmenter,
    SchedInstruction, SchedOpcode, SimdOptimizer, SimpleCallGraph, SimpleProfile, SimpleProfiler,
    SmartPerformanceProfile, SmartResourceOptimizer, SovereignSimdOptimizer, UdfSchedVm,
    UltraKernelSamepageMerger, VmPerformanceMetrics, X86v3v4OptimizationDetector, ZeroCopyMetrics,
    ZeroCopyQueue, GLOBAL_GLARY_RULE, GLOBAL_SMART_OPTIMIZER, QUEUE_SIZE,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore, EverythingSearchEngine, NotepadPlusPlusBuffer, SovereignBrowserEngine, SevenZipEngine,
    CompressionMethod, FlameshotAnnotator, AnnotationShape, ObsStudioMixer,
    AudacityWaveEditor, VlcCodecPipeline, DaVinciTimeline, OneCommanderFileGrid,
    ItemAgeColor, EarTrumpetVolumeMatrix, IrfanViewEngine,
};
pub use resilience::{
    FsSnapshot, RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError,
    SelfHealingModule, SigmaTimeshift, SystemSnapshot, GLOBAL_TIMESHIFT,
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
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
