#![allow(warnings)]
#![allow(clippy::all)]

// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod ai;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod ipc;
pub mod kernel;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;

pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting,
};
pub use ai::{
    AIAgent, SimpleAIAgent,
    LlmConfig, LocalLlmEngine, InferenceRequest, InferenceResponse,
    QuantizationType, InferenceBackend, BatchingStrategy,
    StreamingLlmEngine, StreamingInference,
    AgentOrchestrator, SimpleAgentOrchestrator, AgentState,
    SaiAgent, SaiOrchestrator, AgentTask, SaiTask,
    AiError, ComputeBackend, LocalModel, ModelSize, SaiEngine, Tensor, TensorCore,
    AiSystemService, AiServiceManager, AiServiceConfig, AiServiceState,
    ResourceManagementService, PredictiveMaintenanceService, AdaptiveSchedulingService,
    AiServiceType, ServicePriority, AiServiceMetrics,
    VoiceAssistant, VoiceModel, VoiceRecognizer, VoiceSynthesizer,
    RecognitionResult, SynthesisResult, AudioFormat, SynthesisModel,
    JaxTensorSharding, SwiGluActivation, GrokMoeRouter, RotaryPositionEmbedding,
    GrokGqaMapper, GrokWeightStreamer,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    ApplicationBinary, BIOSGatewayMesh, BinaryFormat, BuildCodexGrid, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ConstellationNode, ContainerRuntime,
    CorebootGatewayMesh, DACConstellation, DotMatrixMesh, DriverArchiveGridV2, EosLogTool,
    EosMirrorReflector, EosUpdateNotifier, EosWelcomeEngine, FhsConventionStatus, FileAlmanacHub,
    FirmwareGatewayMesh, FloppyMesh, GraphicsArchiveGridV2, KernelConstellationGrid,
    LegacyAsmCodexGrid, LegacyCCodexGrid, LegacyCppCodexGrid, LegacyDriverAdapter, LegacyFSAdapter,
    LegacyKernelAdapter, LegacyPackageAdapter, LegacyProtocolAdapter, LegacySecurityAdapter,
    LegacyUIAdapter, LsbProfile, Mirror as EosMirror, NetworkAlmanacHub, NetworkArchiveGridV2,
    PeripheralArchiveMesh, PosixComplianceLevel, ProcessAlmanacHub, SELinuxConstellation,
    SecurityConstellation, StandardsComplianceManager, StorageArchiveGridV2, SyscallAlmanacHub,
    TapeMesh, TargetPlatform, TranslationLayer, UEFIGatewayMesh, WelcomeTab as EosWelcomeTab,
    YayAurHelper, ZeroTrustConstellation,
};
pub use container::{
    ContainerCapability, ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability, RuntimeStats,
    SimpleContainer, SimpleContainerRuntime,
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
    ABIManager, AiNativeRuntime, BuddyAllocator, Channel, EnergyAwareScheduler, FastPathIpc,
    Generation, GenerationManager, InterruptMechanism, IpcError, IpcManager, KernelGraph,
    KernelPersona, KernelPlugin, KernelPluginManager, LegacyScheduler, MemoryBlock, Message,
    MetaKernel, MicroDriver, NetPod, PolicyError, PolicyManager, Priority, PrivacyFirstSandbox,
    PrivilegeLevel, Process, ProcessState, ProtectionDomain, ResourceBroker, RoundRobinConfig,
    RoundRobinScheduler, Scheduler, SchedulerError, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions, PAGE_SIZE,
};
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack, SimpleSigmaDebug, SimpleSigmaMetrics, SimpleSigmaTrace,
};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat, PackageFormatAdapter,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
    SplitDirection as TmuxSplitDirection, LayoutPreset as TmuxLayoutPreset,
    TmuxPane, TmuxWindow, TmuxSession, TmuxSessionManager,
};
pub use remote::{
    FileTransfer, InputAuthGate, PqcVideoCipher, RemoteDesktop, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager, SigmaRendezvous,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    CapabilityGate, CapabilityToken, DomainID, DomainOrchestrator, DomainType, IsolatedDomain,
    IsolationError, Permission, PledgeManager, PledgePromise,
    SecurityEnforcer as AndroidStyleSecurityEnforcer, PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use ipc::{
    IPCEndpoint, IPCError, IPCType, IPCInfo, IPCCapability,
    Pipe, MessageQueue, SharedMemory, IPCManager,
    SerenityIpcMessage, SerenitySharedBackingStore, SerenityIpcSandboxEnforcer,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
    AcpMessage, AcpMessageType, ShellContext, TerminalErrorHook, IntelligentTerminal,
};
pub use sigpkg::{
    AdapterError, BuildSystem, ContentAddressedStore, CryptoVerifier, DebAdapter,
    PackageDependencyResolver, PackageFormatAdapter as SigpkgPackageFormatAdapter, PackageRecipe,
    PacmanAdapter, RecipeError, RecipeManager, RpmAdapter, SatSolver, Transaction,
    UniversalPackageManager as SigpkgUniversalPackageManager, Version, MAX_RECIPE_DEPENDENCIES,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
