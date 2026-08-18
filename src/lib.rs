#![allow(warnings)]
#![allow(clippy::all)]
extern crate alloc;
// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod ai;
pub mod automation;
pub mod boot;
pub mod container;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod distro;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod graphics;
pub mod kernel;
pub mod memory;
pub mod klib;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod performance;
pub mod process;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;
pub mod tracing;
pub mod crash;
pub mod media;
pub mod gpu;

pub mod hardware {
    pub mod compatibility;
    pub mod win32;
}
pub mod power {
    pub mod governor;
}
pub mod observability {
    pub mod profiler;
}
pub mod toolchain {
    pub mod adapter;
    pub mod bootstrap;
    pub mod capsule;
    pub mod codex;
}
pub mod scheduler {
    pub mod numa_scheduler;
}
pub mod crypto {
    pub mod vectorized_pqc;
}

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
    APITimelineManager, BinaryAbiFormat, LinuxBsdAbiBridge, ServiceInitType, ServiceUnitTranslator, TranslatedService, AiResourceScheduler,
    AntixDesktopProfiler, AntixInitManager, AppSuiteBundle, AppSuiteType, ApplicationBinary,
    BinaryCompatMatrix, BinaryFormat, BrailleMatrix, BsdJailSandbox, BundleType, CloudOrchestrator,
    CloudProvider, CompatBinary, CompatBinaryFormat, CompatibilityError, CompatibilityLayer,
    CompatibilityManager, CompatibilityMode, ContainerRuntime, ContinuityCoordinator, DesktopMode,
    DesktopProfile, DesktopTheme, DiscontinuedFS, DistroReleaseChannel, DriverBridge,
    EcosystemSnapshot, FSRevival, FlatpakApp, GraphicsBridge, HandoffTask, InstallerStep,
    KapudanAssistant, KernelPersona, KernelPersonaVM, LanguageTranslationCatalog, LegacyBus,
    LegacyDriver, LegacyMemoryTrimmer, LegacyPluginManager, LibcVersion, LocaleManager,
    MicroService, MicroServiceState, NetworkBridge, ReleaseGovernanceCouncil,
    ReproducibleBuildVerifier, SigmaContainer, SnapshotManager, StorageBridge, SuiteRegistry,
    SyscallAbi, TargetPlatform, TranslationLayer, TribeInstaller, TtsSynthesizer, UnifiedAppStore,
    WorkloadOptimizer, WorkloadProfile, ZorinAppearanceSwitcher, GLOBAL_AKABEI,
    GLOBAL_ANTIX_CONTROL, GLOBAL_ANTIX_DESKTOP, GLOBAL_ANTIX_INIT, GLOBAL_KAPUDAN,
    GLOBAL_MEMORY_TRIMMER, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_TRIBE,
    GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use distro::{
    ArchDependencyResolver, PackageNode, FreeBSDJail, OpenBSDPledge, NixStyleStore,
    PinRule, AptPinStore, OpenRCService, ArchPacmanHooksManager, FlakeInput, GentooPortageUseFlagsEngine,
    NixOSFlakeEngine, PacmanHook, PortagePackage, RunitService, ServiceState, SystemClosure, VoidRunitSupervisor,
    AdminAction, AiSysAdmin, AppManifest, AuditResult, AuditRule, BackupSnapshot, BackupSystem,
    BountyStatus, BugBountyProgram, BugBountyReport, BuildJob, BuildStatus, CanFrame,
    CertificationStatus, CommunityConference, ConferenceTalk, ConfigHook, CrossBuildPipeline, DevTool, DeveloperToolkit, DirectoryService, DirectoryUser,
    DllLoader, DllModule, EcuController, EduChallenge, EduPlayground, ForumChannel, ForumPost,
    GdiObjectType, HardwareCertificate, HardwareCertificationProgram, HardwareProfile,
    HardwareRegressionSuite, HelpSystem, HowToGuide, HpcClusterJob, HpcJobState, ImeCandidate,
    InputMethodEngine, IntegrityState, KernelTrace, LanguagePack, LinuxSyscall, LiveDebugger,
    LivepatchManager, LivepatchPatch, LocaleManager as DistroLocaleManager, ManPage, MpiCommunicator, NetplanConfig,
    NetplanManager, P2pNode, PackageBuildService, PosixTranslation, PqcSelfHealing,
    QAStagedRelease, RegionalSettings, RegistryType, RegistryValue, ReleaseStage, RescueISO,
    RescueISOManager, SoftwareCertificationProgram, SovereignP2PSync, TargetArch,
    TimeTravelCheckpoint, TimeTravelEngine, WikiPage, Win32Gdi, WindowsRegistry,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    DeviceGeneration, GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType,
    InputDriver, InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    PeripheralDevice, PeripheralManager, PowerState, StorageCommand, StorageDriver, StorageError,
    StorageType, UsbHidDriver, VesaDriver, VesaError, VesaModeInfo, E1000RxDescriptor,
    E1000TxDescriptor, IntelE1000Driver, LegacyAudioAc97, ModernAudioIntelHda, ModernNvmeDriver,
    ModernUsbPrinterDriver, ModernWifiDriver, TouchJingosDriver, VirtioBlkDriver, VirtioDeviceType,
    VirtioMmioHeader, VirtioNetDriver, VirtioRngDriver, UnifiedDmaBroker, SelfHealingDriverManager,
    DmaDescriptor, DeviceCommandType, DeviceTransactionLog, GLOBAL_DMA_BROKER, GLOBAL_HEALING_MANAGER,
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
    ABIManager, AcpiInterruptManager, AiNativeRuntime, BpfLsmPolicyGovernor,
    CompletionQueueEntry, EnergyAwareScheduler, FastPathIpc, GapError, Generation, GenerationManager,
    InterruptMechanism, IrqRoutingTable, JournalBlock, JournalState,
    KernelGraph, KernelIoUringEngine, KernelPersona, KernelPlugin, KernelPluginManager,
    LegacyScheduler, LsmHookType, MemfdSecretGuard, MetaKernel, MetadataJournal,
    MicroDriver, NetPod, PageFolio, PageFolioCacheManager, Pml4PageTableEntry, PolicyError,
    PolicyManager, PrivacyFirstSandbox, PrivilegeLevel,
    ProtectionDomain, ResourceBroker,
    SelfHealingKernel, SigmaFsPlusPlus, SubmissionQueueEntry, UniversalAbiTranslator,
    UserDefinedKernelFunctions, VirtualMemoryPagingManager,
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
    CpuPriorityOptimizer, GlarySmartRule, IoPriorityOptimizer, IoSchedClass, IoTaskPriority,
    PerformanceProfileRule, PhysicalPageFrame, Profile, ProfileType, Profiler, ProfilerError,
    RamDefragmenter, SimpleCallGraph, SimpleProfile, SimpleProfiler, SmartPerformanceProfile,
    SmartResourceOptimizer, UltraKernelSamepageMerger, X86v3v4OptimizationDetector,
    GLOBAL_GLARY_RULE, GLOBAL_SMART_OPTIMIZER,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore, EverythingSearchEngine, NotepadPlusPlusBuffer, SovereignBrowserEngine, SevenZipEngine,
    CompressionMethod, FlameshotAnnotator, AnnotationShape, ObsStudioMixer,
    AudacityWaveEditor, VlcCodecPipeline, DaVinciTimeline, OneCommanderFileGrid,
    ItemAgeColor, EarTrumpetVolumeMatrix, IrfanViewEngine,
};
pub use kernel::vmm_paging::{PageTableFlags as VmmPageFlags, PageTableManager as VmmPageTableManager, VirtualMemoryManager as VmmManager, VmArea, VmProtection};
pub use kernel::processor_management::{
    CpuArchitecture, CpuCoreDescriptor, CpuHardwareProtectionEngine, HardwarePerfCounters,
    NumaAffinityMap, SmpTopologyManager,
};
pub use resilience::{
    FsSnapshot, RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError,
    SelfHealingModule, SigmaTimeshift, SystemSnapshot, GLOBAL_TIMESHIFT,
};
pub use security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken, DefensiveAuditSystem,
    ForensicBlock, ForensicStorageFilter, MaliciousSignature, Permission, PledgeManager,
    PledgePromise, RoutingMode, SandboxPolicy, GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain,
    IsolationError, SecurityEnforcer as AndroidStyleSecurityEnforcer,
    PORT_ALLOW_SSL, PORT_ALLOW_TCP,
    MacChanger, KaliPacketSniffer,
    HashAuditor, PacketAnomaly,
    GLOBAL_MACCHANGER, GLOBAL_SNIFFER, GLOBAL_AUDITOR,
    CronDaemon, CronJob, DmesgLog, FirewallRule,
    IptablesFirewall, KaliError, PluggableAuthenticationModule,
    SudoPrivilegeEscalation, SwapSpaceManager, TmuxMultiplexer,
    MAX_AUDIT_BLOCKS, MAX_SIGNATURES, SIGNATURE_LEN,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use memory::{
    BsdZoneAllocator, LinuxKswapd, MemCgroupManager, SimpleVMM, Zone, MemCgroup, PageState,
};

pub use tools::{
    AccessibilityFeature as LibAccessibilityFeature, ClusterNode as LibClusterNode,
    NodeState as LibNodeState, SigmaAccess as LibSigmaAccess, SigmaCluster as LibSigmaCluster,
    SigmaDeploy as LibSigmaDeploy, SigmaIdentity as LibSigmaIdentity,
    SigmaToolError as LibSigmaToolError,
    UserIdentity as LibUserIdentity,
};
