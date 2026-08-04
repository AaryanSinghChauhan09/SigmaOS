// SigmaOS Library
// Core library for the SigmaOS operating system
// Zero-dependency, no-std compatible OS kernel and userspace components

#![allow(warnings)]
#![allow(clippy::all)]

// Core kernel modules
pub mod klib;
pub mod kernel;
pub mod arch;
pub mod boot;
pub mod interrupt;
pub mod init;
pub mod scheduler;
pub mod memory;

// Hardware & Driver modules
pub mod driver;
pub mod drivers;
pub mod device;
pub mod hardware;
pub mod gpu;

// Security & Isolation
pub mod security;
pub mod container;
pub mod virtualization;

// Filesystem & Storage
pub mod filesystem;

// Networking
pub mod network;

// Shell & Userspace
pub mod shell;
pub mod userspace;

// Package Management
pub mod package;
pub mod sigpkg;

// System Services
pub mod accessibility;
pub mod ai;
pub mod audio;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod diagnostics;
pub mod graphics;
pub mod legal;
pub mod logging;
pub mod media;
pub mod ml;
pub mod observability;
pub mod orchestration;
pub mod performance;
pub mod plugin;
pub mod power;
pub mod process;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod toolchain;

// =============================================================
// Re-exports for commonly used types
// =============================================================

pub use klib::{AsyncExecutor, CpuIsaAssessor, IsaLevel, Reducer, Store, Subscriber, Task};

pub use kernel::{
    Scheduler, Process, Priority, ProcessState, BuddyAllocator, MemoryBlock, PAGE_SIZE,
    IpcManager, Channel, Message, IpcError, RoundRobinScheduler, RoundRobinConfig, SchedulerError,
};

pub use security::{
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise,
};

pub use shell::{ShellRepl, ShellCommand};

pub use filesystem::{
    VirtualFilesystem, Inode, FileDescriptor, FileType, FilePermissions, FsError,
};

pub use network::{TcpStack, TcpConnection, TcpSegment, TcpState, TcpError};

pub use drivers::{
    GpuDriver, GpuCommand, GpuError, StorageDriver, StorageCommand, StorageType, StorageError,
    NetworkDriver, NetworkCommand, NetworkType, NetworkError, InputDriver, InputEvent, InputType,
    UsbHidDriver, HidKeyboardEvent, HidReportType, HidError, VesaDriver, VesaModeInfo, VesaError,
};

pub use package::{
    UniversalPackageManager, UnifiedPackage, PackageFormat, PackageSource, PackageAdapter,
    DependencyResolver, ConflictResolution, PackageError,
};

pub use sigpkg::{
    SatSolver, ContentAddressedStore, CryptoVerifier, Transaction, PackageRecipe,
    BuildSystem, RecipeManager, RecipeError,
};

pub use toolchain::self_host::{CompilerConfig, SelfHostingManager, ToolchainError};

pub use virtualization::{
    VirtualizationOrchestrator, VirtualMachine, Container, KubernetesPod, VirtualizationTech,
    VmState, ResourcePool, VirtualizationError,
};

pub use resilience::{
    SelfHealingModule, RecoveryRule, RecoveryAction, SystemSnapshot, RecoveryEventType,
    ResilienceError,
};

pub use orchestration::{
    CrossDeviceOrchestrator, ConnectedDevice, SmartHomeDevice,
    AutomationRule as CrossDeviceAutomationRule, DeviceType as CrossDeviceType, ConnectionStatus,
    DeviceCapability, AutomationTrigger, CrossDeviceAction, OrchestrationError,
};

pub use compatibility::{
    CompatibilityManager, ApplicationBinary, TranslationLayer, ContainerRuntime, TargetPlatform,
    BinaryFormat, CompatibilityMode, CompatibilityError,
};

pub use automation::{
    AiOptimizer, OptimizationRecommendation, SystemState, OptimizationCategory, OptimizationError,
    SystemAutomationManager, SystemAutomationRule, SystemAction, SystemEventType,
    PerformanceProfile, SystemPrediction, PredictiveModel, AutomationError,
};

pub use customization::{
    CustomizationEngine, Routine, Condition, Action, Theme, TriggerType, CustomizationError,
};

pub use dashboard::{
    UnifiedDashboard, DashboardWidget, MetricData, MetricType, WidgetType, SystemMonitor,
};

pub use productivity::{
    GamifiedProductivity, Achievement, Goal, PomodoroTimer, ProductivityScore, AchievementType,
    PomodoroState,
};

pub use accessibility::{
    AccessibilityFramework, AccessibilityProfile, AccessibilitySetting, AccessibilityCategory,
    AccessibilityFeature, AccessibilityError,
};
