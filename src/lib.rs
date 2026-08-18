// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod ai;
pub mod automation;
pub mod boot;
pub mod compatibility;
pub mod container;
pub mod dashboard;
pub mod device;
pub mod distro;
pub mod driver;
pub mod drivers;
pub mod ecosystem;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod unimplemented_features;
pub mod unimplemented_tools;
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
    pub mod stack;
}
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
    AccessibilityProfile, AccessibilitySetting,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
    ScriptArgumentRouter,
};
pub use compatibility::{
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
    EnterprisePartner, CodeSnippet, NDArray,
    SigmaGrpcEngine, GrpcServiceStub, MachMessageHeader, MachPort, MachZone,
    SigmaFreeTypeFont, UiRect, NavigationDirection, SpatialNavigationEngine,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType as DashboardMetricType, SystemMonitor, UnifiedDashboard, WidgetType,
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
    ABIManager, AiNativeRuntime, BuddyAllocator, Channel, EnergyAwareScheduler, FastPathIpc,
    Generation, GenerationManager, InterruptMechanism, IpcError, IpcManager, KernelGraph, KernelPersona, KernelPlugin,
    KernelPluginManager, LegacyScheduler, MemoryBlock, Message, MetaKernel, MicroDriver, NetPod,
    PAGE_SIZE, PolicyError, PolicyManager, PrivacyFirstSandbox, Priority, Process, ProcessState,
    ProtectionDomain, PrivilegeLevel, ResourceBroker, RoundRobinConfig, RoundRobinScheduler,
    Scheduler, SchedulerError, SelfHealingKernel, SigmaFsPlusPlus, UniversalAbiTranslator,
    UserDefinedKernelFunctions, GapError, Pml4PageTableEntry, VirtualMemoryPagingManager,
    IrqRoutingTable, AcpiInterruptManager, JournalState, JournalBlock, MetadataJournal,
    VirtualCpuError, CpuMode, CpuRing, RegisterSet, SovereignVirtualCPU, Instruction,
    Irql, CpuArch as WdkCpuArch, SecurityToken as WdkSecurityToken, AddressSpace as WdkAddressSpace, ExecutionContext as WdkExecutionContext,
    ThreadState as WdkThreadState, ApcMode as WdkApcMode, Apc as WdkApc, Dpc as WdkDpc, WorkItem as WdkWorkItem, WdkThread,
    EventType as WdkEventType, EventObject as WdkEventObject, SpinLock as WdkSpinLock, MutexObject as WdkMutexObject, FastMutex as WdkFastMutex, GuardedMutex as WdkGuardedMutex, EResource as WdkEResource,
    WdkTimer, TimerTable as WdkTimerTable, Prcb as WdkPrcb,
    PoolType as WdkPoolType, PoolAllocation as WdkPoolAllocation, KernelPoolMemory as WdkKernelPoolMemory,
    IoStatusBlock as WdkIoStatusBlock, IoctlControl as WdkIoctlControl, IRP as WdkIRP, WdkDriverObject, BugCheckData as WdkBugCheckData, BugCheckRegistry as WdkBugCheckRegistry,
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
    DebianPackageHeader, DebianPackageParser, AptSandboxedDeployment, DebianParityVerifier, SandboxCapability as PackageSandboxCapability,
};
pub use remote::{
    FileTransfer, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use kernel::vmm_paging::{PageTableFlags as VmmPageFlags, PageTableManager as VmmPageTableManager, VirtualMemoryManager as VmmManager, VmArea, VmProtection};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken, DefensiveAuditSystem,
    ArithmeticSubstitutionDeobfuscator,
    ForensicBlock, ForensicStorageFilter, MaliciousSignature, Permission, PledgeManager,
    PledgePromise, RoutingMode, SandboxPolicy, GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
    MAX_AUDIT_BLOCKS, MAX_SIGNATURES, SIGNATURE_LEN,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageDependencyResolver, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, Version, MAX_RECIPE_DEPENDENCIES, AdapterError,
    DebAdapter, RpmAdapter, PacmanAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
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
