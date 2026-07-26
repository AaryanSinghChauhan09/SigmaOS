#![allow(warnings, clippy::all)]

// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod distro;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod unimplemented_features;
pub mod virtualization;

pub use unimplemented_features::{
    UniversalAbiTranslator, SigmaFsPlus, SelfHealingKernel, AiNativeRuntime,
    EnergyAwareScheduler, UserDefinedKernelFunctions, PrivacyFirstSandbox,
    CrossDeviceContinuity, GuestOsType, FsPluginType, ModelType, ContinuationTask,
};

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
    AiTaskOrchestrator, ApplicationBinary, ArchiveProfile, BinaryFormat, BootInterface,
    BuildArchive, BuildCapsule, BuildLedgerSystem, BuildProfile, CapsuleVersion, ChronicleType,
    CompatibilityError, CompatibilityManager, CompatibilityMode, ConstellationNode,
    ConstellationSecurityModel, ContainerRuntime, D3dToVulkanTranslator, DriverClass,
    DriverEmulator, DriverMuseum, DriverRepositoryManager, EmulatedPeripheral, EmulatorProfile,
    ExhibitType, FirmwareBridgeManager, FirmwarePavilion, FirmwarePersona, FirmwareType,
    GapSandboxPolicy, HardwareDriver, HidGraphicsDriver, JobClass, KernelConstellation,
    KernelModule, KernelModuleManager, KernelShard, LedgerSnapshot, MemoryProtection, ModuleState,
    NetworkStackGateway, ObsoleteDevice, ObsoletePeripheral, PavilionType, PeFormat, PeLoader,
    PeripheralEmulationLibrary, PeripheralMuseum, PeripheralPod, RegistryManager, SecurityGrid,
    SecurityModel, SecurityPavilion, SecurityPolicyManager, ShardType, SyscallCapsule,
    SyscallChronicle, SyscallCompatibilityRegistry, TargetPlatform, TranslationLayer,
    User32MessageQueue, VirtualMemoryManager, Win32Error, Win32Message, WinSockAdapter,
};
pub use customization::{
    Action, AutoThemeScheduler, Condition, CustomizationEngine, CustomizationError, Routine,
    SituationalPersonalizer, Theme, TriggerType, WindowGridLayout, WorkspaceLayoutCustomizer,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use distro::{
    CanFrame, DiagnosticLogTool, EcuController, EduChallenge, EduPlayground, EosUpdateNotifier,
    EosWelcomeEngine, HpcClusterJob, HpcJobState, MirrorRanker, MpiCommunicator,
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
pub use security::{
    CapabilityGate, CapabilityToken, DecoyHoneyPot, ForensicAnalyzer, KAslrHardener,
    KaliSnifferAudit, PassComplexityAuditor, Permission, PledgeManager, PledgePromise,
    RecoveredFile, SigmaPortScanner, StackCanaryGuard, VulnerabilitySeverity, WxorEPageGuard,
    ZeroizeSec,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    AurRecipeCompiler, ContentAddressedStore, CryptoVerifier, PackageRecipe, PacmanDbAdapter,
    RollingSyncManager, SatSolver, Transaction,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};

pub mod virt;
pub use virt::hypervisor::{
    Guest, GuestID, GuestState, Hypervisor, HypervisorError, SimpleGuest, SimpleHypervisor,
    VirtualizationGeneration,
};
pub use virt::microvm::{
    MicroVM, MicroVMState, SandboxManager, SandboxPolicy, SimpleMicroVM, SimpleSandboxManager,
};
