// SigmaOS Kernel Module
// Following kernel-specific AGENTS.md guidelines for no_std compliance and safe Rust

// Core Kernel Modules
pub mod architecture;
pub mod atomic_extended;
pub mod console;
pub mod cpu_features;
pub mod device;
pub mod driver;
pub mod exports;
pub mod meta;
pub mod object;
pub mod structures;
pub mod traits;
pub mod virtual_cpu;

// Memory Management (Following no_std guidelines)
pub mod memory;
pub mod paging;
pub mod vmm_paging;
pub mod numa_allocator;
pub mod generation_manager;

// Process & Scheduling (Following lock hierarchy: Scheduler -> Memory Manager -> VFS -> Device/Driver)
pub mod process;
pub mod scheduler;
pub mod sched;
pub mod roundrobin;
pub mod bore;
pub mod numa_scheduler;

// System Call & IPC (Following privilege separation guidelines)
pub mod ipc;
pub mod pipes;
pub mod kqueue;
pub mod kqueue_event;
pub mod namespaces;
pub mod policy_mechanism;

// eBPF & Security (Following eBPF verification guidelines)
pub mod ebpf;
pub mod ebpf_vm;
pub mod ebpf_verification;

// Cgroup & Resource Management
pub mod cgroup_v2;
pub mod cgroup_controllers;

// Linux/BSD Compatibility Layers
pub mod linux_parity;
pub mod linux_bsd_innovations;
pub mod linux_absorb;
pub mod missing_linux_kernel_components;
pub mod dtrace_compat;

// Advanced Features
pub mod io_uring;
pub mod nextgen_breakthroughs;
pub mod breakthroughs;
pub mod breakthroughs_v2;
pub mod breakthrough;
pub mod gap_closing;
pub mod gap_filling;
pub mod os_innovations;
pub mod performance;
pub mod classic_os;
pub mod component;
pub mod module_loader;
pub mod net;
pub mod bus;
pub mod block_dev;
pub mod cpufreq;

// Sovereign Phase 1 Modules
pub mod sigma_version;
pub mod sigma_kernel_autotuner_v2;
pub mod xdp_engine_sovereign;
pub mod live_migration_engine;

// Re-exports (Following Interface Segregation Principle - specific exports rather than glob)
// Linux Parity Components
pub use linux_parity::{
    CmaRegion, KernelTimer, LinuxCmaAllocatorEngine, LinuxKernelTimerWheel,
    LinuxKernelWorkqueueEngine, LinuxRcuSynchronizationEngine, RcuCallback, WorkItem,
};

// Memory Management Components
pub use memory::{
    BuddyAllocator, ContainerResourceGovernor, DmaRingBufferAllocator, HardenedGuardPageAllocator,
    MemoryBlock, PcieResourceAllocator, ResourceLimits, SigmaResourceAllocatorHub,
    SlabObjectCacheAllocator, SlabSizeClass, PAGE_SIZE,
};

// IPC Components
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use pipes::*;

// Memory Management Components
pub use paging::{PageTable, PageTableEntry, PageTableFlags, VirtualMemoryManagerV2};
pub use vmm_paging::{PageTableManager, VirtualMemoryManager};
pub use generation_manager::{Generation, GenerationManager};

// Scheduler Components
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use roundrobin::{
    RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError,
};

// I/O Components
pub use io_uring::{CompletionQueueEntry, IoUringEngine, IoUringOpcode, SubmissionQueueEntry};

// Kqueue Components
pub use kqueue_event::{Kqueue, KqueueManager, Kevent, FilterType, FilterFlags, Interest};

// Breakthrough Components
pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
};

// Gap Closing Components
pub use gap_closing::{
    AcpiInterruptManager, GapError, IrqRoutingTable, JournalBlock, JournalState, MetadataJournal,
    Pml4PageTableEntry, VirtualMemoryPagingManager,
};

// Linux Kernel Components
pub use missing_linux_kernel_components::{
    BpfRingBufferStreamEngine, EpollCtlOp, EpollEvent, KernelAuditRecord, KernelAuditRecordType,
    KprobeEntry, LinuxEpollEventPollEngine, LinuxKernelAuditSubsystemEngine,
    LinuxKprobesTracepointEngine, LinuxMemoryCgroupV2OomKillerEngine,
    LinuxSeccompBpfSyscallFilterEngine, MemcgProcessEntry, SeccompAction, UffdFaultEvent, UffdMode,
    UffdRegisteredRange, UserfaultfdSubsystemEngine, VirtioBalloonDriverEngine,
};

// Meta Components
pub use meta::{
    ABIManager, KernelGraph, KernelPersona, KernelPlugin, KernelPluginManager, LegacyScheduler,
    MetaKernel, MicroDriver, NetPod,
};

// Note: Comprehensive linux_bsd_innovations exports are available through the module
// Consumers should use specific imports for better type safety and compilation efficiency
