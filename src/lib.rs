#![allow(warnings)]
#![allow(clippy::all)]

// SigmaOS Library
// Core library for SigmaOS operating system

pub mod audio {
    pub mod driver;
    pub mod editor;
}
pub use audio::driver::{
    AudioDevice, AudioDeviceID, AudioError, AudioManager, AudioMixer, AudioStream, AudioType,
    SimpleAudioDevice, SimpleAudioManager, SimpleAudioMixer, SimpleAudioStream,
};
pub use audio::editor::{
    AmplifyEffect, AudioEditor, AudioEffect, AudioTrack, EchoEffect, LowPassFilter,
    MultiTrackSession, NoiseGateEffect,
};

pub mod auth;
pub use auth::{
    UserID, UserState, User, AuthError, SimpleUser, AuthService, SimpleAuthService,
    PermissionID, PermissionType as AuthPermissionType, AccessResult, Permission as AuthPermission, SimplePermission, AccessControl,
    AccessError as AuthAccessError, SimpleAccessControl,
    IdentityID, IdentityType, IdentityError as AuthIdentityError, DigitalIdentity, SimpleDigitalIdentity,
    IdentityManager, SimpleIdentityManager, CredentialManager, SimpleCredentialManager,
    DecentralizedAuth, SimpleDecentralizedAuth,
};

pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod network;
pub mod orchestration;
pub mod distro;
pub mod package;
pub mod process;
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
    pub mod win32;
}
pub mod power {
    pub mod governor;
}
pub mod observability {
    pub mod profiler;
}
pub mod ai {
    pub mod agent;
    pub mod orchestrator;
}
pub mod boot;
pub mod toolchain {
    pub mod adapter;
    pub mod capsule;
    pub mod codex;
    pub mod bootstrap;
}
pub mod scheduler {
    pub mod numa_scheduler;
}
pub mod crypto {
    pub mod vectorized_pqc;
}

pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting, BrailleDisplay, ColorFilter, KeyID, KeyType,
    Magnifier, MagnifierID, MagnifierManager, OnScreenKeyboard, ScreenReader, SimpleBrailleDisplay,
    SimpleColorFilter, SimpleMagnifier, SimpleMagnifierManager, SimpleOnScreenKeyboard,
    SimpleScreenReader, SimpleStickyKeys, SimpleVirtualKey, SimpleVoice, StickyKeys, VirtualKey,
    Voice, VoiceGender, VoiceID,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer, WasmState, WasmModule, WasmSandboxEngine,
    PledgePermission, PledgeUnveilSandbox, PqcSecureChannel, Literal, Clause,
    DpllSatSolver, CasObject, ContentAddressedStorage,
    TinyCoreBootConfig, TczExtension, TceLoader, FiletoolOverlay, FrugalLoader,
    MetricAggregation, OssieMetric, OssieDimension, OssieRelationship, OssieCatalog,
    SemanticRow, OssieInterpreter, OssieOntology,
    EverySearch, SysDiag, ProcessExplorerState, ProcMonitor, CreativeMatrix, ImageLayer,
    FancyZonesManager, LayoutZone, JoplinE2ee, SpreadsheetCore,
    UseFlagManager, OpenRcRunlevel, ServiceStatus, OpenRcService, OpenRcManager, EbuildPackage, PortageEngine,
    NDArray, ImageMat, DependencyProperty, VisualState, SovereignControl,
    GrpcFrame, SovereignGrpcChannel, MachMessage, MachPort, SovereignXnuKernel,
    Glyph, SovereignFreeTypeEngine, NavDirection, NavElement, SovereignSpatialNavigation,
    ApplicationBinary, BinaryFormat, CasObject, Clause, CompatibilityError, CompatibilityManager,
    CompatibilityMode, ContainerRuntime, ContentAddressedStorage, CreativeMatrix, DpllSatSolver,
    EverySearch, FancyZonesManager, FiletoolOverlay, FrugalLoader, ImageLayer, JoplinE2ee,
    LayoutZone, Literal, MetricAggregation, OssieCatalog, OssieDimension, OssieInterpreter,
    OssieMetric, OssieOntology, OssieRelationship, PledgePermission, PledgeUnveilSandbox,
    PqcSecureChannel, ProcMonitor, ProcessExplorerState, SemanticRow, SpreadsheetCore, SysDiag,
    TargetPlatform, TceLoader, TczExtension, TinyCoreBootConfig, TranslationLayer, WasmModule,
    WasmSandboxEngine, WasmState,
    NDArray, ImageMat, DependencyProperty, VisualState, SovereignControl,
    GrpcFrame, SovereignGrpcChannel, MachMessage, MachPort, SovereignXnuKernel,
    Glyph, SovereignFreeTypeEngine, NavDirection, NavElement, SovereignSpatialNavigation,
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
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process as KernelProcess,
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
pub use process::spawn::{
    ProcessID, SIGINT, SIGKILL, SIGUSR1, SIGSEGV, SIGTERM, ProcessState as SpawnProcessState,
    ProcessError, SimpleProcess, ProcessSpawner, SimpleProcessSpawner, ProcessWaiter,
    SimpleProcessWaiter, ProcessGroup, SimpleProcessGroup,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::hardening;
pub use security::{
    secure_zeroize, AuditLogEntry, CapabilityGate, CapabilityToken, ExploitPayload,
    HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity, PenetrationAssistant, Permission,
    PledgeManager, PledgePromise, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    AptDebManifest, BuildSystem, ContentAddressedStore, CryptoVerifier, FlatpakManifest,
    PackageRecipe, PacmanPkgbuild, RecipeError, RecipeManager, SatSolver, SnapcraftManifest,
    Transaction, UniversalPackageAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{SystemdEngine, SystemdUnit, UnitState, UnitType};

pub mod ai {
    pub mod next_gen;
    pub mod wandr;
}
pub use ai::next_gen::{
    AIModel, AdaptiveKernelPersona, AiScheduler, AiTask, DeviceTargetType, EnergyAwareScheduler,
    EnergyGovernorMode, ModelType, MultiModelOrchestrator, PredictiveSyscallTranslator,
    WorkloadType,
};
pub use ai::wandr::{
    ResearchResult, SigmaWandrAgent, WandrDocument, WandrEvaluator, WandrResearchAgent, WandrTask,
};

pub mod virt;
pub use virt::hypervisor::{
    Guest, GuestID, GuestState, Hypervisor, HypervisorError, SimpleGuest, SimpleHypervisor,
    VirtualizationGeneration,
};
pub use virt::microvm::{
    MicroVM, MicroVMState, SandboxManager, SandboxPolicy, SimpleMicroVM, SimpleSandboxManager,
};
