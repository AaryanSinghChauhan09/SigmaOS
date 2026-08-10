// SigmaOS Compatibility Module
pub mod antix;
pub mod canonical;
pub mod chakra;
pub mod cross_platform;
pub mod legacy_adapters;

pub use legacy_adapters::{KernelPersona, SyscallAbi};
pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};

pub use legacy_adapters::{
    APITimelineManager, BinaryCompatMatrix, DiscontinuedFS, DriverBridge, FSRevival,
    GraphicsBridge, KernelPersona, KernelPersonaVM, LegacyBus, LegacyDriver, LegacyPluginManager,
    LibcVersion, NetworkBridge, StorageBridge, SyscallAbi, WorkloadOptimizer, WorkloadProfile,
    GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_WORKLOAD_OPTIMIZER,
};

pub use chakra::{
    AkabeiBundle, AkabeiPackageEngine, BundleType, DesktopTheme, InstallerStep, KapudanAssistant,
    TribeInstaller, GLOBAL_AKABEI, GLOBAL_KAPUDAN, GLOBAL_TRIBE,
};

pub use antix::{
    AntixControlCenter, AntixDesktopProfiler, AntixInitManager, DesktopProfile,
    LegacyMemoryTrimmer, MicroService, MicroServiceState, GLOBAL_ANTIX_CONTROL,
    GLOBAL_ANTIX_DESKTOP, GLOBAL_ANTIX_INIT, GLOBAL_MEMORY_TRIMMER,
};

pub use canonical::{
    AiResourceScheduler, AppSuiteBundle, AppSuiteType, BrailleMatrix, BsdJailSandbox,
    CloudOrchestrator, CloudProvider, CompatBinary, CompatBinaryFormat, CompatibilityLayer,
    ContinuityCoordinator, DesktopMode, DistroReleaseChannel, EcosystemSnapshot, FlatpakApp,
    HandoffTask, LanguageTranslationCatalog, LocaleManager, ReleaseGovernanceCouncil,
    ReproducibleBuildVerifier, SigmaContainer, SnapshotManager, SuiteRegistry, TtsSynthesizer,
    UnifiedAppStore, ZorinAppearanceSwitcher,
};
