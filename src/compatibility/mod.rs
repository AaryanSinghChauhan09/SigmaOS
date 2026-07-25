// SigmaOS Compatibility Module
pub mod constellation;
pub mod cross_platform;
pub mod gap_closure;
pub mod historic_linux;
pub mod personality;
pub mod sigmawin;

pub use gap_closure::{
    AiTaskOrchestrator, BootInterface, BuildLedgerSystem, DriverClass, DriverRepositoryManager,
    EmulatedPeripheral, FirmwareBridgeManager, GapSandboxPolicy, HardwareDriver, HidGraphicsDriver,
    JobClass, KernelModule, KernelModuleManager, LedgerSnapshot, MemoryProtection, ModuleState,
    NetworkStackGateway, PeripheralEmulationLibrary, SecurityPolicyManager,
    SyscallCompatibilityRegistry, VirtualMemoryManager,
};

pub use constellation::{
    ArchiveProfile, BuildArchive, ChronicleType, ConstellationNode, DriverMuseum, ExhibitType,
    FirmwarePavilion, KernelConstellation, ObsoletePeripheral, PavilionType, PeripheralMuseum,
    SecurityModel as ConstellationSecurityModel, SecurityPavilion, SyscallChronicle,
};

pub use personality::{
    BuildCapsule, BuildProfile, CapsuleVersion, DriverEmulator, EmulatorProfile, FirmwarePersona,
    FirmwareType, KernelShard, ObsoleteDevice, PeripheralPod, SecurityGrid, SecurityModel,
    ShardType, SyscallCapsule,
};

pub use sigmawin::{
    D3dToVulkanTranslator, PeFormat, PeLoader, RegistryManager, User32MessageQueue, Win32Error,
    Win32Message, WinSockAdapter,
};

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};
