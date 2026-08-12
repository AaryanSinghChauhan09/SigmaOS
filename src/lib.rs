#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system

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
pub mod kernel;
pub mod network;
pub mod orchestration;
pub mod distro;
pub mod package;
pub mod performance;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod tools;
pub mod virtualization;
pub mod unimplemented_features;
pub mod unimplemented_tools;

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
    APITimelineManager, AkabeiBundle, AkabeiPackageEngine, AntixControlCenter,
    AntixDesktopProfiler, AntixInitManager, ApplicationBinary, BinaryCompatMatrix, BinaryFormat,
    BundleType, CompatibilityError, CompatibilityManager, CompatibilityMode, ContainerRuntime,
    DesktopProfile, DesktopTheme, DiscontinuedFS, DriverBridge, FSRevival, GraphicsBridge,
    InstallerStep, KapudanAssistant, KernelPersona, KernelPersonaVM, LegacyBus, LegacyDriver,
    LegacyMemoryTrimmer, LegacyPluginManager, LibcVersion, MicroService, MicroServiceState,
    NetworkBridge, StorageBridge, SyscallAbi, TargetPlatform, TranslationLayer, TribeInstaller,
    WorkloadOptimizer, WorkloadProfile, GLOBAL_AKABEI, GLOBAL_ANTIX_CONTROL, GLOBAL_ANTIX_DESKTOP,
    GLOBAL_ANTIX_INIT, GLOBAL_KAPUDAN, GLOBAL_MEMORY_TRIMMER, GLOBAL_PERSONA_VM,
    GLOBAL_PLUGIN_MANAGER, GLOBAL_TRIBE, GLOBAL_WORKLOAD_OPTIMIZER,
    EcosystemSnapshot, SnapshotManager, CompatBinaryFormat, CompatBinary, CompatibilityLayer,
    BsdJailSandbox, FlatpakApp, UnifiedAppStore, HandoffTask, ContinuityCoordinator,
    DesktopMode, ZorinAppearanceSwitcher, AiResourceScheduler,
    DistroReleaseChannel, ReproducibleBuildVerifier, ReleaseGovernanceCouncil,
    LanguageTranslationCatalog, LocaleManager, TtsSynthesizer, BrailleMatrix,
    AppSuiteType, AppSuiteBundle, SuiteRegistry,
    CloudProvider, SigmaContainer, CloudOrchestrator,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
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
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
    ArchitectureEngine, CpuRegisters, HardwareException, InstructionCyclePhase as ArchInstructionCyclePhase, Irql,
    LookasideList, MemoryDescriptorList, Pcb, PoolType, ProcessorInitState, Tcb, ThreadState,
    InterruptClass, InstructionCyclePhase, IoWaitProfile, KernelMechanism, KernelPolicy,
    PolicyMechanismCoordinator, SovereignMechanism, AdaptivePolicy,
    SinglyLinkedList, SequencedSinglyLinkedList, CircularDoublyLinkedList,
    SystemThread, WorkItem, ApcMode, Apc, ApcQueue, CpuArchitectureClass,
    EdfTask, LotteryTask, AuditBlock, LcgRandom, AdvancedAlgorithmsManager,
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
    ConflictResolution, DebPackageDriverTranslator, DependencyResolver, GenericLinuxTranslationUdf,
    LinuxDriverPackageTranslator, LinuxTranslationService, PackageAdapter, PackageError,
    PackageFormat, PackageSource, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, SigmaSoftwareStore, SoftwareRegistryEntry, UnifiedPackage,
    UniversalPackageManager, GLOBAL_SOFTWARE_STORE, GLOBAL_TRANSLATION_SERVICE,
    GLOBAL_TRANSLATION_UDF,
    UseFlagManager, SlottedPackage, PortageSlotResolver, EbuildSandbox,
    OptLevel, GccOptimizationTuner, GenkernelOrchestrator,
};
pub use performance::{
    CallGraph, CpuPriorityOptimizer, GlarySmartRule, IoPriorityOptimizer, IoTaskPriority,
    PerformanceProfileRule, Profile, ProfileType, Profiler, ProfilerError, RamDefragmenter,
    SimpleCallGraph, SimpleProfile, SimpleProfiler, SmartPerformanceProfile,
    SmartResourceOptimizer, GLOBAL_GLARY_RULE, GLOBAL_SMART_OPTIMIZER,
    BoreScheduler, AnanicyRule, IoSchedClass, AnanicyCppDaemon, PhysicalPageFrame,
    UltraKernelSamepageMerger, X86v3v4OptimizationDetector, CachyKernelManager,
};
pub use productivity::{
    Achievement, AchievementType, AudioChannel, GamifiedProductivity, Goal, PomodoroState,
    PomodoroTimer, ProductivityScore, SigmaMediaEngine, GLOBAL_MEDIA_ENGINE,
};
pub use resilience::{
    FsSnapshot, RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError,
    SelfHealingModule, SigmaTimeshift, SystemSnapshot, GLOBAL_TIMESHIFT,
};
pub use security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken, DefensiveAuditSystem,
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
