// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod india_stack;
pub mod interim;
pub mod jehanne;
pub mod mint_linux;
pub mod reactos;
pub mod legacy_adapters;
pub mod chakra;
pub mod antix;
pub mod relay_nexus;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use india_stack::{GstCalculator, IndiaStackError, MockUPIService, MultilingualSupport};
pub use interim::{InterimLispVM, LispVal, MntReformLpcDriver, ReformPowerStats};
pub use jehanne::{
    ComputeNode, DistributedComputeHandoff, JehanneError, JehanneNamespace, NamespaceBindEntry,
    Plan9pMessage, Plan9pMsgType,
};
pub use mint_linux::{
    MintBackupTool, MintSoftwareManager, MintUpdateItem, MintUpdateLevel, MintUpdateManager,
    SoftwareMeta, WindowCoordinates, ZenithDisplayCompositor,
};
pub use reactos::{
    NtHandle, NtHandleEntry, NtObjectManager, NtObjectType, NtStatus, PortableExecutableLoader,
    RegistryHive,
};
pub use legacy_adapters::{
    APITimelineManager, BinaryCompatMatrix, DiscontinuedFS, DriverBridge, FSRevival,
    GraphicsBridge, KernelPersona, KernelPersonaVM, LegacyBus, LegacyPluginManager,
    LibcVersion, NetworkBridge, StorageBridge, SyscallAbi, WorkloadOptimizer, WorkloadProfile,
    GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use chakra::{
    TribeInstaller, GLOBAL_TRIBE, AkabeiBundle, KapudanAssistant, GLOBAL_KAPUDAN,
    AkabeiPackageEngine, GLOBAL_AKABEI, BundleType, InstallerStep, DesktopTheme,
};
pub use antix::{
    AntixControlCenter, GLOBAL_ANTIX_CONTROL, AntixDesktopProfiler, GLOBAL_ANTIX_DESKTOP,
    AntixInitManager, GLOBAL_ANTIX_INIT, DesktopProfile as AntixDesktopProfile, DesktopProfile,
    LegacyMemoryTrimmer, GLOBAL_MEMORY_TRIMMER, MicroService as AntixMicroService, MicroService,
    MicroServiceState as AntixMicroServiceState, MicroServiceState,
};
pub use relay_nexus::LegacyDriver;
