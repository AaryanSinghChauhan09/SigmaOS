extern crate alloc;
// SigmaOS Library
// Core library for SigmaOS operating system

pub mod klib;
pub mod accessibility;
pub mod ai;
pub mod audio;
pub mod ipc;
pub mod init;
pub mod app;
pub mod auth;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod crypto;
pub mod filesystem;
pub mod futuristic_modules;
pub mod kernel;
pub mod klib;
pub use klib::ZeroDependencyPrimitiveHub;
pub mod memory;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod process;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod runtime;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod storage;
pub mod virtualization;

pub mod interrupt;


pub mod hardware {
    pub mod compatibility;
    pub mod win32;
}
pub mod power {
    pub mod governor;
}
pub mod boot;
pub use boot::*;
pub mod toolchain {
    pub mod adapter;
    pub mod bootstrap;
    pub mod capsule;
    pub mod codex;
}
pub mod scheduler;
pub mod logging;
pub mod system;
pub use system::{AutomationTaskKind, SovereignAutomationEngine, TaskStatus};
pub mod update {
    pub mod distro_update_parity;
}
pub use update::distro_update_parity::{
    SovereignSystemUpdateAndTestingEngine, SystemDiagnosticReport,
};
pub mod installer;
pub mod iot;
pub mod ml;
pub mod performance;

pub mod distro;
pub mod distro_innovations;
pub mod arch_kernel_inspirations;
pub mod distro_inspirations;
pub mod linuxmint_inspirations;
pub mod innovation;
pub use innovation::{
    BootStageKind, BootStageRecipe, ComposableBootSequencesEngine, DriverShard,
    FilesystemAsDatabaseEngine, HardwareAbstractionShardsEngine, ImmutableUserlandLayersEngine,
    KernelPersonality, LayeredKernelPersonalitiesEngine, LegacyAbiEnvironment,
    NetworkNativeOsStateEngine, OsSessionState, ProgrammableSchedulerEngine,
    RetroSandboxSession, RetrocompatibilitySandboxEngine, SchedulingPolicyRule,
    UserlandOverlayLayer, VfsObjectRecord,
};

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{SystemdEngine, SystemdUnit, UnitState, UnitType};

pub use ai::next_gen::{
    AIModel, AdaptiveKernelPersona, AiTask, DeviceTargetType, EnergyGovernorMode, ModelType,
    MultiModelOrchestrator, PredictiveSyscallTranslator, WorkloadType,
};
pub use ai::wandr::{
    ResearchResult, SigmaWandrAgent, WandrDocument, WandrEvaluator, WandrResearchAgent, WandrTask,
};
pub use distro::{
    AkmKernelManager, CalamaresConfig, CalamaresInstaller, EosLogTool, EosWelcomeApp,
    ReflectorMirrorManager, YayParuHelper,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    Bdle, Ch340Driver, DeviceGeneration, E1000Driver, GpuCommand, GpuCommandBuffer, GpuDriver,
    GpuError, GpuPipeline, GpuShader, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, IntelHdaDriver, NetworkCommand, NetworkDriver, NetworkError,
    NetworkType, NvmeCmd, NvmeCqe, NvmeDriver, PeripheralDevice, PeripheralManager, PowerState,
    RxDescriptor, ShaderStage, StorageCommand, StorageDriver, StorageError, StorageType,
    TxDescriptor, UsbHidDriver, VesaDriver, VesaError, VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, LegacyLinuxRule, LinuxPersonaRule,
    SmartSymlink, SymlinkResolverRule, VirtualFilesystem,
};
pub use graphics::paint::ColorRgba;
pub use kernel::{
    AdaptivePolicy, AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, ArchitectureEngine,
    AuditBlock, BuddyAllocator, Channel, CircularDoublyLinkedList, CpuArchitectureClass,
    CpuRegisters, EdfTask, HardwareException, InstructionCyclePhase as ArchInstructionCyclePhase,
    InstructionCyclePhase, InterruptClass, IoWaitProfile, IpcError, IpcManager, Irql,
    KernelMechanism, KernelPolicy, LcgRandom, LookasideList, LotteryTask, MemoryBlock,
    MemoryDescriptorList, Message, Pcb, PolicyMechanismCoordinator, PoolType, Priority, Process,
    ProcessState, ProcessorInitState, RoundRobinConfig, RoundRobinScheduler, Scheduler,
    SchedulerError, SequencedSinglyLinkedList, SinglyLinkedList, SovereignMechanism, SystemThread,
    Tcb, ThreadState, WorkItem, PAGE_SIZE,
};
pub use network::{
    FirewallAction, FirewallCommand, FirewallFilterRule, IpRoute2Command, LinkState, PingCommand,
    SocketStatsCommand, SocketStatsEntry, TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
    UfwDefaultRule, GLOBAL_FIREWALL, GLOBAL_IP_COMMAND, GLOBAL_UFW_RULE,
};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use performance::{
    AnanicyCppDaemon, AnanicyRule, BoreScheduler, CachyKernelManager, CallGraph,
    CpuPriorityOptimizer, GlarySmartRule, IoPriorityOptimizer, IoSchedClass, IoTaskPriority,
    PerformanceProfileRule, PhysicalPageFrame, Profile, ProfileType, Profiler, ProfilerError,
    RamDefragmenter, SimpleCallGraph, SimpleProfile, SimpleProfiler, SmartPerformanceProfile,
    SmartResourceOptimizer, UltraKernelSamepageMerger, X86v3v4OptimizationDetector,
    GLOBAL_GLARY_RULE, GLOBAL_SMART_OPTIMIZER,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore, EverythingSearchEngine, NotepadPlusPlusBuffer, SovereignBrowserEngine, SevenZipEngine,
    CompressionMethod, FlameshotAnnotator, AnnotationShape, ObsStudioMixer,
    AudacityWaveEditor, VlcCodecPipeline, DaVinciTimeline, OneCommanderFileGrid,
    ItemAgeColor, EarTrumpetVolumeMatrix, IrfanViewEngine,
};
pub use resilience::{
    FsSnapshot, RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError,
    SelfHealingModule, SigmaTimeshift, SystemSnapshot, GLOBAL_TIMESHIFT,
};
pub use security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken, CrowdStrikeFalconAi,
    DefensiveAuditSystem, ForensicBlock, ForensicStorageFilter, IntrusionDetectionSystem,
    LinuxCapability, MaliciousSignature, MAX_AUDIT_BLOCKS, MAX_SIGNATURES, PamError, PamGroup,
    PamUser, Permission, PledgeManager, PledgePromise, RoutingMode, SandboxPolicy, Securelevel,
    SIGNATURE_LEN, SnortRule, SnortSignatureFirewall, SovereignPamManager,
    SovereignSecurelevelManager, GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use storage::{
    BioCmd, BioRequest, GeomClassType, GeomConsumer, GeomEliConfig, GeomProvider, GeomTopology,
    PartitionEntry,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use tools::native_userland_replacements::MasterNativeUserlandReplacements;
