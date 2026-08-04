#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod init;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
pub mod orchestration;
pub mod package;
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
pub use arch::cpu_sys::{
    FastSyscallDispatcher as CpuFastSyscallDispatcher, GdtDescriptor as CpuGdtDescriptor,
    IdtGate as CpuIdtGate, ProcessorInitSuite as CpuProcessorInitSuite,
    SegmentType as CpuSegmentType, VirtualMemoryRegion as CpuVirtualMemoryRegion,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ConfigSysSetting, ContainerRuntime, FatDirectoryEntry, FreeDosEmulator, TargetPlatform,
    TranslationLayer, TsrProgram,
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
    ClusterState as DefragClusterState, DefragStats, DiskDefragmenter, FileDescriptor,
    FilePermissions, FileType, FragmentedFile, FsError, Inode, VirtualFilesystem,
};
pub use init::{
    BIOSPort, CorebootPort, DACPort, DependencyResolver as InitDependencyResolver, FirmwarePort,
    InitError, InitSystem, Runlevel, SELinuxPort, SecurityPort, Service as InitService,
    ServiceMonitor, ServiceState as InitServiceState, SigmaInit, SimpleDependencyResolver,
    SimpleService as InitSimpleService, SimpleServiceMonitor, UEFIPort, ZeroTrustPort,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, KernelModule as LkmModule, KpatchManager,
    KpatchPatch, LkmLoader, MemoryBlock, Message, ModuleLoadError as LkmLoadError, Priority,
    Process, ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError,
    PAGE_SIZE,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat, PackageFormatAdapter,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, LayoutPreset as TmuxLayoutPreset,
    PomodoroState, PomodoroTimer, ProductivityScore, SplitDirection as TmuxSplitDirection,
    TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    AnonsurfEngine, AnonymityMode, CapabilityGate, CapabilityToken, ForensicsAuditTool,
    IntrusionAlert, IntrusionSeverity, KaliSniffer, PentestAssistant, Permission, PledgeManager,
    PledgePromise, RecoveredFile, SecureWipeTool, SigmaIDS, SniffedPacket,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use toolchain::self_host::{CompilerConfig, SelfHostingManager, ToolchainError};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
