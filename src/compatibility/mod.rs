// SigmaOS Compatibility Module
pub mod constellation_mesh;
pub mod cross_platform;
pub mod india_stack;
pub mod jehanne;
pub mod mint_linux;
pub mod reactos;
pub mod lubuntu;
pub mod antix;
pub mod bodhi_moksha;
pub mod cachy_os;
pub mod chakra;
pub mod chimera_linux;
pub mod endeavour;
pub mod garuda_zen;
pub mod gentoo;
pub mod tiny_core;
pub mod localsend;

pub use legacy_adapters::{
    KernelPersona, KernelPersonaVM, LibcVersion, SyscallAbi, BinaryCompatMatrix,
    APITimelineManager, LegacyBus, StorageBridge, GraphicsBridge, WorkloadProfile,
    WorkloadOptimizer, DiscontinuedFS, DriverBridge, FSRevival,
    LegacyPluginManager, NetworkBridge, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER,
    GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use freedos::{ConfigSysSetting, TsrProgram, FatDirectoryEntry, FreeDosEmulator};

pub use constellation_mesh::{
    BIOSGatewayMesh, BuildCodexGrid, CRTMesh, ConstellationNode, CorebootGatewayMesh,
    DACConstellation, DotMatrixMesh, DriverArchiveGridV2, FileAlmanacHub, FirmwareGatewayMesh,
    FloppyMesh, GraphicsArchiveGridV2, KernelConstellationGrid, LegacyAsmCodexGrid,
    LegacyCCodexGrid, LegacyCppCodexGrid, NetworkAlmanacHub, NetworkArchiveGridV2,
    PeripheralArchiveMesh, ProcessAlmanacHub, SELinuxConstellation, SecurityConstellation,
    StorageArchiveGridV2, SyscallAlmanacHub, TapeMesh, UEFIGatewayMesh, ZeroTrustConstellation,
};
pub use cross_platform::{
    ApplicationBinary, BinaryFormat, BinaryFormat as CrossPlatformBinaryFormat,
    CompatibilityError, CompatibilityError as CrossPlatformError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability, MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};
pub use india_stack_localization::{IndianLanguage, LocalizationManager, LocalizationProvider};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};

pub use mint_linux::{
    MintUpdateLevel, MintUpdatePackage, MintUpdateManager, MintBackupTool,
    MintAppMetadata, MintSoftwareManager, MintReportAlertSeverity, MintReportAlert,
    MintReportSystem,
};

pub use chimera_linux::{
    DinitServiceState, DinitService, DinitServiceManager, BsdUserlandCompat,
    ApkPackageMetadata, ApkPackageStore,
};
pub use lubuntu::{
    CpuGovernor, SystemPressure, LubuntuHealthReport, LubuntuSystemManager,
    LxqtSessionManager, LxqtSessionState, PcmanfmQtAdapter, FileNode,
    DiscoverPackageAdapter, AptPackage, FeatherpadEditor, QTerminalEmulator, TerminalTab,
    CalamaresInstallerShim, CalamaresStage
};
pub use gentoo::{EbuildPackage, OpenRcManager, OpenRcRunlevel, OpenRcService, PortageEngine, ServiceStatus, UseFlagManager};
pub use tiny_core::{FiletoolOverlay, FrugalLoader, TceLoader, TczExtension, TinyCoreBootConfig};
pub use localsend::{LocalSendBridgeManager, LocalSendDevice, LocalSendDeviceType, LocalSendFileMetadata, LocalSendSession};
