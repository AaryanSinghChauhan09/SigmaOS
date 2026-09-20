#![allow(ambiguous_glob_reexports)]
// SigmaOS Kernel Module
pub mod architecture;
pub mod atomic_extended;
pub mod cgroup_v2;
pub mod kqueue_event;
pub mod cgroup_controllers;
pub mod block_dev;
pub mod bore;
pub mod breakthrough;
pub mod breakthroughs;
pub mod breakthroughs_v2;
pub mod bus;
pub mod classic_os;
pub mod component;
pub mod console;
pub mod missing_linux_kernel_components;
pub mod cpu_features;
pub mod cpufreq;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod drm;
pub mod dtrace_compat;
pub mod ebpf;
pub mod structures;
pub mod virtual_cpu;
pub mod vmm_paging;
pub mod ebpf_vm;
pub mod ebpf_verification;
pub mod exports;
pub mod gap_closing;
pub mod gap_filling;
pub mod generation_manager;
pub mod io_uring;
pub mod ipc;
pub mod kqueue;
pub mod livepatch;
pub mod lockdep;
pub mod cgroups;
pub mod kobject;
pub mod ftrace;
pub mod perf;
pub mod seccomp;
pub mod iommu;
pub mod interrupt;
pub mod panic_nvram;
pub mod linux_absorb;
pub mod ebpf_jit;
pub use ebpf_jit::*;
pub mod elf_loader;
pub use elf_loader::*;
pub mod hypervisor;
pub use hypervisor::*;
pub mod preempt_rt;
pub use preempt_rt::*;
pub mod drm_gem;
pub use drm_gem::*;
pub mod entry;
pub use entry::*;

pub mod linux_bsd_innovations;
pub mod linux_parity;
pub use linux_parity::{
    CmaRegion, KernelTimer, LinuxCmaAllocatorEngine, LinuxKernelTimerWheel,
    LinuxKernelWorkqueueEngine, LinuxRcuSynchronizationEngine, RcuCallback, WorkItem,
};
pub mod memory;
pub mod meta;
pub mod module_loader;
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
pub mod process;
pub mod policy_mechanism;
pub mod rcu;
pub mod roundrobin;
pub mod sched;
pub mod scheduler;
pub mod structures;
pub mod subsystems;
pub mod timer_subsystem;
pub mod virtual_cpu;
pub mod workqueue;

pub use missing_linux_kernel_components::{
    BpfRingBufferStreamEngine, EpollCtlOp, EpollEvent, KernelAuditRecord, KernelAuditRecordType,
    KprobeEntry, LinuxEpollEventPollEngine, LinuxKernelAuditSubsystemEngine,
    LinuxKprobesTracepointEngine, LinuxMemoryCgroupV2OomKillerEngine,
    LinuxSeccompBpfSyscallFilterEngine, MemcgProcessEntry, SeccompAction, UffdFaultEvent, UffdMode,
    UffdRegisteredRange, UserfaultfdSubsystemEngine, VirtioBalloonDriverEngine,
};
pub mod traits;
pub mod vmm_paging;

#[allow(ambiguous_glob_reexports)]
pub use architecture::*;
pub use bus::*;
pub use crate::kernel::linux_bsd_innovations::*;
pub use pipes::*;
pub use policy_mechanism::*;
#[allow(ambiguous_glob_reexports)]
pub use structures::*;
pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
};
pub use gap_closing::{
    AcpiInterruptManager, GapError, IrqRoutingTable, JournalBlock, JournalState, MetadataJournal,
    Pml4PageTableEntry, VirtualMemoryPagingManager,
};
pub use generation_manager::{Generation, GenerationManager};
pub use io_uring::{CompletionQueueEntry, IoUringEngine, IoUringOpcode, SubmissionQueueEntry};
pub use ipc::{Channel, IpcError, IpcManager, Message};
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
pub use roundrobin::{
    RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError,
};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use vmm_paging::{PageTableManager, VirtualMemoryManager};
// Note: linux_bsd_innovations types fully re-exported via `pub use crate::kernel::linux_bsd_innovations::*` above.
pub use kqueue_event::{Kqueue, KqueueManager, Kevent, FilterType, FilterFlags, Interest};

// ─── Phase 1: Safe-Rust Kernel Foundation — New Sovereign Modules ─────────────
pub mod sigma_version;
pub mod sigma_kernel_autotuner_v2;
pub mod xdp_engine_sovereign;

// ─── Live Migration Engine (CRIU / QEMU inspired) ─────────────────────────────
pub mod live_migration_engine;

// ─── Universal Modular Kernel System ──────────────────────────────────────────
pub mod universal_modular_system;
pub use universal_modular_system::{
    SovereignDriverManager, SovereignModularKernelEngine, SovereignNetworkStackManager,
    SovereignPeripheralAccessManager, SovereignProcessControlManager, SovereignVfsStorageManager,
};

// ─── Universal Kernel Format Engine (Linux & BSD Parity) ───────────────────────
pub mod universal_kernel_format;
pub use universal_kernel_format::{
    KernelArch, KernelCompression, KernelFormat, KernelFormatSymbol, KernelSection,
    ParsedKernelImage, SigmaKernelExecutionPayload, UniversalKernelFormatEngine,
};

// ─── Hardware Boot-to-Userspace Path Subsystem ──────────────────────────────
pub mod boot_to_userspace;
pub use boot_to_userspace::*;
