#![allow(ambiguous_glob_reexports)]
// SigmaOS Kernel Module
pub mod panic_handler;
pub use panic_handler::*;

pub mod sovereign_smp_xhci_apc_synthesis;
pub use sovereign_smp_xhci_apc_synthesis::*;
pub mod architecture;
pub mod atomic_extended;
pub mod block_dev;
pub mod process_monitor;
pub mod bore;
pub mod breakthrough;
pub mod breakthroughs;
pub mod breakthroughs_v2;
pub mod bus;
pub mod cgroup_controllers;
pub mod cgroup_v2;
pub mod classic_os;
pub mod component;
pub mod console;
pub mod cpu_features;
pub mod cpufreq;
pub mod device;
pub mod driver;
pub mod dtrace_compat;
pub mod ebpf;
pub mod ebpf_verification;
pub mod ebpf_vm;
pub mod exports;
pub mod gap_closing;
pub mod gap_filling;
pub mod generation_manager;
pub mod io_uring;
pub mod ipc;
pub mod kprintf;
pub mod kqueue;
pub mod kqueue_event;
pub mod linux_absorb;
pub mod linux_bsd_innovations;
pub mod linux_parity;
pub mod missing_linux_kernel_components;
pub mod sysfs_manager;
pub use linux_parity::{
    CmaRegion, KernelTimer, LinuxCmaAllocatorEngine, LinuxKernelTimerWheel,
    LinuxKernelWorkqueueEngine, LinuxRcuSynchronizationEngine, RcuCallback, WorkItem,
};
pub mod memory;
pub mod meta;
pub mod module_loader;
pub mod rump_modules;
pub use rump_modules::{
    KernelModule, ModuleState, ModulePriority, RumpModuleLoader,
};
pub mod kptr_restrict;
pub use kptr_restrict::{
    KptrRestrictLevel, SecurityMitigations, get_kptr_restrict, set_kptr_restrict,
    should_hide_kptr, format_kptr, get_security_mitigations,
};
pub mod perf_events;
pub use perf_events::{
    PerfEventType, HardwareEvent, SoftwareEvent, CacheEvent,
    PerfCounter, PerfEventGroup, SystemPerfCounters,
    record_page_fault, record_context_switch, record_cache_miss,
};
pub mod module_loading_control;
pub mod module_tools;
pub mod namespaces;
pub mod net;
pub mod nextgen_breakthroughs;
pub mod numa_allocator;
pub mod numa_scheduler;
pub mod object;
pub mod os_innovations;
pub mod paging;
pub mod performance;
pub mod pipes;
pub mod policy_mechanism;
pub mod process;
pub mod roundrobin;
pub mod sched;
pub mod scheduler;
pub mod structures;
pub mod uts_namespace;
pub mod virtual_cpu;

pub use missing_linux_kernel_components::{
    BpfRingBufferStreamEngine, EpollCtlOp, EpollEvent, KernelAuditRecord, KernelAuditRecordType,
    KprobeEntry, LinuxEpollEventPollEngine, LinuxKernelAuditSubsystemEngine,
    LinuxKprobesTracepointEngine, LinuxMemoryCgroupV2OomKillerEngine,
    LinuxSeccompBpfSyscallFilterEngine, MemcgProcessEntry, SeccompAction, UffdFaultEvent, UffdMode,
    UffdRegisteredRange, UserfaultfdSubsystemEngine, VirtioBalloonDriverEngine,
};
pub mod traits;
pub mod vmm_paging;

