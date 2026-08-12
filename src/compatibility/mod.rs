// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod linux_adapter;
pub mod persona;
pub mod abi_translator;
pub mod lattice;
pub mod prism;
pub mod canonical;
pub mod fedora;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use linux_adapter::{
    LinuxKernelVersion, LegacyKernelAdapter, LegacyPackageAdapter, LegacySecurityAdapter, LegacyUIAdapter,
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
pub use canonical::{
    SigmaSubiquity, SigmaNetplan, SigmaCloudInit, SigmaMultipass, SigmaCurtin,
};
pub use fedora::{
    DnfPackageResolver, MockChrootBuilder, KojiBuildServer, BodhiUpdateTriage,
};

pub mod debian;
pub use debian::{
    DebianChannel, AptRepositorySync, SysVRunlevel, SysVInitEngine,
    AlternativeLink, DebianAlternativesSystem, DebootstrapEngine,
};

pub mod innovations;
pub use innovations::{
    WorkloadCategory, ISchedulerPolicy, MlAcceleratedPolicy, GreenComputingPolicy,
    SigmaScheduler, ISyscallTranslator, LinuxTranslator, WindowsTranslator,
    UniversalAbiTranslator, IFileSystemCore, ISemanticSearchPlugin, ICasDeduplicator,
    SigmaFsPlusPlus, IRecoveryStrategy, RollbackRecovery, SelfHealingOS,
};

pub mod fedora;
pub use fedora::{
    DnfPackageResolver, MockChrootBuilder, KojiBuildServer, BodhiUpdateTriage,
};

pub mod india_professional_tools;
pub use india_professional_tools::{
    JudicialTimelinePlanner, MsmeComplianceEngine, PMWaniHotspotController,
    AyushFormularyHelper, DigiYatraPassScanner, IrctcPnrTracker,
};

pub mod india_stack;
pub use india_stack::{MockUPIService, IndiaStackError};

pub mod india_stack_localization;
pub use india_stack_localization::{IndianLanguage, LocalizationProvider, LocalizationManager};
