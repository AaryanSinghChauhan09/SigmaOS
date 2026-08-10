// SigmaOS Library
// Core library for SigmaOS operating system

pub mod audio {
    pub mod driver;
    pub mod editor;
    pub mod sigma_audio;
}
pub use audio::driver::{
    AudioDeviceID, AudioType, AudioError as AudioDriverError, AudioDevice as AudioDriverDevice, SimpleAudioDevice, AudioManager,
    SimpleAudioManager, AudioMixer, SimpleAudioMixer, AudioStream, SimpleAudioStream,
};
pub use audio::editor::{
    AudioTrack, MultiTrackSession, AudioEffect, AmplifyEffect, EchoEffect, LowPassFilter, NoiseGateEffect, AudioEditor,
};
pub use audio::sigma_audio::{
    AudioNode, AudioNodeType, AudioFormat, AudioLink, AudioGraph, GraphState,
    AudioDevice, DeviceType, AudioProfile, AudioSession, SessionState,
    SigmaAudio, AudioStats, AudioError as SigmaAudioError,
};

pub mod accessibility;
pub mod automation;
pub mod boot;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod edge;
pub mod filesystem;
pub mod iot;
pub mod klib;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod kernel;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod power;
pub mod productivity;
pub mod resilience;
pub mod release;
pub mod rt;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod testing;
pub mod virtualization;

pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting,
    VoiceID, VoiceGender, SimpleVoice, Voice, ScreenReader, SimpleScreenReader, BrailleDisplay, SimpleBrailleDisplay,
    MagnifierID, Magnifier, SimpleMagnifier, MagnifierManager, SimpleMagnifierManager, ColorFilter, SimpleColorFilter,
    KeyID, KeyType, VirtualKey, SimpleVirtualKey, OnScreenKeyboard, SimpleOnScreenKeyboard, StickyKeys, SimpleStickyKeys,
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
    DnfPackageResolver, MockChrootBuilder, KojiBuildServer, BodhiUpdateTriage,
    FirewalldZone, RichRule, FirewalldZoneManager, PartitionLayout, AnacondaKickstartInstaller,
    CoprBuildJob, CoprUserRepoBuilder, IpaUser, HbacRule, FreeIpaDirectoryService,
    BsdJail, FreeBsdJailManager, OpenBsdSysctlKernelMib,
    WorkloadCategory, SigmaScheduler, UniversalAbiTranslator, SigmaFsPlusPlus, SelfHealingOS,
    JudicialTimelinePlanner, MsmeComplianceEngine, AyushFormularyHelper, PMWaniHotspotController,
    DigiYatraPassScanner, IrctcPnrTracker,
    SysVinitRunlevel, SysVinitManager, AptPackageMetadata, AptRepositorySynchronizer,
    AlternativeProvider, DebianAlternativesSystem, DebootstrapEngine,
    BinderTransactionType, BinderParcel, AospBinderIpc, LaunchdServiceState, LaunchdService,
    MacosLaunchdDaemon, SecureEnclaveKeyStore,
    YastModuleType, YastCentralControlCenter, SlackwarePackage, SlackwarePkgTools,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use release::{
    Release, ReleaseType, ReleaseStatus, ReleaseManager, ReleaseError,
    VersionManager, VersionError,
};
pub use rt::{
    RealTimeTask, TaskState, SchedulingPolicy, LatencyMonitor, LatencyMeasurement,
    TimingAnalyzer, SigmaRT, RTStats, RTError,
};
pub use observability::{
    Metric, MetricType, MetricsCollector, LogEntry, LogLevel, LogAggregator,
    TraceSpan, TracingSystem, Dashboard, Panel, PanelType, SigmaObservability,
    ObservabilityStats, ObservabilityError,
};
pub use testing::{
    TestSuite, TestCase, TestResult, TestSummary,
    UnitTestFramework, IntegrationTestFramework, PerformanceTestFramework,
    SecurityTestFramework, FuzzingTestFramework,
    Benchmark, BenchmarkResult, PerformanceSummary,
    SecurityTest, SecurityTestResult, SecuritySummary, SecuritySeverity,
    Fuzzer, FuzzerResult, FuzzingSummary,
    OverallTestSummary,
};
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
    StoragePool, RaidLevel, Dataset, DatasetType, Snapshot, Zvol, SendStream, ReceiveStream,
    SigmaFSManager, FilesystemStats, SigmaFSError, CompressionAlgorithm,
};
pub use container::{
    Container, ContainerState, ContainerError, ContainerImage, ContainerRuntime,
    Pod, PortMapping, VolumeMount, ContainerNetwork, Volume, RuntimeStats, RestartPolicy,
};
pub use edge::{
    EdgeNode, EdgeNodeState, EdgeApplication, EdgeAppState, EdgeGateway,
    DataPipeline, SyncPolicyConfig, ConflictResolution, SigmaEdge, EdgeStats, EdgeError,
};
pub use iot::{
    IoTDevice, DeviceState, IoTDeviceType, ProtocolType, TelemetryData,
    IoTGateway, DataLake, DigitalTwin, TwinState, SigmaIoT, IoTStats, IoTError,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
};
pub use network::{
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
    VirtualSwitch, SwitchPort, PortType, FlowRule, FlowMatch, FlowAction,
    SDNController, SDNControllerType, SigmaSDN, SDNStats, NetworkError as SDNError,
};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
    Cluster, ClusterState, Node, NodeState, Pod, PodPhase, Service, ServiceType,
    Deployment, DeploymentStrategy, ContainerSpec, ResourceRequirements, ContainerPort,
    ServicePort, PodTemplate, Metadata, PodSpec, SigmaKube, ClusterStats,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use power::{
    PowerProfile, BatteryState, Battery, ThermalZone, CPUGovernor,
    PowerProfileConfig, PowerManager, PowerStats, PowerError,
};
pub use boot::{
    BootEntry, BootTheme, ThemeColors, BootManager, BootStats, BootError,
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
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise,
    VulnerabilityClass, VulnerabilityReport, SecurityScanner, ExploitPayload,
    PenetrationAssistant, secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
pub use security::hardening;
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, AptDebManifest, PacmanPkgbuild, SnapcraftManifest, FlatpakManifest, UniversalPackageAdapter,
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

pub mod ai {
    pub mod next_gen;
    pub mod wandr;
}
pub use ai::next_gen::{
    WorkloadType, AdaptiveKernelPersona, PredictiveSyscallTranslator, DeviceTargetType,
    AiTask, AiScheduler, EnergyGovernorMode, EnergyAwareScheduler, ModelType,
    AIModel, MultiModelOrchestrator,
};
pub use ai::wandr::{
    WandrTask, WandrDocument, ResearchResult, WandrEvaluator, SigmaWandrAgent, WandrResearchAgent,
};

pub mod virt;
pub use virt::hypervisor::{
    Guest, GuestID, GuestState, Hypervisor, HypervisorError, SimpleGuest, SimpleHypervisor,
    VirtualizationGeneration,
};
pub use virt::microvm::{
    MicroVM, MicroVMState, SandboxManager, SandboxPolicy, SimpleMicroVM, SimpleSandboxManager,
};
pub use driver::shims::{
    IntelE1000Driver, HdaSampleRate, IntelHdaDriver, VirtioBlockOp, VirtioBlockRequest, VirtioBlockDriver,
};
