#![allow(warnings)]
#![allow(clippy::all)]

||||||| 43be3a7e8
#![allow(warnings)]
#![allow(clippy::all)]
||||||| 43be3a7e8
#![allow(warnings)]
#![allow(clippy::all)]

// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod init;
pub mod customization;
pub mod dashboard;
||||||| 43be3a7e8
pub mod security;
pub mod sigpkg;
pub mod kernel;
pub mod network;
pub mod filesystem;
pub mod drivers;
pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
||||||| 43be3a7e8
pub mod security;
pub mod sigpkg;
pub mod kernel;
pub mod network;
pub mod filesystem;
pub mod drivers;
pub mod accessibility;
pub mod ai;
pub mod audio;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
||||||| 43be3a7e8
pub mod shell;
pub mod dashboard;
pub mod accessibility;
pub mod customization;
pub mod automation;
pub mod resilience;
pub mod productivity;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod network;
||||||| 43be3a7e8
pub mod shell;
pub mod dashboard;
pub mod accessibility;
pub mod customization;
pub mod automation;
pub mod resilience;
pub mod productivity;
pub mod drivers;
pub mod filesystem;
pub mod graphics;
pub mod hardware;
pub mod init;
pub mod interrupt;
pub mod kernel;
pub mod klib;
pub mod logging;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
||||||| 43be3a7e8
pub mod compatibility;
pub mod performance;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
||||||| 43be3a7e8
pub mod compatibility;
pub mod plugin;
pub mod process;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;
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
pub mod observability {
    pub mod profiler;
}
pub mod ai;
pub mod arch;
pub mod boot;
pub mod toolchain;
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
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
    ConfigSysSetting, TsrProgram, FatDirectoryEntry, FreeDosEmulator,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
    ClusterState as DefragClusterState, FragmentedFile, DefragStats, DiskDefragmenter,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
    ModuleLoadError as LkmLoadError, KernelModule as LkmModule, LkmLoader, KpatchPatch, KpatchManager,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
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
pub use security::{
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise,
    AnonymityMode, AnonsurfEngine, RecoveredFile, ForensicsAuditTool, SniffedPacket,
    KaliSniffer, PentestAssistant, SecureWipeTool, IntrusionSeverity, IntrusionAlert, SigmaIDS,
};
pub use init::{
    Runlevel, ServiceState as InitServiceState, InitError, Service as InitService, SimpleService as InitSimpleService,
    InitSystem, SigmaInit, DependencyResolver as InitDependencyResolver, SimpleDependencyResolver, ServiceMonitor, SimpleServiceMonitor,
    FirmwarePort, BIOSPort, UEFIPort, CorebootPort, SecurityPort, DACPort, SELinuxPort, ZeroTrustPort,
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
pub use toolchain::self_host::{ToolchainError, CompilerConfig, SelfHostingManager};
pub use arch::cpu_sys::{
    SegmentType as CpuSegmentType, GdtDescriptor as CpuGdtDescriptor, IdtGate as CpuIdtGate,
    VirtualMemoryRegion as CpuVirtualMemoryRegion, ProcessorInitSuite as CpuProcessorInitSuite,
    FastSyscallDispatcher as CpuFastSyscallDispatcher,
};
||||||| 43be3a7e8
pub use security::{CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise};
pub use sigpkg::{SatSolver, ContentAddressedStore, CryptoVerifier, Transaction, PackageRecipe, BuildSystem, RecipeManager, RecipeError};
pub use kernel::{Scheduler, Process, Priority, ProcessState, BuddyAllocator, MemoryBlock, PAGE_SIZE, IpcManager, Channel, Message, IpcError, RoundRobinScheduler, RoundRobinConfig, SchedulerError};
pub use network::{TcpStack, TcpConnection, TcpSegment, TcpState, TcpError};
pub use filesystem::{VirtualFilesystem, Inode, FileDescriptor, FileType, FilePermissions, FsError};
pub use drivers::{GpuDriver, GpuCommand, GpuError, StorageDriver, StorageCommand, StorageType, StorageError, NetworkDriver, NetworkCommand, NetworkType, NetworkError, InputDriver, InputEvent, InputType, UsbHidDriver, HidKeyboardEvent, HidReportType, HidError, VesaDriver, VesaModeInfo, VesaError};
pub use shell::{ShellRepl, ShellCommand};
pub use dashboard::{UnifiedDashboard, DashboardWidget, MetricData, MetricType, WidgetType, SystemMonitor};
pub use accessibility::{AccessibilityFramework, AccessibilityProfile, AccessibilitySetting, AccessibilityCategory, AccessibilityFeature, AccessibilityError};
pub use customization::{CustomizationEngine, Routine, Condition, Action, Theme, TriggerType, CustomizationError};
pub use automation::{AiOptimizer, OptimizationRecommendation, SystemState, OptimizationCategory, OptimizationError, SystemAutomationManager, SystemAutomationRule, SystemAction, SystemEventType, PerformanceProfile, SystemPrediction, PredictiveModel, AutomationError};
pub use resilience::{SelfHealingModule, RecoveryRule, RecoveryAction, SystemSnapshot, RecoveryEventType, ResilienceError};
pub use productivity::{GamifiedProductivity, Achievement, Goal, PomodoroTimer, ProductivityScore, AchievementType, PomodoroState};
pub use orchestration::{CrossDeviceOrchestrator, ConnectedDevice, SmartHomeDevice, AutomationRule as CrossDeviceAutomationRule, DeviceType as CrossDeviceType, ConnectionStatus, DeviceCapability, AutomationTrigger, CrossDeviceAction, OrchestrationError};
pub use package::{UniversalPackageManager, UnifiedPackage, PackageFormat, PackageSource, PackageAdapter, DependencyResolver, ConflictResolution, PackageError};
pub use compatibility::{CompatibilityManager, ApplicationBinary, TranslationLayer, ContainerRuntime, TargetPlatform, BinaryFormat, CompatibilityMode, CompatibilityError};
pub use virtualization::{VirtualizationOrchestrator, VirtualMachine, Container, KubernetesPod, VirtualizationTech, VmState, ResourcePool, VirtualizationError};
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
    SmartSymlink, SymlinkResolverRule, VirtualFilesystem, O_APPEND, O_CREAT, O_EXCL, O_RDONLY,
    O_RDWR, O_TRUNC, O_WRONLY,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
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
};
pub use performance::{
    CallGraph, CpuPriorityOptimizer, GlarySmartRule, IoPriorityOptimizer, IoTaskPriority,
    PerformanceProfileRule, Profile, ProfileType, Profiler, ProfilerError, RamDefragmenter,
    SimpleCallGraph, SimpleProfile, SimpleProfiler, SmartPerformanceProfile,
    SmartResourceOptimizer, GLOBAL_GLARY_RULE, GLOBAL_SMART_OPTIMIZER,
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
||||||| 43be3a7e8
pub use security::{CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise};
pub use sigpkg::{SatSolver, ContentAddressedStore, CryptoVerifier, Transaction, PackageRecipe, BuildSystem, RecipeManager, RecipeError};
pub use kernel::{Scheduler, Process, Priority, ProcessState, BuddyAllocator, MemoryBlock, PAGE_SIZE, IpcManager, Channel, Message, IpcError, RoundRobinScheduler, RoundRobinConfig, SchedulerError};
pub use network::{TcpStack, TcpConnection, TcpSegment, TcpState, TcpError};
pub use filesystem::{VirtualFilesystem, Inode, FileDescriptor, FileType, FilePermissions, FsError};
pub use drivers::{GpuDriver, GpuCommand, GpuError, StorageDriver, StorageCommand, StorageType, StorageError, NetworkDriver, NetworkCommand, NetworkType, NetworkError, InputDriver, InputEvent, InputType, UsbHidDriver, HidKeyboardEvent, HidReportType, HidError, VesaDriver, VesaModeInfo, VesaError};
pub use shell::{ShellRepl, ShellCommand};
pub use dashboard::{UnifiedDashboard, DashboardWidget, MetricData, MetricType, WidgetType, SystemMonitor};
pub use accessibility::{AccessibilityFramework, AccessibilityProfile, AccessibilitySetting, AccessibilityCategory, AccessibilityFeature, AccessibilityError};
pub use customization::{CustomizationEngine, Routine, Condition, Action, Theme, TriggerType, CustomizationError};
pub use automation::{AiOptimizer, OptimizationRecommendation, SystemState, OptimizationCategory, OptimizationError, SystemAutomationManager, SystemAutomationRule, SystemAction, SystemEventType, PerformanceProfile, SystemPrediction, PredictiveModel, AutomationError};
pub use resilience::{SelfHealingModule, RecoveryRule, RecoveryAction, SystemSnapshot, RecoveryEventType, ResilienceError};
pub use productivity::{GamifiedProductivity, Achievement, Goal, PomodoroTimer, ProductivityScore, AchievementType, PomodoroState};
pub use orchestration::{CrossDeviceOrchestrator, ConnectedDevice, SmartHomeDevice, AutomationRule as CrossDeviceAutomationRule, DeviceType as CrossDeviceType, ConnectionStatus, DeviceCapability, AutomationTrigger, CrossDeviceAction, OrchestrationError};
pub use package::{UniversalPackageManager, UnifiedPackage, PackageFormat, PackageSource, PackageAdapter, DependencyResolver, ConflictResolution, PackageError};
pub use compatibility::{CompatibilityManager, ApplicationBinary, TranslationLayer, ContainerRuntime, TargetPlatform, BinaryFormat, CompatibilityMode, CompatibilityError};
pub use virtualization::{VirtualizationOrchestrator, VirtualMachine, Container, KubernetesPod, VirtualizationTech, VmState, ResourcePool, VirtualizationError};
pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting,
};
pub use ai::{
    AIAgent, AIAgentManager, AIError, AIStats, AgentCapability, AgentInfo, Intent, IntentType,
    ManagerCapability as AiManagerCapability, Pattern, SimpleAIAgent, SimpleAIAgentManager,
    SovereignWikiEngine, WikiArticle,
};
pub use audio::{
    AudioClip, AudioMasteringEffect, AudioTrack, PodcastEpisode, PodcastFeed, PodcastRecorder,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime as CompatibilityContainerRuntime, CpuGovernor, InterimLispVM, LispVal,
    LubuntuHealthReport, LubuntuSystemManager, MntReformLpcDriver, ReformPowerStats,
    SystemPressure, TargetPlatform, TranslationLayer,
};
pub use container::{
    Container, ContainerCapability, ContainerError, ContainerID, ContainerInfo, ContainerRuntime,
    ContainerState, Namespace, OciContainer, OciContainerError, OciContainerID,
    OciContainerRuntime, OciContainerState, RuntimeCapability, RuntimeStats, Sandbox,
    SimpleContainer, SimpleContainerRuntime, SimpleOciContainer, SimpleOciContainerRuntime,
    SimpleSandbox,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use desktop::{
    ShellIntegration, SimpleShellIntegration, SimpleTerminal, SimpleTerminalManager, Terminal,
    TerminalError, TerminalID, TerminalManager,
};
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, SovereignProcFS, VirtualFilesystem,
};
pub use graphics::{
    BitmapSurface, Color, Compositor, Position, Rectangle, SimpleCompositor, SimpleWindow, Size,
    Surface, VideoClip, VideoEffect, VideoTimeline, VideoTrack, Window,
};
pub use hardware::{
    CompatibilityMatrix, Device as HardwareDevice, DeviceID as HardwareDeviceID,
    DeviceType as HardwareDeviceType, DiagnosticResult as HardwareDiagnosticResult,
    DriverManager as HardwareDriverManager, HardwareDiagnostics, SimpleCompatibilityMatrix,
    SimpleDevice as SimpleHardwareDevice, SimpleDriverManager as SimpleHardwareDriverManager,
    SimpleHardwareDiagnostics, SupportStatus as HardwareSupportStatus,
};
pub use interrupt::{
    ColorCode, ExceptionType, InterruptDescriptor as HardwareInterruptDescriptor,
    InterruptError as HardwareInterruptError, InterruptHandler as HardwareInterruptHandler,
    InterruptManager as HardwareInterruptManager, ScreenChar,
    SimpleInterruptHandler as SimpleHardwareInterruptHandler, TaskStateSegment, VGAColor,
    VGATextBuffer, GDT, IDT, PIC,
};
pub use kernel::{
    BoreScheduler, BoreTask, BuddyAllocator, Channel, CpuError, CpuMode, CpuRing, IpcError,
    IpcManager, MemoryBlock, Message, Priority, Process as KernelProcess,
    ProcessState as KernelProcessState, RegisterSet, RoundRobinConfig, RoundRobinScheduler,
    Scheduler, SchedulerError, SovereignVirtualCPU, PAGE_SIZE,
};
pub use klib::{AsyncExecutor, CpuIsaAssessor, IsaLevel, Reducer, Store, Subscriber, Task};
pub use logging::{
    ConsoleLogTarget, FileLogTarget, LogError, LogLevel, LogTarget, LoggerCapability,
    MemoryLogTarget, NetworkLogTarget, SimpleUnifiedLogger, TargetCapability, TargetInfo,
    TargetType, UnifiedLogEntry, UnifiedLogStats, UnifiedLogger,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use plugin::{
    ManagerCapability, Plugin, PluginCapability, PluginError, PluginID, PluginInfo, PluginManager,
    PluginState, PluginStats, SimplePlugin, SimplePluginManager,
};
pub use process::{
    Process as UserspaceProcess, ProcessError as UserspaceProcessError, ProcessGroup,
    ProcessID as UserspaceProcessID, ProcessSpawner, ProcessState as UserspaceProcessState,
    ProcessWaiter, SimpleProcess, SimpleProcessGroup, SimpleProcessSpawner, SimpleProcessWaiter,
    SIGINT, SIGKILL, SIGTERM, SIGUSR1,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    CapabilityGate, CapabilityToken, LinuxCapability, Permission, PledgeManager, PledgePromise,
    Securelevel, SovereignSecurelevelManager, UnveilManager, UnveilPermission, UnveilRestriction,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use virtualization::{
    Container as VirtualContainer, KubernetesPod, ResourcePool, VirtualMachine,
    VirtualizationError, VirtualizationOrchestrator, VirtualizationTech, VmState,
};
