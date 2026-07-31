// SigmaOS Compatibility Module
pub mod chakra;
pub mod constellation_mesh;
pub mod cross_platform;
pub mod endeavour;
pub mod historic_linux;
pub mod legacy_adapters;
pub mod standards;
pub mod constellation_mesh;
pub mod endeavour;
pub mod legacy_adapters;
pub mod linux_adapter;
pub mod lattice_grid;
pub mod solid_kernel;
pub mod historic_linux;
pub mod oldlinux;
pub mod wasm_sandbox;
pub mod templeos;

pub use chakra::{
    AkabeiBundle, AkabeiPackageEngine, BundleType, DesktopTheme, InstallerStep, KapudanAssistant,
    TribeInstaller, GLOBAL_AKABEI, GLOBAL_KAPUDAN, GLOBAL_TRIBE,
};
pub use constellation_mesh::{
    BIOSGatewayMesh, BuildCodexGrid, CRTMesh, ConstellationNode, CorebootGatewayMesh,
    DACConstellation, DotMatrixMesh, DriverArchiveGridV2, FileAlmanacHub, FirmwareGatewayMesh,
    FloppyMesh, GraphicsArchiveGridV2, KernelConstellationGrid, LegacyAsmCodexGrid,
    LegacyCCodexGrid, LegacyCppCodexGrid, NetworkAlmanacHub, NetworkArchiveGridV2,
    PeripheralArchiveMesh, ProcessAlmanacHub, SELinuxConstellation, SecurityConstellation,
    StorageArchiveGridV2, SyscallAlmanacHub, TapeMesh, UEFIGatewayMesh, ZeroTrustConstellation,
};
pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use endeavour::{
    EosLogTool, EosMirrorReflector, EosUpdateNotifier, EosWelcomeEngine, Mirror, WelcomeTab,
    YayAurHelper,
};
pub use historic_linux::{
    APITimelineManager, BinaryCompatMatrix, DiscontinuedFS, DriverBridge, FSRevival,
    GraphicsBridge, KernelPersona, KernelPersonaVM, LegacyBus, LegacyDriver, LegacyPluginManager,
    LibcVersion, NetworkBridge, StorageBridge, SyscallAbi, WorkloadOptimizer, WorkloadProfile,
    GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use legacy_adapters::{
    LegacyDriverAdapter, LegacyFSAdapter, LegacyKernelAdapter, LegacyPackageAdapter,
    LegacyProtocolAdapter, LegacySecurityAdapter, LegacyUIAdapter,
};
pub use standards::{
    FhsConventionStatus, LsbProfile, PosixComplianceLevel, StandardsComplianceManager,
};

pub use india_professional_tools::{
    JudicialTimelinePlanner, MsmeComplianceEngine, AyushFormularyHelper,
    PMWaniHotspotController, DigiYatraPassScanner, IrctcPnrTracker,
};

// Re-exports for advanced compatibility layouts (Phase J & K)
pub use constellation_mesh::{BIOSGatewayMesh, BuildCodexGrid, ConstellationNode, CorebootGatewayMesh, DACConstellation, DotMatrixMesh, DriverArchiveGridV2, FileAlmanacHub, FirmwareGatewayMesh, FloppyMesh, GraphicsArchiveGridV2, KernelConstellationGrid, LegacyAsmCodexGrid, LegacyCCodexGrid, LegacyCppCodexGrid, NetworkAlmanacHub, NetworkArchiveGridV2, PeripheralArchiveMesh, ProcessAlmanacHub, SELinuxConstellation, SecurityConstellation, StorageArchiveGridV2, SyscallAlmanacHub, TapeMesh, UEFIGatewayMesh, ZeroTrustConstellation};
pub use endeavour::{EosLogTool, EosMirrorReflector, EosUpdateNotifier, EosWelcomeEngine, Mirror, WelcomeTab, YayAurHelper};
pub use legacy_adapters::{LegacyDriverAdapter, LegacyFSAdapter, LegacyKernelAdapter, LegacyPackageAdapter, LegacyProtocolAdapter, LegacySecurityAdapter, LegacyUIAdapter};
pub use lattice_grid::{BIOSNexusGrid, BuildChronicleMesh, CodexSyscall, CorebootNexusGrid, DACArchiveGrid, DockedDriver, DriverArchiveDock, FileCodexGrid, FirmwareNexusGrid, FloppyNexus, GraphicsDock, KernelLattice, LatticeNode, LatticePersona, LegacyCChronicleMesh, LegacyCppChronicleMesh, NetworkCodexGrid, NetworkDock, NexusType, PeripheralNexus, ProcessCodexGrid, SELinuxArchiveGrid, SecurityArchiveGrid, StorageDock, SyscallCodexGrid, TapeNexus, UEFINexusGrid, ZeroTrustArchiveGrid};
pub use solid_kernel::{AuditBlock, ComplianceScheduler, IScheduler, PrioritySchedulerPort, RoundRobinSchedulerPort, SigmaFSPlusPlus, SolidKernelCore};
pub use historic_linux::{APITimelineManager, BinaryCompatMatrix, DiscontinuedFS, DriverBridge, Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, FSRevival, HistoricError, HistoricSyscallEmulator, HistoricalCpuState, VintageDriverTranslator, VintagePackageConverter, VintageVirtualizationSandbox, WorkloadOptimizer, WorkloadProfile};
pub use oldlinux::{OldLinuxCompatManager, OldLinuxRelease};
pub use wasm_sandbox::{WasmModule, WasmSandboxEngine, WasmState};
pub use templeos::{CooperativeTask, HolyCShell, HolySpiritOracle, RedSeaFilesystem, RingZeroSandbox};
