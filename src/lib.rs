// SigmaOS Library
// Core library for SigmaOS operating system

extern crate alloc;

// Core working modules
pub mod accessibility;
pub mod access;
pub mod ai {
    pub mod agent;
    pub mod orchestrator;
    pub mod agentic_os_runtime;
}
pub mod boot;
pub use boot::*;
pub mod community;
pub mod compatibility;
pub mod crypto {
    pub mod vectorized_pqc;
}
pub mod customization;
pub mod dashboard;
pub mod distro;
pub mod driver;
pub mod expanded_wiki_innovations;
pub mod filesystem;
pub mod graphics {
    pub mod compositor;
    pub mod paint;
    pub mod video;
}
pub mod hardware {
    pub mod compatibility;
    pub mod win32;
}
pub mod interrupt;
pub mod kernel;
pub mod klib;
pub mod memory;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod power {
    pub mod governor;
}
pub mod process;
pub use process::{
    ProcessControlError, ProcessVmReadWriteEngine, JobState, CoreDumpMetadata, ProcessJobEntry,
    JobControlLifecycleEngine, WNOHANG, WUNTRACED, WCONTINUED, BsdRusage, WaitStatus,
    ProcessWaiterAndRusageCollector, CancellationType, ProcessCancelState,
    ProcessCancellationAndTerminationManager, PosixMessage, PosixMessageQueue, EventFd,
    SigQueuePayload, AdvancedIpcHub, SovereignProcessState, SovereignProcess, ZeroCopyIpcChannel,
    SovereignProcessManager, JobLimitViolation, JobObjectLimits, JobObjectAccounting,
    SovereignJobObject, JobObjectManager,
};
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod scheduler;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod storage;
pub mod thread;
pub mod toolchain {
    pub mod adapter;
    pub mod bootstrap;
    pub mod capsule;
    pub mod codex;
}
pub mod tools;
pub mod unimplemented_features;
pub use unimplemented_features::{
    AntiXLowRamSysVInitGovernor, BareMetalPeripheralManager, BareMetalUnifiedPeripheral,
    GenerationManager, GentooPortageMaskEngine, HaikuMediaTranslator, HaikuTranslatorEngine, Jbd2TransactionLedger,
    LegacyController, ModernController, PciBusScanner, PowerState, SatSolverEngine,
    SerenityIpcEvent, SerenityOsAsyncIpcLoop, SovereignIpcBus, UdfVm, ZorinAppMapping,
    ZorinWinAppDbRegistry, AlpineApkPackageIndex, DragonFlyHammer2FsSnapshot, NixOsDeclarativeConfigEngine,
};
pub mod unimplemented_tools;
pub mod userland;
pub mod virtualization;
