#![no_std]
#![allow(clippy::all, unused)]

#[macro_use]
extern crate alloc;

// SigmaOS Library
// Core library for SigmaOS operating system

#[macro_use]
pub mod klib;

pub mod accessibility;
pub mod ai;
pub mod arch;
pub mod audio;
pub mod automation;
pub mod boot;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod distro;
pub mod driver;
pub mod filesystem;
pub mod kernel;
pub mod memory;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod process;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod runtime;
pub mod sigpkg;
pub mod storage;
pub mod thread;
pub use process::{
    ProcessControlError, ProcessVmReadWriteEngine, JobState, CoreDumpMetadata, ProcessJobEntry,
    JobControlLifecycleEngine, WNOHANG, WUNTRACED, WCONTINUED, BsdRusage, WaitStatus,
    ProcessWaiterAndRusageCollector, CancellationType, ProcessCancelState,
    ProcessCancellationAndTerminationManager, PosixMessage, PosixMessageQueue, EventFd,
    SigQueuePayload, AdvancedIpcHub, SovereignProcessState, SovereignProcess, ZeroCopyIpcChannel,
    SovereignProcessManager,
};
pub mod community;
pub mod access;
pub mod tools;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod userland;
pub mod open_source_obsoletion;
pub mod open_source_os_gap_closure;

pub use open_source_os_gap_closure::{
    BfsAttribute, BfsIndexedFile, CrossbowVnic, DriverHealthState, HaikuBfsAttributeEngine,
    ManagedDriverUnit, Minix3ReincarnationServer, NetBsdRumpKernelEngine, Plan9Message,
    Plan9MessageType, Plan9P2000ProtocolEngine, Plan9RforkFlags, RumpDeviceNode,
    SmartOsCrossbowVnicEngine,
};

pub use unimplemented_features::{
    AntiXLowRamSysVInitGovernor, BareMetalPeripheralManager, BareMetalUnifiedPeripheral,
    GenerationManager, GentooPortageMaskResolver, HaikuMediaTranslator, HaikuTranslatorEngine, Jbd2TransactionLedger,
    LegacyController, ModernController, PciBusScanner, PowerState, SatSolverEngine,
    SerenityIpcEvent, SerenityOsAsyncIpcLoop, SovereignIpcBus, UdfVm, ZorinAppMapping,
    ZorinWinAppDbRegistry, AlpineApkPackageIndex, DragonFlyHammer2FsSnapshot, NixOsDeclarativeConfigEngine,
};
pub use distro::{
    ApkChrootBuildSandboxEngine, OpenBsdFdPledgeGate, FreeBsdGeomVdevTopology, GeomVdevNode,
    HermeticStoreClosureEngine, StoreClosurePackage, System76PowerGovernor, PowerProfileMode,
    GpuSwitchMode, Hammer2PfsClusterQuorumEngine, HardenedBsdPaxGuardEngine, PaxViolationType,
};
pub use security::{
    HardenedSyscallDispatcher, HardenedSyscallError, MemoryAccessError,
    PagePermissions, RetpolineKptiMitigationEngine, SmepSmapEnforcer, SovereignKaslrEngine,
    KaliAirgeddonWifiAudit, KaliMetasploitPayloadFilter, KaliWiresharkPacketAnalyzer,
    PcapPacketHeader, WifiFrameType,
};
pub mod expanded_wiki_innovations;
pub mod virtualization;

pub mod interrupt;

pub mod graphics {
    pub mod compositor;
    pub mod paint;
    pub mod video;
}
pub mod hardware {
    pub mod compatibility;
    pub mod win32;
}
pub mod init;
pub mod ml;
pub mod performance;
pub mod power {
    pub mod governor;
}
pub mod scheduler {
    pub mod numa_scheduler;
}
pub mod toolchain;
pub mod ui;
pub mod crypto {
    pub mod vectorized_pqc;
}
pub mod distro_innovations;
pub mod extended_distro_matrix;
pub mod linuxmint_inspirations;
pub mod arch_kernel_inspirations;
pub mod distro_inspirations;

pub use compatibility::mint_linux::{
    LoopbackDiskFormat, Mint4WinInstallationConfig, Mint4WinInstallerEngine,
    MintAppMetadata, MintBackupTool, MintReportAlert, MintReportAlertSeverity, MintReportSystem,
    MintSoftwareManager, MintUpdateLevel, MintUpdateManager, MintUpdatePackage,
    WindowsBootloaderType,
};

