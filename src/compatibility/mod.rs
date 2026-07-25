// SigmaOS Compatibility Module
pub mod constellation;
pub mod cross_platform;
pub mod legacy_adapters;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat as CrossPlatformBinaryFormat,
    CompatibilityError as CrossPlatformError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability, MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};

pub use historic_linux::{
    LinuxEra, HistoricalCpuState, HistoricSyscallEmulator, Era0_11SyscallEmulator,
    Era1_0SyscallEmulator, Era2_4SyscallEmulator, VintageVirtualizationSandbox,
    VintageDriverTranslator, VintagePackageConverter, HistoricError,
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
pub use persona::{
    PersonaVersion, KernelPersonaContainer, SyscallCategory, SyscallNode, SyscallGraph,
};
pub use abi_translator::{
    CpuArchitecture, ABITranslator,
};
pub use lattice::{
    LatticeFeature, KernelLattice, SyscallLifecycle, SyscallHistory, SyscallTracker,
};
pub use prism::{
    PrismFacet, KernelPrism, LedgerEntry, SyscallLedgerbook,
};

pub use legacy_adapters::{
    APITimelineManager, BinaryCompatMatrix, DiscontinuedFS, DriverBridge, FSRevival,
    GraphicsBridge, KernelPersona, KernelPersonaVM, LegacyBus, LegacyDriver,
    LegacyPluginManager, LibcVersion, NetworkBridge, StorageBridge, SyscallAbi,
    WorkloadOptimizer, WorkloadProfile, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER,
    GLOBAL_WORKLOAD_OPTIMIZER,
};
