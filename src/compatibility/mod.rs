// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod legacy_adapters;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};

pub use legacy_adapters::{
    APITimelineManager, BinaryCompatMatrix, DiscontinuedFS, DriverBridge, FSRevival,
    GraphicsBridge, KernelPersona, KernelPersonaVM, LegacyBus, LegacyDriver,
    LegacyPluginManager, LibcVersion, NetworkBridge, StorageBridge, SyscallAbi,
    WorkloadOptimizer, WorkloadProfile, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER,
    GLOBAL_WORKLOAD_OPTIMIZER,
};
