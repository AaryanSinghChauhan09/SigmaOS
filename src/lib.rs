#![allow(warnings, clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod accessibility;
pub mod ai;
pub mod audio;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod distro;
pub mod driver;
pub mod drivers;
pub mod ecosystem;
pub mod education;
pub mod fs;
pub mod init;
pub mod net;
pub mod filesystem;
pub mod finance;
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
pub use ai::{
    Agent, Agent as SaiAgent, AgentOrchestrator, AgentOrchestrator as SaiOrchestrator, AgentRole,
    AgentState, AgentTask, AgentTask as SaiTask, AiError, ComputeBackend, LocalModel, ModelSize,
    SaiEngine, Task as AiTask, TaskStatus, Tensor, TensorCore,
};
pub use audio::{
    AlsaAudioStack, AudioChannels, AudioCodec, AudioDirection as AlsaDirection, AudioDriver,
    AudioDriverError, AudioDriverResult, AudioFormat, AudioFormat as AlsaFormat, AudioMetadata,
    AudioSampleRate, ChannelConfig, DecodedAudio, MixerControl, PcmStream,
    SampleRate as AlsaSampleRate,
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
pub use finance::{
    GoodsType, GstCalculator, GstRate, GstRegime, GstResult, GstState, TdsCalculator, TdsResult,
    TdsSection,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
};
pub use legal::{
    ComplianceCert, ComponentLicense, LegalComplianceRegistry, LicenseType, PatentRecord,
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

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{SystemdEngine, SystemdUnit, UnitState, UnitType};

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
pub use compatibility::canonical::{
    SigmaSubiquity, SigmaNetplan, SigmaCloudInit, SigmaMultipass, SigmaCurtin,
};
pub use scheduler::numa_scheduler::{
    NumaNode, NumaScheduler, Node as LFNode, MichaelScottQueue, TreiberStack,
};
pub use crypto::vectorized_pqc::{
    VectorizedPqcEngine,
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
