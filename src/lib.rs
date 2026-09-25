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
pub use drivers::sovereign_comprehensive_drivers::*;
pub mod governance;
pub mod crypto;
pub mod filesystem;
pub mod futuristic_modules;
pub mod kernel;
pub mod klib;
pub mod memory;
pub use memory::low_level;
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
pub use package::{
    SovereignUniversalPackageFormatMasterEngine, UniversalPackageFormatKind,
};
pub use sigpkg::{
    SovereignUniversalPackageManagerInteropEngine, SovereignUniversalPackageTranslationBridge,
};
pub use filesystem::{
    EphemeralTmpfsMountGovernor, SovereignAtomicGenerationRootfsGuard, SovereignCanonicalFhsResolver,
    SyntheticProcSysfsProvider,
};
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
pub use distro::sovereign_linux_bsd_master_synthesis::*;
pub use distro::additional_linux_bsd_components::*;
pub use distro::sovereign_media_and_distro_unimplemented_innovations::*;
pub use distro::sovereign_omni_tech_media_and_distro_synthesis::*;
pub use distro::sovereign_2050_distro_supremacy_engine::*;
pub mod sovereign_wiki_master_engine;
pub use sovereign_wiki_master_engine::*;
pub mod open_source_obsoletion;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod wiki_unimplemented_ideas;
pub use wiki_unimplemented_ideas::*;
pub mod userland;
pub mod arch;
pub mod compiler;


pub mod audio;
pub mod audit;
pub mod backup;
pub mod bluetooth;
pub mod boot;
pub mod buildfarm;
pub mod camera;
pub mod cloud;
pub mod cluster;
pub mod compliance;
pub mod compositor;
pub mod compression;
pub mod config;
pub mod core;
pub mod crash;
pub mod debugger;
pub mod dev;
pub mod diagnostics;
pub mod docs;
pub mod ecosystem;
pub mod edge;
pub mod education;
pub mod embedded;
pub mod event;
pub mod finance;
pub mod fingerprint;
pub mod fs;
pub mod functions;
pub mod gamepad;
pub mod gpu;
pub mod graphics;
pub mod hal;
pub mod init;
pub mod innovation;
pub mod input;
pub mod integration;
pub mod iot;
pub mod ipc;
pub mod iso;
pub mod lang;
pub mod launch_ready;
pub mod launcher;
pub mod legal;
pub mod loader;
pub mod location;
pub mod logging;
pub mod media;
pub mod microphone;
pub mod mm;
pub mod monitor;
pub mod monitoring;
pub mod net;
pub mod networking;
pub mod nlp;
pub mod notification;
pub mod onboarding;
pub mod performance;
pub mod pillars;
pub mod plugin;
pub mod power;
pub mod print;
pub mod privacy;
pub mod provisioning;
pub mod recovery;
pub mod release;
pub mod resource;
pub mod robotics;
pub mod rt;
pub mod scheduler;
pub mod scientific;
pub mod secure;
pub mod sensor;
#[path = "sigma-boot/mod.rs"]
pub mod sigma_boot;
pub mod sigma_sandbox;
pub mod sigma_validation;
pub mod signal;
pub mod smartcard;
pub mod support;
pub mod syscall;
pub mod testing;
pub mod theming;
pub mod thermal;
pub mod time;
pub mod timer;
pub mod toolchain;
pub mod touchscreen;
pub mod tpm;
pub mod tracing;
pub mod ui;
pub mod update;
pub mod usb;
pub mod userspace;
pub mod vfs;
pub mod virt;
pub mod vm;
pub mod wireless;
pub mod workflow;
