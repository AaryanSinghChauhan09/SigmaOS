#![allow(warnings)]
#![allow(clippy::all)]

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
pub mod observability;
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
    ContainerRuntime, FhsConventionStatus, LsbProfile, PosixComplianceLevel,
    StandardsComplianceManager, TargetPlatform, TranslationLayer,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    create_cga_graphics, create_floppy_disk, create_parallel_printer, create_sound_blaster_16,
    AdLibSynthDriver, AppleSiliconUnifiedMemoryBus, Bluetooth5_4_Adapter, CgaGraphicsDriver,
    CxlMemoryDriver, FloppyDiskDriver, GameportJoystickDriver, GpuCommand, GpuDriver, GpuError,
    HidError, HidKeyboardEvent, HidReportType, IdeControllerDriver, InputDriver, InputEvent,
    InputType, IntelXeGpuDriver, KernelReleaseInfo, LinuxReleaseDriver, Longterm5_10_TpmDriver,
    Longterm5_15_SerialDriver, Longterm6_12_NetworkDriver, Longterm6_18_StorageDriver,
    Longterm6_1_InputDriver, Longterm6_6_AudioDriver, MainlineGpuDriver, Ne2000NetworkDriver,
    NetworkCommand, NetworkDriver, NetworkError, NetworkType, NvlinkBusDriver,
    ParallelPrinterDriver, PciIdeBridge, PcieGen5NvmeDriver, PcieGen6Bridge,
    Prepatch6_23_Rc1_AiDriver, Ps2MouseDriver, Sata3Controller, SerialMouseDriver,
    SoundBlaster16Driver, Stable6_22_SensorDriver, StorageCommand, StorageDriver, StorageError,
    StorageType, Thunderbolt4Controller, UdfAncientDevice, Ufs4StorageDriver, Usb4HostController,
    UsbHidDriver, VesaDriver, VesaError, VesaModeInfo, VgaTextModeDriver, Wifi7Adapter,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack,
};
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
