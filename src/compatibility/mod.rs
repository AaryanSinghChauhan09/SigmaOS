// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod federation;
pub mod historic_linux;
pub mod innovations;
pub mod lattice_grid;
pub mod mesh_hub;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};

pub use federation::{
    BIOSBoot, BootInterface, BootManager, CompilerPod, CorebootBoot, DriverEra, DriverTimeline,
    FederatedNode, FileSyscallVM, FloppySim, KernelFederation, LegacyCPod, LegacyCppPod,
    LegacyDACSandbox, NetworkSyscallVM, PeripheralSimulator, PersonaType, ProcessSyscallVM,
    SecuritySandbox, SyscallContext, SyscallVM, TapeDriveSim, UEFIBoot, ZeroTrustSandbox,
};

pub use innovations::{
    GreenComputingPolicy, ICasDeduplicator, IFileSystemCore, IRecoveryStrategy,
    ISemanticSearchPlugin, ISyscallTranslator, LinuxTranslator, MlAcceleratedPolicy,
    RollbackRecovery, SelfHealingOS, SigmaFsPlusPlus, SigmaScheduler, UniversalAbiTranslator,
    WindowsTranslator, WorkloadCategory,
};

pub use lattice_grid::{
    BIOSNexusGrid, BuildChronicleMesh, CodexSyscall, CorebootNexusGrid, DACArchiveGrid,
    DriverArchiveDock, FileCodexGrid, FirmwareNexusGrid, FloppyNexus, GraphicsDock, KernelLattice,
    LatticeNode, LatticePersona, LegacyCChronicleMesh, LegacyCppChronicleMesh, NetworkCodexGrid,
    NetworkDock, NexusType, PeripheralNexus, ProcessCodexGrid, SELinuxArchiveGrid,
    SecurityArchiveGrid, StorageDock, SyscallCodexGrid, TapeNexus, UEFINexusGrid,
    ZeroTrustArchiveGrid,
};

pub use mesh_hub::{
    BIOSCrossDock, BuildChronicleHub, CorebootCrossDock, DACRelay, DriverRepoGrid, FileAnthology,
    FirmwareCrossDock, FirmwareDockType, FloppyRepoHub, GraphicsRepoGrid, HistoricalSyscall,
    KernelRelayMesh, LegacyCChronicleHub, LegacyCppChronicleHub, LegacyDriver, MeshNode,
    MeshPersona, NetworkAnthology, NetworkRepoGrid, PeripheralRepoHub, ProcessAnthology,
    SELinuxRelay, SecurityRelayMesh, StorageRepoGrid, SyscallAnthology, TapeRepoHub, UEFICrossDock,
    ZeroTrustRelay,
};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};