pub use linuxmint_inspirations::{
    AppTheme, BulkyRenamer, CaptainInstaller, ConfigBackend, DebPackage, DiagnosticField,
    FsFormat, HypnotixIptvPlayer, IsolationMode, LanPeer, LanWarpEngine, MintConfigHub,
    MintNannyFilter, MintReportDiagnostics, MintStickFormatter, MintWelcomeFlow, NannyDecision,
    ProviderType, RenameRule, TransferOutcome, TvChannel, WebEngineKind, Webapp, WebappManager,
    XAppThemeEngine, ThingyEntry, ThingyKind, ThingyRecentDocs, WelcomeStep,
    WARP_AUTH_PORT, WARP_MDNS_UDP_PORT, WARP_TRANSFER_PORT,
};

pub use arch_kernel_inspirations::{
    AdvisorySeverity, AlpmAction, AlpmPackage, AlpmResolutionError, AlpmTransactionEngine,
    AlpmTransactionItem, Expectation, ExpectationKind, HookAction, InitramfsHook, KUnitEngine,
    KUnitSuiteResult, KUnitTestCase, MkinitcpioHookFramework, PackageSignoff, RebuildOrderSolver,
    ReproducibleBuildVerdict, ReproducibleStatus, SecurityAdvisory, SecurityAdvisoryTracker,
    Signer, SignerPolicy, SignoffCount, SignoffEntry, SignstarService,
};

pub use distro_inspirations::{
    AppStreamModuleStream, BlackarchCategory, BlackarchRepository, BlackarchTool, BlackmanBuild,
    ElevateMigration, FlatcarImmutableRootfs, FormFactor, FreePolicyVerdict, GamescopeCompositor,
    InterfaceFlag, IsolationKind, KaliMetapackage, KaliToolGroup, NebraskaInstance,
    NebraskaUpdateServer, PhoshConvergence, PressureVessel, PuppySaveSession, PureosFreePolicy,
    RancherOsCloudConfig, RaspiConfigTool, ReleaseChannel, SaveMode, SigRepository, SteamABImageUpdate,
    TorStreamIsolation, UpdateStrategy, WhonixSplit, WoofCeLayer, ZincatiUpdateAgent,
};

pub use tools::simple_scan::{
    SaneScanOptions, SaneScannerDevice, ScanColorMode, ScanExportFormat, ScanSource,
    ScannedPage, SovereignSimpleScanEngine,
};

pub use compatibility::fedora::{
    CryptoPolicyLevel, FedoraAdwaitaIconThemeEngine, FedoraAnacondaKickstartGenerator,
    FedoraBtrfsSnapshot, FedoraBtrfsSnapperSnapshotEngine, FedoraCockpitWebConsoleEngine,
    FedoraCoprRepositoryEngine, FedoraCryptoPoliciesEngine, FedoraDeskletItem,
    FedoraDeskletWidgetEngine, FedoraDnf5PackageEngine, FedoraDnfHistoryRollbackEngine,
    FedoraDnfTransaction, FedoraFirewalldPolicyEngine, FedoraFlatpakSandboxManager,
    FedoraFolderColorSwitcherEngine, FedoraGettextL10nEngine, FedoraGnomeCinnamonShellBridge,
    FedoraGpuPowerMode, FedoraKeyringPamModule, FedoraKojiTaskRunner,
    FedoraLiveMediaOverlayEngine, FedoraMediaWriterEngine, FedoraMockChrootEnvironment,
    FedoraNautilusFileBrowserEngine, FedoraNvidiaPrimeSwitcherEngine,
    FedoraPipewireAudioSessionEngine, FedoraSilverblueRpmOstreeEngine,
    FedoraSsdEnterpriseDirectoryClient, FedoraWebappContainerEngine, FedoraWebappProfile,
    FedoraWelcomeInitialSetupEngine, FolderColor,
};

pub mod auth;
pub mod app;
pub mod drivers;

pub use desktop::mate_betsy::{
    AtrilDocumentViewer, CajaFileManager, EyeOfMateImageViewer, MarcoWindowManager,
    MateBetsyDesktopEnvironment, PlumaTextEditor,
};
