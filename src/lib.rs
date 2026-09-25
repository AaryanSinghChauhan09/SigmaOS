// SigmaOS Library
// Core library for SigmaOS operating system

extern crate alloc;


// Core working modules
pub mod accessibility;
pub mod ai;
pub mod app;
pub mod auth;
pub mod automation;
pub mod community;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod distro;
pub mod driver;
pub mod drivers;
pub mod governance;
pub mod crypto;
pub mod filesystem;
pub mod futuristic_modules;
pub mod kernel;
pub mod klib;
pub mod memory;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod process;
pub mod productivity;
pub use productivity::*;
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
    CachyosGamescopeHandheldOverlay, ItsFossQuickShareAndBackupHud, PhoronixPerformanceBenchmarkWidget,
    PopOsKdeTilingWorkspaceGridEngine, SovereignUxMasterEngine, WindowsCopilotAiAssistantSidebar,
};
pub use process::{
    AdvancedIpcHub, BsdRusage, CancellationType, CoreDumpMetadata, EventFd,
    JobControlLifecycleEngine, JobState, PosixMessage, PosixMessageQueue, ProcessCancelState,
    ProcessCancellationAndTerminationManager, ProcessControlError, ProcessJobEntry,
    ProcessVmReadWriteEngine, ProcessWaiterAndRusageCollector, SigQueuePayload, SovereignProcess,
    SovereignProcessManager, SovereignProcessState, WaitStatus, ZeroCopyIpcChannel, WCONTINUED,
    WNOHANG, WUNTRACED,
};
pub mod access;
pub mod open_source_os_gap_closure;
pub mod tools;
pub use open_source_os_gap_closure::*;
pub use kernel::tss_ring3_user_mode::*;
pub use tools::tech_media_extended_suite::*;
pub mod sovereign_wiki_master_engine;
pub use sovereign_wiki_master_engine::*;
pub mod open_source_obsoletion;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod wiki_unimplemented_ideas;
pub use wiki_unimplemented_ideas::*;
pub mod userland;