pub use crate::kernel::linux_bsd_innovations::*;
pub use crate::kernel::linux_bsd_innovations::{
    AlpineHardenedEnv, AndroidBinderIpc, AndroidBroadcastReceiverRegistry, ArchUserRepoManager,
    BinderNode, BottomHalfKernelThread, BoundedBufferProducerConsumer, BroadcastReceiver,
    BsdPfStateTable, CapabilityDerivationTree, CarpSecurityRouter, CgroupResourceLimits, CowBlock,
    CowStorageEngine, CpuIsaMicroarch, DevlinkHealthReporter, DynamicLkmLoader, EbpfInstruction,
    EbpfRuntime, ExokernelHardwareMultiplexer, FastPacketFrame, FreeBsdCapsicumEngine,
    FreeBsdGeomTopology, FreeBsdJail, FreeBsdVfsNullfs, FreeBsdVnetManager, FutexOp, FutexWaiter,
    GcdDispatchQueue, GcdPriority, GcdTask, GentooUseFlags, GeomClass, GeomProvider,
    Hammer2PfsSnapshot, HammerBlockTransaction, HammerHistoryFilesystem, HurdTranslator,
    HybridKernelManager, HybridTask, IntelClearLinuxStatelessEngine, InteractiveHybridScheduler,
    KernelAccessController, KernelCapability, KernelFastPacketEngine, KernelModule, KmdfDriver,
    KmdfIoRequest, KmdfPnpState, KmdfPowerState, LandlockAccessRight, LandlockPathRule,
    LinuxDevlinkHealthMonitor, LinuxFutexEngine, LinuxLandlockLsmRuleEngine,
    MemoryCompactionSuperpagesAllocator, MicrokernelCore, MicrokernelTranslatorRegistry,
    MultikernelMessage, MultikernelMessagePassing, NamespaceType, NanokernelHardwareBroker,
    NanokernelIrq, NetBsdRumpKernel, NinePProtocolTranslator, NinePResource,
    NixOsDeclarativeManager, NtExecutiveService, NullfsLayerNode, OpenBsdPledge,
    OpenBsdUnveilEngine, OpenSuseSnapperEngine, PfFiveTuple, PfStateEntry, PhysicalFrameBlock,
    ReactorEvent, ReactorRegistration, ResourceBinding, RumpComponent, SnapperSnapshot,
    SoftIrqType, SovereignCgroupGovernor, SovereignEventReactor, SovereignNamespaceContainer,
    SovereignSwapEngine, SovereignZone, SovereignZonesManager, SwapDeviceConfig, SwapPage,
    UnveilPathRule, VnetNetworkStack, VoidLinuxRunitSupervisor, VoidRunitInit, VoidRunitService,
    VoidRunitStage, XdpAction, ZramCompressedPage, CAP_MMAP_FLAG, CAP_READ_FLAG, CAP_SEEK_FLAG,
    CAP_WRITE_FLAG, PLEDGE_CPATH, PLEDGE_DPATH, PLEDGE_EXEC, PLEDGE_INET, PLEDGE_RPATH,
    PLEDGE_STDIO, PLEDGE_UNIX, PLEDGE_WPATH,
};
#[allow(ambiguous_glob_reexports)]
pub use architecture::*;
pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
};
pub use bus::*;
pub use gap_closing::{
    AcpiInterruptManager, GapError, IrqRoutingTable, JournalBlock, JournalState, MetadataJournal,
    Pml4PageTableEntry, VirtualMemoryPagingManager,
};
pub use generation_manager::{Generation, GenerationManager};
pub use io_uring::{CompletionQueueEntry, IoUringEngine, IoUringOpcode, SubmissionQueueEntry};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use linux_parity::*;
pub use memory::{
    BuddyAllocator, ContainerResourceGovernor, DmaRingBufferAllocator, HardenedGuardPageAllocator,
    MemoryBlock, PcieResourceAllocator, ResourceLimits, SigmaResourceAllocatorHub,
    SlabObjectCacheAllocator, SlabSizeClass, PAGE_SIZE,
};
pub use meta::{
    ABIManager, KernelGraph, KernelPersona, KernelPlugin, KernelPluginManager, LegacyScheduler,
    MetaKernel, MicroDriver, NetPod,
};
pub use nextgen_breakthroughs::*;
pub use paging::{PageTable, PageTableEntry, PageTableFlags, VirtualMemoryManagerV2};
pub use pipes::*;
pub use policy_mechanism::*;
pub use roundrobin::{
    RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError,
};
#[allow(ambiguous_glob_reexports)]
pub use structures::*;
pub use uts_namespace::{NamespaceId, UtsNamespaceManager};
pub use vmm_paging::{PageTableManager, VirtualMemoryManager};
// Note: linux_bsd_innovations types fully re-exported via `pub use crate::kernel::linux_bsd_innovations::*` above.
pub use kqueue_event::{FilterFlags, FilterType, Interest, Kevent, Kqueue, KqueueManager};
pub use tss_ring3_user_mode::{
    IretqStackFrame, SovereignRing3UserModeEngine, SovereignTaskStateSegment64,
    UserModeProcessContext,
};

// ─── Phase 1: Safe-Rust Kernel Foundation — New Sovereign Modules ─────────────
pub mod hardened_security_mitigations;
pub mod sigma_kernel_autotuner_v2;
pub mod sigma_version;
pub mod tss_ring3_user_mode;
pub mod xdp_engine_sovereign;

pub use hardened_security_mitigations::{
    CfiFunctionSignature, SovereignHardenedSecurityMitigationsEngine,
};

// ─── Live Migration Engine (CRIU / QEMU inspired) ─────────────────────────────
pub mod live_migration_engine;

pub mod sovereign_kernel_pr_gateway;
pub use sovereign_kernel_pr_gateway::*;

pub mod low_level_hardware;
pub use low_level_hardware::*;

pub mod kptr_restrict;
pub mod pidfd;
pub use pidfd::{
    PidFd, PidFdCapabilities, PidfdProcDescManager, ProcDesc, ProcDescCapabilities, SubreaperEntry,
};
pub mod cfi;
pub mod cfs_scheduler;
pub mod dma;
pub mod interrupt;
pub mod interrupt_controller;
pub use cfi::{CfiEngine, CfiTarget, CfiViolation};
pub use interrupt::{
    InterruptController, InterruptDescriptor, InterruptType, InterruptVector, IrqLine,
    IrqTriggerType,
};
pub use kptr_restrict::{
    get_security_mitigations, DmesgRestrictLevel, KernelSecurityMitigations, KptrRestrictLevel,
};
pub use scheduler::{
    CfsScheduler, Priority, ProcessState, ProcessTask, RtScheduler, SchedulerPolicy, ThermalState,
};

pub mod procfs_linux;
pub use procfs_linux::{ProcFs, ProcessInfo};
pub use sysfs_manager::{Sysfs, SysfsAttribute, SysfsKobject};

pub mod karl;

pub mod retguard;

pub mod stack_protect;
