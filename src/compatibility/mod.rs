// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod federation;
pub mod historic_linux;

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

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};
