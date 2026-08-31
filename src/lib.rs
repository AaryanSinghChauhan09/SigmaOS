// SigmaOS Library
// Core library for SigmaOS operating system


// Core working modules
pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod driver;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod memory;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod process;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod runtime;
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

pub use unimplemented_features::{
    AntiXLowRamSysVInitGovernor, BareMetalPeripheralManager, BareMetalUnifiedPeripheral,
    GenerationManager, GentooPortageMaskResolver, HaikuMediaTranslator, HaikuTranslatorEngine, Jbd2TransactionLedger,
    LegacyController, ModernController, PciBusScanner, PowerState, SatSolverEngine,
    SerenityIpcEvent, SerenityOsAsyncIpcLoop, SovereignIpcBus, UdfVm, ZorinAppMapping,
    ZorinWinAppDbRegistry, AlpineApkPackageIndex, DragonFlyHammer2FsSnapshot, NixOsDeclarativeConfigEngine,
};
pub use distro::{
    ApkChrootBuildSandboxEngine, OpenBsdFdPledgeGate, FreeBsdGeomVdevTopology, GeomVdevNode,
    HermeticStoreClosureEngine, StoreClosurePackage,
    missing_distro_innovations::{LinuxBsdSysctlEngine, IoUringEngine, IoUringOp, SubmissionQueueEntry, CompletionQueueEntry},
};
pub use security::{
    HardenedSyscallDispatcher, HardenedSyscallError, MemoryAccessError,
    PagePermissions, RetpolineKptiMitigationEngine, SmepSmapEnforcer, SovereignKaslrEngine,
    KaliAirgeddonWifiAudit, KaliMetasploitPayloadFilter, KaliWiresharkPacketAnalyzer,
    PcapPacketHeader, WifiFrameType, PiaDedicatedIpBinding, PiaMaceAdBlocker,
    PiaMultiHopShadowsocksBridge, PiaPortForwardingEngine, PiaServerRegion,
    PiaSplitTunnelGovernor, PiaStrictKillSwitch, PiaVpnManager, SplitTunnelRule,
    GksuAuthBackend, GksuDisplayServer, GksuExecutionRequest, GksuExecutionResult,
    GksuSecurityGuard, LibGksuGraphicalSudoEngine,
};
pub use driver::{
    DkmsAbiRebuildEngine, DkmsModuleSpec, DriverHardwareCategory, DriverLicense,
    UbuntuAdditionalDriversRegistry, UbuntuCommonDriverEngine, UbuntuDriverPackage,
    UbuntuLivepatchDriverHook,
};
pub mod expanded_wiki_innovations;
pub mod virtualization;

pub mod interrupt;

pub mod graphics {
    pub mod compositor;
    pub mod gpu_driver;
    pub mod nvidia_prime;
    pub mod paint;
    pub mod video;
}
pub mod hardware {
    pub mod compatibility;
    pub mod win32;
}
pub mod power {
    pub mod governor;
}
pub mod boot;
pub use boot::*;
pub mod toolchain {
    pub mod adapter;
    pub mod bootstrap;
    pub mod capsule;
    pub mod codex;
}
pub mod scheduler;
pub mod crypto {
    pub mod vectorized_pqc;
}

pub mod logging;
pub mod ai;
pub mod system;
pub mod installer;
pub mod performance;
pub mod ml;
pub mod iot;

// Temporarily disabled problematic modules
// pub mod accessibility;
// pub mod automation;
// pub mod compatibility;
// pub mod container;
// pub mod customization;
// pub mod dashboard;
// pub mod desktop;
// pub mod device;
// pub mod driver;
// pub mod filesystem;
// pub mod ml;
// pub mod network;
// pub mod observability;
// pub mod orchestration;
pub mod distro;
// pub mod package;
// pub mod performance;
// pub mod productivity;
// pub mod remote;
// pub mod resilience;
// pub mod shell;
// pub mod sigpkg;
// pub mod virtualization;
// pub mod graphics {
//     pub mod compositor;
//     pub mod paint;
//     pub mod video;
// }
// pub mod power {
//     pub mod governor;
// }
// pub mod ai {
//     pub mod agent;
//     pub mod orchestrator;
// }
// pub mod boot;
// pub mod system;
// pub mod installer;
