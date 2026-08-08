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
pub mod crash;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod driver;
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
pub mod plugin;
pub mod process;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod ui;
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
pub use crash::{
    Anonymizer, CoredumpCollector, CrashError, CrashPipeline, CrashReport, CrashReportID,
    CrashStatistics, CrashType, CrashUploader, SimpleAnonymizer, SimpleCoredumpCollector,
    SimpleCrashPipeline, SimpleCrashReport, SimpleCrashUploader,
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
pub use driver::windows_compat::{
    ImageDosHeader, ImageFileHeader, ImageOptionalHeader64, ImageSectionHeader, MajorFunction,
    PeDriverLoader, WddmMiniportDriver, WdfIoQueueDispatchType, WdfQueueContext,
    WindowsDriverAdapter, WindowsNdisAdapter, WindowsStorportAdapter, WindowsWddmAdapter,
    DEVICE_OBJECT, DRIVER_OBJECT, DXGKARG_ADDDEVICE, DXGKARG_STARTDEVICE, DXGKRNL_INTERFACE,
    DXGK_DEVICE_INFO, DXGK_DISPLAY_INFORMATION, HW_INITIALIZATION_DATA, IRP, KSPIN_LOCK,
    NDIS_HANDLE, NDIS_MINIPORT_DRIVER_CHARACTERISTICS, NDIS_OID_REQUEST, NDIS_PORT_NUMBER,
    NDIS_STATUS, NET_BUFFER_LIST, NTSTATUS, PORT_CONFIGURATION_INFORMATION, SCSI_REQUEST_BLOCK,
    STATUS_BUFFER_TOO_SMALL, STATUS_INVALID_PARAMETER, STATUS_NOT_IMPLEMENTED, STATUS_PENDING,
    STATUS_SUCCESS, STATUS_UNSUCCESSFUL, WDFDEVICE, WDFDEVICE_INIT, WDFDRIVER, WDFQUEUE,
    WDFREQUEST, WDF_DRIVER_CONFIG, WDF_IO_QUEUE_CONFIG,
};
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use filesystem::{
    ConfigFileNode, ConfigFileType, FileDescriptor, FilePermissions, FileType, FsError, Inode,
    SovereignConfigFS, SovereignProcFS, VirtualFilesystem,
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
pub use ui::{
    LayoutCapability, LayoutStats, PlotFunction, SimpleUILayout, SimpleWidget,
    SovereignMathPlotter, UIError, UILayout, Widget, WidgetCapability, WidgetID, WidgetInfo,
    WidgetState,
};
pub use virtualization::{
    Container as VirtualContainer, KubernetesPod, ResourcePool, VirtualMachine,
    VirtualizationError, VirtualizationOrchestrator, VirtualizationTech, VmState,
};
