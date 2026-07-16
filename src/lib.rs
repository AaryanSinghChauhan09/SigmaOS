// SigmaOS Library
// Core library for SigmaOS operating system

#![allow(warnings, clippy::all)]

pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod network;
pub mod orchestration;
pub mod package;
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
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    AppleSiliconUnifiedMemoryBus, CgaGraphicsDriver, CxlMemoryDriver, FloppyDiskDriver,
    GameportJoystickDriver, IdeControllerDriver, IntelXeGpuDriver, ParallelPrinterDriver,
    PcieGen5NvmeDriver, SoundBlaster16Driver, Thunderbolt4Controller, Wifi7Adapter,
};
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use drivers::{
    KernelReleaseInfo, LinuxReleaseDriver, Longterm5_10_TpmDriver, Longterm5_15_SerialDriver,
    Longterm6_12_NetworkDriver, Longterm6_18_StorageDriver, Longterm6_1_InputDriver,
    Longterm6_6_AudioDriver, MainlineGpuDriver, Prepatch6_23_Rc1_AiDriver, Stable6_22_SensorDriver,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
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
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
