#![no_std]
#![allow(clippy::all, unused)]

#[macro_use]
extern crate alloc;

// SigmaOS Library
// Core library for SigmaOS operating system

#[macro_use]
pub mod klib;

pub mod accessibility;
pub mod ai;
pub mod arch;
pub mod audio;
pub mod automation;
pub mod boot;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod distro;
pub mod driver;
pub mod filesystem;
pub mod kernel;
pub mod memory;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod process;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod storage;
pub mod thread;
pub use process::{
    ProcessControlError, ProcessVmReadWriteEngine, JobState, CoreDumpMetadata, ProcessJobEntry,
    JobControlLifecycleEngine, WNOHANG, WUNTRACED, WCONTINUED, BsdRusage, WaitStatus,
    ProcessWaiterAndRusageCollector, CancellationType, ProcessCancelState,
    ProcessCancellationAndTerminationManager, PosixMessage, PosixMessageQueue, EventFd,
    SigQueuePayload, AdvancedIpcHub, SovereignProcessState, SovereignProcess, ZeroCopyIpcChannel,
    SovereignProcessManager,
};
pub mod community;
pub mod access;
pub mod tools;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod userland;
pub mod open_source_obsoletion;

pub use unimplemented_features::{
    AntiXLowRamSysVInitGovernor, BareMetalPeripheralManager, BareMetalUnifiedPeripheral,
    GenerationManager, GentooPortageMaskResolver, HaikuMediaTranslator, HaikuTranslatorEngine, Jbd2TransactionLedger,
    LegacyController, ModernController, PciBusScanner, PowerState, SatSolverEngine,
    SerenityIpcEvent, SerenityOsAsyncIpcLoop, SovereignIpcBus, UdfVm, ZorinAppMapping,
    ZorinWinAppDbRegistry, AlpineApkPackageIndex, DragonFlyHammer2FsSnapshot, NixOsDeclarativeConfigEngine,
};
pub use distro::{
    ApkChrootBuildSandboxEngine, OpenBsdFdPledgeGate, FreeBsdGeomVdevTopology, GeomVdevNode,
    HermeticStoreClosureEngine, StoreClosurePackage, BedrockStratum, BedrockLinuxStrataEngine,
    SmartOsVmBrand, SmartOsVmState, SmartOsImage, SmartOsVmConfig, SmartOsZoneEngine,
};
pub use security::{
    HardenedSyscallDispatcher, HardenedSyscallError, MemoryAccessError,
    PagePermissions, RetpolineKptiMitigationEngine, SmepSmapEnforcer, SovereignKaslrEngine,
};
pub mod expanded_wiki_innovations;
pub mod virtualization;

pub mod interrupt;

pub mod graphics {
    pub mod compositor;
    pub mod paint;
    pub mod video;
}
pub mod hardware {
    pub mod compatibility;
    pub mod win32;
}
pub mod init;
pub mod ml;
pub mod performance;
pub mod power {
    pub mod governor;
}
pub mod scheduler {
    pub mod numa_scheduler;
}
pub mod toolchain;
pub mod ui;
pub mod crypto {
    pub mod vectorized_pqc;
}
pub mod distro_innovations;
pub mod extended_distro_matrix;
pub mod linuxmint_inspirations;

pub use linuxmint_inspirations::{
    AppTheme, BulkyRenamer, CaptainInstaller, ConfigBackend, DebPackage, DiagnosticField,
    FsFormat, HypnotixIptvPlayer, IsolationMode, LanPeer, LanWarpEngine, MintConfigHub,
    MintNannyFilter, MintReportDiagnostics, MintStickFormatter, MintWelcomeFlow, NannyDecision,
    ProviderType, RenameRule, TransferOutcome, TvChannel, WebEngineKind, Webapp, WebappManager,
    XAppThemeEngine, ThingyEntry, ThingyKind, ThingyRecentDocs, WelcomeStep,
    WARP_AUTH_PORT, WARP_MDNS_UDP_PORT, WARP_TRANSFER_PORT,
};
