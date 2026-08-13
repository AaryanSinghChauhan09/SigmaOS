// SigmaOS Compatibility Module
pub mod antix;
pub mod chakra;
pub mod cross_platform;
pub mod legacy_adapters;
pub mod canonical;
pub mod arch;

pub use arch::{
    PkgbuildMeta, PkgSandboxConfig, AurSandboxOrchestrator,
};

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
    EcosystemSnapshot, SnapshotManager, CompatBinaryFormat, CompatBinary, CompatibilityLayer,
    BsdJailSandbox, FlatpakApp, UnifiedAppStore, HandoffTask, ContinuityCoordinator,
    DesktopMode, ZorinAppearanceSwitcher, AiResourceScheduler,
    DistroReleaseChannel, ReproducibleBuildVerifier, ReleaseGovernanceCouncil,
    LanguageTranslationCatalog, LocaleManager, TtsSynthesizer, BrailleMatrix,
    AppSuiteType, AppSuiteBundle, SuiteRegistry,
    CloudProvider, SigmaContainer, CloudOrchestrator,
};
