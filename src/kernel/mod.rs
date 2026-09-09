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
pub mod bsd_kernel_parity;
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
pub mod policy_mechanism;
pub mod roundrobin;
pub mod sched;
pub mod scheduler;
pub mod structures;
pub mod traits;

#[allow(ambiguous_glob_reexports)]
pub use architecture::*;
pub use bus::*;
pub use linux_bsd_innovations::*;
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

pub mod cgroups_v2_sovereign;
pub use cgroups_v2_sovereign::{SovereignCgroupsV2Manager, CgroupController, CgroupNode, CpuAccounting, MemoryAccounting, PidAccounting};
pub mod bsd_jails_sovereign;
pub use bsd_jails_sovereign::{SovereignBsdJailManager, SovereignJail, JailState, JailPermissions, JailNetworkConfig, JailProcess};

pub mod ftrace_sovereign;
pub use ftrace_sovereign::{SovereignFtracer, TraceEvent, TraceEventKind, TraceRingBuffer, TracerFilter, LatencyHistogram};
