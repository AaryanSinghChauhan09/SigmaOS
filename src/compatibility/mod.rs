// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod standards;
pub mod legacy_adapters;
pub mod constellation_mesh;
pub mod endeavour;
pub mod linux_adapter;
pub mod lattice_grid;
pub mod solid_kernel;
pub mod oldlinux;
pub mod wasm_sandbox;
pub mod templeos;
pub mod india_professional_tools;
pub mod relay_nexus;
pub mod bodhi_moksha;

pub use endeavour::{
    EosMirrorReflector, EosWelcomeEngine, EosUpdateNotifier, EosLogTool, YayAurHelper,
    Mirror, WelcomeTab,
};
pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use historic_linux::{
    APITimelineManager, BinaryCompatMatrix, DiscontinuedFS, DriverBridge, FSRevival,
    GraphicsBridge, KernelPersona, KernelPersonaVM, LegacyBus, LegacyDriver, LegacyPluginManager,
    LibcVersion, NetworkBridge, StorageBridge, SyscallAbi, WorkloadOptimizer, WorkloadProfile,
    GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use linux_adapter::{
    LegacyDriverAdapter, LegacyFSAdapter, LegacyKernelAdapter, LegacyPackageAdapter,
    LegacyProtocolAdapter, LegacySecurityAdapter, LegacyUIAdapter,
};
pub use standards::{
    FhsConventionStatus, LsbProfile, PosixComplianceLevel, StandardsComplianceManager,
};
pub use legacy_adapters::{
    LegacyKernelAdapter, LegacyDriverAdapter, LegacyPackageAdapter, LegacyFSAdapter,
    LegacyProtocolAdapter, LegacySecurityAdapter, LegacyUIAdapter,
};
pub use constellation_mesh::{
    KernelConstellationGrid, ConstellationNode, SyscallAlmanacHub, FileAlmanacHub, NetworkAlmanacHub,
    ProcessAlmanacHub, DriverArchiveGridV2, StorageArchiveGridV2, NetworkArchiveGridV2,
    GraphicsArchiveGridV2, FirmwareGatewayMesh, BIOSGatewayMesh, UEFIGatewayMesh,
    CorebootGatewayMesh, BuildCodexGrid, LegacyCCodexGrid, LegacyCppCodexGrid, LegacyAsmCodexGrid,
    SecurityConstellation, DACConstellation, SELinuxConstellation, ZeroTrustConstellation,
    PeripheralArchiveMesh, FloppyMesh, TapeMesh, CRTMesh, DotMatrixMesh,
};
pub use relay_nexus::{WandrEvent, AtifTrajectoryMonitor, VerifierConsensus, RelayNexus};
pub use bodhi_moksha::{EflCanvasElement, MokshaProfile, MokshaDesktopManager};
// Re-exports for advanced compatibility layouts (Phase J & K)
pub use lattice_grid::{BIOSNexusGrid, BuildChronicleMesh, CodexSyscall, CorebootNexusGrid, DACArchiveGrid, DockedDriver, DriverArchiveDock, FileCodexGrid, FirmwareNexusGrid, FloppyNexus, GraphicsDock, KernelLattice, LatticeNode, LatticePersona, LegacyCChronicleMesh, LegacyCppChronicleMesh, NetworkCodexGrid, NetworkDock, NexusType, PeripheralNexus, ProcessCodexGrid, SELinuxArchiveGrid, SecurityArchiveGrid, StorageDock, SyscallCodexGrid, TapeNexus, UEFINexusGrid, ZeroTrustArchiveGrid};
pub use solid_kernel::{AuditBlock, ComplianceScheduler, IScheduler, PrioritySchedulerPort, RoundRobinSchedulerPort, SigmaFSPlusPlus, SolidKernelCore};
pub use oldlinux::{OldLinuxCompatManager, OldLinuxRelease};
pub use wasm_sandbox::{WasmModule, WasmSandboxEngine, WasmState};
pub use templeos::{CooperativeTask, HolyCShell, HolySpiritOracle, RedSeaFilesystem, RingZeroSandbox};
