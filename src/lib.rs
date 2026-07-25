#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod klib;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod graphics;
pub mod init;
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
pub mod boot {
    pub mod firmware_bridge;
    pub mod bridge_grid;
}
pub mod toolchain {
    pub mod adapter;
    pub mod capsule;
    pub mod codex;
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
    APITimelineManager, BinaryCompatMatrix, DiscontinuedFS, DriverBridge, FSRevival,
    GraphicsBridge, KernelPersona, KernelPersonaVM, LegacyBus, LegacyDriver,
    LegacyPluginManager, LibcVersion, NetworkBridge, StorageBridge, SyscallAbi,
    WorkloadOptimizer, WorkloadProfile, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER,
    GLOBAL_WORKLOAD_OPTIMIZER,
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
pub use kernel::{
    ABIManager, BuddyAllocator, Channel, IpcError, IpcManager, KernelGraph, KernelPersona,
    KernelPlugin, KernelPluginManager, LegacyScheduler, MemoryBlock, Message, MetaKernel, MicroDriver,
    NetPod, PAGE_SIZE, Priority, Process, ProcessState, RoundRobinConfig, RoundRobinScheduler,
    Scheduler, SchedulerError,
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

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{
    SystemdEngine, SystemdUnit, UnitState, UnitType,
};

pub mod virt;
pub use virt::hypervisor::{
    Guest, GuestID, GuestState, Hypervisor, HypervisorError, SimpleGuest, SimpleHypervisor,
    VirtualizationGeneration,
};
pub use virt::microvm::{
    MicroVM, MicroVMState, SandboxManager, SandboxPolicy, SimpleMicroVM, SimpleSandboxManager,
};
pub use boot::firmware_bridge::{
    FirmwareType, FirmwareBridge,
};
pub use boot::bridge_grid::{
    BIOSBridgeGrid, UEFIBridgeGrid, CorebootBridgeGrid, FirmwareBridgeGrid,
};
pub use toolchain::adapter::{
    ToolchainProfile, ToolchainAdapter,
};
pub use toolchain::capsule::{
    CapsuleProfile, BuildCapsule,
};
pub use toolchain::codex::{
    CodexCategory, CodexEntry, BuildCodex,
};
pub use compatibility::persona::{
    PersonaVersion, KernelPersonaContainer, SyscallCategory, SyscallNode, SyscallGraph,
};
pub use compatibility::abi_translator::{
    CpuArchitecture, ABITranslator,
};
pub use compatibility::lattice::{
    LatticeFeature, KernelLattice, SyscallLifecycle, SyscallHistory, SyscallTracker,
};
pub use compatibility::prism::{
    PrismFacet, KernelPrism, LedgerEntry, SyscallLedgerbook,
};
pub use network::revival::{
    RevivalProtocol, NetRevival,
};
pub use driver::simulation::{
    SimType, PeripheralSim,
};
pub use driver::mapper::{
    MapperCategory, DriverMapper,
};
pub use driver::pods::{
    PodType, PeripheralPod,
};
pub use driver::vault::{
    VaultEntry, DriverArchiveVault,
};
pub use driver::grid::{
    GridSlotType, PeripheralArchiveGrid,
};
pub use security::bridge::{
    LegacySecurityType, SecurityBridge,
};
pub use security::prism::{
    SecurityFacet, SecurityPrism,
};
