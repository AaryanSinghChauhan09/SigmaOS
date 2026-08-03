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
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod performance;
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
