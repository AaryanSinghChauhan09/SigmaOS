#![allow(warnings)]
#![allow(clippy::all)]

// SigmaOS Library
// Core library for SigmaOS operating system

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
pub mod graphics;
pub mod init;
pub mod interrupt;
pub mod kernel;
pub mod klib;
pub mod logging;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod plugin;
pub mod process;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;

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
pub use interrupt::{
    ColorCode, ExceptionType, InterruptDescriptor as HardwareInterruptDescriptor,
    InterruptError as HardwareInterruptError, InterruptHandler as HardwareInterruptHandler,
    InterruptManager as HardwareInterruptManager, ScreenChar,
    SimpleInterruptHandler as SimpleHardwareInterruptHandler, TaskStateSegment, VGAColor,
    VGATextBuffer, GDT, IDT, PIC,
};
pub use kernel::{
    BoreScheduler, BoreTask, BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message,
    Priority, Process as KernelProcess, ProcessState as KernelProcessState, RoundRobinConfig,
    RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
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
