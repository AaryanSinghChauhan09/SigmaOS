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
pub mod cpu_features;
pub mod cpufreq;
pub mod device;
pub mod driver;
pub mod dtrace_compat;
pub mod ebpf;
pub mod ebpf_vm;
pub mod ebpf_verification;
pub mod exports;
pub mod gap_closing;
pub mod gap_filling;
pub mod generation_manager;
pub mod io_uring;
pub mod ipc;
pub mod kqueue;
pub mod linux_absorb;
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
pub mod policy_mechanism;
pub mod roundrobin;
pub mod sched;
pub mod scheduler;
pub mod missing_linux_kernel_components;
pub mod structures;

pub use missing_linux_kernel_components::{
    BpfRingBufferStreamEngine, KernelAuditRecord, KernelAuditRecordType,
    LinuxKernelAuditSubsystemEngine, UffdFaultEvent, UffdMode, UffdRegisteredRange,
    UserfaultfdSubsystemEngine, VirtioBalloonDriverEngine,
};
pub mod traits;

#[allow(ambiguous_glob_reexports)]
pub use architecture::*;
pub use bus::*;
pub use linux_bsd_innovations::*;
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
pub use virtual_cpu::SovereignVirtualCPU as VirtualCpu;
pub use vmm_paging::{PageTableManager, VirtualMemoryManager};

pub mod sigma_kthread;
pub mod sigma_timer;
pub mod sigma_workqueue;
pub mod sigma_cgroup_v2;
pub mod sigma_signal;
pub mod missing_linux_kernel_components;
pub use missing_linux_kernel_components::*;

pub mod linux_kernel_parity_synthesis;
pub use linux_kernel_parity_synthesis::{
    DamonRegionNode, DmTargetDevice, DmTargetType, FutexSize, FutexWaitvEntry,
    LinuxDamonAccessMonitorEngine, LinuxDeviceMapperEngine, LinuxKernelFutex2WaitvEngine,
    LinuxPressureStallInfoEngine, PsiResourceKind, PsiStallMetrics,
    SovereignLinuxKernelParitySynthesisSuite,
};
