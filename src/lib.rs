// SigmaOS Library
// Core library for SigmaOS operating system

pub mod ai;
pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod distro;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod graphics;
pub mod kernel;
pub mod network;
pub mod orchestration;
pub mod distro;
pub mod package;
pub mod performance;
pub mod productivity;
pub mod resilience;
pub mod resource;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod tools;
pub mod virtualization;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod klib;
pub use klib::{SplayTree, RadixTree, SovereignPriorityQueue};

pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
    ScriptArgumentRouter,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ComputeNode, ContainerRuntime, DistributedComputeHandoff, GstCalculator, IndiaStackError,
    InterimLispVM, JehanneError, JehanneNamespace, LispVal, MintBackupTool, MintSoftwareManager,
    MintUpdateItem, MintUpdateLevel, MintUpdateManager, MntReformLpcDriver, MockUPIService,
    MultilingualSupport, NamespaceBindEntry, NtHandle, NtObjectManager, NtObjectType, NtStatus,
    Plan9pMessage, Plan9pMsgType, PortableExecutableLoader, ReformPowerStats, RegistryHive,
    SoftwareMeta, TargetPlatform, TranslationLayer, WindowCoordinates, ZenithDisplayCompositor,
    JudicialTimelinePlanner, MsmeComplianceEngine, AyushFormularyHelper,
    PMWaniHotspotController, DigiYatraPassScanner, IrctcPnrTracker,
    Literal, SpacSatResolver,
    ApkInstalledPackage, ApkDatabaseIndex, SyslogSeverity, SyslogMessage,
    AlpineSyslogManager, BusyBoxMulticall,
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
    ApsrFlags, ArmExecutionState,
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
pub use resilience::{
    FsSnapshot, RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError,
    SelfHealingModule, SigmaTimeshift, SystemSnapshot, GLOBAL_TIMESHIFT,
};
pub use security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken, DefensiveAuditSystem,
    ArithmeticSubstitutionDeobfuscator,
    ForensicBlock, ForensicStorageFilter, MaliciousSignature, Permission, PledgeManager,
    PledgePromise, RoutingMode, SandboxPolicy, GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
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

// Temporarily disabled problematic modules
// pub mod accessibility;
// pub mod automation;
// pub mod container;

// #[cfg(test)]
// #[path = "compatibility/fedora.rs"]
// pub mod fedora_compat_test;
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
// pub mod distro;
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
// pub mod hardware {
//     pub mod compatibility;
//     pub mod win32;
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
