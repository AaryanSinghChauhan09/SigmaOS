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
pub mod ecosystem;
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
pub mod unimplemented_features;
pub mod unimplemented_tools;
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
    pub mod stack;
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
    ApplicationBinary, BIOSGatewayMesh, BinaryFormat, BuildCodexGrid, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ConstellationNode, ContainerRuntime, CorebootGatewayMesh,
    DACConstellation, DotMatrixMesh, DriverArchiveGridV2, FhsConventionStatus, FileAlmanacHub,
    FirmwareGatewayMesh, FloppyMesh, GraphicsArchiveGridV2, KernelConstellationGrid,
    LegacyAsmCodexGrid, LegacyCCodexGrid, LegacyCppCodexGrid, LegacyDriverAdapter, LegacyFSAdapter,
    LegacyKernelAdapter, LegacyPackageAdapter, LegacyProtocolAdapter, LegacySecurityAdapter,
    LegacyUIAdapter, LsbProfile, NetworkAlmanacHub, NetworkArchiveGridV2, PeripheralArchiveMesh,
    PosixComplianceLevel, ProcessAlmanacHub, SELinuxConstellation, SecurityConstellation,
    StandardsComplianceManager, StorageArchiveGridV2, SyscallAlmanacHub, TapeMesh, TargetPlatform,
    TranslationLayer, UEFIGatewayMesh, ZeroTrustConstellation,
    EosMirrorReflector, EosWelcomeEngine, EosUpdateNotifier, EosLogTool, YayAurHelper,
    Mirror as EosMirror, WelcomeTab as EosWelcomeTab,
    DnfPackageResolver, MockChrootBuilder, KojiBuildServer, BodhiUpdateTriage,
    SigmaChangeProposal, SigmaChangeProcessEngine, SigmaNextChannel,
};
pub use container::{
    ContainerCapability, ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability, RuntimeStats,
    SimpleContainer, SimpleContainerRuntime,
};
pub use ecosystem::{
    ArchTier, ArchitecturePort, EcosystemCertification, EcosystemManager, EcosystemPlatform,
    EnterprisePartner, KimiCodeAssistant, CodeSnippet, NDArray, numpy_mean, numpy_std_dev,
    CvImage, WinUiControl, WinUiState, WinUiPanel,
    SigmaGrpcEngine, GrpcServiceStub, MachMessageHeader, MachPort, MachZone,
    SigmaFreeTypeFont, UiRect, NavigationDirection, SpatialNavigationEngine,
    DnfPackageResolver, MockChrootBuilder, KojiBuildServer, BodhiUpdateTriage,
    FirewalldZone, RichRule, FirewalldZoneManager, PartitionLayout, AnacondaKickstartInstaller,
    CoprBuildJob, CoprUserRepoBuilder, IpaUser, HbacRule, FreeIpaDirectoryService,
    BsdJail, FreeBsdJailManager, OpenBsdSysctlKernelMib,
    WorkloadCategory, SigmaScheduler, UniversalAbiTranslator, SigmaFsPlusPlus, SelfHealingOS,
    JudicialTimelinePlanner, MsmeComplianceEngine, AyushFormularyHelper, PMWaniHotspotController,
    DigiYatraPassScanner, IrctcPnrTracker,
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
    Ch340Driver, CH340_VENDOR_ID, CH340_PRODUCT_ID,
    E1000Driver, RxDescriptor, TxDescriptor,
    IntelHdaDriver, Bdle,
    LegacyFloppyDisk,
    LegacySerialPort,
    ModernWifiDriver,
    ModernNvmeDriver, ModernNvmeCmd, NvmeSubmissionQueue, NvmeCompletionQueue, SmartTelemetry, AhciCommandHeader, AhciPort,
    LegacyParallelPrinter,
    TouchJingosDriver,
    ModernAudioIntelHda,
    LegacyAudioAc97,
    ModernUsbPrinterDriver,
    NvmeDriver, NvmeStorageCmd, NvmeCqe,
    PinController, ClockController, GenericPin, GenericClock, SocPinController, SocClockController, UnifiedSocController, PinDirection, PinPull, PinError, ClockError,
    BluetoothHciDriver, BluetoothMode, AclPacket, ScoPacket, L2capChannel, L2capState, BluetoothError,
    PrinterCupsDriver, PrinterProtocol, PrinterBackend, PrintJob, PrintFormat, JobStatus, PrinterError,
    GpuAccelerationDriver, CommandBuffer, SuiteGpuCommand, PrimitiveType, CommandStatus, FlipRequest, DisplayMode, PixelFormat, SuiteGpuError,
    AlsaSoundDriver, RingBuffer, SampleFormat, AlsaError,
    WifiFullStackDriver, WifiState, ScanResult, SecurityType, BssInfo, WpaToken, WpaTokenType, QosMapping, WifiError,
    MultiTouchDriver, TouchProtocol, TouchContact, GestureState, GestureType, TouchError,
    VesaFramebufferDriver, Cursor, VesaFramebufferError,
    UsbHidFullDriver, HidInputReport, HidOutputReport, HidFullError,
    AncientDeviceLayer, Uart8250, IsaBus, IsaDevice, Ne2000Ethernet, MfmDiskInterface, AdLibSynth, EgaCgaAdapter, VideoMode, AncientError, HidTokenType, PrinterFormat,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process as KernelProcess,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
    ABIManager, AiNativeRuntime, BuddyAllocator, Channel, EnergyAwareScheduler, FastPathIpc,
    Generation, GenerationManager, InterruptMechanism, IpcError, IpcManager, KernelGraph, KernelPersona, KernelPlugin,
    KernelPluginManager, LegacyScheduler, MemoryBlock, Message, MetaKernel, MicroDriver, NetPod,
    PAGE_SIZE, PolicyError, PolicyManager, PrivacyFirstSandbox, Priority, Process, ProcessState,
    ProtectionDomain, PrivilegeLevel, ResourceBroker, RoundRobinConfig, RoundRobinScheduler,
    Scheduler, SchedulerError, SelfHealingKernel, SigmaFsPlusPlus, UniversalAbiTranslator,
    UserDefinedKernelFunctions, GapError, Pml4PageTableEntry, VirtualMemoryPagingManager,
    IrqRoutingTable, AcpiInterruptManager, JournalState, JournalBlock, MetadataJournal,
    VirtualCpuError, CpuMode, CpuRing, RegisterSet, SovereignVirtualCPU, Instruction,
};
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use observability::stack::{
    Metric, MetricCapability, MetricID, MetricInfo, MetricType, ObservabilityError,
    ObservabilityStack, ObservabilityStats, SigmaDebug, SigmaMetrics, SigmaTrace, SimpleMetric,
    SimpleObservabilityStack, SimpleSigmaDebug, SimpleSigmaMetrics, SimpleSigmaTrace, SimpleSpan,
    Span, SpanCapability, SpanInfo, StackCapability, TraceID,
};
pub use distro::{
    AppManifest, CertificationStatus, ComponentType, HardwareCertificate,
    HardwareCertificationProgram, HardwareProfile, HardwareRegressionSuite, QAStagedRelease,
    ReleaseStage, SoftwareCertificationProgram,
    BountyStatus, BugBountyProgram, BugBountyReport, CommunityConference, ConferenceTalk,
    ForumChannel, ForumPost, HelpSystem, HowToGuide, ManPage, WikiPage,
    DllLoader, DllModule, GdiObjectType, LinuxSyscall, PosixTranslation, RegistryType,
    RegistryValue, Win32Gdi, WindowsRegistry,
    BuildJob, BuildStatus, CrossBuildPipeline, DevTool, DeveloperToolkit, PackageBuildService,
    TargetArch,
    AuditResult, AuditRule, ComplianceAuditor, ConfigHook, DirectoryService, DirectoryUser,
    ImeCandidate, InputMethodEngine, LanguagePack, LocaleManager, RegionalSettings,
    AdminAction, AiSysAdmin, IntegrityState, P2pNode, PqcSelfHealing, SovereignP2PSync,
    TimeTravelCheckpoint, TimeTravelEngine, NetplanConfig, NetplanManager,
    LivepatchPatch, LivepatchManager,
    BackupSnapshot, BackupSystem, KernelTrace, LiveDebugger, RescueISO, RescueISOManager,
    CanFrame, EcuController, EduChallenge, EduPlayground, HpcClusterJob, HpcJobState,
    MpiCommunicator,
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
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageDependencyResolver, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, Version, MAX_RECIPE_DEPENDENCIES, AdapterError,
    DebAdapter, RpmAdapter, PacmanAdapter,
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
pub use unimplemented_features::{
    PciDevice, PciClass, PciBusScanner, Generation as NixGeneration, GenerationManager as NixGenerationManager,
    SovereignIpcBus, IpcMessage, SovereignSignal, SignalDispatcher, PageTableEntry, PagingController,
    PackageVersion as SpecPackageVersion, PackageRecipe as SpecPackageRecipe, PackageDependencyResolver as SpecPackageDependencyResolver,
    CapabilityToken as SandboxCapabilityToken, SecurityEnforcer as SandboxSecurityEnforcer,
    Rect as CompositeRect, ZenithWindow, ZenithCompositor, SysCommandType as ShellSysCommandType, MultiCallShell,
    GdtEntry, NimPOSTManager, TraceEvent, TraceSpan, SigmaTrace as SysSigmaTrace, SigmaFsCasEngine,
    FileMetadata, SovereignCleanupEngine, ThreadPriority, ActiveProcessThread, AutoResourceOptimizer as SpecAutoResourceOptimizer,
    Package as OopPackage, PackageType as OopPackageType, RpmPackage, DebPackage as OopDebPackage, SnapPackage, FlatpakPackage, AppImagePackage, SigmaPackage, UnifiedPackageManager as OopUnifiedPackageManager,
    SecurityContext, SecurityContextClass, MacPermission, AccessVectorCacheEntry, FedoraSELinuxMacEngine,
    ServiceState as SystemdServiceState, SystemdService, FedoraSystemdSupervisor, DeltaRpmDiffBlock, FedoraDeltaRpmEngine,
    PageDirectoryEntry, VirtualMemoryManager as SpecVirtualMemoryManager, NetworkProtocolType, NetworkPacket as SpecNetworkPacket, ZeroCopyNetworkStack,
    VmGuestRegisters, VmExitReason, SovereignVmm, NamespaceConfig, ContainerIsolationGuard,
    BuddyAllocator as SpecBuddyAllocator, LinuxLtsInterface, HardwareCompatibilityMatrix, NativeDriverProgram,
    SovereignGraphicalInstaller, LightweightInitSystem, SystemdCompatShim, TransactionalFsMountManager,
    PowerManagementStack, RealTimePreemptRtKernel, MeasuredBootValidator, MicroVmSandbox,
    KernelHardeningManager, UnifiedCryptographicLogger, CrashReportingPipeline, DeviceProvisioningService,
    SovereignDiagnosticsTui, OciContainerRuntime, VirtualizationCliGate, ModularKernelPackLoader,
    BootPerformanceOptimizer,
};
pub use unimplemented_tools::{
    AudioTrack, AudioEditor, PodcastRecorder, GifConverter, OverlayWidget, StreamingOverlayManager,
    CameraFilter, WebcamEffects, SubtitleLine, SubtitleEditor, SmartCleanup, PerformanceOptimizer as ToolPerformanceOptimizer,
    DiskDefragmenter, DuplicateFileFinder, BatterySaver, MemoryLeakDetector, ProcessSandbox, StartupOptimizer,
    SecureFileShredder, SystemRestoreSnapshot, AccessibilitySuite as ToolAccessibilitySuite, DiagnosticMetric, PredictiveMaintenance,
    MockHttpRequest, ApiTestingTool, GitCommitNode, GitGuiClient, GamifiedTodoTask, GamifiedTodo,
    MindMapNode, MindMapCreator, KanbanColumn, KanbanTask, KanbanBoard, GameDetails, GameHubLauncher,
    EmulatorCore, EmulatorManager, GameRecorder, GamePerformanceBooster, CloudGaming, VrPose, VrArRuntime,
    ButtonToKeyMapping, ControllerMapper, ModDetails, GameModManager, AiDifficultyDirector, GamifiedDesktop,
    GanttTask, GanttChartPlanner, IPdfCompressor, IPdfMerger, IPdfSigner, PdfEditor, DocumentScanner,
    ProfileSample, CodeProfiler, StaticAnalysisWarning, StaticAnalyzer, PackagePublishingHub,
    AdaptiveUxAgent, AiSearchAssistant, NaturalLanguageShell, AiCodeAssistant, AiFileOrganizer,
    Notification, SmartNotificationManager, RemoteDesktop as ToolRemoteDesktop, MeshPeer, MeshNetworking,
    IotDevice, IotDeviceManager, CloudBackupUtility, SecureFileSharing, AutomationRoutine, AiScheduler,
    AiComplianceDashboard, AppStoreItem, GuiAppStore, DisplayScreen, MultiMonitorManager, GestureControl,
    VoiceControl, AiTaskbar, CrossDeviceSync, FlatpakSnapLayer, DeclarativeBuildSystem, AiDependencyResolver,
    ZeroTrustTpmBoot, ForensicSnapshot, AiAnomalyFirewall, SecureContainer, PrivacyDashboard,
    OfflinePackageInstaller, AppSandboxing, CrossLanguageBuildTool, PluginDetails, PluginMarketplace,
    MusicTrack, MusicLibraryManager,
};
