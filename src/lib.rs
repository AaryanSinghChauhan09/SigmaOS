// SigmaOS Library
// Core library for SigmaOS operating system

extern crate alloc;

// Core working modules
pub mod open_source_os_gap_closure;
pub mod accessibility;
pub mod ai;
pub mod audio;
pub mod ipc;
pub mod init;
pub mod app;
pub mod auth;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod driver;
pub mod crypto;
pub mod filesystem;
pub mod futuristic_modules;
pub mod kernel;
pub mod klib;
// pub use klib::ZeroDependencyPrimitiveHub;
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
pub mod system;
pub mod hardware;
pub mod installer;
pub mod ml;
pub mod virtualization;
pub mod interrupt;
pub use desktop::{
    Gnome46MutterEngine, KdePlasma6Engine, LuminaBsdDesktopEngine, SwayRegolithWmEngine, Xfce418Engine,
};
pub use process::{
    AdvancedIpcHub, BsdRusage, CancellationType, CoreDumpMetadata, EventFd,
    JobControlLifecycleEngine, JobState, PosixMessage, PosixMessageQueue, ProcessCancelState,
    ProcessCancellationAndTerminationManager, ProcessControlError, ProcessJobEntry,
    ProcessVmReadWriteEngine, ProcessWaiterAndRusageCollector, SigQueuePayload, SovereignProcess,
    SovereignProcessManager, SovereignProcessState, WaitStatus, ZeroCopyIpcChannel, WCONTINUED,
    WNOHANG, WUNTRACED,
};
pub mod linuxmint_inspirations;
pub use linuxmint_inspirations::{
    AppTheme, BulkyRenamer, CaptainInstaller, CaptainSource, ConfigBackend, DebPackage,
    DesktopIconFlags, DiagnosticField, FsFormat, HypnotixIptvPlayer, IsolationMode, IptvProvider,
    LanPeer, LanWarpEngine, MintConfigHub, MintDesktopEngine, MintLocaleEngine, MintMenuEngine,
    MintMenuItem, MintNannyFilter, MintReportDiagnostics, MintStickFormatter, MintStickIsoVerifier,
    MintUpgradeEngine, MintUpgradePhase, MintWelcomeFlow, NannyDecision, PartitionScheme,
    ProviderType, RenameConflict, RenameRule, RenamedFile, RequestIncoming, SessionControlAction,
    StickyNote, StickyNotesManager, ThingyEntry, ThingyKind, ThingyRecentDocs, TransferOutcome,
    TransferRequest, TvChannel, UsbDevice, WARP_AUTH_PORT, WARP_MDNS_UDP_PORT, WARP_TRANSFER_PORT,
    WebEngineKind, Webapp, WebappManager, WelcomeStep, XAppImageViewer, XAppStatusIconBadgeManager,
    XAppTextEditor, XAppThemeEngine, XAppTrayBadge,
};
pub mod tools;
pub use open_source_os_gap_closure::*;
pub mod sovereign_wiki_master_engine;
pub use sovereign_wiki_master_engine::*;
pub mod open_source_obsoletion;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod userland;
pub mod distro;
pub mod drivers;
pub mod community;
pub mod governance;
