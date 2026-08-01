#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod accessibility;
pub mod ai {
    pub mod agent;
    pub mod orchestrator;
    pub mod runtime;
}
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod graphics {
    pub mod compositor;
    pub mod paint;
    pub mod render3d;
    pub mod video;
}
pub mod init;
pub mod interrupt;
pub mod kernel;
pub mod klib;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod plugin;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;
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
pub mod boot {
    pub mod bridge_grid;
    pub mod firmware_bridge;
}
pub mod toolchain {
    pub mod adapter;
    pub mod bootstrap;
    pub mod capsule;
    pub mod codex;
}
pub mod scheduler {
    pub mod energy_aware;
    pub mod numa_scheduler;
}
pub mod crypto {
    pub mod vectorized_pqc;
}

pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting,
};
pub use ai::{
    AIAgent, AIAgentManager, AIError, AIStats, AgentCapability, AgentInfo, Intent, IntentType,
    ManagerCapability as AiManagerCapability, Pattern, SimpleAIAgent, SimpleAIAgentManager,
    SovereignWikiEngine, WikiArticle,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    ApplicationBinary, AtifTrajectoryMonitor, BinaryFormat, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ContainerRuntime as CompatibilityContainerRuntime,
    EflCanvasElement, MokshaDesktopManager, MokshaProfile, RelayNexus, TargetPlatform,
    TranslationLayer, VerifierConsensus, WandrEvent,
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
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use interrupt::{
    ColorCode, ExceptionType, InterruptDescriptor as HardwareInterruptDescriptor,
    InterruptError as HardwareInterruptError, InterruptHandler as HardwareInterruptHandler,
    InterruptManager as HardwareInterruptManager, ScreenChar,
    SimpleInterruptHandler as SimpleHardwareInterruptHandler, TaskStateSegment, VGAColor,
    VGATextBuffer, GDT, IDT, PIC,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
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
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise, UnveilManager,
    UnveilPermission, UnveilRestriction,
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
